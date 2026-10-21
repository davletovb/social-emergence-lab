//! Hiyal-native parameter sweeps; results remain explicitly exploratory.
//! Trials are event-sourced, fully verified, and resumed without overwriting
//! existing arm histories. One society/seed is the statistical unit.
use crate::{
    Allocate, Feedback, OpportunityPolicy, Society, gini,
    mobility::{mean_rank_movement, rank_persistence},
    reference,
    stats::{Pair, summarize},
    top_quintile_share,
};
use hiyal::{
    core::{AgentId, TrialId},
    experiment::Trial,
    research::{paired_forks, resume_fork, verify_fork},
    runtime::ScriptedBackend,
    scheduler::Scheduling,
    storage::Store,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write as FmtWrite,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    sync::Arc,
    time::Instant,
};
const KIND: &str = "social-emergence-lab.inequality.v1";
const HIYAL_REV: &str = "c6f3d5d650457077eaff4e26a0060bc6cee5dbdd";
const MAX_STEPS: u64 = 120_000;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub actors: usize,
    pub initial: u64,
    pub offset: u64,
    pub horizon: u64,
}
impl Cell {
    fn validate(&self) -> Result<(), String> {
        Society::new(self.actors, self.initial, self.offset, Feedback::Uniform)?;
        if !(16..=1_000).contains(&self.horizon) {
            return Err("horizon outside 16..=1000".into());
        }
        Ok(())
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
    pub fn small(first_seed: u64, pairs: usize) -> Self {
        Self {
            schema: 1,
            purpose: "exploratory_hiyal_smoke".into(),
            hiyal_revision: HIYAL_REV.into(),
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
        }
    }
    pub fn grid(first_seed: u64, pairs: usize) -> Self {
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
        Self {
            schema: 1,
            purpose: "exploratory_hiyal_grid".into(),
            hiyal_revision: HIYAL_REV.into(),
            first_seed,
            pairs,
            cells,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != 1
            || self.hiyal_revision != HIYAL_REV
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
        let mut seen = std::collections::BTreeSet::new();
        let mut total = 0u64;
        for c in &self.cells {
            c.validate()?;
            if !seen.insert(c.identity()) {
                return Err("duplicate cell".into());
            }
            total = total
                .checked_add(c.horizon * 2 * self.pairs as u64)
                .ok_or("study bound overflow")?;
        }
        if total > MAX_STEPS {
            return Err(format!(
                "at most {MAX_STEPS} total accepted events per study (both arms)"
            ));
        }
        Ok(())
    }
}
#[derive(Debug, Serialize)]
struct Trajectory {
    actors: usize,
    offset: u64,
    horizon: u64,
    seed: u64,
    arm: String,
    step: u64,
    gini: f64,
    top_share: f64,
    rank_persistence: Option<f64>,
    mean_rank_movement: Option<f64>,
}
#[derive(Debug, Serialize)]
struct RunBenchmark {
    actors: usize,
    offset: u64,
    horizon: u64,
    seed: u64,
    arm: String,
    new_accepted_steps: u64,
    run_ms: u128,
    verification_ms: u128,
    journal_bytes: u64,
    snapshot_bytes: u64,
    state_json_bytes: usize,
}
#[derive(Debug, Serialize)]
struct CellSummary {
    actors: usize,
    offset: u64,
    horizon: u64,
    pairs: usize,
    control_mean_gini: f64,
    treatment_mean_gini: f64,
    mean_delta: f64,
    bootstrap_lo: Option<f64>,
    bootstrap_hi: Option<f64>,
}

fn save_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temporary = path.with_extension("tmp");
    let mut f = File::create(&temporary).map_err(|e| e.to_string())?;
    f.write_all(bytes).map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())?;
    drop(f);
    fs::rename(&temporary, path).map_err(|e| e.to_string())?;
    Ok(())
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
        // Creation is exclusive. Refuse to adopt pre-existing unplanned evidence.
        if fs::read_dir(root)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
        {
            return Err("study output exists without a study-plan.json".into());
        }
        save_atomic(
            &file,
            &serde_json::to_vec_pretty(plan).map_err(|e| e.to_string())?,
        )?;
    }
    Ok(())
}

fn initial_trial(parent: &Path, cell: &Cell, seed: u64) -> Result<(), String> {
    let name = format!("inequality-study-{}-{}", cell.identity(), seed);
    let trial = Trial::create(
        parent,
        &name,
        seed,
        KIND,
        Society::new(cell.actors, cell.initial, cell.offset, Feedback::Uniform)?,
        OpportunityPolicy,
        Arc::new(ScriptedBackend),
        (0..cell.actors).map(|id| AgentId(id as u64)).collect(),
        Scheduling::RoundRobin,
    )?;
    trial.checkpoint()?;
    Ok(())
}
fn create_or_check_parent(parent: &Path, cell: &Cell, seed: u64) -> Result<(), String> {
    if !parent.exists() {
        initial_trial(parent, cell, seed)?;
    }
    let ro = Store::<Society, Allocate>::inspect_read_only(parent)?;
    if ro.genesis.trial_id
        != TrialId::new(
            &format!("inequality-study-{}-{}", cell.identity(), seed),
            seed,
        )
        .0
        || ro.genesis.world_kind != KIND
        || ro.logical_step != 0
        || ro.world != Society::new(cell.actors, cell.initial, cell.offset, Feedback::Uniform)?
    {
        return Err("existing parent does not match intended cell and seed".into());
    }
    Ok(())
}
fn pair(
    parent: &Path,
    arms: &Path,
    cell: &Cell,
) -> Result<
    (
        Trial<Society, OpportunityPolicy>,
        Trial<Society, OpportunityPolicy>,
    ),
    String,
> {
    let c = arms.join("control");
    let t = arms.join("treatment");
    let backend = Arc::new(ScriptedBackend);
    let params = serde_json::json!({"feedback":"linear","offset":cell.offset,"actors":cell.actors,"horizon":cell.horizon});
    let result = match (c.exists(), t.exists()) {
        (false, false) => paired_forks(
            parent,
            arms,
            "wealth-feedback-v1",
            params.clone(),
            || OpportunityPolicy,
            |world: &mut Society| {
                world.feedback = Feedback::Linear;
                Ok(())
            },
            backend,
        )?,
        (true, true) => {
            let ca = verify_fork::<Society>(&c)?;
            let ta = verify_fork::<Society>(&t)?;
            if ca.intervention != "wealth-feedback-v1"
                || ta.intervention != "wealth-feedback-v1"
                || ca.arm != "control"
                || ta.arm != "treatment"
                || ca.parameters != serde_json::json!({})
                || ta.parameters != params
                || ca.parent_id != ta.parent_id
                || ca.parent_head != ta.parent_head
            {
                return Err("existing paired fork attribution differs from study plan".into());
            }
            (
                resume_fork(&c, OpportunityPolicy, backend.clone(), false)?,
                resume_fork(&t, OpportunityPolicy, backend, false)?,
            )
        }
        _ => return Err("only one fork exists: refusing to discard or infer data".into()),
    };
    if result.0.store.world().feedback != Feedback::Uniform
        || result.1.store.world().feedback != Feedback::Linear
        || result.0.store.world().wealth.len() != cell.actors
        || result.1.store.world().wealth.len() != cell.actors
        || result.0.store.world().offset != cell.offset
        || result.1.store.world().offset != cell.offset
    {
        return Err("existing fork world differs from expected cell".into());
    }
    Ok(result)
}

async fn complete_arm(
    trial: &mut Trial<Society, OpportunityPolicy>,
    cell: &Cell,
) -> Result<(u64, u128), String> {
    let completed = trial.store.logical_step();
    if completed > cell.horizon {
        return Err("run progressed beyond intended horizon".into());
    }
    let remaining = cell.horizon - completed;
    let start = Instant::now();
    if trial.run_attempts(remaining).await? != remaining {
        return Err("not all opportunities were accepted".into());
    }
    trial.store.world().verify_accounting(cell.initial)?;
    trial.checkpoint()?;
    Ok((remaining, start.elapsed().as_millis()))
}

/// Hiyal verifies the ledger and ancestry; a *separate* vector reducer
/// validates the terminal model state. A journal scan derives all trajectories,
/// including earlier checkpoints after a previously interrupted run.
fn verified_outcomes(
    dir: &Path,
    cell: &Cell,
    seed: u64,
    arm: &str,
    new_steps: u64,
    run_ms: u128,
) -> Result<(Vec<Trajectory>, RunBenchmark, f64), String> {
    let started = Instant::now();
    verify_fork::<Society>(dir)?;
    let ro = Store::<Society, Allocate>::inspect_read_only(dir)?;
    if ro.logical_step != cell.horizon
        || ro.world.opportunities != cell.horizon
        || ro.world.wealth.len() != cell.actors
        || ro.world.offset != cell.offset
    {
        return Err("terminal world/size/horizon mismatch".into());
    }
    ro.world.verify_accounting(cell.initial)?;
    let verified = reference::replay(
        &dir.join("events.jsonl"),
        &ro.genesis.initial_world.wealth,
        ro.log_bytes,
    )?;
    if verified.opportunities != ro.logical_step || verified.wealth != ro.world.wealth {
        return Err("independent reference replay differs from Hiyal".into());
    }
    let early = (cell.horizon / 4).max(1);
    let checkpoints: std::collections::BTreeSet<u64> =
        [early, (cell.horizon / 2).max(1), cell.horizon]
            .into_iter()
            .collect();
    let mut state = ro.genesis.initial_world.wealth.clone();
    let mut rows = Vec::new();
    let mut observed = 0u64;
    let mut early_balances = None;
    let file = File::open(dir.join("events.jsonl")).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file.take(ro.log_bytes));
    for record in reader.lines() {
        let record = record.map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(&record).map_err(|e| e.to_string())?;
        let outcome = value.get("outcome").ok_or("event has no outcome")?;
        match outcome
            .get("outcome")
            .and_then(|x| x.as_str())
            .ok_or("event has no kind")?
        {
            "accepted" => {
                let recipient = outcome
                    .get("action")
                    .and_then(|x| x.get("recipient"))
                    .and_then(|x| x.as_u64())
                    .ok_or("no action recipient")? as usize;
                let w = state.get_mut(recipient).ok_or("out-of-range recipient")?;
                *w = w.checked_add(1).ok_or("reducer overflow")?;
                observed += 1;
                if observed == early {
                    early_balances = Some(state.clone());
                }
                if checkpoints.contains(&observed) {
                    let prior = early_balances
                        .as_ref()
                        .ok_or("missing early ranking baseline")?;
                    rows.push(Trajectory {
                        actors: cell.actors,
                        offset: cell.offset,
                        horizon: cell.horizon,
                        seed,
                        arm: arm.into(),
                        step: observed,
                        gini: gini(&state)?,
                        top_share: top_quintile_share(&state)?,
                        rank_persistence: rank_persistence(prior, &state)?,
                        mean_rank_movement: Some(mean_rank_movement(prior, &state)?),
                    });
                }
            }
            "control" | "rejected" => {}
            _ => return Err("unexpected event outcome".into()),
        }
    }
    if observed != cell.horizon || state != ro.world.wealth || rows.len() != checkpoints.len() {
        return Err("trajectory projection did not match verified terminal state".into());
    }
    let terminal_gini = gini(&state)?;
    let verification_ms = started.elapsed().as_millis();
    let benchmark = RunBenchmark {
        actors: cell.actors,
        offset: cell.offset,
        horizon: cell.horizon,
        seed,
        arm: arm.into(),
        new_accepted_steps: new_steps,
        run_ms,
        verification_ms,
        journal_bytes: ro.log_bytes,
        snapshot_bytes: fs::metadata(dir.join("snapshot.json"))
            .map_err(|e| e.to_string())?
            .len(),
        state_json_bytes: serde_json::to_vec(&ro.world)
            .map_err(|e| e.to_string())?
            .len(),
    };
    Ok((rows, benchmark, terminal_gini))
}

fn csv_option(value: Option<f64>) -> String {
    value.map(|v| format!("{v:.12}")).unwrap_or_default()
}
pub async fn run(root: &Path, plan: StudyPlan) -> Result<(), String> {
    enforce_plan(root, &plan)?;
    let mut trajectories = Vec::new();
    let mut benchmarks = Vec::new();
    let mut summaries = Vec::new();
    for cell in &plan.cells {
        let mut paired = Vec::new();
        for i in 0..plan.pairs {
            let seed = plan.first_seed + i as u64;
            let seed_root = root.join(cell.identity()).join(format!("seed-{seed:020}"));
            fs::create_dir_all(&seed_root).map_err(|e| e.to_string())?;
            let parent = seed_root.join("parent");
            create_or_check_parent(&parent, cell, seed)?;
            let arms = seed_root.join("arms");
            let (mut control, mut treatment) = pair(&parent, &arms, cell)?;
            let (csteps, cms) = complete_arm(&mut control, cell).await?;
            let (tsteps, tms) = complete_arm(&mut treatment, cell).await?;
            drop(control);
            drop(treatment);
            let (ct, cb, cg) =
                verified_outcomes(&arms.join("control"), cell, seed, "control", csteps, cms)?;
            let (tt, tb, tg) = verified_outcomes(
                &arms.join("treatment"),
                cell,
                seed,
                "treatment",
                tsteps,
                tms,
            )?;
            trajectories.extend(ct);
            trajectories.extend(tt);
            benchmarks.push(cb);
            benchmarks.push(tb);
            paired.push(Pair {
                seed,
                horizon: cell.horizon,
                control: cg,
                treatment: tg,
            });
        }
        let stat = summarize(&paired, 2_000, 0x51_2026_001)?;
        let bounds = stat.bootstrap_percentile_95;
        summaries.push(CellSummary {
            actors: cell.actors,
            offset: cell.offset,
            horizon: cell.horizon,
            pairs: plan.pairs,
            control_mean_gini: stat.mean_control,
            treatment_mean_gini: stat.mean_treatment,
            mean_delta: stat.mean_delta,
            bootstrap_lo: bounds.map(|b| b[0]),
            bootstrap_hi: bounds.map(|b| b[1]),
        });
    }
    // Derived CSV is regenerated from verified histories on every completed invocation.
    let mut tr = String::from(
        "actors,offset,horizon,seed,arm,step,gini,top_share,rank_persistence,mean_rank_movement\n",
    );
    for x in &trajectories {
        writeln!(
            tr,
            "{},{},{},{},{},{},{:.12},{:.12},{},{}",
            x.actors,
            x.offset,
            x.horizon,
            x.seed,
            x.arm,
            x.step,
            x.gini,
            x.top_share,
            csv_option(x.rank_persistence),
            csv_option(x.mean_rank_movement)
        )
        .map_err(|e| e.to_string())?;
    }
    save_atomic(&root.join("hiyal-trajectories.csv"), tr.as_bytes())?;
    let mut perf = String::from(
        "actors,offset,horizon,seed,arm,new_accepted_steps,run_ms,verification_ms,journal_bytes,snapshot_bytes,state_json_bytes\n",
    );
    for b in &benchmarks {
        writeln!(
            perf,
            "{},{},{},{},{},{},{},{},{},{},{}",
            b.actors,
            b.offset,
            b.horizon,
            b.seed,
            b.arm,
            b.new_accepted_steps,
            b.run_ms,
            b.verification_ms,
            b.journal_bytes,
            b.snapshot_bytes,
            b.state_json_bytes
        )
        .map_err(|e| e.to_string())?;
    }
    save_atomic(&root.join("hiyal-benchmarks.csv"), perf.as_bytes())?;
    let mut summary = String::from(
        "actors,offset,horizon,pairs,control_mean_gini,treatment_mean_gini,paired_delta,bootstrap_95_lo,bootstrap_95_hi\n",
    );
    for s in &summaries {
        writeln!(
            summary,
            "{},{},{},{},{:.12},{:.12},{:.12},{},{}",
            s.actors,
            s.offset,
            s.horizon,
            s.pairs,
            s.control_mean_gini,
            s.treatment_mean_gini,
            s.mean_delta,
            csv_option(s.bootstrap_lo),
            csv_option(s.bootstrap_hi)
        )
        .map_err(|e| e.to_string())?;
    }
    save_atomic(&root.join("hiyal-summary.csv"), summary.as_bytes())?;
    let plan_hash = hiyal::storage::digest(&plan)?;
    let report = serde_json::json!({
        "status":"exploratory-only",
        "study_plan_digest":plan_hash,
        "model":"20/10/40-agent integer wealth allocation; Hiyal-ChaCha20",
        "hiyal_git_revision":HIYAL_REV,
        "cells":plan.cells.len(),"pairs_per_cell":plan.pairs,
        "scientific_unit":"one independently seeded society pair",
        "uncertainty":"paired percentile bootstrap 95%, 2000 draws, omitted for n<10",
        "benchmark_scope":"wall-clock for newly accepted steps + checkpoint; separate full verification, on shared machine; no measured peak process RSS",
        "reporting":"all configured cells, null and negative effects retained",
    });
    save_atomic(
        &root.join("hiyal-study-report.json"),
        &serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plans_are_bounded_and_distinct() {
        let a = StudyPlan::small(101, 2);
        a.validate().unwrap();
        let b = StudyPlan::grid(101, 10);
        b.validate().unwrap();
        assert_eq!(b.cells.len(), 18);
        assert!(StudyPlan::grid(101, 11).validate().is_err());
        assert!(StudyPlan::grid(u64::MAX, 2).validate().is_err());
    }
    #[tokio::test]
    async fn study_is_idempotent_and_rejects_changed_plan() {
        let root =
            std::env::temp_dir().join(format!("social-hiyal-grid-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let plan = StudyPlan::small(2026, 1);
        run(&root, plan.clone()).await.unwrap();
        let first = fs::read(root.join("hiyal-trajectories.csv")).unwrap();
        run(&root, plan.clone()).await.unwrap();
        assert_eq!(
            first,
            fs::read(root.join("hiyal-trajectories.csv")).unwrap()
        );
        assert!(run(&root, StudyPlan::small(2027, 1)).await.is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
