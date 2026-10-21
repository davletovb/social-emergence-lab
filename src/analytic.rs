//! Closed-form calibration for the exact linear reinforcement mechanism.
//!
//! In experiment 001 with feedback=Linear, the draw weight is wealth_i+offset
//! and each accepted opportunity increases wealth_i by exactly one. This is
//! an Eggenberger–Pólya urn, with equal initial concentrations (initial+offset).
//! Uniform control has a multinomial allocation; reinforced treatment has a
//! Dirichlet–multinomial allocation. These results are not novel sociology.
use crate::{Feedback, Society};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MomentPrediction {
    /// E[additional allocation count per agent]
    pub mean_increments_per_actor: f64,
    /// Var[additional allocations per actor] under uniform allocation
    pub uniform_variance: f64,
    /// Var[additional allocations per actor] under linear reinforcement
    pub reinforced_variance: f64,
    /// Predicted ratio reinforced / uniform variances; greater than 1 for t>1
    pub variance_ratio: f64,
}

/// Exact finite-sample first and second moments, computed as f64 after checking
/// integer parameter bounds. Does not approximate Gini or rank correlations.
pub fn predict_moments(
    actors: usize,
    initial: u64,
    offset: u64,
    opportunities: u64,
) -> Result<MomentPrediction, String> {
    if opportunities == 0 {
        return Err("require a positive horizon".into());
    }
    // The simulated model's own bounds, including that actors × (initial +
    // offset) does not overflow.
    Society::new(actors, initial, offset, Feedback::Uniform)?;
    let total_concentration = (initial + offset) * actors as u64;
    let n = actors as f64;
    let t = opportunities as f64;
    let a = total_concentration as f64;
    let p = 1.0 / n;
    let uniform_variance = t * p * (1.0 - p);
    let variance_ratio = (a + t) / (a + 1.0);
    let reinforced_variance = uniform_variance * variance_ratio;
    Ok(MomentPrediction {
        mean_increments_per_actor: t * p,
        uniform_variance,
        reinforced_variance,
        variance_ratio,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polya_urn_closed_form_matches_small_exact_case() {
        // Two agents: initial wealth 1, offset 1 => two red and two blue balls.
        // After two draws, X_0 has probabilities (0:0.3, 1:0.4, 2:0.3).
        // Thus E[X_0] = 1 and Var[X_0] = 0.6.
        let m = predict_moments(2, 1, 1, 2).unwrap();
        assert_eq!(m.mean_increments_per_actor, 1.0);
        assert!((m.uniform_variance - 0.5).abs() < 1e-14);
        assert!((m.reinforced_variance - 0.6).abs() < 1e-14);
        assert!((m.variance_ratio - 1.2).abs() < 1e-14);
    }
    #[test]
    fn reinforcement_has_no_variance_effect_after_one_draw() {
        for offset in [1, 10, 100] {
            let m = predict_moments(20, 10, offset, 1).unwrap();
            assert_eq!(m.variance_ratio, 1.0);
        }
    }
    #[test]
    fn longer_horizon_and_smaller_offset_increase_variance_ratio() {
        let short = predict_moments(20, 10, 1, 100).unwrap();
        let long = predict_moments(20, 10, 1, 500).unwrap();
        let diluted = predict_moments(20, 10, 100, 500).unwrap();
        assert!(long.variance_ratio > short.variance_ratio);
        assert!(long.variance_ratio > diluted.variance_ratio);
        assert!((long.variance_ratio - 720.0 / 221.0).abs() < 1e-13);
        assert!(predict_moments(1, 10, 1, 500).is_err());
        assert!(predict_moments(20, u64::MAX, 1, 500).is_err());
        assert!(predict_moments(20, 10, 1, 0).is_err());
    }

    /// Mean over agents of each agent's across-society variance of its
    /// allocation increment.
    fn pooled_variance(societies: &[Vec<u64>], initial: u64) -> f64 {
        let reps = societies.len() as f64;
        let actors = societies[0].len();
        (0..actors)
            .map(|i| {
                let x: Vec<f64> = societies.iter().map(|w| (w[i] - initial) as f64).collect();
                let mean = x.iter().sum::<f64>() / reps;
                x.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (reps - 1.0)
            })
            .sum::<f64>()
            / actors as f64
    }

    #[test]
    fn simulated_allocation_variances_match_closed_form() {
        use crate::{OpportunityPolicy, reference::simulate_reference};
        use hiyal::{
            core::{AgentId, World},
            experiment::EffectPolicy,
            rng::Streams,
        };
        // Expected variances 6.4 (uniform) and 22.0 (ratio 55/16); with 3,000
        // societies the pooled estimates' sampling error is about 2%, so a
        // weighting or selection bug (e.g. weight = wealth, ratio 50/11)
        // falls far outside the 8% tolerance.
        let (actors, initial, offset, steps, reps) = (5, 2, 1, 40, 3_000);
        let m = predict_moments(actors, initial, offset, steps).unwrap();
        for (feedback, expected) in [
            (Feedback::Uniform, m.uniform_variance),
            (Feedback::Linear, m.reinforced_variance),
        ] {
            let linear = feedback == Feedback::Linear;
            let reference: Vec<_> = (0..reps)
                .map(|r| {
                    simulate_reference(1_000 + r, actors, initial, offset, steps, linear)
                        .unwrap()
                        .wealth
                })
                .collect();
            // The production policy, drawing from Hiyal's named streams.
            let mut streams = Streams::new("analytic-check".into(), u64::from(linear) + 1);
            let policy: Vec<_> = (0..reps)
                .map(|_| {
                    let mut world = Society::new(actors, initial, offset, feedback).unwrap();
                    for _ in 0..steps {
                        let obs = world.observe(AgentId(0)).unwrap();
                        let action = OpportunityPolicy
                            .decide_with_streams(AgentId(0), &obs, &[], &mut streams)
                            .unwrap();
                        world.apply(AgentId(0), action).unwrap();
                    }
                    world.wealth
                })
                .collect();
            for (source, societies) in [("reference", reference), ("policy", policy)] {
                let observed = pooled_variance(&societies, initial);
                assert!(
                    (observed / expected - 1.0).abs() < 0.08,
                    "{source} {feedback:?}: variance {observed} vs closed form {expected}"
                );
            }
        }
    }
}
