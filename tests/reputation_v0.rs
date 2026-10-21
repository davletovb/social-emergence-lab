//! Experiment 002: mechanism, conservation, and genuine Hiyal fork integration.
use hiyal::{
    core::{AgentId, World},
    experiment::EffectPolicy,
    rng::Streams,
    storage::Store,
};
use social_emergence_lab::reputation::{
    Action, Arm, Config, Removal, ReputationPolicy, ReputationWorld, Rewire, flow, regular_graph,
    setup, validate_graph, window,
};
use social_emergence_lab::reputation_pilot::{control_config, run, verify};
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
    // A zero budget is a configuration error; a budget of one is exhausted by
    // any rejected pairing, which most N=6, d=2 pairings are.
    let mut budget = w.config.clone();
    budget.graph_attempts = 0;
    assert!(budget.validate().is_err());
    budget.graph_attempts = 1;
    let error = (0..64)
        .find_map(|seed| {
            let mut rng = Streams::new("reputation-budget".into(), seed);
            regular_graph(&budget, &mut rng, "setup/initial_graph").err()
        })
        .unwrap();
    assert!(error.contains("not found after 1 bounded draws"), "{error}");
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
    // One rewiring makes the in-degrees irregular; the degree-only
    // expectation must still agree with a direct in-degree calculation.
    w.config.removal = Removal::Uniform;
    w.config.rewire_every = 1;
    let removed = w.graph[0][0];
    let added = (1..10).find(|i| !w.graph[0].contains(i)).unwrap();
    let action = Action {
        recipient: w.graph[0][1],
        success: true,
        pre_flow: flow(&w).unwrap(),
        annealed_graph: None,
        rewire: Some(Rewire {
            allocator: 0,
            removed,
            added,
        }),
    };
    w.apply(AgentId(0), action).unwrap();
    assert!(validate_graph(&w.graph, 10, 2).is_err());
    let result = flow(&w).unwrap();
    assert!(result.selection.abs() < 1e-12);
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
        annealed_graph: None,
    };
    assert!(w.apply(AgentId(0), bad).is_err());
    assert_eq!(w, prior);
    let mut rng = Streams::new("actor-validation".into(), 23);
    for t in 1..=50 {
        let actor = AgentId((t % 6) as u64);
        let observation = w.observe(actor).unwrap();
        let a = ReputationPolicy
            .decide_with_streams(actor, &observation, &[], &mut rng)
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
    let annealed = control_config(20, 4, "annealed-mixing").unwrap();
    assert!(annealed.annealed);
    assert_eq!(annealed.removal, Removal::None);
    let complete = control_config(20, 4, "full-mixing").unwrap();
    assert_eq!(complete.degree, 19);
    assert_eq!(complete.removal, Removal::None);
    assert_eq!(
        control_config(20, 4, "rewire-every-2")
            .unwrap()
            .rewire_every,
        2
    );
    assert!(control_config(20, 19, "default").is_err());
    let mut rng = Streams::new("reputation-test".into(), 5);
    let symmetric = setup(&null, &mut rng).unwrap();
    assert!(symmetric.public.iter().all(|&p| p == 0.5));
    assert!(symmetric.beliefs.iter().flatten().all(|&p| p == 0.5));
}

#[tokio::test]
async fn fixed_graph_control_never_rewires_in_any_arm() {
    let root = temp();
    let cfg = control_config(6, 2, "fixed-graph").unwrap();
    run(&root, 81403, cfg, 20, 60).await.unwrap();
    let parent = Store::<ReputationWorld, Action>::inspect_read_only(root.join("parent")).unwrap();
    let initial = &parent.genesis.initial_world;
    assert_eq!(parent.world.graph, initial.graph);
    for arm in Arm::ALL {
        let ro =
            Store::<ReputationWorld, Action>::inspect_read_only(root.join(arm.name())).unwrap();
        let expected = match arm {
            Arm::Structure | Arm::Both => &initial.repair_graph,
            Arm::Sham | Arm::Information => &initial.graph,
        };
        assert_eq!(&ro.world.graph, expected, "{}", arm.name());
        assert!(ro.world.trace.iter().all(|t| t.rewire.is_none()));
    }
    verify(&root).unwrap();
    std::fs::remove_dir_all(root).unwrap();
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
    // The report is re-derivable from the journals alone, and only from them.
    let verified = verify(&root).unwrap();
    assert_eq!(
        serde_json::to_value(&verified).unwrap(),
        serde_json::to_value(&report).unwrap()
    );
    let path = root.join("report.json");
    let original = std::fs::read_to_string(&path).unwrap();
    std::fs::write(
        &path,
        original.replacen("\"seed\": 81402", "\"seed\": 81403", 1),
    )
    .unwrap();
    assert!(verify(&root).unwrap_err().contains("not derivable"));
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

#[test]
fn removed_then_readded_checkpoint_edge_is_not_counted_as_inherited() {
    let mut w = built(6, 2, 777);
    w.config.removal = Removal::Uniform;
    w.config.rewire_every = 1;
    w.intervene(Arm::Sham).unwrap();
    let before = w.graph[0].clone();
    let old = before[0];
    let added = (0..6).find(|&i| i != 0 && !before.contains(&i)).unwrap();
    for (remove, insert) in [(old, added), (added, old)] {
        let action = Action {
            recipient: w.graph[0][1],
            success: true,
            pre_flow: flow(&w).unwrap(),
            annealed_graph: None,
            rewire: Some(Rewire {
                allocator: 0,
                removed: remove,
                added: insert,
            }),
        };
        w.apply(AgentId(0), action).unwrap();
    }
    assert!(w.graph[0].contains(&old), "the original edge was re-added");
    assert!(
        !w.surviving_edges.as_ref().unwrap()[0].contains(&old),
        "re-added edges must never be called uninterrupted inheritance"
    );
    w.verify().unwrap();
    let measurement = flow(&w).unwrap();
    let reference_inherited = w
        .surviving_edges
        .as_ref()
        .unwrap()
        .iter()
        .flat_map(|row| row.iter())
        .map(|&i| if w.boost[i] { 1.0 } else { -1.0 })
        .sum::<f64>()
        / 12.0;
    assert!((measurement.inherited - reference_inherited).abs() < 1e-12);
    assert!((measurement.inherited + measurement.added - measurement.degree).abs() < 1e-12);
}

#[test]
fn v1_independent_reference_agrees_before_and_after_real_rewires() {
    use social_emergence_lab::reputation_reference;
    let mut w = built(10, 3, 601);
    let mut rng = Streams::new("reference-policy-v1".into(), 81001);
    for step in 0..40 {
        let j = step % 10;
        reputation_reference::check(&w, 1e-12).unwrap();
        let observation = w.observe(AgentId(j as u64)).unwrap();
        let action = ReputationPolicy
            .decide_with_streams(AgentId(j as u64), &observation, &[], &mut rng)
            .unwrap();
        w.apply(AgentId(j as u64), action).unwrap();
    }
    w.intervene(Arm::Information).unwrap();
    for step in 40..80 {
        let j = step % 10;
        reputation_reference::check(&w, 1e-12).unwrap();
        let observation = w.observe(AgentId(j as u64)).unwrap();
        let action = ReputationPolicy
            .decide_with_streams(AgentId(j as u64), &observation, &[], &mut rng)
            .unwrap();
        w.apply(AgentId(j as u64), action).unwrap();
    }
    reputation_reference::check(&w, 1e-12).unwrap();
    assert!(
        window(&w, 41, 80)
            .unwrap()
            .mean_surviving_edge_fraction
            .is_some()
    );
}
#[test]
fn v1_annealed_degree_matched_control_replaces_eligibility_in_logged_action() {
    use social_emergence_lab::reputation_reference;
    let mut w = built(10, 3, 602);
    w.config.removal = Removal::None;
    w.config.annealed = true;
    let mut rng = Streams::new("annealed-policy-v1".into(), 82001);
    for step in 0..12 {
        let j = step % 10;
        let observation = w.observe(AgentId(j as u64)).unwrap();
        let action = ReputationPolicy
            .decide_with_streams(AgentId(j as u64), &observation, &[], &mut rng)
            .unwrap();
        let sampled = action.annealed_graph.clone().expect("logged sampled graph");
        validate_graph(&sampled, 10, 3).unwrap();
        assert!(sampled[j].contains(&action.recipient));
        let mut reference = w.clone();
        reference.graph = sampled;
        if let Some(ref mut surviving) = reference.surviving_edges {
            for (row, links) in surviving.iter_mut().zip(&reference.graph) {
                row.retain(|i| links.contains(i));
            }
        }
        reputation_reference::check(&reference, 1e-12).unwrap();
        assert!((action.pre_flow.total - flow(&reference).unwrap().total).abs() < 1e-12);
        w.apply(AgentId(j as u64), action).unwrap();
    }
    assert_eq!(w.step, 12);
}
#[test]
fn v1_complete_network_control_is_regular_and_has_no_rewiring() {
    use social_emergence_lab::reputation_reference;
    let c = control_config(20, 4, "full-mixing").unwrap();
    assert_eq!(c.degree, 19);
    let mut rng = Streams::new("full-mixing-v1".into(), 83001);
    let w = setup(&c, &mut rng).unwrap();
    validate_graph(&w.graph, 20, 19).unwrap();
    assert!(w.graph.iter().enumerate().all(|(j, row)| !row.contains(&j)));
    reputation_reference::check(&w, 1e-12).unwrap();
    assert_eq!(w.config.removal, Removal::None);
}
#[tokio::test]
async fn v1_annealed_four_fork_stand_in_replays() {
    let root = temp();
    let c = control_config(6, 2, "annealed-mixing").unwrap();
    let report = run(&root, 82995, c, 20, 60).await.unwrap();
    assert_eq!(report.per_arm.len(), 4);
    assert!(report.per_arm.iter().all(|a| a.full.rewire_events == 0));
    let check = social_emergence_lab::reputation_pilot::verify(&root).unwrap();
    assert_eq!(
        serde_json::to_value(check).unwrap(),
        serde_json::to_value(report).unwrap()
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn v1_legacy_v0_trace_hash_semantics_remain_unchanged() {
    // A historical V0 Config did not contain the opt-in V1 accounting flag.
    // The deserializer must leave it false, and the reducer must not invent
    // formerly absent per-event trace fields on fork replay.
    let mut stored = serde_json::to_value(Config::default_for(6, 2)).unwrap();
    let object = stored.as_object_mut().unwrap();
    object.remove("survival_accounting");
    object.remove("annealed");
    let cfg: Config = serde_json::from_value(stored).unwrap();
    assert!(!cfg.survival_accounting);
    let mut rng = Streams::new("legacy-fork-v0".into(), 98231);
    let mut w = setup(&cfg, &mut rng).unwrap();
    w.config.removal = Removal::None;
    w.intervene(Arm::Information).unwrap();
    let mut policy_rng = Streams::new("legacy-policy-v0".into(), 98232);
    let observation = w.observe(AgentId(0)).unwrap();
    let action = ReputationPolicy
        .decide_with_streams(AgentId(0), &observation, &[], &mut policy_rng)
        .unwrap();
    w.apply(AgentId(0), action).unwrap();
    assert!(w.trace.last().unwrap().surviving_count.is_none());
    let serialized = serde_json::to_value(&w).unwrap();
    assert!(
        serialized
            .get("config")
            .unwrap()
            .get("survival_accounting")
            .is_none()
    );
    assert!(
        serialized.get("trace").unwrap()[0]
            .get("surviving_count")
            .is_none()
    );
    let v1 = Config::default_for(6, 2);
    assert!(v1.survival_accounting);
}

#[test]
fn archived_v0_run_still_verifies() {
    // One run of the archived V0 batch (see tests/fixtures/README.md).
    let root = temp();
    std::fs::create_dir_all(&root).unwrap();
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/reputation-v0-bc870fe-n6-default-81000.tar.gz"
    );
    let status = std::process::Command::new("tar")
        .arg("-xzf")
        .arg(fixture)
        .arg("-C")
        .arg(&root)
        .status()
        .unwrap();
    assert!(status.success());
    let report = verify(&root.join("n6-default-81000")).unwrap();
    assert_eq!(report.seed, 81000);
    assert_eq!(
        report.hiyal_revision,
        "4bb53aca5cffa6bc01b7558efbd690af7dbed79f"
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn reference_replay_follows_the_journal_and_rejects_a_wrong_start() {
    use social_emergence_lab::reputation_reference::replay_journal;
    let root = temp();
    run(
        &root,
        81404,
        control_config(6, 2, "default").unwrap(),
        20,
        60,
    )
    .await
    .unwrap();
    let arm = root.join("sham");
    let ro = Store::<ReputationWorld, Action>::inspect_read_only(&arm).unwrap();
    let journal = arm.join("events.jsonl");
    let initial = &ro.genesis.initial_world;
    let replayed = replay_journal(&journal, ro.log_bytes, initial, 1e-9).unwrap();
    assert_eq!(replayed, ro.world);
    // Swapping one boosted and one unboosted label keeps the world valid but
    // contradicts every recorded expectation.
    let mut wrong = initial.clone();
    let on = wrong.boost.iter().position(|&b| b).unwrap();
    let off = wrong.boost.iter().position(|&b| !b).unwrap();
    wrong.boost.swap(on, off);
    let error = replay_journal(&journal, ro.log_bytes, &wrong, 1e-9).unwrap_err();
    assert!(error.contains("differs from the reference"), "{error}");
    std::fs::remove_dir_all(root).unwrap();
}
