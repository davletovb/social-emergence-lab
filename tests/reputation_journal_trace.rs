//! Journal-trace world kind: same actions and measurements as the V0 kind,
//! with the trajectory rebuilt from the journal instead of stored in the world.
use hiyal::{core::AgentId, core::World, experiment::EffectPolicy, rng::Streams, storage::Store};
use social_emergence_lab::reputation::{
    Action, Config, KIND, KIND_JOURNAL_TRACE, ReputationPolicy, ReputationWorld, kind_for, setup,
    window, window_of,
};
use social_emergence_lab::reputation_pilot::{control_config, run, verify};
use social_emergence_lab::reputation_reference::replay_journal_trace;
/// `hiyal::storage::digest` of each fork's journal-derived trajectory for
/// seed 82102 (N=10, d=3, checkpoint 20, total 60).
const GOLDEN_TRACE_DIGESTS: [&str; 4] = [
    "information:2371e1bc80c4166eeb638ffcbaf8d54b5d73d2f6f51a825e28ecaced1e3d3a9e",
    "structure:d209f742a2ac75c494578a4b46ae75451c6679ce68eea705a8903fc3b8e278c4",
    "both:e7af40e22262121b1c8eb61ac9c243fab9219326ac275def450b2edb4e80cec2",
    "sham:85e55d0c662e74a192038e6bc1fb38d50ef14b4fd80aa2e523965175c945b52b",
];

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

fn temp() -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    std::env::temp_dir().join(format!(
        "social-journal-trace-test-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ))
}

fn journal(path: &Path) -> Vec<serde_json::Value> {
    fs::read_to_string(path.join("events.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn legacy_configs_serialize_and_kind_exactly_as_before() {
    let config = Config::default_for(10, 3);
    let encoded = serde_json::to_value(&config).unwrap();
    assert!(encoded.get("journal_trace").is_none());
    assert_eq!(kind_for(&config), KIND);
    let mut compact = config;
    compact.journal_trace = true;
    assert_eq!(kind_for(&compact), KIND_JOURNAL_TRACE);
    assert_eq!(
        serde_json::to_value(&compact).unwrap()["journal_trace"],
        true
    );
}

#[test]
fn journal_trace_worlds_keep_no_trace_and_reject_one() {
    let mut config = Config::default_for(10, 3);
    config.journal_trace = true;
    let mut rng = Streams::new("journal-trace".into(), 5);
    let mut world = setup(&config, &mut rng).unwrap();
    let mut legacy = world.clone();
    legacy.config.journal_trace = false;
    for step in 0..12u64 {
        let actor = AgentId(step % 10);
        let observation = world.observe(actor).unwrap();
        let action: Action = ReputationPolicy
            .decide_with_streams(actor, &observation, &[], &mut rng)
            .unwrap();
        world.apply(actor, action.clone()).unwrap();
        legacy.apply(actor, action).unwrap();
    }
    assert!(world.trace.is_empty());
    assert_eq!(legacy.trace.len(), 12);
    world.trace = legacy.trace.clone();
    assert!(world.verify().is_err());
    legacy.trace.pop();
    assert!(legacy.verify().is_err());
}

#[tokio::test]
async fn journal_trace_run_matches_v0_actions_and_reports() {
    let root = temp();
    let legacy_config = control_config(10, 3, "default").unwrap();
    let mut compact_config = legacy_config.clone();
    compact_config.journal_trace = true;
    let legacy = run(&root.join("legacy"), 82100, legacy_config, 20, 60)
        .await
        .unwrap();
    let compact = run(&root.join("compact"), 82100, compact_config, 20, 60)
        .await
        .unwrap();
    // Same seed: every committed actor, action and RNG position is identical,
    // so the measured opportunity windows must be too.
    for trial in ["parent", "information", "structure", "both", "sham"] {
        let a = journal(&root.join("legacy").join(trial));
        let b = journal(&root.join("compact").join(trial));
        assert_eq!(a.len(), b.len(), "{trial}");
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(x["actor"], y["actor"], "{trial}");
            assert_eq!(x["outcome"]["action"], y["outcome"]["action"], "{trial}");
            assert_eq!(
                x["after_streams"]["positions"], y["after_streams"]["positions"],
                "{trial}"
            );
        }
    }
    assert_eq!(
        serde_json::to_value(&legacy.per_arm).unwrap(),
        serde_json::to_value(&compact.per_arm).unwrap()
    );
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("compact/parent/manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["world_kind"], KIND_JOURNAL_TRACE);
    assert_eq!(
        serde_json::to_value(verify(&root.join("compact")).unwrap().per_arm).unwrap(),
        serde_json::to_value(&compact.per_arm).unwrap()
    );
    verify(&root.join("legacy")).unwrap();

    // The rebuilt trajectory equals the V0 world's stored trace suffix.
    for arm in ["information", "both"] {
        let old =
            Store::<ReputationWorld, Action>::inspect_read_only(root.join("legacy").join(arm))
                .unwrap();
        let new =
            Store::<ReputationWorld, Action>::inspect_read_only(root.join("compact").join(arm))
                .unwrap();
        assert!(new.world.trace.is_empty());
        let (replayed, trace) = replay_journal_trace(
            &root.join("compact").join(arm).join("events.jsonl"),
            new.log_bytes,
            &new.genesis.initial_world,
            1e-9,
        )
        .unwrap();
        assert_eq!(replayed, new.world);
        assert_eq!(&old.world.trace[20..], &trace[..]);
        assert_eq!(
            serde_json::to_value(window(&old.world, 21, 60).unwrap()).unwrap(),
            serde_json::to_value(window_of(&new.world.config, &trace, 21, 60).unwrap()).unwrap()
        );
    }
    fs::remove_dir_all(root).unwrap();
}

/// Trajectory of a journal-trace run, extracted without `StepTrace::after`:
/// every field is read from the raw journal JSON and the genesis world, and
/// the uninterrupted-link count from a separately replayed world.
fn independent_trace(dir: &Path) -> Vec<serde_json::Value> {
    let ro = Store::<ReputationWorld, Action>::inspect_read_only(dir).unwrap();
    let mut world = ro.genesis.initial_world.clone();
    let mut step = world.step;
    let mut rows = Vec::new();
    for record in journal(dir) {
        let outcome = &record["outcome"];
        if outcome["outcome"] != "accepted" {
            continue;
        }
        let actor: u64 = record["actor"]
            .as_str()
            .map(|s| s.parse().unwrap())
            .unwrap_or_else(|| record["actor"].as_u64().unwrap());
        let action: Action = serde_json::from_value(outcome["action"].clone()).unwrap();
        world.apply(AgentId(actor), action).unwrap();
        step += 1;
        let recipient = outcome["action"]["recipient"].as_u64().unwrap() as usize;
        let mut row = serde_json::json!({
            "step": step,
            "allocator": actor,
            "recipient": recipient,
            "sign": if ro.genesis.initial_world.boost[recipient] { 1 } else { -1 },
            "expectation": outcome["action"]["pre_flow"],
            "rewire": outcome["action"].get("rewire").cloned().unwrap_or(serde_json::Value::Null),
        });
        if world.config.survival_accounting
            && let Some(rows) = &world.surviving_edges
        {
            let count: usize = rows.iter().map(|r| r.len()).sum();
            row["surviving_count"] = serde_json::json!(count);
        }
        rows.push(row);
    }
    rows
}

#[tokio::test]
async fn journal_trajectory_matches_an_independent_extraction_and_golden_digest() {
    let root = temp();
    let mut config = control_config(10, 3, "default").unwrap();
    config.journal_trace = true;
    run(&root, 82102, config, 20, 60).await.unwrap();
    let mut digests = Vec::new();
    for arm in ["information", "structure", "both", "sham"] {
        let dir = root.join(arm);
        let ro = Store::<ReputationWorld, Action>::inspect_read_only(&dir).unwrap();
        let (_, trace) = replay_journal_trace(
            &dir.join("events.jsonl"),
            ro.log_bytes,
            &ro.genesis.initial_world,
            1e-9,
        )
        .unwrap();
        let derived = serde_json::to_value(&trace).unwrap();
        assert_eq!(
            derived,
            serde_json::Value::Array(independent_trace(&dir)),
            "{arm}"
        );
        assert!(window(&ro.world, 21, 60).is_err());
        digests.push(format!("{arm}:{}", hiyal::storage::digest(&trace).unwrap()));
    }
    // Pinned for seed 82102: a change in how trajectories are derived must
    // be deliberate and re-pinned here.
    assert_eq!(digests, GOLDEN_TRACE_DIGESTS);
    fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn edited_journal_trace_report_is_rejected() {
    let root = temp();
    let mut config = control_config(10, 3, "default").unwrap();
    config.journal_trace = true;
    run(&root, 82101, config, 20, 60).await.unwrap();
    let path = root.join("report.json");
    let mut report: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    report["per_arm"][0]["late"]["opportunity_gap"] = serde_json::json!(0.123);
    fs::write(&path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    assert!(verify(&root).is_err());
    fs::remove_dir_all(root).unwrap();
}
