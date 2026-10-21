//! Independently re-execute accepted journal actions and compare to Hiyal's
//! fully verified state. This deliberately does not reuse the engine reducer.
use hiyal::{research::verify_fork, storage::Store};
use social_emergence_lab::{
    Allocate, Society, gini,
    reference::{reference_gini, reference_top_share, replay},
    top_quintile_share,
};
use std::{env, path::Path};
fn validate(path: &Path) -> Result<String, String> {
    {
        // Forked arms: check attribution and the copied parent ancestry bytes.
        if path.join("fork.json").exists() {
            verify_fork::<Society>(path)?;
        }
        // First: independent Hiyal ledger/hash/ancestry check via public read-only API.
        let ro = Store::<Society, Allocate>::inspect_read_only(path)?;
        // Then: deliberately independent, plain-vector reference implementation,
        // over exactly the journal prefix Hiyal just verified.
        let initial = &ro.genesis.initial_world;
        let expected = replay(&path.join("events.jsonl"), &initial.wealth, ro.log_bytes)?;
        // A fork starts from the parent's world (which may already count
        // opportunities) while its own logical step restarts at zero.
        let total_opportunities = initial
            .opportunities
            .checked_add(expected.opportunities)
            .ok_or("opportunity overflow")?;
        if expected.wealth != ro.world.wealth
            || expected.opportunities != ro.logical_step
            || total_opportunities != ro.world.opportunities
        {
            return Err("independent reference disagrees with verified Hiyal world".into());
        }
        let actual_gini = gini(&ro.world.wealth)?;
        let check_gini = reference_gini(&expected.wealth)?;
        let actual_top = top_quintile_share(&ro.world.wealth)?;
        let check_top = reference_top_share(&expected.wealth)?;
        if (actual_gini - check_gini).abs() > 1e-12 || (actual_top - check_top).abs() > 1e-12 {
            return Err("independent reference metrics disagree".into());
        }
        Ok(format!(
            "Reference validation passed: {} accepted opportunities, Gini {:.12}, chain head {}",
            expected.opportunities, check_gini, ro.chain_head
        ))
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: cargo run --locked --bin validate -- <trial-directory>...");
        std::process::exit(2);
    }
    for arg in &args {
        match validate(Path::new(arg)) {
            Ok(message) => println!("{arg}: {message}"),
            Err(e) => {
                eprintln!("reference validation failed for {arg}: {e}");
                std::process::exit(2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hiyal::{
        core::AgentId, experiment::Trial, research::fork_at_head, runtime::ScriptedBackend,
        scheduler::Scheduling,
    };
    use social_emergence_lab::{ACTORS, Feedback, INITIAL_WEALTH, KIND, OFFSET, OpportunityPolicy};
    use std::sync::Arc;

    #[tokio::test]
    async fn validates_arm_forked_after_parent_steps() {
        let root = std::env::temp_dir().join(format!("validate-late-fork-{}", std::process::id()));
        let parent = root.join("parent");
        let child = root.join("child");
        let mut trial = Trial::create(
            &parent,
            "validate-late-fork",
            5,
            KIND,
            Society::new(ACTORS, INITIAL_WEALTH, OFFSET, Feedback::Uniform).unwrap(),
            OpportunityPolicy,
            Arc::new(ScriptedBackend),
            (0..ACTORS).map(|i| AgentId(i as u64)).collect(),
            Scheduling::RoundRobin,
        )
        .unwrap();
        assert_eq!(trial.run_attempts(4).await.unwrap(), 4);
        trial.checkpoint().unwrap();
        drop(trial);
        let mut branch = fork_at_head(
            &parent,
            &child,
            "late-fork",
            "treatment",
            serde_json::json!({}),
            |world: &mut Society| {
                world.feedback = Feedback::Linear;
                Ok(())
            },
            OpportunityPolicy,
            Arc::new(ScriptedBackend),
        )
        .unwrap();
        assert_eq!(branch.run_attempts(3).await.unwrap(), 3);
        branch.checkpoint().unwrap();
        drop(branch);
        let message = validate(&child).unwrap();
        assert!(message.contains("3 accepted opportunities"), "{message}");
        validate(&parent).unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
}
