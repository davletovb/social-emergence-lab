//! Independent, algebraic evaluator of signed opportunity access.
//! Does not invoke production flow(), selection_probs() or softmax().
use crate::reputation::{Action, ReputationWorld, StepTrace};
use hiyal::core::{AgentId, World};
use serde_json::Value;
use std::{fs, path::Path};

#[derive(Clone, Debug, PartialEq)]
pub struct Reference {
    pub expected: f64,
    pub degree: f64,
    pub inherited: f64,
    pub newly_formed: f64,
    pub selection: f64,
    pub boost_indegree_minus_other: i64,
}

/// Re-evaluate the N-by-d choice probabilities from world state and the exact
/// observed graph, without using the production probability helper.
pub fn calculate(w: &ReputationWorld) -> Result<Reference, String> {
    w.config.validate()?;
    let n = w.config.actors;
    let d = w.config.degree;
    if w.graph.len() != n || w.boost.len() != n || w.beliefs.len() != n {
        return Err("inconsistent world dimensions".into());
    }
    let mut probability_by_candidate = vec![0f64; n];
    let mut incoming = vec![0i64; n];
    for j in 0..n {
        let row = w.graph.get(j).ok_or("missing adjacency row")?;
        if row.len() != d || w.beliefs[j].len() != n {
            return Err("incomplete reference state".into());
        }
        let mut used = vec![false; n];
        let mut maximum = f64::NEG_INFINITY;
        for &i in row {
            if i >= n || i == j || used[i] {
                return Err("malformed reference adjacency".into());
            }
            used[i] = true;
            incoming[i] += 1;
            maximum = maximum.max(w.config.beta * w.beliefs[j][i]);
        }
        let scaled = row
            .iter()
            .map(|&i| (w.config.beta * w.beliefs[j][i] - maximum).exp())
            .collect::<Vec<_>>();
        let normalizer = scaled.iter().sum::<f64>();
        if !normalizer.is_finite() || normalizer <= 0.0 {
            return Err("invalid probability normalizer".into());
        }
        for (k, &i) in row.iter().enumerate() {
            let p = w.config.epsilon / d as f64 + (1.0 - w.config.epsilon) * scaled[k] / normalizer;
            probability_by_candidate[i] += p / n as f64;
        }
    }
    let sign = |i: usize| if w.boost[i] { 1.0 } else { -1.0 };
    let expected = (0..n)
        .map(|i| sign(i) * probability_by_candidate[i])
        .sum::<f64>();
    let degree = (0..n).map(|i| sign(i) * incoming[i] as f64).sum::<f64>() / (n * d) as f64;
    let inherited = if let Some(unchanged) = &w.surviving_edges {
        if unchanged.len() != n {
            return Err("bad survival rows".into());
        }
        unchanged
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row.iter()
                    .map(|&i| {
                        if !w.graph[j].contains(&i) {
                            return f64::NAN;
                        }
                        sign(i) / (n * d) as f64
                    })
                    .sum::<f64>()
            })
            .sum::<f64>()
    } else {
        degree
    };
    if !inherited.is_finite() {
        return Err("survivor missing from graph".into());
    }
    let boost_indegree_minus_other = (0..n)
        .map(|i| {
            if w.boost[i] {
                incoming[i]
            } else {
                -incoming[i]
            }
        })
        .sum();
    Ok(Reference {
        expected,
        degree,
        inherited,
        newly_formed: degree - inherited,
        selection: expected - degree,
        boost_indegree_minus_other,
    })
}

pub fn check(w: &ReputationWorld, tolerance: f64) -> Result<(), String> {
    if !(tolerance.is_finite() && tolerance >= 0.0) {
        return Err("bad tolerance".into());
    }
    let independent = calculate(w)?;
    let p = crate::reputation::flow(w)?;
    for (a, b) in [
        (independent.expected, p.total),
        (independent.degree, p.degree),
        (independent.inherited, p.inherited),
        (independent.newly_formed, p.added),
        (independent.selection, p.selection),
    ] {
        if (a - b).abs() > tolerance {
            return Err(format!(
                "independent access identity differs by {}",
                (a - b).abs()
            ));
        }
    }
    if w.config.beta == 0.0 && independent.selection.abs() > tolerance {
        return Err("beta=0 produced an inexplicable selection component".into());
    }
    Ok(())
}

/// Replay a trial's Hiyal-verified journal prefix from its initial world,
/// checking every pre-event state against this reference and the action's
/// recorded expectation. Returns the replayed final world.
pub fn replay_journal(
    journal: &Path,
    verified_bytes: u64,
    initial: &ReputationWorld,
    tolerance: f64,
) -> Result<ReputationWorld, String> {
    replay_journal_trace(journal, verified_bytes, initial, tolerance).map(|(world, _)| world)
}

/// `replay_journal`, also returning one `StepTrace` per accepted event in
/// this journal, built from each event's action and post-event state. For a
/// journal-trace world this is its trajectory; for a legacy world it equals
/// the suffix of `world.trace` that the journal appended.
pub fn replay_journal_trace(
    journal: &Path,
    verified_bytes: u64,
    initial: &ReputationWorld,
    tolerance: f64,
) -> Result<(ReputationWorld, Vec<StepTrace>), String> {
    let bytes = fs::read(journal).map_err(|e| e.to_string())?;
    let length = usize::try_from(verified_bytes).map_err(|e| e.to_string())?;
    let prefix = bytes
        .get(..length)
        .ok_or("journal is shorter than the verified prefix")?;
    if prefix.last().is_some_and(|&b| b != b'\n') {
        return Err("verified prefix ends inside a record".into());
    }
    let mut world = initial.clone();
    let mut trace = Vec::new();
    for (index, line) in prefix.split(|&b| b == b'\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let record: Value = serde_json::from_slice(line)
            .map_err(|e| format!("journal record {}: {e}", index + 1))?;
        let outcome = record.get("outcome").ok_or("missing outcome")?;
        match outcome.get("outcome").and_then(Value::as_str) {
            Some("accepted") => {}
            Some("rejected" | "control") => continue,
            other => return Err(format!("unexpected outcome kind: {other:?}")),
        }
        let actor = record
            .get("actor")
            .and_then(|v| v.as_str().and_then(|s| s.parse().ok()).or(v.as_u64()))
            .ok_or("missing actor")?;
        let action: Action =
            serde_json::from_value(outcome.get("action").cloned().ok_or("missing action")?)
                .map_err(|e| e.to_string())?;
        let pre = world.pre_event_state(&action)?;
        check(&pre, tolerance).map_err(|e| format!("journal record {}: {e}", index + 1))?;
        if (calculate(&pre)?.expected - action.pre_flow.total).abs() > tolerance {
            return Err(format!(
                "journal record {}: recorded expectation differs from the reference",
                index + 1
            ));
        }
        let allocator = usize::try_from(actor).map_err(|e| e.to_string())?;
        world.apply(AgentId(actor), action.clone())?;
        trace.push(StepTrace::after(&world, allocator, &action));
    }
    Ok((world, trace))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reputation::{Config, setup};
    use hiyal::rng::Streams;
    #[test]
    fn independent_reference_agrees_across_parameters_and_graph_changes() {
        for seed in [11, 12, 19, 23] {
            let cfg = Config::default_for(10, 3);
            let mut rng = Streams::new("v1-reference".into(), seed);
            let mut w = setup(&cfg, &mut rng).unwrap();
            for beta in [0.0, 0.1, 4.0, 17.0] {
                w.config.beta = beta;
                check(&w, 1e-12).unwrap();
            }
            w.intervene(crate::reputation::Arm::Information).unwrap();
            for beta in [0.0, 3.0, 15.0] {
                w.config.beta = beta;
                check(&w, 1e-12).unwrap();
            }
        }
    }
    #[test]
    fn degree_only_formula_is_exact_for_flat_selection() {
        let mut cfg = Config::default_for(6, 2);
        cfg.beta = 0.0;
        let mut rng = Streams::new("v1-null".into(), 23);
        let w = setup(&cfg, &mut rng).unwrap();
        let r = calculate(&w).unwrap();
        assert!((r.expected - r.boost_indegree_minus_other as f64 / 12.0).abs() < 1e-12);
        assert!(r.selection.abs() < 1e-12);
    }
}
