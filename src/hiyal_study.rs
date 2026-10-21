//! Hiyal-native parameter sweeps; results remain explicitly exploratory.
//! Trials are event-sourced, fully verified, and resumed without overwriting
//! existing arm histories. One society/seed is the statistical unit.
//!
//! Uniform controls ignore the feedback offset, so every (actors, initial,
//! horizon, seed) group has one parent and one control, and each offset cell
//! forks its own linear-feedback treatment from that same parent. Offset rows
//! within a group therefore share a control sample and are not independent.
use crate::{
    Allocate, Feedback, KIND, OpportunityPolicy, Society, gini,
    mobility::{mean_rank_movement, rank_persistence},
    reference,
    runner::{create_parent_staged, hiyal_revision, sync_dir, write_atomic},
    stats::{Pair, summarize},
    top_quintile_share,
};
use hiyal::{
    core::TrialId,
    experiment::Trial,
    research::{fork_at_head, verify_fork},
    runtime::ScriptedBackend,
    storage::{ReadOnly, Store},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as FmtWrite,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
    sync::Arc,
    time::Instant,
};
/// Schema 2: one shared control per group (schema 1 forked a control per cell).
const SCHEMA: u32 = 2;
const MAX_STEPS: u64 = 120_000;
/// Column layout of `hiyal-trajectories.csv`, shared with its readers.
pub const TRAJECTORY_HEADER: &str = "actors,initial,offset,horizon,seed,arm,step,gini,top_share,rank_persistence,mean_rank_movement";
/// The control world's offset field; unused under uniform feedback.
const CONTROL_OFFSET: u64 = 1;
const INTERVENTION: &str = "wealth-feedback-v1";
const BENCHMARK_HEADER: &str = "actors,initial,offset,horizon,seed,arm,start_step,new_accepted_steps,run_ms,verification_ms,journal_bytes,snapshot_bytes,state_json_bytes\n";

type ResearchTrial = Trial<Society, OpportunityPolicy>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub actors: usize,
    pub initial: u64,
    pub offset: u64,
    pub horizon: u64,
}
impl Cell {
    fn validate(&self) -> Result<(), String> {
        self.society()?;
        if !(16..=1_000).contains(&self.horizon) {
            return Err("horizon outside 16..=1000".into());
        }
        Ok(())
    }
    fn society(&self) -> Result<Society, String> {
        Society::new(self.actors, self.initial, self.offset, Feedback::Uniform)
    }
    fn identity(&self) -> String {
        format!(
            "actors-{}-initial-{}-offset-{}-steps-{}",
            self.actors, self.initial, self.offset, self.horizon
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyPlan {
    pub schema: u32,
    pub purpose: String,
    pub hiyal_revision: String,
    pub first_seed: u64,
    pub pairs: usize,
    pub cells: Vec<Cell>,
}
impl StudyPlan {
    pub fn small(first_seed: u64, pairs: usize) -> Result<Self, String> {
        Ok(Self {
            schema: SCHEMA,
            purpose: "exploratory_hiyal_smoke".into(),
            hiyal_revision: hiyal_revision()?.into(),
            first_seed,
            pairs,
            cells: vec![
                Cell {
                    actors: 10,
                    initial: 10,
                    offset: 1,
                    horizon: 30,
                },
                Cell {
                    actors: 20,
                    initial: 10,
                    offset: 10,
                    horizon: 50,
                },
            ],
        })
    }
    pub fn grid(first_seed: u64, pairs: usize) -> Result<Self, String> {
        let mut cells = Vec::new();
        for actors in [10, 20, 40] {
            for offset in [1, 10, 100] {
                for horizon in [100, 500] {
                    cells.push(Cell {
                        actors,
                        initial: 10,
                        offset,
                        horizon,
                    });
                }
            }
        }
        Ok(Self {
            schema: SCHEMA,
            purpose: "exploratory_hiyal_grid".into(),
            hiyal_revision: hiyal_revision()?.into(),
            first_seed,
            pairs,
            cells,
        })
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != SCHEMA
            || self.hiyal_revision != hiyal_revision()?
            || self.cells.is_empty()
            || self.cells.len() > 18
            || self.pairs == 0
            || self.pairs > 10
        {
            return Err("invalid study plan (max 10 pairs and 18 cells)".into());
        }
        let _ = self
            .first_seed
            .checked_add((self.pairs - 1) as u64)
            .ok_or("seed range overflow")?;
        let mut seen = BTreeSet::new();
        for c in &self.cells {
            c.validate()?;
            if !seen.insert(c.identity()) {
                return Err("duplicate cell".into());
            }
        }
        let mut total = 0u64;
        for (group, cells) in self.groups() {
            // One shared control plus one treatment per offset cell.
            let arms = 1 + cells.len() as u64;
            total = total
                .checked_add(group.horizon * arms * self.pairs as u64)
                .ok_or("study bound overflow")?;
        }
        if total > MAX_STEPS {
            return Err(format!(
                "at most {MAX_STEPS} total accepted events per study (controls + treatments)"
            ));
        }
        Ok(())
    }
    /// Cells grouped by shared control, in order of first appearance.
    fn groups(&self) -> Vec<(Group, Vec<&Cell>)> {
        let mut groups: Vec<(Group, Vec<&Cell>)> = Vec::new();
        for cell in &self.cells {
            let group = Group::of(cell);
            match groups.iter_mut().find(|(g, _)| *g == group) {
                Some((_, cells)) => cells.push(cell),
                None => groups.push((group, vec![cell])),
            }
        }
        groups
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Group {
    actors: usize,
    initial: u64,
    horizon: u64,
}
impl Group {
    fn of(cell: &Cell) -> Self {
        Self {
            actors: cell.actors,
            initial: cell.initial,
            horizon: cell.horizon,
        }
    }
    fn identity(&self) -> String {
        format!(
            "actors-{}-initial-{}-steps-{}",
            self.actors, self.initial, self.horizon
        )
    }
    /// Specification of the shared control arm.
    fn control(&self) -> Cell {
        Cell {
            actors: self.actors,
            initial: self.initial,
            offset: CONTROL_OFFSET,
            horizon: self.horizon,
        }
    }
}
struct Trajectory {
    step: u64,
    gini: f64,
    top_share: f64,
    rank_persistence: Option<f64>,
    mean_rank_movement: f64,
}

fn enforce_plan(root: &Path, plan: &StudyPlan) -> Result<(), String> {
    plan.validate()?;
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let file = root.join("study-plan.json");
    if file.exists() {
        let prior: StudyPlan = serde_json::from_slice(&fs::read(&file).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if &prior != plan {
            return Err("existing study directory belongs to a different plan".into());
        }
    } else {
        // A crash while writing the plan leaves only its temporary file, and
        // no evidence is ever created before the plan is durable.
        let temporary = file.with_extension("tmp");
        if temporary.exists() {
            fs::remove_file(&temporary).map_err(|e| e.to_string())?;
        }
        // Creation is exclusive. Refuse to adopt pre-existing unplanned evidence.
        if fs::read_dir(root)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
        {
            return Err("study output exists without a study-plan.json".into());
        }
        write_atomic(
            &file,
            &serde_json::to_vec_pretty(plan).map_err(|e| e.to_string())?,
        )?;
    }
    Ok(())
}

fn parent_name(group: &Group, seed: u64) -> String {
    format!("inequality-study-{}-{}", group.identity(), seed)
}

/// Returns the parent's trial ID and chain head, which every arm of this
/// group and seed must descend from.
fn create_or_check_parent(
    parent: &Path,
    group: &Group,
    seed: u64,
) -> Result<(String, String), String> {
    let world = group.control().society()?;
    if !parent.exists() {
        create_parent_staged(parent, &parent_name(group, seed), seed, world.clone())?;
    }
    let ro = Store::<Society, Allocate>::inspect_read_only(parent)?;
    if ro.genesis.trial_id != TrialId::new(&parent_name(group, seed), seed).0
        || ro.genesis.world_kind != KIND
        || ro.logical_step != 0
        || ro.world != world
    {
        return Err("existing parent does not match intended group and seed".into());
    }
    Ok((ro.genesis.trial_id, ro.chain_head))
}

fn treatment_parameters(cell: &Cell) -> serde_json::Value {
    serde_json::json!({"feedback":"linear","offset":cell.offset,"actors":cell.actors,"horizon":cell.horizon})
}

/// Forks are deterministic in the parent head, arm and parameters, so a
/// missing arm (for example after a kill between forks) is simply forked.
fn fork_control(
    parent: &Path,
    dir: &Path,
    backend: Arc<ScriptedBackend>,
) -> Result<ResearchTrial, String> {
    fork_at_head(
        parent,
        dir,
        INTERVENTION,
        "control",
        serde_json::json!({}),
        |_world: &mut Society| Ok(()),
        OpportunityPolicy,
        backend,
    )
}

fn fork_treatment(
    parent: &Path,
    dir: &Path,
    cell: &Cell,
    backend: Arc<ScriptedBackend>,
) -> Result<ResearchTrial, String> {
    let offset = cell.offset;
    fork_at_head(
        parent,
        dir,
        INTERVENTION,
        "treatment",
        treatment_parameters(cell),
        move |world: &mut Society| {
            world.feedback = Feedback::Linear;
            world.offset = offset;
            Ok(())
        },
        OpportunityPolicy,
        backend,
    )
}

fn check_world(world: &Society, spec: &Cell, feedback: Feedback) -> Result<(), String> {
    if world.feedback != feedback
        || world.wealth.len() != spec.actors
        || world.offset != spec.offset
    {
        return Err("existing fork world differs from expected cell".into());
    }
    Ok(())
}

/// Ties an existing arm to its intervention and to this group's and seed's
/// parent, so arms copied from another seed or cell are refused.
fn check_attribution(
    dir: &Path,
    arm: &str,
    parameters: &serde_json::Value,
    parent: &(String, String),
) -> Result<(), String> {
    let meta = verify_fork::<Society>(dir)?;
    if meta.intervention != INTERVENTION
        || meta.arm != arm
        || meta.parameters != *parameters
        || meta.parent_id != parent.0
        || meta.parent_head != parent.1
        || meta.parent_logical_step != 0
    {
        return Err("existing fork attribution differs from study plan, seed or parent".into());
    }
    Ok(())
}

/// Runs the remaining steps and returns (start step, new steps, run ms).
/// Invariants are checked before checkpointing so a violating state never
/// becomes the trusted snapshot.
async fn complete_arm(trial: &mut ResearchTrial, spec: &Cell) -> Result<(u64, u64, u128), String> {
    let completed = trial.store.logical_step();
    if completed > spec.horizon {
        return Err("run progressed beyond intended horizon".into());
    }
    let remaining = spec.horizon - completed;
    let start = Instant::now();
    if trial.run_attempts(remaining).await? != remaining {
        return Err("not all opportunities were accepted".into());
    }
    trial.store.world().verify_accounting(spec.initial)?;
    if trial.store.world().opportunities != spec.horizon {
        return Err("opportunity counter differs from committed steps".into());
    }
    trial.checkpoint()?;
    Ok((completed, remaining, start.elapsed().as_millis()))
}

/// Hiyal verifies the ledger (`ro`); a *separate* vector reducer validates the
/// terminal model state and, in the same pass, derives all trajectories.
fn verified_outcomes(
    dir: &Path,
    ro: &ReadOnly<Society>,
    spec: &Cell,
) -> Result<(Vec<Trajectory>, f64), String> {
    if ro.logical_step != spec.horizon
        || ro.world.opportunities != spec.horizon
        || ro.world.wealth.len() != spec.actors
        || ro.world.offset != spec.offset
    {
        return Err("terminal world/size/horizon mismatch".into());
    }
    ro.world.verify_accounting(spec.initial)?;
    let early = (spec.horizon / 4).max(1);
    let checkpoints: BTreeSet<u64> = [early, (spec.horizon / 2).max(1), spec.horizon]
        .into_iter()
        .collect();
    let mut rows = Vec::new();
    let mut early_balances: Option<Vec<u64>> = None;
    let verified = reference::replay_with(
        &dir.join("events.jsonl"),
        &ro.genesis.initial_world.wealth,
        ro.log_bytes,
        |observed, state| {
            if observed == early {
                early_balances = Some(state.to_vec());
            }
            if checkpoints.contains(&observed) {
                let prior = early_balances
                    .as_ref()
                    .ok_or("missing early ranking baseline")?;
                rows.push(Trajectory {
                    step: observed,
                    gini: gini(state)?,
                    top_share: top_quintile_share(state)?,
                    rank_persistence: rank_persistence(prior, state)?,
                    mean_rank_movement: mean_rank_movement(prior, state)?,
                });
            }
            Ok(())
        },
    )?;
    if verified.opportunities != ro.logical_step
        || verified.wealth != ro.world.wealth
        || rows.len() != checkpoints.len()
    {
        return Err("independent reference replay differs from Hiyal".into());
    }
    Ok((rows, gini(&ro.world.wealth)?))
}

/// One run segment's cost: (start step, new steps, run ms, verification ms,
/// journal bytes, snapshot bytes, serialized state bytes).
type Segment = (u64, u64, u128, u128, u64, u64, usize);

/// Brings one arm to its horizon (forking it if missing) and verifies it.
/// Complete arms are only inspected (once), never reopened for writing.
async fn process_arm(
    dir: &Path,
    spec: &Cell,
    feedback: Feedback,
    attribution: (&str, &serde_json::Value, &(String, String)),
    backend: &Arc<ScriptedBackend>,
    fork: impl FnOnce() -> Result<ResearchTrial, String>,
) -> Result<(Vec<Trajectory>, f64, Option<Segment>), String> {
    let mut complete = None;
    let trial = if dir.exists() {
        check_attribution(dir, attribution.0, attribution.1, attribution.2)?;
        let ro = Store::<Society, Allocate>::inspect_read_only(dir)?;
        check_world(&ro.world, spec, feedback)?;
        if ro.logical_step < spec.horizon {
            Some(Trial::resume(
                dir,
                KIND,
                OpportunityPolicy,
                backend.clone(),
                false,
            )?)
        } else {
            complete = Some(ro);
            None
        }
    } else {
        Some(fork()?)
    };
    let run = match trial {
        Some(mut trial) => {
            check_world(trial.store.world(), spec, feedback)?;
            let run = complete_arm(&mut trial, spec).await?;
            drop(trial);
            Some(run)
        }
        None => None,
    };
    let started = Instant::now();
    let ro = match complete {
        Some(ro) => ro,
        None => Store::<Society, Allocate>::inspect_read_only(dir)?,
    };
    check_world(&ro.world, spec, feedback)?;
    let (rows, terminal_gini) = verified_outcomes(dir, &ro, spec)?;
    let verification_ms = started.elapsed().as_millis();
    let segment = match run {
        Some((start_step, new_steps, run_ms)) => Some((
            start_step,
            new_steps,
            run_ms,
            verification_ms,
            ro.log_bytes,
            fs::metadata(dir.join("snapshot.json"))
                .map_err(|e| e.to_string())?
                .len(),
            serde_json::to_vec(&ro.world)
                .map_err(|e| e.to_string())?
                .len(),
        )),
        None => None,
    };
    Ok((rows, terminal_gini, segment))
}

/// Appends one durable benchmark row per run segment, so timings survive
/// interruption and are never overwritten by a later invocation. A torn final
/// row from a crash mid-append is dropped first.
fn append_benchmark(path: &Path, row: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    let mut existing = Vec::new();
    file.read_to_end(&mut existing).map_err(|e| e.to_string())?;
    if !existing.is_empty() && !existing.ends_with(b"\n") {
        let keep = existing
            .iter()
            .rposition(|b| *b == b'\n')
            .map_or(0, |i| i + 1);
        file.set_len(keep as u64).map_err(|e| e.to_string())?;
        existing.truncate(keep);
    }
    if existing.is_empty() {
        file.write_all(BENCHMARK_HEADER.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    file.write_all(row.as_bytes()).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    sync_dir(path.parent().ok_or("benchmark path has no parent")?)
}

fn csv_option(value: Option<f64>) -> String {
    value.map(|v| format!("{v:.12}")).unwrap_or_default()
}

/// Writes one arm's trajectory rows and, if it ran, its benchmark row. The
/// shared control has no offset, so its offset column is empty.
fn record_arm(
    root: &Path,
    trajectories: &mut String,
    group: &Group,
    offset: Option<u64>,
    seed: u64,
    arm: &str,
    rows: &[Trajectory],
    segment: Option<Segment>,
) -> Result<(), String> {
    let offset = offset.map(|o| o.to_string()).unwrap_or_default();
    let label = format!(
        "{},{},{},{},{},{}",
        group.actors, group.initial, offset, group.horizon, seed, arm
    );
    for x in rows {
        writeln!(
            trajectories,
            "{label},{},{:.12},{:.12},{},{:.12}",
            x.step,
            x.gini,
            x.top_share,
            csv_option(x.rank_persistence),
            x.mean_rank_movement
        )
        .map_err(|e| e.to_string())?;
    }
    if let Some((start, steps, run_ms, verify_ms, journal, snapshot, state)) = segment {
        append_benchmark(
            &root.join("hiyal-benchmarks.csv"),
            &format!("{label},{start},{steps},{run_ms},{verify_ms},{journal},{snapshot},{state}\n"),
        )?;
    }
    Ok(())
}

pub async fn run(root: &Path, plan: StudyPlan) -> Result<(), String> {
    enforce_plan(root, &plan)?;
    let backend = Arc::new(ScriptedBackend);
    let mut trajectories = format!("{TRAJECTORY_HEADER}\n");
    let mut pairs_by_cell: BTreeMap<String, Vec<Pair>> = BTreeMap::new();
    for (group, cells) in plan.groups() {
        let control_spec = group.control();
        for i in 0..plan.pairs {
            let seed = plan.first_seed + i as u64;
            let seed_root = root.join(group.identity()).join(format!("seed-{seed:020}"));
            fs::create_dir_all(&seed_root).map_err(|e| e.to_string())?;
            let parent = seed_root.join("parent");
            let parent_ref = create_or_check_parent(&parent, &group, seed)?;
            let arms = seed_root.join("arms");
            let control_dir = arms.join("control");
            let (rows, control_gini, segment) = process_arm(
                &control_dir,
                &control_spec,
                Feedback::Uniform,
                ("control", &serde_json::json!({}), &parent_ref),
                &backend,
                || fork_control(&parent, &control_dir, backend.clone()),
            )
            .await?;
            record_arm(
                root,
                &mut trajectories,
                &group,
                None,
                seed,
                "control",
                &rows,
                segment,
            )?;
            for cell in &cells {
                let dir = arms.join(format!("treatment-offset-{}", cell.offset));
                let params = treatment_parameters(cell);
                let (rows, treatment_gini, segment) = process_arm(
                    &dir,
                    cell,
                    Feedback::Linear,
                    ("treatment", &params, &parent_ref),
                    &backend,
                    || fork_treatment(&parent, &dir, cell, backend.clone()),
                )
                .await?;
                record_arm(
                    root,
                    &mut trajectories,
                    &group,
                    Some(cell.offset),
                    seed,
                    "treatment",
                    &rows,
                    segment,
                )?;
                pairs_by_cell
                    .entry(cell.identity())
                    .or_default()
                    .push(Pair {
                        seed,
                        horizon: cell.horizon,
                        control: control_gini,
                        treatment: treatment_gini,
                    });
            }
        }
    }
    // Derived CSVs are regenerated from verified histories on every completed
    // invocation; the benchmark log above is append-only.
    write_atomic(
        &root.join("hiyal-trajectories.csv"),
        trajectories.as_bytes(),
    )?;
    let mut summary = String::from(
        "actors,initial,offset,horizon,pairs,control_mean_gini,treatment_mean_gini,paired_delta,bootstrap_95_lo,bootstrap_95_hi\n",
    );
    for cell in &plan.cells {
        let paired = pairs_by_cell
            .get(&cell.identity())
            .ok_or("cell has no verified pairs")?;
        let stat = summarize(paired, 2_000, 0x51_2026_001)?;
        let bounds = stat.bootstrap_percentile_95;
        writeln!(
            summary,
            "{},{},{},{},{},{:.12},{:.12},{:.12},{},{}",
            cell.actors,
            cell.initial,
            cell.offset,
            cell.horizon,
            paired.len(),
            stat.mean_control,
            stat.mean_treatment,
            stat.mean_delta,
            csv_option(bounds.map(|b| b[0])),
            csv_option(bounds.map(|b| b[1]))
        )
        .map_err(|e| e.to_string())?;
    }
    write_atomic(&root.join("hiyal-summary.csv"), summary.as_bytes())?;
    let plan_hash = hiyal::storage::digest(&plan)?;
    let actors = plan
        .cells
        .iter()
        .map(|c| c.actors)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("/");
    let report = serde_json::json!({
        "status":"exploratory-only",
        "study_plan_digest":plan_hash,
        "model":format!("{actors}-agent integer wealth allocation; Hiyal-ChaCha20"),
        "hiyal_git_revision":plan.hiyal_revision,
        "cells":plan.cells.len(),"pairs_per_cell":plan.pairs,
        "scientific_unit":"one independently seeded society pair",
        "control_sharing":"one uniform control per (actors, initial wealth, horizon, seed), shared by every offset cell in that group; offset rows within a group are not independent",
        "uncertainty":"paired percentile bootstrap 95%, 2000 draws, omitted for n<10",
        "benchmark_scope":"append-only log, one row per run segment: wall-clock for newly accepted steps + checkpoint, then full ledger and reference verification; on shared machine; no measured peak process RSS",
        "reporting":"all configured cells, null and negative effects retained",
    });
    write_atomic(
        &root.join("hiyal-study-report.json"),
        &serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn test_root(label: &str) -> std::path::PathBuf {
        let idx = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "social-hiyal-grid-{label}-{}-{idx}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        root
    }

    fn derived(root: &Path) -> (Vec<u8>, Vec<u8>) {
        (
            fs::read(root.join("hiyal-trajectories.csv")).unwrap(),
            fs::read(root.join("hiyal-summary.csv")).unwrap(),
        )
    }

    fn seed_dir(root: &Path, cell: &Cell, seed: u64) -> std::path::PathBuf {
        root.join(Group::of(cell).identity())
            .join(format!("seed-{seed:020}"))
    }

    fn copy_dir(from: &Path, to: &Path) {
        fs::create_dir_all(to).unwrap();
        for entry in fs::read_dir(from).unwrap() {
            let entry = entry.unwrap();
            let target = to.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_dir(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).unwrap();
            }
        }
    }

    #[test]
    fn plans_are_bounded_and_distinct() {
        let a = StudyPlan::small(101, 2).unwrap();
        a.validate().unwrap();
        let b = StudyPlan::grid(101, 10).unwrap();
        b.validate().unwrap();
        assert_eq!(b.cells.len(), 18);
        assert_eq!(b.groups().len(), 6);
        assert!(b.groups().iter().all(|(_, cells)| cells.len() == 3));
        assert!(StudyPlan::grid(101, 11).unwrap().validate().is_err());
        assert!(StudyPlan::grid(u64::MAX, 2).unwrap().validate().is_err());
        let mut stale = StudyPlan::small(101, 2).unwrap();
        stale.hiyal_revision = "0000000000000000000000000000000000000000".into();
        assert!(stale.validate().is_err());
        let mut old_schema = StudyPlan::small(101, 2).unwrap();
        old_schema.schema = 1;
        assert!(old_schema.validate().is_err());
    }

    #[tokio::test]
    async fn study_is_idempotent_and_rejects_changed_plan() {
        let root = test_root("idempotent");
        let plan = StudyPlan::small(2026, 1).unwrap();
        run(&root, plan.clone()).await.unwrap();
        let first = derived(&root);
        let benchmarks = fs::read(root.join("hiyal-benchmarks.csv")).unwrap();
        assert_eq!(String::from_utf8_lossy(&benchmarks).lines().count(), 1 + 4);
        let snapshot = seed_dir(&root, &plan.cells[0], 2026).join("arms/control/snapshot.json");
        let modified = fs::metadata(&snapshot).unwrap().modified().unwrap();
        run(&root, plan.clone()).await.unwrap();
        assert_eq!(first, derived(&root));
        // A completed rerun adds no benchmark rows and does not rewrite evidence.
        assert_eq!(
            benchmarks,
            fs::read(root.join("hiyal-benchmarks.csv")).unwrap()
        );
        assert_eq!(
            modified,
            fs::metadata(&snapshot).unwrap().modified().unwrap()
        );
        let report = fs::read_to_string(root.join("hiyal-study-report.json")).unwrap();
        assert!(report.contains("10/20-agent"), "{report}");
        assert!(
            run(&root, StudyPlan::small(2027, 1).unwrap())
                .await
                .is_err()
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn offset_cells_share_one_control() {
        let root = test_root("shared-control");
        let cell = |offset| Cell {
            actors: 10,
            initial: 10,
            offset,
            horizon: 20,
        };
        let plan = StudyPlan {
            schema: SCHEMA,
            purpose: "test".into(),
            hiyal_revision: hiyal_revision().unwrap().into(),
            first_seed: 5050,
            pairs: 1,
            cells: vec![cell(1), cell(10)],
        };
        run(&root, plan.clone()).await.unwrap();
        let mut arms: Vec<_> = fs::read_dir(seed_dir(&root, &plan.cells[0], 5050).join("arms"))
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .filter(|name| !name.starts_with('.'))
            .collect();
        arms.sort();
        assert_eq!(
            arms,
            ["control", "treatment-offset-1", "treatment-offset-10"]
        );
        let trajectories = fs::read_to_string(root.join("hiyal-trajectories.csv")).unwrap();
        let control_rows = trajectories
            .lines()
            .filter(|l| l.starts_with("10,10,,20,5050,control,"))
            .count();
        assert_eq!(control_rows, 3);
        let summary = fs::read_to_string(root.join("hiyal-summary.csv")).unwrap();
        let controls: Vec<_> = summary
            .lines()
            .skip(1)
            .map(|l| l.split(',').nth(5).unwrap().to_owned())
            .collect();
        assert_eq!(controls.len(), 2);
        assert_eq!(controls[0], controls[1]);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn interrupted_runs_resume_to_identical_results() {
        let clean = test_root("clean");
        let plan = StudyPlan::small(3030, 1).unwrap();
        run(&clean, plan.clone()).await.unwrap();
        let backend = Arc::new(ScriptedBackend);
        let cell = &plan.cells[0];
        let group = Group::of(cell);

        // Control forked but treatment missing, as after a kill between forks.
        let missing = test_root("missing-arm");
        enforce_plan(&missing, &plan).unwrap();
        let seed = seed_dir(&missing, cell, 3030);
        fs::create_dir_all(&seed).unwrap();
        create_or_check_parent(&seed.join("parent"), &group, 3030).unwrap();
        fork_control(
            &seed.join("parent"),
            &seed.join("arms/control"),
            backend.clone(),
        )
        .unwrap();
        run(&missing, plan.clone()).await.unwrap();
        assert_eq!(derived(&clean), derived(&missing));

        // Arm stopped part-way through its horizon.
        let partial = test_root("partial-arm");
        enforce_plan(&partial, &plan).unwrap();
        let seed = seed_dir(&partial, cell, 3030);
        fs::create_dir_all(&seed).unwrap();
        create_or_check_parent(&seed.join("parent"), &group, 3030).unwrap();
        let mut control =
            fork_control(&seed.join("parent"), &seed.join("arms/control"), backend).unwrap();
        assert_eq!(control.run_attempts(5).await.unwrap(), 5);
        control.checkpoint().unwrap();
        drop(control);
        run(&partial, plan.clone()).await.unwrap();
        assert_eq!(derived(&clean), derived(&partial));
        let log = fs::read_to_string(partial.join("hiyal-benchmarks.csv")).unwrap();
        let resumed = log
            .lines()
            .find(|l| l.starts_with("10,10,,30,3030,control,"))
            .unwrap();
        assert!(
            resumed.starts_with("10,10,,30,3030,control,5,25,"),
            "{resumed}"
        );

        // Leftover plan temporary file and parent staging directory.
        let staged = test_root("staged");
        fs::create_dir_all(&staged).unwrap();
        fs::write(staged.join("study-plan.tmp"), b"partial").unwrap();
        enforce_plan(&staged, &plan).unwrap();
        let seed = seed_dir(&staged, cell, 3030);
        let stage = seed.join(format!(".parent.pending-{}", std::process::id()));
        fs::create_dir_all(&stage).unwrap();
        fs::write(stage.join("manifest.json"), b"partial").unwrap();
        run(&staged, plan).await.unwrap();
        assert!(!stage.exists());
        assert_eq!(derived(&clean), derived(&staged));

        for root in [clean, missing, partial, staged] {
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[tokio::test]
    async fn arms_copied_from_another_seed_are_refused() {
        let root = test_root("copied");
        let plan = StudyPlan::small(4040, 2).unwrap();
        run(&root, plan.clone()).await.unwrap();
        let cell = &plan.cells[0];
        let target = seed_dir(&root, cell, 4041).join("arms");
        fs::remove_dir_all(&target).unwrap();
        copy_dir(&seed_dir(&root, cell, 4040).join("arms"), &target);
        let error = run(&root, plan).await.unwrap_err();
        assert!(error.contains("attribution"), "{error}");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn benchmark_log_drops_torn_row_and_appends() {
        let root = test_root("benchmark-log");
        fs::create_dir_all(&root).unwrap();
        let log = root.join("hiyal-benchmarks.csv");
        append_benchmark(&log, "a\n").unwrap();
        let mut file = OpenOptions::new().append(true).open(&log).unwrap();
        file.write_all(b"torn").unwrap();
        drop(file);
        append_benchmark(&log, "b\n").unwrap();
        assert_eq!(
            fs::read_to_string(&log).unwrap(),
            format!("{BENCHMARK_HEADER}a\nb\n")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
