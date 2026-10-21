//! Crash-safe building blocks shared by the pilot and the Hiyal study runner:
//! staged parent creation, lone-arm recovery, durable writes and provenance.
use crate::{Allocate, KIND, OpportunityPolicy, Society};
use hiyal::{
    core::AgentId, experiment::Trial, runtime::ScriptedBackend, scheduler::Scheduling,
    storage::Store,
};
use std::{
    fs::{self, File},
    io::Write as _,
    path::Path,
    sync::Arc,
};

/// Hiyal revision actually compiled in, read from the committed lockfile so the
/// exported provenance cannot drift from the pinned dependency.
pub fn hiyal_revision() -> Result<&'static str, String> {
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

pub fn sync_dir(dir: &Path) -> Result<(), String> {
    #[cfg(unix)]
    File::open(dir)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

pub fn write_atomic(path: &Path, content: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    let mut file = File::create(&tmp).map_err(|e| e.to_string())?;
    file.write_all(content).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    sync_dir(path.parent().ok_or("output path has no parent")?)
}

/// Build a zero-event parent trial in a staging directory and publish it by
/// rename, so a crash never leaves a half-built `parent` that blocks every
/// later run.
pub fn create_parent_staged(
    dir: &Path,
    name: &str,
    seed: u64,
    world: Society,
) -> Result<(), String> {
    let root = dir.parent().ok_or("parent path has no directory")?;
    let stage = root.join(format!(".parent.pending-{}", std::process::id()));
    if stage.exists() {
        fs::remove_dir_all(&stage).map_err(|e| e.to_string())?;
    }
    let agents = (0..world.wealth.len()).map(|i| AgentId(i as u64)).collect();
    let trial = Trial::create(
        &stage,
        name,
        seed,
        KIND,
        world,
        OpportunityPolicy,
        Arc::new(ScriptedBackend),
        agents,
        Scheduling::RoundRobin,
    )?;
    trial.checkpoint()?;
    drop(trial);
    fs::rename(&stage, dir).map_err(|e| e.to_string())?;
    sync_dir(root)
}

/// `fork_at_head` publishes each arm atomically, so a lone arm means the run
/// stopped between the two forks. Discard it only while it is still at the
/// fork boundary; an arm with committed events is never deleted.
pub fn discard_lone_arm(dir: &Path) -> Result<(), String> {
    let arm = Store::<Society, Allocate>::inspect_read_only(dir)?;
    if arm.logical_step != 0 {
        return Err("only one arm exists and it has committed events; refusing to replace".into());
    }
    fs::remove_dir_all(dir).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revision_matches_cargo_toml_pin() {
        let manifest = include_str!("../Cargo.toml");
        let rev = hiyal_revision().unwrap();
        assert!(manifest.contains(&format!("rev = \"{rev}\"")));
    }
}
