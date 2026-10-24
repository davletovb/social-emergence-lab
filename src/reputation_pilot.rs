//! Bounded, replay-verified *exploratory* Experiment 002 runs. No confirmatory claims.
use crate::reputation::{
    Action, Arm, Config, Removal, ReputationPolicy, ReputationWorld, StepTrace, Window, kind_for,
    setup, window_of,
};
use crate::reputation_reference;
use hiyal::{
    core::AgentId,
    experiment::{Trial, verify_setup},
    research::{fork_at_head, verify_fork_views},
    runtime::ScriptedBackend,
    scheduler::Scheduling,
    storage::{ReadOnly, Store},
};
use serde::Serialize;
use std::{fs, path::Path, sync::Arc};

const INTERVENTION: &str = "information-network-2x2";

#[derive(Debug, Clone, Serialize)]
pub struct ArmReport {
    pub arm: String,
    pub full: Window,
    pub late: Window,
    pub quarters: Vec<Window>,
    pub final_tokens: Vec<u64>,
    pub all_events: u64,
}
#[derive(Debug, Clone, Serialize)]
pub struct PilotReport {
    pub scientific_status: &'static str,
    pub seed: u64,
    pub config: Config,
    pub checkpoint: u64,
    pub total_steps: u64,
    /// Hiyal revision recorded in the parent genesis by the engine build.
    pub hiyal_revision: String,
    pub per_arm: Vec<ArmReport>,
    pub primary_information_late_gap: f64,
    pub repair_after_truth_late: f64,
    pub repair_without_truth_full: f64,
    pub variance_and_significance_claims: &'static str,
}

/// Who verifies a run's evidence and derives its `report.json`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verification {
    /// `run` itself: it verifies the setup and every arm, then writes the
    /// report. The default, and the only check a standalone pilot gets.
    InRun,
    /// The caller, which must call `verify_and_record` on the tree before
    /// reporting anything. `run` then only creates and runs the trials;
    /// `verify_and_record` performs every check `verify` does, once, and
    /// writes the report from the verified journals.
    ByCaller,
}

pub async fn run(
    root: &Path,
    seed: u64,
    config: Config,
    checkpoint: u64,
    total: u64,
) -> Result<PilotReport, String> {
    run_checked(root, seed, config, checkpoint, total, Verification::InRun)
        .await?
        .ok_or_else(|| "in-run verification produced no report".into())
}

pub(crate) async fn run_checked(
    root: &Path,
    seed: u64,
    config: Config,
    checkpoint: u64,
    total: u64,
    verification: Verification,
) -> Result<Option<PilotReport>, String> {
    config.validate()?;
    if checkpoint == 0 || total <= checkpoint || total > 1000 || checkpoint > 500 || root.exists() {
        return Err("pilot needs a new path, 0<checkpoint<total<=1000 and checkpoint<=500".into());
    }
    let kind = kind_for(&config);
    let journal_trace = config.journal_trace;
    let config_for_genesis = config.clone();
    let agent_ids = (0..config.actors as u64).map(AgentId).collect();
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let parent_dir = root.join("parent");
    let backend = Arc::new(ScriptedBackend);
    let mut parent = Trial::create_with_setup(
        &parent_dir,
        "reputation-v0",
        seed,
        kind,
        move |rng| setup(&config_for_genesis, rng),
        ReputationPolicy,
        backend.clone(),
        agent_ids,
        Scheduling::Random,
    )?;
    let attempts = checkpoint;
    if parent
        .run_attempts(attempts)
        .await
        .map_err(|e| e.to_string())?
        != attempts
    {
        return Err("parent did not commit all opportunities".into());
    }
    parent.checkpoint()?;
    parent.store.world().verify()?;
    drop(parent);
    let in_run = verification == Verification::InRun;
    if in_run {
        verify_setup(&parent_dir, {
            let setup_config = config.clone();
            move |rng| setup(&setup_config, rng)
        })?;
    }
    let parent = if in_run {
        Some(Store::<ReputationWorld, Action>::inspect_read_only_cached(
            &parent_dir,
        )?)
    } else {
        None
    };
    let mut reports = Vec::new();
    for arm in Arm::ALL {
        let path = root.join(arm.name());
        let mut trial = fork_at_head(
            &parent_dir,
            &path,
            INTERVENTION,
            arm.name(),
            serde_json::json!({"checkpoint":checkpoint,"arm":arm.name()}),
            |w: &mut ReputationWorld| w.intervene(arm),
            ReputationPolicy,
            backend.clone(),
        )?;
        if trial.store.world().step != checkpoint {
            return Err("fork does not start at checkpoint".into());
        }
        let post = total - checkpoint;
        if trial.run_attempts(post).await.map_err(|e| e.to_string())? != post {
            return Err("fork did not commit all post-intervention opportunities".into());
        }
        trial.checkpoint()?;
        drop(trial);
        if !in_run {
            continue;
        }
        let ro = verify_fork_views::<ReputationWorld>(&path)?.branch;
        ro.world.verify()?;
        if ro.world.step != total || ro.logical_step != total - checkpoint {
            return Err("final world/ledger step mismatch".into());
        }
        let derived;
        let trace = if journal_trace {
            derived = replay_reference(&path, &ro)?;
            &derived
        } else {
            &ro.world.trace
        };
        reports.push(arm_report(&ro.world, trace, arm, checkpoint, total)?);
    }
    let Some(parent) = parent else {
        return Ok(None);
    };
    let answer = pilot_report(&parent, config, checkpoint, total, reports)?;
    fs::write(
        root.join("report.json"),
        serde_json::to_vec_pretty(&answer).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(Some(answer))
}

/// Re-derive a finished run from its Hiyal evidence alone: the parent's
/// setup and replay, each arm's ancestry, attribution, declared intervention
/// and replay, every pre-event state against the independent reference, and
/// finally the exact bytes of `report.json`.
pub fn verify(root: &Path) -> Result<PilotReport, String> {
    let report = verified_report(root)?;
    let recorded = fs::read(root.join("report.json")).map_err(|e| e.to_string())?;
    if recorded != serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())? {
        return Err("report.json is not derivable from the verified journals".into());
    }
    Ok(report)
}

/// Every check of `verify`, then write `report.json` from the verified
/// journals instead of comparing it: the second half of a
/// `Verification::ByCaller` run.
pub(crate) fn verify_and_record(root: &Path) -> Result<PilotReport, String> {
    let report = verified_report(root)?;
    fs::write(
        root.join("report.json"),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(report)
}

/// The report derived from a run tree after verifying all of it: setup,
/// parent and arm replays, fork attribution, interventions and the
/// independent reference replay.
fn verified_report(root: &Path) -> Result<PilotReport, String> {
    let parent_dir = root.join("parent");
    let parent = Store::<ReputationWorld, Action>::inspect_read_only_cached(&parent_dir)?;
    let config = parent.genesis.initial_world.config.clone();
    if parent.genesis.world_kind != kind_for(&config) {
        return Err("parent world kind does not match its reputation configuration".into());
    }
    verify_setup(&parent_dir, |rng| setup(&config, rng))?;
    parent.world.verify()?;
    replay_reference(&parent_dir, &parent)?;
    let checkpoint = parent.logical_step;
    if parent.world.step != checkpoint {
        return Err("parent world and ledger disagree on the checkpoint".into());
    }
    let mut total = None;
    let mut reports = Vec::new();
    for arm in Arm::ALL {
        let path = root.join(arm.name());
        let fork = verify_fork_views::<ReputationWorld>(&path)?;
        let meta = &fork.metadata;
        if meta.parent_id != parent.genesis.trial_id
            || meta.parent_head != parent.chain_head
            || meta.intervention != INTERVENTION
            || meta.arm != arm.name()
            || meta.parameters != serde_json::json!({"checkpoint":checkpoint,"arm":arm.name()})
        {
            return Err(format!("{} is not a fork of this run's parent", arm.name()));
        }
        let ro = fork.branch;
        let mut expected = parent.world.clone();
        expected.intervene(arm)?;
        if ro.genesis.initial_world != expected {
            return Err(format!(
                "{} does not start from its declared intervention",
                arm.name()
            ));
        }
        ro.world.verify()?;
        let replayed_trace = replay_reference(&path, &ro)?;
        let trace = if config.journal_trace {
            &replayed_trace
        } else {
            // The journal-derived trajectory must match the stored one.
            let start = usize::try_from(checkpoint).map_err(|e| e.to_string())?;
            if ro.world.trace.get(start..) != Some(&replayed_trace[..]) {
                return Err(format!(
                    "{}: stored trace differs from its journal",
                    arm.name()
                ));
            }
            &ro.world.trace
        };
        let steps = *total.get_or_insert(ro.world.step);
        if ro.world.step != steps || ro.logical_step != steps - checkpoint {
            return Err("arms disagree on the run length".into());
        }
        reports.push(arm_report(&ro.world, trace, arm, checkpoint, steps)?);
    }
    let report = pilot_report(
        &parent,
        config,
        checkpoint,
        total.ok_or("no arms")?,
        reports,
    )?;
    Ok(report)
}

/// Replay `dir`'s journal against the independent reference and return the
/// trajectory of its own events.
fn replay_reference(dir: &Path, ro: &ReadOnly<ReputationWorld>) -> Result<Vec<StepTrace>, String> {
    let (replayed, trace) = reputation_reference::replay_journal_trace(
        &dir.join("events.jsonl"),
        ro.log_bytes,
        &ro.genesis.initial_world,
        1e-9,
    )?;
    if replayed != ro.world {
        return Err(format!(
            "{}: reference replay disagrees with the verified world",
            dir.display()
        ));
    }
    Ok(trace)
}

/// Windows are read from `trace`: the world's own trace, or for a
/// journal-trace world the trajectory rebuilt from its journal.
fn arm_report(
    world: &ReputationWorld,
    trace: &[StepTrace],
    arm: Arm,
    checkpoint: u64,
    total: u64,
) -> Result<ArmReport, String> {
    let last = total.saturating_sub(99).max(checkpoint + 1);
    let quarters = if total - checkpoint >= 400 && (total - checkpoint) % 4 == 0 {
        let chunk = (total - checkpoint) / 4;
        (0..4)
            .map(|i| {
                window_of(
                    &world.config,
                    trace,
                    checkpoint + 1 + i * chunk,
                    checkpoint + (i + 1) * chunk,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        vec![]
    };
    Ok(ArmReport {
        arm: arm.name().into(),
        full: window_of(&world.config, trace, checkpoint + 1, total)?,
        late: window_of(&world.config, trace, last, total)?,
        quarters,
        final_tokens: world.wealth.clone(),
        all_events: world.step,
    })
}

fn pilot_report(
    parent: &ReadOnly<ReputationWorld>,
    config: Config,
    checkpoint: u64,
    total: u64,
    reports: Vec<ArmReport>,
) -> Result<PilotReport, String> {
    let gap = |name: &str, late: bool| {
        reports
            .iter()
            .find(|x| x.arm == name)
            .map(|x| {
                if late {
                    x.late.opportunity_gap
                } else {
                    x.full.opportunity_gap
                }
            })
            .ok_or_else(|| format!("missing {name} arm"))
    };
    let hiyal_revision = parent
        .genesis
        .build
        .as_ref()
        .and_then(|b| b.source_revision.clone())
        .ok_or("parent genesis records no Hiyal revision")?;
    Ok(PilotReport {
        scientific_status: "EXPLORATORY_ONLY_NOT_PREREGISTERED",
        seed: parent.genesis.initial_streams.root_seed,
        config,
        checkpoint,
        total_steps: total,
        hiyal_revision,
        primary_information_late_gap: gap("information", true)?,
        repair_after_truth_late: gap("information", true)? - gap("both", true)?,
        repair_without_truth_full: gap("sham", false)? - gap("structure", false)?,
        variance_and_significance_claims: "One parent seed; no uncertainty estimate, power or empirical causal inference. Run and report independent parent societies only after reviewing scientific design.",
        per_arm: reports,
    })
}
/// Declared negative controls for *exploratory-only* model tests.
pub fn control_config(n: usize, d: usize, case: &str) -> Result<Config, String> {
    let mut c = Config::default_for(n, d);
    match case {
        "default" => {}
        "no-false-prior" => c.delta = 0.0,
        "no-selection-bias" => c.beta = 0.0,
        "fixed-graph" => c.removal = Removal::None,
        "random-churn" => {
            c.removal = Removal::Uniform;
            c.kappa = 0.0;
        }
        "annealed-mixing" => {
            c.removal = Removal::None;
            c.annealed = true;
        }
        "full-mixing" => {
            c.removal = Removal::None;
            c.degree = n - 1;
        }
        "rewire-every-2" => c.rewire_every = 2,
        "rewire-every-20" => c.rewire_every = 20,
        "rewire-every-50" => c.rewire_every = 50,
        _ => return Err(
            "unknown control: default|no-false-prior|no-selection-bias|fixed-graph|random-churn|annealed-mixing|full-mixing|rewire-every-2|rewire-every-20|rewire-every-50"
                .into(),
        ),
    }
    c.validate()?;
    Ok(c)
}
