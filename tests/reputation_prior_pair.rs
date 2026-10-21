//! V2 speed-path and paired-prior scientific-control regression.
use hiyal::core::{AgentId,World};
use hiyal::rng::Streams;
use hiyal::storage::Store;
use social_emergence_lab::reputation::{Action,Config,ReputationWorld,setup};
use social_emergence_lab::reputation_prior_pair::{run_pair,verify_pair};
use std::{path::PathBuf,sync::atomic::{AtomicUsize,Ordering}};
fn root()->PathBuf {
    static COUNTER:AtomicUsize=AtomicUsize::new(0);
    std::env::temp_dir().join(format!("reputation-prior-pair-{}-{}",std::process::id(),COUNTER.fetch_add(1,Ordering::Relaxed)))
}
#[test]
fn compact_policy_observation_excludes_growing_trace_and_preserves_input() {
    let mut rng=Streams::new("v2-observe".into(),8101);
    let mut w=setup(&Config::default_for(6,2),&mut rng).unwrap();
    let original=serde_json::to_vec(&w).unwrap();
    let view=w.observe(AgentId(0)).unwrap();
    assert_eq!(view.step,w.step);
    assert_eq!(view.graph,w.graph);
    assert_eq!(view.beliefs,w.beliefs);
    assert_eq!(view.boost,w.boost);
    assert_eq!(view.public,w.public);
    assert_eq!(original,serde_json::to_vec(&w).unwrap());
    // A large recorded trace never travels through observe().
    // The transient type contains no trace at all.
    w.trace.reserve(1000);
    let observed=w.observe(AgentId(1)).unwrap();
    assert_eq!(observed.step,w.step);
    assert_eq!(observed.surviving_edges,w.surviving_edges);
}
#[tokio::test]
async fn same_seed_false_prior_and_zero_prior_share_genesis_but_not_history() {
    let dir=root();
    let result=run_pair(&dir,654321,Config::default_for(6,2),20,60).await.unwrap();
    assert_eq!(result.seed,654321);
    assert!(result.matched_setup);
    let with=Store::<ReputationWorld,Action>::inspect_read_only(dir.join("false-prior/parent")).unwrap();
    let without=Store::<ReputationWorld,Action>::inspect_read_only(dir.join("no-false-prior/parent")).unwrap();
    assert_eq!(with.genesis.initial_world.boost,without.genesis.initial_world.boost);
    assert_eq!(with.genesis.initial_world.graph,without.genesis.initial_world.graph);
    assert_eq!(with.genesis.initial_world.repair_graph,without.genesis.initial_world.repair_graph);
    assert_eq!(with.genesis.initial_streams,without.genesis.initial_streams);
    assert_eq!(with.genesis.initial_world.config.delta,0.2);
    assert_eq!(without.genesis.initial_world.config.delta,0.0);
    let saved=verify_pair(&dir).unwrap();
    assert!((saved.matched_seed_prior_effect-result.matched_seed_prior_effect).abs()<1e-12);
    assert!(dir.join("false-prior/both/events.jsonl").exists());
    assert!(dir.join("no-false-prior/information/events.jsonl").exists());
    // Cached pair reports cannot silently be edited or substituted.
    let report=dir.join("paired-report.json");
    let original=std::fs::read(&report).unwrap();
    std::fs::write(&report,b"{}").unwrap();
    assert!(verify_pair(&dir).unwrap_err().contains("differs from verified journals"));
    std::fs::write(&report,original).unwrap();
    assert!(verify_pair(&dir).is_ok());
    std::fs::remove_dir_all(dir).unwrap();
}
