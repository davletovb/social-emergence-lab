//! Small exploratory paired-fork runner. No LLM calls or cloud services.
use hiyal::{
    core::{AgentId, TrialId},
    experiment::Trial,
    research::{paired_forks, resume_fork},
    runtime::ScriptedBackend,
    scheduler::Scheduling,
    storage::Store,
};
use serde::Serialize;
use social_emergence_lab::{
    ACTORS, Allocate, Feedback, INITIAL_WEALTH, KIND, OFFSET, OpportunityPolicy, Society, gini,
    top_quintile_share,
};
use std::{
    env,
    fmt::Write as _,
    fs::{self, File},
    io::Write as _,
    path::{Path, PathBuf},
    sync::Arc,
};

type ResearchTrial = Trial<Society, OpportunityPolicy>;

/// Upper bound on pairs * accepted steps. Each journal entry is roughly 2.2 KB
/// at 20 actors, so this caps journals at about 4.4 GB per pilot invocation.
const MAX_PAIR_STEPS: u64 = 1_000_000;

/// Hiyal revision actually compiled in, read from the committed lockfile so the
/// exported provenance cannot drift from the pinned dependency.
fn hiyal_revision() -> Result<&'static str, String> {
    let lock = include_str!("../Cargo.lock");
    lock.split("[[package]]")
        .find(|block| block.lines().any(|line| line.trim() == "name = \"hiyal\""))
        .and_then(|block| {
            block
                .lines()
                .find_map(|line| line.trim().strip_prefix("source = \""))
        })
        .and_then(|source| source.trim_end_matches('"').rsplit_once('#'))
        .map(|(_, rev)| rev)
        .ok_or_else(|| "Cargo.lock does not pin a hiyal git revision".into())
}

#[derive(Serialize)]
struct ResultRow {
    seed: u64,
    arm: String,
    attempts: u64,
    opportunities: u64,
    gini: f64,
    top_quintile_share: f64,
    chain_head: String,
}

fn sync_dir(dir: &Path) -> Result<(), String> {
    #[cfg(unix)]
    File::open(dir)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

fn write_atomic(path: &Path, content: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    let mut file = File::create(&tmp).map_err(|e| e.to_string())?;
    file.write_all(content).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    sync_dir(path.parent().ok_or("output path has no parent")?)
}

/// Build the parent in a staging directory and publish it by rename, so a
/// crash never leaves a half-built `parent` that blocks every later run.
fn create_parent(dir: &Path, seed: u64) -> Result<(), String> {
    let root = dir.parent().ok_or("parent path has no directory")?;
    let stage = root.join(format!(".parent.pending-{}", std::process::id()));
    if stage.exists() {
        fs::remove_dir_all(&stage).map_err(|e| e.to_string())?;
    }
    let initial = Society::new(ACTORS, INITIAL_WEALTH, OFFSET, Feedback::Uniform)?;
    let trial = Trial::create(
        &stage,
        &format!("inequality-001-{seed}"),
        seed,
        KIND,
        initial,
        OpportunityPolicy,
        Arc::new(ScriptedBackend),
        (0..ACTORS).map(|i| AgentId(i as u64)).collect(),
        Scheduling::RoundRobin,
    )?;
    trial.checkpoint()?;
    drop(trial);
    fs::rename(&stage, dir).map_err(|e| e.to_string())?;
    sync_dir(root)
}

fn check_parent(dir: &Path, seed: u64) -> Result<(), String> {
    let parent = Store::<Society, Allocate>::inspect_read_only(dir)?;
    if parent.genesis.trial_id != TrialId::new(&format!("inequality-001-{seed}"), seed).0
        || parent.genesis.world_kind != KIND
        || parent.logical_step != 0
        || parent.world != Society::new(ACTORS, INITIAL_WEALTH, OFFSET, Feedback::Uniform)?
    {
        return Err("parent world, seed, kind or fork boundary changed".into());
    }
    Ok(())
}

async fn advance(trial: &mut ResearchTrial, target: u64) -> Result<(), String> {
    let progress = trial.store.logical_step();
    if progress > target {
        return Err("existing trial exceeds requested horizon".into());
    }
    let remaining = target - progress;
    let accepted = trial.run_attempts(remaining).await?;
    if accepted != remaining {
        return Err("unexpected rejection/no eligible actor".into());
    }
    // Check invariants before checkpointing so a violating state never
    // becomes the trusted snapshot.
    trial.store.world().verify_accounting(INITIAL_WEALTH)?;
    if trial.store.world().opportunities != target {
        return Err("opportunity counter differs from committed steps".into());
    }
    trial.checkpoint()?;
    Ok(())
}

/// Derived outputs live under `outputs/`, outside the Hiyal-owned arm
/// directories, so they never mix with audited evidence.
fn row(trial: &ResearchTrial, outputs: &Path, seed: u64, arm: &str) -> Result<ResultRow, String> {
    let world = trial.store.world();
    world.verify_accounting(INITIAL_WEALTH)?;
    let mut out = String::from("agent_id,wealth\n");
    for (id, wealth) in world.wealth.iter().enumerate() {
        writeln!(out, "{id},{wealth}").map_err(|e| e.to_string())?;
    }
    write_atomic(&outputs.join(format!("{arm}-wealth.csv")), out.as_bytes())?;
    Ok(ResultRow {
        seed,
        arm: arm.into(),
        attempts: trial.store.sequence(),
        opportunities: trial.store.logical_step(),
        gini: gini(&world.wealth)?,
        top_quintile_share: top_quintile_share(&world.wealth)?,
        chain_head: trial.store.chain_head().to_owned(),
    })
}

/// `fork_at_head` publishes each arm atomically, so a lone arm means the run
/// stopped between the two forks. Discard it only while it is still at the
/// fork boundary; an arm with committed events is never deleted.
fn discard_lone_arm(dir: &Path) -> Result<(), String> {
    let arm = Store::<Society, Allocate>::inspect_read_only(dir)?;
    if arm.logical_step != 0 {
        return Err("only one arm exists and it has committed events; refusing to replace".into());
    }
    fs::remove_dir_all(dir).map_err(|e| e.to_string())
}

async fn one(root: &Path, seed: u64, target: u64) -> Result<Vec<ResultRow>, String> {
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let parent = root.join("parent");
    if !parent.exists() {
        create_parent(&parent, seed)?;
    }
    check_parent(&parent, seed)?;
    let arms_dir = root.join("arms");
    let control_dir = arms_dir.join("control");
    let treatment_dir = arms_dir.join("treatment");
    match (control_dir.exists(), treatment_dir.exists()) {
        (true, false) => discard_lone_arm(&control_dir)?,
        (false, true) => discard_lone_arm(&treatment_dir)?,
        _ => {}
    }
    let backend = Arc::new(ScriptedBackend);
    let (mut control, mut treatment) = match (control_dir.exists(), treatment_dir.exists()) {
        (false, false) => paired_forks(
            &parent,
            &arms_dir,
            "linear-feedback-v1",
            serde_json::json!({"feedback":"linear", "offset":OFFSET}),
            || OpportunityPolicy,
            |world: &mut Society| {
                world.feedback = Feedback::Linear;
                Ok(())
            },
            backend,
        )?,
        (true, true) => (
            resume_fork(&control_dir, OpportunityPolicy, backend.clone(), false)?,
            resume_fork(&treatment_dir, OpportunityPolicy, backend, false)?,
        ),
        _ => return Err("arm directories changed during startup".into()),
    };
    if control.store.world().feedback != Feedback::Uniform
        || treatment.store.world().feedback != Feedback::Linear
        || control.store.world().offset != OFFSET
        || treatment.store.world().offset != OFFSET
    {
        return Err("existing fork arm configuration changed".into());
    }
    advance(&mut control, target).await?;
    advance(&mut treatment, target).await?;

    let outputs = root.join("outputs");
    fs::create_dir_all(&outputs).map_err(|e| e.to_string())?;
    let c = row(&control, &outputs, seed, "control")?;
    let t = row(&treatment, &outputs, seed, "treatment")?;
    if c.opportunities != target || t.opportunities != target {
        return Err("unexpected number of steps".into());
    }
    Ok(vec![c, t])
}

pub async fn pilot(root: &Path, pairs: usize, steps: u64, first_seed: u64) -> Result<(), String> {
    if !(1..=500).contains(&pairs) || !(1..=100_000).contains(&steps) {
        return Err("use 1..=500 pairs and 1..=100000 accepted steps per arm".into());
    }
    if (pairs as u64) * steps > MAX_PAIR_STEPS {
        return Err(format!(
            "pairs * steps must be at most {MAX_PAIR_STEPS} (~2.2 KB of journal per step per arm)"
        ));
    }
    let hiyal_revision = hiyal_revision()?;
    let mut all = Vec::with_capacity(pairs * 2);
    for i in 0..pairs {
        let seed = first_seed.checked_add(i as u64).ok_or("seed overflow")?;
        let results = one(&root.join(format!("seed-{seed:020}")), seed, steps).await?;
        all.extend(results);
    }

    // Published summary is nonauthoritative: restart can reconstruct from verified journals.
    let mut csv =
        String::from("seed,arm,attempts,opportunities,gini,top_quintile_share,chain_head\n");
    for result in &all {
        writeln!(
            csv,
            "{},{},{},{},{:.12},{:.12},{}",
            result.seed,
            result.arm,
            result.attempts,
            result.opportunities,
            result.gini,
            result.top_quintile_share,
            result.chain_head
        )
        .map_err(|e| e.to_string())?;
    }
    write_atomic(&root.join("results.csv"), csv.as_bytes())?;
    let summary = serde_json::json!({
        "study": "001-emergence-of-inequality",
        "status": "exploratory-pilot",
        "hiyal_revision": hiyal_revision,
        "pairs": pairs,
        "first_seed": first_seed,
        "accepted_opportunities_per_arm": steps,
        "actors": ACTORS,
        "initial_wealth": INITIAL_WEALTH,
        "feedback_offset": OFFSET,
        "randomization": "paired_forks shared RNG domain; exact stream consumption may diverge",
        "results": all
    });
    write_atomic(
        &root.join("pilot-summary.json"),
        &serde_json::to_vec_pretty(&summary).map_err(|e| e.to_string())?,
    )?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = async {
        if args.len() != 5 || args[0] != "pilot" {
            return Err("usage: social-emergence-lab pilot <output-dir> <pairs> <accepted-steps> <first-seed>".to_owned());
        }
        let root = PathBuf::from(&args[1]);
        let pairs: usize = args[2].parse::<usize>().map_err(|e|e.to_string())?;
        let steps: u64 = args[3].parse::<u64>().map_err(|e|e.to_string())?;
        let seed: u64 = args[4].parse::<u64>().map_err(|e|e.to_string())?;
        pilot(&root,pairs,steps,seed).await?;
        println!("Exploratory pilot completed: {} pairs, {} accepted steps/arm; {}",pairs,steps,root.display());
        Ok::<(),String>(())
    }.await;
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn test_root(label: &str) -> PathBuf {
        let idx = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("social-lab-{label}-{}-{idx}", std::process::id()))
    }

    #[tokio::test]
    async fn paired_trial_restart_and_reference_are_deterministic() {
        let a = test_root("restart");
        let b = test_root("uninterrupted");
        assert!(!a.exists() && !b.exists());
        pilot(&a, 2, 9, 420).await.unwrap();
        let first = fs::read_to_string(a.join("results.csv")).unwrap();
        pilot(&a, 2, 9, 420).await.unwrap();
        assert_eq!(first, fs::read_to_string(a.join("results.csv")).unwrap());
        pilot(&a, 2, 19, 420).await.unwrap();
        pilot(&b, 2, 19, 420).await.unwrap();
        assert_eq!(
            fs::read(a.join("results.csv")).unwrap(),
            fs::read(b.join("results.csv")).unwrap()
        );

        let c = a.join("seed-00000000000000000420/arms/control");
        let t = a.join("seed-00000000000000000420/arms/treatment");
        let ctrl = Store::<Society, Allocate>::inspect_read_only(&c).unwrap();
        let trt = Store::<Society, Allocate>::inspect_read_only(&t).unwrap();
        assert_eq!(ctrl.world.opportunities, 19);
        assert_eq!(trt.world.opportunities, 19);
        assert_eq!(ctrl.world.feedback, Feedback::Uniform);
        assert_eq!(trt.world.feedback, Feedback::Linear);
        assert_eq!(ctrl.world.total().unwrap(), 20 * 10 + 19);
        assert_eq!(trt.world.total().unwrap(), 20 * 10 + 19);
        fs::remove_dir_all(a).unwrap();
        fs::remove_dir_all(b).unwrap();
    }

    #[tokio::test]
    async fn fork_tamper_is_detected_before_resume() {
        let a = test_root("tamper");
        pilot(&a, 1, 3, 99).await.unwrap();
        let p = a.join("seed-00000000000000000099/arms/control/ancestry/manifest.json");
        let mut bytes = fs::read(&p).unwrap();
        bytes.push(b' ');
        fs::write(p, bytes).unwrap();
        let error = pilot(&a, 1, 5, 99).await.unwrap_err();
        assert!(error.contains("parent evidence bytes altered"), "{error}");
        fs::remove_dir_all(a).unwrap();
    }

    #[tokio::test]
    async fn untampered_fork_resumes() {
        let a = test_root("untampered");
        pilot(&a, 1, 3, 99).await.unwrap();
        pilot(&a, 1, 4, 99).await.unwrap();
        fs::remove_dir_all(a).unwrap();
    }

    #[tokio::test]
    async fn lone_arm_at_fork_boundary_is_recreated() {
        let a = test_root("lone-boundary");
        let b = test_root("lone-clean");
        let seed = a.join("seed-00000000000000000007");
        fs::create_dir_all(&seed).unwrap();
        create_parent(&seed.join("parent"), 7).unwrap();
        // Simulates a kill after paired_forks published control but not treatment.
        hiyal::research::fork_at_head(
            seed.join("parent"),
            seed.join("arms/control"),
            "linear-feedback-v1",
            "control",
            serde_json::json!({}),
            |_world: &mut Society| Ok(()),
            OpportunityPolicy,
            Arc::new(ScriptedBackend),
        )
        .unwrap();
        pilot(&a, 1, 6, 7).await.unwrap();
        pilot(&b, 1, 6, 7).await.unwrap();
        assert_eq!(
            fs::read(a.join("results.csv")).unwrap(),
            fs::read(b.join("results.csv")).unwrap()
        );
        fs::remove_dir_all(a).unwrap();
        fs::remove_dir_all(b).unwrap();
    }

    #[tokio::test]
    async fn leftover_parent_staging_is_ignored() {
        let a = test_root("parent-stage");
        let b = test_root("parent-clean");
        let seed = a.join("seed-00000000000000000008");
        let stage = seed.join(format!(".parent.pending-{}", std::process::id()));
        fs::create_dir_all(&stage).unwrap();
        fs::write(stage.join("manifest.json"), b"partial").unwrap();
        pilot(&a, 1, 6, 8).await.unwrap();
        assert!(!stage.exists());
        pilot(&b, 1, 6, 8).await.unwrap();
        assert_eq!(
            fs::read(a.join("results.csv")).unwrap(),
            fs::read(b.join("results.csv")).unwrap()
        );
        fs::remove_dir_all(a).unwrap();
        fs::remove_dir_all(b).unwrap();
    }

    #[tokio::test]
    async fn lone_arm_with_events_is_not_discarded() {
        let a = test_root("lone-advanced");
        pilot(&a, 1, 3, 11).await.unwrap();
        fs::remove_dir_all(a.join("seed-00000000000000000011/arms/treatment")).unwrap();
        let error = pilot(&a, 1, 3, 11).await.unwrap_err();
        assert!(error.contains("committed events"), "{error}");
        fs::remove_dir_all(a).unwrap();
    }

    #[test]
    fn summary_revision_matches_cargo_toml_pin() {
        let manifest = include_str!("../Cargo.toml");
        let rev = hiyal_revision().unwrap();
        assert!(manifest.contains(&format!("rev = \"{rev}\"")));
    }

    #[tokio::test]
    async fn oversized_runs_are_rejected_before_writing() {
        let a = test_root("oversized");
        assert!(pilot(&a, 500, 100_000, 0).await.is_err());
        assert!(!a.exists());
    }
}
