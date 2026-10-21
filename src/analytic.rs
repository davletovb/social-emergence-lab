//! Closed-form calibration for the exact linear reinforcement mechanism.
//!
//! In experiment 001 with feedback=Linear, the draw weight is wealth_i+offset
//! and each accepted opportunity increases wealth_i by exactly one. This is
//! an Eggenberger–Pólya urn, with equal initial concentrations (initial+offset).
//! Uniform control has a multinomial allocation; reinforced treatment has a
//! Dirichlet–multinomial allocation. These results are not novel sociology.

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
    if !(2..=1000).contains(&actors) || initial == 0 || offset == 0 || opportunities == 0 {
        return Err("require >=2 agents, positive initial/offset and positive horizon".into());
    }
    let concentration = initial
        .checked_add(offset)
        .ok_or("initial concentration overflow")?;
    let total_concentration = concentration
        .checked_mul(u64::try_from(actors).map_err(|e| e.to_string())?)
        .ok_or("total concentration overflow")?;
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
    }
}
