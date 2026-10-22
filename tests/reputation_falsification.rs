//! Experiment 002 falsification/identification tests. These demonstrate
//! counterexamples to claims that a positive late gap requires ongoing
//! misinformation or belief selection.
use hiyal::{core::{AgentId, World}, experiment::EffectPolicy, rng::Streams};
use social_emergence_lab::reputation::{Arm, Config, Removal, ReputationPolicy, flow, setup, window};

#[test]
fn a_positive_gap_can_persist_with_zero_reputation_selection_and_no_rewiring() {
    let mut rng = Streams::new("falsification-graph".into(), 4021);
    let mut w = setup(&Config::default_for(10, 2), &mut rng).unwrap();

    // Create precisely ONE extra incoming edge for a randomly boosted label
    // by replacing a link to an unboosted label. This is a valid out-degree
    // graph, even though the in-degrees are now unequal.
    let mut change = None;
    for j in 0..w.config.actors {
        let old = w.graph[j].iter().position(|&i| !w.boost[i]);
        let new = (0..w.config.actors)
            .find(|&i| i != j && w.boost[i] && !w.graph[j].contains(&i));
        if let (Some(index), Some(target)) = (old, new) {
            change = Some((j, index, target));
            break;
        }
    }
    let (j, old, boosted) = change.expect("a replaceable unbiased initial edge exists");
    w.graph[j][old] = boosted;
    w.config.beta = 0.0; // Opportunity selection is exactly uniform.
    w.config.removal = Removal::None; // All opportunity links persist indefinitely.
    w.intervene(Arm::Information).unwrap(); // All beliefs are truthfully reset.
    let first = flow(&w).unwrap();
    assert!(first.degree > 0.0);
    assert!((first.degree - 2.0 / 20.0).abs() < 1e-12);
    assert!((first.inherited - first.degree).abs() < 1e-12);
    assert!(first.added.abs() < 1e-12);
    assert!(first.selection.abs() < 1e-12);
    assert!((first.total - first.inherited).abs() < 1e-12);

    // Learning may update beliefs, but beta=0 makes them irrelevant to
    // allocation. There is no new access mechanism and no network turnover.
    let mut decisions = Streams::new("falsification-policies".into(), 4052);
    for step in 0..20 {
        let actor = AgentId((step % w.config.actors) as u64);
        let observation = w.observe(actor).unwrap();
        let action = ReputationPolicy
            .decide_with_streams(actor, &observation, &[], &mut decisions)
            .unwrap();
        w.apply(actor, action).unwrap();
        let f = flow(&w).unwrap();
        assert!((f.total - first.total).abs() < 1e-12);
        assert!((f.inherited - first.inherited).abs() < 1e-12);
        assert!(f.added.abs() < 1e-12);
        assert!(f.selection.abs() < 1e-12);
    }
    let last = window(&w, 1, 20).unwrap();
    assert!(last.inherited > 0.0);
    assert!(last.added.abs() < 1e-12);
    assert!(last.selection.abs() < 1e-12);
    // Realized winners are stochastic: no assertion on observed signed gap.
}

#[test]
fn truth_plus_independent_graph_repair_makes_expected_gap_zero_at_reset() {
    let mut rng = Streams::new("falsification-reset".into(), 4031);
    let mut w = setup(&Config::default_for(10, 2), &mut rng).unwrap();
    w.intervene(Arm::Both).unwrap();
    let exact = flow(&w).unwrap();
    // The independent d-in/d-out regular graph gives label balance in
    // expected exposure. Truthfully reset beliefs produce uniform softmax
    // conditional choice even when beta is nonzero.
    assert!(exact.total.abs() < 1e-12);
    assert!(exact.degree.abs() < 1e-12);
    assert!(exact.selection.abs() < 1e-12);
    assert!((exact.inherited + exact.added).abs() < 1e-12);
    // Do not confuse a zero expected gap with zero realized outcomes.
}

#[test]
fn combined_repair_erases_initial_reputation_difference_from_access_state() {
    let mut cfg = Config::default_for(10, 2);
    let mut s0 = Streams::new("matched-prior-setup".into(), 4051);
    let mut a = setup(&cfg, &mut s0).unwrap();
    cfg.delta = 0.0;
    let mut s1 = Streams::new("matched-prior-setup".into(), 4051);
    let mut b = setup(&cfg, &mut s1).unwrap();
    assert_eq!(a.boost, b.boost);
    assert_eq!(a.graph, b.graph);
    assert_eq!(a.repair_graph, b.repair_graph);
    assert_eq!(s0.positions, s1.positions);
    let mut d0 = Streams::new("matched-prior-decisions".into(), 4052);
    let mut d1 = Streams::new("matched-prior-decisions".into(), 4052);
    for t in 0..30 {
        let actor = AgentId((t % 10) as u64);
        let aa = ReputationPolicy.decide_with_streams(actor, &a.observe(actor).unwrap(), &[], &mut d0).unwrap();
        let ab = ReputationPolicy.decide_with_streams(actor, &b.observe(actor).unwrap(), &[], &mut d1).unwrap();
        a.apply(actor, aa).unwrap();
        b.apply(actor, ab).unwrap();
    }
    a.intervene(Arm::Both).unwrap();
    b.intervene(Arm::Both).unwrap();
    assert_eq!(a.graph, b.graph);
    assert_eq!(a.public, b.public);
    assert_eq!(a.beliefs, b.beliefs);
    assert!((flow(&a).unwrap().total - flow(&b).unwrap().total).abs() < 1e-12);
    // Historical wealth, trace and link survival are not necessarily reset.
    // This only proves equality of the current conditional allocation rule,
    // NOT pathwise equality of post-reset random events.
}
