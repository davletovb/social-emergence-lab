//! The in-place reducer of `ReputationWorld` (`apply_undoable`) against the
//! copy-based reducer it replaced, kept here verbatim as the reference:
//! same accepted world bit for bit, same rejections with the same messages
//! (they are recorded in journals), and an exact undo.
use hiyal::{
    core::{AgentId, World},
    experiment::EffectPolicy,
    rng::Streams,
    storage::digest,
};
use social_emergence_lab::reputation::{
    Action, Arm, Config, Removal, ReputationPolicy, ReputationWorld, StepTrace, flow, setup,
};

/// The reducer before in-place staging, unchanged apart from taking the
/// world as an argument.
fn apply_by_copy(world: &mut ReputationWorld, actor: AgentId, a: Action) -> Result<(), String> {
    let j = usize::try_from(actor.0).map_err(|e| e.to_string())?;
    let n = world.config.actors;
    if j >= n {
        return Err("allocator out of range".into());
    }
    let pre = world.pre_event_state(&a)?;
    if !pre.graph[j].contains(&a.recipient) {
        return Err("recipient not eligible".into());
    }
    let now = flow(&pre)?;
    for (v, x) in [
        (now.degree, a.pre_flow.degree),
        (now.inherited, a.pre_flow.inherited),
        (now.added, a.pre_flow.added),
        (now.selection, a.pre_flow.selection),
        (now.total, a.pre_flow.total),
    ] {
        if (v - x).abs() > 1e-9 {
            return Err("conditional exposure reference mismatch".into());
        }
    }
    let step = world.step.checked_add(1).ok_or("step overflow")?;
    let requires = world.config.removal != Removal::None && step % world.config.rewire_every == 0;
    if requires != a.rewire.is_some() {
        return Err("incorrect rewiring event timing".into());
    }
    // Stage all changes atomically, as Hiyal also stages its own world.
    let mut w = pre;
    w.wealth[a.recipient] = w.wealth[a.recipient]
        .checked_add(1)
        .ok_or("wealth overflow")?;
    let r = a.recipient;
    w.beliefs[j][r] = (1.0 - w.config.learning) * w.beliefs[j][r]
        + w.config.learning * f64::from(u8::from(a.success));
    w.public[r] = (1.0 - w.config.public_learning) * w.public[r]
        + w.config.public_learning * f64::from(u8::from(a.success));
    if let Some(ref change) = a.rewire {
        if change.allocator >= n
            || change.added >= n
            || change.removed >= n
            || change.added == change.removed
            || change.added == change.allocator
            || !w.graph[change.allocator].contains(&change.removed)
            || w.graph[change.allocator].contains(&change.added)
        {
            return Err("invalid replacement edge".into());
        }
        if w.config.removal == Removal::LowestBelief {
            let low = w.graph[change.allocator]
                .iter()
                .map(|&i| w.beliefs[change.allocator][i])
                .fold(f64::INFINITY, f64::min);
            if (w.beliefs[change.allocator][change.removed] - low).abs() > 1e-12 {
                return Err("rewiring did not remove lowest-belief neighbor".into());
            }
        }
        let idx = w.graph[change.allocator]
            .iter()
            .position(|&x| x == change.removed)
            .ok_or("missing edge")?;
        w.graph[change.allocator][idx] = change.added;
        if let Some(ref mut surviving) = w.surviving_edges {
            surviving[change.allocator].retain(|&i| i != change.removed);
        }
    }
    w.step = step;
    if !w.config.journal_trace {
        let entry = StepTrace::after(&w, j, &a);
        w.trace.push(entry);
    }
    w.verify()?;
    *world = w;
    Ok(())
}

/// Apply `action` with both reducers to copies of `world`; they must agree.
/// The in-place undo must then restore `world` exactly. Returns the result.
fn agree(
    world: &ReputationWorld,
    actor: AgentId,
    action: &Action,
) -> Result<ReputationWorld, String> {
    let before = digest(world).unwrap();
    let mut by_copy = world.clone();
    let expected = apply_by_copy(&mut by_copy, actor, action.clone());
    let mut in_place = world.clone();
    let mut streams = Streams::new("unused".into(), 0);
    let got = in_place.apply_undoable(actor, action.clone(), &mut streams);
    match (&expected, got) {
        (Ok(()), Ok(undo)) => {
            assert_eq!(
                digest(&in_place).unwrap(),
                digest(&by_copy).unwrap(),
                "same world"
            );
            undo.undo(&mut in_place);
            assert_eq!(digest(&in_place).unwrap(), before, "exact undo");
            Ok(by_copy)
        }
        (Err(e), Err(g)) => {
            assert_eq!(e, &g, "same rejection message");
            assert_eq!(
                digest(&in_place).unwrap(),
                before,
                "a rejection leaves the world as it was"
            );
            Err(g)
        }
        (e, g) => panic!("reducers disagree: copy {e:?}, in place {:?}", g.map(drop)),
    }
}

/// Variants of a valid action that each break one rule the reducer checks,
/// some of them only after it has already changed the world.
fn broken(world: &ReputationWorld, actor: AgentId, action: &Action) -> Vec<(AgentId, Action)> {
    let n = world.config.actors;
    let j = actor.0 as usize;
    let mut out = vec![(AgentId(n as u64), action.clone())];
    let mut a = action.clone();
    a.recipient = (0..n).find(|r| !world.graph[j].contains(r)).unwrap_or(j);
    out.push((actor, a));
    let mut a = action.clone();
    a.pre_flow.total += 1.0;
    out.push((actor, a));
    let mut a = action.clone();
    a.annealed_graph = if a.annealed_graph.is_some() {
        None
    } else {
        Some(world.graph.clone())
    };
    out.push((actor, a));
    let mut a = action.clone();
    a.rewire = match a.rewire.take() {
        Some(_) => None,
        None => Some(social_emergence_lab::reputation::Rewire {
            allocator: j,
            removed: 0,
            added: 1,
        }),
    };
    out.push((actor, a));
    if let Some(rewire) = &action.rewire {
        // Rejected after wealth and beliefs were updated in place.
        let mut a = action.clone();
        a.rewire.as_mut().unwrap().added = rewire.removed;
        out.push((actor, a));
        let row = &world.graph[rewire.allocator];
        if let Some(&other) = row.iter().find(|&&i| i != rewire.removed) {
            let mut a = action.clone();
            a.rewire.as_mut().unwrap().removed = other;
            out.push((actor, a));
        }
    }
    out
}

/// Rejection messages seen, so the tests can show every path was taken.
type Seen = std::collections::BTreeMap<String, u64>;

fn run(config: Config, seed: u64, steps: u64, arm: Option<Arm>, seen: &mut Seen) {
    let mut rng = Streams::new(format!("in-place-{seed}"), seed);
    let mut world = setup(&config, &mut rng).unwrap();
    let n = config.actors as u64;
    for t in 0..steps {
        if t == steps / 2
            && let Some(arm) = arm
        {
            world.intervene(arm).unwrap();
        }
        let actor = AgentId(t % n);
        let observation = world.observe(actor).unwrap();
        let action = ReputationPolicy
            .decide_with_streams(actor, &observation, &[], &mut rng)
            .unwrap();
        // Not every variant is illegal (uniform removal may drop any
        // neighbour, beliefs may tie): what must hold is that both agree.
        for (who, variant) in broken(&world, actor, &action) {
            if let Err(message) = agree(&world, who, &variant) {
                *seen.entry(message).or_default() += 1;
            }
        }
        world = agree(&world, actor, &action).unwrap();
    }
    assert_eq!(world.step, steps);
}

#[test]
fn in_place_reducer_matches_the_copy_reducer_across_configurations() {
    let base = Config::default_for(12, 3);
    let mut uniform = base.clone();
    uniform.removal = Removal::Uniform;
    let mut fixed = base.clone();
    fixed.removal = Removal::None;
    let mut annealed = fixed.clone();
    annealed.annealed = true;
    let mut journal_trace = base.clone();
    journal_trace.journal_trace = true;
    let mut legacy = base.clone();
    legacy.survival_accounting = false;
    let mut fast_rewire = base.clone();
    fast_rewire.rewire_every = 1;
    let mut seen = Seen::new();
    for (config, arm) in [
        (base.clone(), None),
        (base.clone(), Some(Arm::Both)),
        (uniform, Some(Arm::Information)),
        (fixed, Some(Arm::Structure)),
        (annealed, Some(Arm::Sham)),
        (journal_trace, Some(Arm::Both)),
        (legacy, None),
        (fast_rewire, Some(Arm::Structure)),
    ] {
        for seed in [1, 2] {
            run(config.clone(), seed, 60, arm, &mut seen);
        }
    }
    // Every rejection path was exercised, including the two that fire after
    // wealth and beliefs were already changed in place.
    for message in [
        "allocator out of range",
        "recipient not eligible",
        "conditional exposure reference mismatch",
        "unexpected annealed graph",
        "annealed policy must provide a fresh graph",
        "incorrect rewiring event timing",
        "invalid replacement edge",
        "rewiring did not remove lowest-belief neighbor",
    ] {
        assert!(
            seen.get(message).is_some_and(|&n| n > 0),
            "never saw {message:?}: {seen:?}"
        );
    }
}

#[test]
fn a_wealth_overflow_is_rejected_by_both_reducers_and_changes_nothing() {
    let config = Config::default_for(10, 3);
    let mut rng = Streams::new("overflow".into(), 9);
    let mut world = setup(&config, &mut rng).unwrap();
    let actor = AgentId(0);
    let observation = world.observe(actor).unwrap();
    let action = ReputationPolicy
        .decide_with_streams(actor, &observation, &[], &mut rng)
        .unwrap();
    world.wealth[action.recipient] = u64::MAX;
    assert_eq!(
        agree(&world, actor, &action).unwrap_err(),
        "wealth overflow"
    );
}

#[test]
fn trace_records_match_in_both_reducers() {
    // StepTrace is part of legacy V0 state: built by the same rule either way.
    let config = Config::default_for(10, 3);
    let mut rng = Streams::new("trace".into(), 4);
    let mut world = setup(&config, &mut rng).unwrap();
    for t in 0..25 {
        let actor = AgentId(t % 10);
        let observation = world.observe(actor).unwrap();
        let action = ReputationPolicy
            .decide_with_streams(actor, &observation, &[], &mut rng)
            .unwrap();
        world = agree(&world, actor, &action).unwrap();
    }
    assert_eq!(world.trace.len(), 25);
    let last: &StepTrace = world.trace.last().unwrap();
    assert_eq!(last.step, 25);
    let _ = flow(&world).unwrap();
}
