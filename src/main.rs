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
    ACTORS, Allocate, Feedback, INITIAL_WEALTH, KIND, OFFSET, OpportunityPolicy, Society,
    gini, top_quintile_share,
};
use std::{
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

type ResearchTrial = Trial<Society, OpportunityPolicy>;

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

fn write_atomic(path: &Path, content: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, content).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

fn create_parent(dir: &Path, seed: u64) -> Result<(), String> {
    let initial = Society::new(ACTORS, INITIAL_WEALTH, OFFSET, Feedback::Uniform)?;
    let trial = Trial::create(
        dir,
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
    Ok(())
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
    if progress > target { return Err("existing trial exceeds requested horizon".into()); }
    let remaining = target - progress;
    let accepted = trial.run_attempts(remaining).await?;
    if accepted != remaining { return Err("unexpected rejection/no eligible actor".into()); }
    trial.checkpoint()?;
    trial.store.world().verify_accounting(INITIAL_WEALTH)?;
    if trial.store.world().opportunities != target {
        return Err("opportunity counter differs from committed steps".into());
    }
    Ok(())
}

fn row(dir: &Path, seed: u64, arm: &str) -> Result<ResultRow, String> {
    let ro = Store::<Society, Allocate>::inspect_read_only(dir)?;
    ro.world.verify_accounting(INITIAL_WEALTH)?;
    let mut out = String::from("agent_id,wealth\n");
    for (id, wealth) in ro.world.wealth.iter().enumerate() {
        writeln!(out, "{id},{wealth}").map_err(|e| e.to_string())?;
    }
    write_atomic(&dir.join("wealth.csv"), out.as_bytes())?;
    Ok(ResultRow {
        seed,
        arm: arm.into(),
        attempts: ro.sequence,
        opportunities: ro.logical_step,
        gini: gini(&ro.world.wealth)?,
        top_quintile_share: top_quintile_share(&ro.world.wealth)?,
        chain_head: ro.chain_head,
    })
}

async fn one(root: &Path, seed: u64, target: u64) -> Result<Vec<ResultRow>, String> {
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let parent = root.join("parent");
    if !parent.exists() { create_parent(&parent,seed)?; }
    check_parent(&parent,seed)?;
    let arms_dir = root.join("arms");
    let control_dir = arms_dir.join("control");
    let treatment_dir = arms_dir.join("treatment");
    let present = (control_dir.exists(), treatment_dir.exists());
    let backend = Arc::new(ScriptedBackend);
    let (mut control,mut treatment) = match present {
        (false,false) => paired_forks(
            &parent,
            &arms_dir,
            "linear-feedback-v1",
            serde_json::json!({"feedback":"linear", "offset":OFFSET}),
            || OpportunityPolicy,
            |world: &mut Society| { world.feedback = Feedback::Linear; Ok(()) },
            backend,
        )?,
        (true,true) => (
            resume_fork(&control_dir, OpportunityPolicy, backend.clone(), false)?,
            resume_fork(&treatment_dir, OpportunityPolicy, backend, false)?,
        ),
        _ => return Err("only one arm exists; refusing to silently replace pair".into()),
    };
    if control.store.world().feedback != Feedback::Uniform
        || treatment.store.world().feedback != Feedback::Linear
        || control.store.world().offset != OFFSET
        || treatment.store.world().offset != OFFSET
    {
        return Err("existing fork arm configuration changed".into());
    }
    advance(&mut control,target).await?;
    advance(&mut treatment,target).await?;
    drop(control);
    drop(treatment);

    let c = row(&control_dir,seed,"control")?;
    let t = row(&treatment_dir,seed,"treatment")?;
    if c.opportunities != target || t.opportunities != target {
        return Err("unexpected number of steps".into());
    }
    Ok(vec![c,t])
}

pub async fn pilot(root: &Path, pairs: usize, steps: u64, first_seed: u64) -> Result<(), String> {
    if !(1..=500).contains(&pairs) || !(1..=100_000).contains(&steps) {
        return Err("use 1..=500 pairs and 1..=100000 accepted steps per arm".into());
    }
    let mut all = Vec::with_capacity(pairs * 2);
    for i in 0..pairs {
        let seed = first_seed.checked_add(i as u64).ok_or("seed overflow")?;
        let results = one(&root.join(format!("seed-{seed:020}")),seed,steps).await?;
        all.extend(results);
    }

    // Published summary is nonauthoritative: restart can reconstruct from verified journals.
    let mut csv = String::from("seed,arm,attempts,opportunities,gini,top_quintile_share,chain_head\n");
    for result in &all {
        writeln!(csv,"{},{},{},{},{:.12},{:.12},{}",
            result.seed,result.arm,result.attempts,result.opportunities,
            result.gini,result.top_quintile_share,result.chain_head).map_err(|e| e.to_string())?;
    }
    write_atomic(&root.join("results.csv"),csv.as_bytes())?;
    let summary = serde_json::json!({
        "study": "001-emergence-of-inequality",
        "status": "exploratory-pilot",
        "hiyal_revision": "c6f3d5d650457077eaff4e26a0060bc6cee5dbdd",
        "pairs": pairs,
        "first_seed": first_seed,
        "accepted_opportunities_per_arm": steps,
        "actors": ACTORS,
        "initial_wealth": INITIAL_WEALTH,
        "feedback_offset": OFFSET,
        "randomization": "paired_forks shared RNG domain; exact stream consumption may diverge",
        "results": all
    });
    write_atomic(&root.join("pilot-summary.json"),
        &serde_json::to_vec_pretty(&summary).map_err(|e|e.to_string())?)?;
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
    use std::sync::atomic::{AtomicUsize,Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn test_root(label: &str) -> PathBuf {
        let idx = NEXT.fetch_add(1,Ordering::Relaxed);
        std::env::temp_dir().join(format!("social-lab-{label}-{}-{idx}", std::process::id()))
    }

    #[tokio::test]
    async fn paired_trial_restart_and_reference_are_deterministic() {
        let a=test_root("restart");
        let b=test_root("uninterrupted");
        assert!(!a.exists() && !b.exists());
        pilot(&a,2,9,420).await.unwrap();
        let first = fs::read_to_string(a.join("results.csv")).unwrap();
        pilot(&a,2,9,420).await.unwrap();
        assert_eq!(first,fs::read_to_string(a.join("results.csv")).unwrap());
        pilot(&a,2,19,420).await.unwrap();
        pilot(&b,2,19,420).await.unwrap();
        assert_eq!(fs::read(a.join("results.csv")).unwrap(),fs::read(b.join("results.csv")).unwrap());

        let c = a.join("seed-00000000000000000420/arms/control");
        let t = a.join("seed-00000000000000000420/arms/treatment");
        let ctrl = Store::<Society, Allocate>::inspect_read_only(&c).unwrap();
        let trt = Store::<Society, Allocate>::inspect_read_only(&t).unwrap();
        assert_eq!(ctrl.world.opportunities,19);
        assert_eq!(trt.world.opportunities,19);
        assert_eq!(ctrl.world.feedback,Feedback::Uniform);
        assert_eq!(trt.world.feedback,Feedback::Linear);
        assert_eq!(ctrl.world.total().unwrap(),20*10+19);
        assert_eq!(trt.world.total().unwrap(),20*10+19);
        fs::remove_dir_all(a).unwrap();
        fs::remove_dir_all(b).unwrap();
    }

    #[tokio::test]
    async fn fork_tamper_is_detected_before_resume() {
        let a=test_root("tamper");
        pilot(&a,1,3,99).await.unwrap();
        let p=a.join("seed-00000000000000000099/arms/control/ancestry/manifest.json");
        let mut bytes=fs::read(&p).unwrap();
        bytes.push(b' ');
        fs::write(p,bytes).unwrap();
        assert!(pilot(&a,1,4,99).await.is_err());
        fs::remove_dir_all(a).unwrap();
    }
}
