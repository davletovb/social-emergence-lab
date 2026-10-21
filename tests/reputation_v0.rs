//! Experiment 002: mechanism, conservation, and genuine Hiyal fork integration.
use hiyal::{
    core::{AgentId, World},
    experiment::EffectPolicy,
    rng::Streams,
};
use social_emergence_lab::reputation::{
    Action, Arm, Config, Removal, ReputationPolicy, ReputationWorld, flow, regular_graph, setup,
    validate_graph, window,
};
use social_emergence_lab::reputation_pilot::{control_config, run};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

fn temp() -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    std::env::temp_dir().join(format!(
        "social-reputation-test-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ))
}
fn built(n: usize, d: usize, seed: u64) -> ReputationWorld {
    let cfg = Config::default_for(n, d);
    let mut rng = Streams::new("reputation-test".into(), seed);
    setup(&cfg, &mut rng).unwrap()
}

#[test]
fn graph_sampling_is_bounded_and_has_exact_degrees() {
    let w = built(6, 2, 11);
    for a in [&w.graph, &w.repair_graph] {
        validate_graph(a, 6, 2).unwrap();
    }
    assert_eq!(w.boost.iter().filter(|&&x| x).count(), 3);
    assert!(
        w.boost
            .iter()
            .zip(&w.public)
            .all(|(b, p)| (*p - (if *b { 0.7 } else { 0.3 })).abs() < 1e-12)
    );
    let mut rng = Streams::new("reputation-test".into(), 11);
    assert_eq!(setup(&w.config, &mut rng).unwrap(), w);
    assert_eq!(rng.positions.len(), 3);
    let mut impossible = w.config.clone();
    impossible.graph_attempts = 0;
    assert!(regular_graph(&impossible, &mut rng, "setup/bad").is_err());
}
#[test]
fn exact_exposure_identity_and_zero_beta_selection() {
    let mut w = built(10, 2, 13);
    let result = flow(&w).unwrap();
    assert!((result.degree + result.selection - result.total).abs() < 1e-12);
    assert!((result.inherited + result.added - result.degree).abs() < 1e-12);
    w.config.beta = 0.0;
    let result = flow(&w).unwrap();
    assert!(result.selection.abs() < 1e-12);
    assert!((result.degree - result.total).abs() < 1e-12);
    // An irregular graph is permitted after rewiring; its degree-only
    // expectation must agree with direct in-degree calculation.
    let direct = w
        .graph
        .iter()
        .flat_map(|row| row.iter())
        .map(|&i| if w.boost[i] { 1.0 } else { -1.0 })
        .sum::<f64>()
        / (w.config.actors * w.config.degree) as f64;
    assert!((result.degree - direct).abs() < 1e-12);
}
#[test]
fn valid_policy_actions_preserve_conservation_and_invalid_actions_do_not_mutate() {
    let mut w = built(6, 2, 51);
    let prior = w.clone();
    let bad = Action {
        recipient: 6,
        success: true,
        rewire: None,
        pre_flow: flow(&w).unwrap(),
    };
    assert!(w.apply(AgentId(0), bad).is_err());
    assert_eq!(w, prior);
    let mut rng = Streams::new("actor-validation".into(), 23);
    for t in 1..=50 {
        let actor = AgentId((t % 6) as u64);
        let a = ReputationPolicy
            .decide_with_streams(actor, &w, &[], &mut rng)
            .unwrap();
        assert!(w.graph[actor.0 as usize].contains(&a.recipient));
        w.apply(actor, a).unwrap();
        w.verify().unwrap();
    }
    assert_eq!(w.wealth.iter().sum::<u64>(), 110);
    assert_eq!(w.trace.len(), 50);
    let measured = window(&w, 1, 50).unwrap();
    assert!(
        (measured.degree + measured.selection + measured.innovation - measured.opportunity_gap)
            .abs()
            < 1e-10
    );
    let mut corrected = w.clone();
    corrected.intervene(Arm::Both).unwrap();
    assert_eq!(corrected.graph, corrected.repair_graph);
    assert!(
        corrected
            .beliefs
            .iter()
            .all(|v| v.iter().all(|&x| x == 0.5))
    );
    assert!(corrected.public.iter().all(|&x| x == 0.5));
    assert!(corrected.intervene(Arm::Both).is_err());
}
#[test]
fn negative_control_variants_are_explicit() {
    let null = control_config(10, 2, "no-false-prior").unwrap();
    assert_eq!(null.delta, 0.0);
    let flat = control_config(10, 2, "no-selection-bias").unwrap();
    assert_eq!(flat.beta, 0.0);
    let fixed = control_config(10, 2, "fixed-graph").unwrap();
    assert_eq!(fixed.removal, Removal::None);
    let churn = control_config(10, 2, "random-churn").unwrap();
    assert_eq!(churn.removal, Removal::Uniform);
    assert_eq!(churn.kappa, 0.0);
    assert!(control_config(20, 19, "default").is_err());
}
#[tokio::test]
async fn four_arm_hiyal_exploration_is_replayable() {
    let root = temp();
    let cfg = control_config(6, 2, "default").unwrap();
    let report = run(&root, 81402, cfg, 20, 60).await.unwrap();
    assert_eq!(report.per_arm.len(), 4);
    for arm in &report.per_arm {
        assert_eq!(arm.all_events, 60);
        assert_eq!(arm.final_tokens.iter().sum::<u64>(), 120);
        assert_eq!(arm.full.from, 21);
        assert_eq!(arm.full.through, 60);
        assert_eq!(arm.late.from, 21); // no artificial 100-event window with only 40 events
        assert_eq!(arm.late.through, 60);
        assert!(
            (arm.full.degree + arm.full.selection + arm.full.innovation - arm.full.opportunity_gap)
                .abs()
                < 1e-9
        );
    }
    assert!(root.join("parent/manifest.json").is_file());
    assert!(root.join("both/fork.json").is_file());
    assert!(root.join("report.json").is_file());
    assert!(
        run(
            &root,
            81402,
            control_config(6, 2, "default").unwrap(),
            20,
            60
        )
        .await
        .is_err()
    );
    std::fs::remove_dir_all(root).unwrap();
}
