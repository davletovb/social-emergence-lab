//! Precision planning from *exploratory, paired-society* terminal outcomes.
//! The design is paired (one treatment − control difference per society), so
//! sizes come from the one-sample t distribution of those differences under an
//! assumed, inflated SD. A design heuristic, NOT a power proof.
use crate::{
    hiyal_study::TRAJECTORY_HEADER,
    stats::{Pair, delta_moments},
};
use std::{collections::BTreeMap, fs, path::Path};

/// Largest sample size the calculator will report.
const MAX_SOCIETIES: usize = 100_000;

/// The pilot population, treatment offset and horizon to read.
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct PilotSpec {
    pub actors: usize,
    pub initial: u64,
    pub offset: u64,
    pub horizon: u64,
}

/// Read one primary population/treatment/horizon from a complete Hiyal
/// trajectory export, pairing the shared uniform control with the treatment.
pub fn terminal_pairs(csv: &str, spec: &PilotSpec) -> Result<Vec<Pair>, String> {
    if spec.actors < 2 || spec.initial == 0 || spec.offset == 0 || spec.horizon == 0 {
        return Err("invalid design parameters".into());
    }
    let mut lines = csv.lines();
    if lines.next() != Some(TRAJECTORY_HEADER) {
        return Err("unrecognized Hiyal trajectories schema".into());
    }
    let columns = TRAJECTORY_HEADER.split(',').count();
    let offset = spec.offset.to_string();
    let mut groups: BTreeMap<u64, (Option<f64>, Option<f64>)> = BTreeMap::new();
    for (i, line) in lines.enumerate() {
        let p: Vec<_> = line.split(',').collect();
        if p.len() != columns {
            return Err(format!("bad row {}", i + 2));
        }
        let n = p[0].parse::<usize>().map_err(|_| "invalid actor count")?;
        let w0 = p[1].parse::<u64>().map_err(|_| "invalid starting wealth")?;
        let t = p[3].parse::<u64>().map_err(|_| "invalid horizon")?;
        let step = p[6].parse::<u64>().map_err(|_| "invalid recorded step")?;
        if n != spec.actors || w0 != spec.initial || t != spec.horizon || step != spec.horizon {
            continue;
        }
        let seed = p[4].parse::<u64>().map_err(|_| "invalid seed")?;
        let gini = p[7].parse::<f64>().map_err(|_| "invalid gini")?;
        if !gini.is_finite() || !(0.0..=1.0).contains(&gini) {
            return Err("invalid terminal gini".into());
        }
        let record = groups.entry(seed).or_default();
        match p[5] {
            "control" if p[2].is_empty() => {
                if record.0.replace(gini).is_some() {
                    return Err("duplicate uniform control".into());
                }
            }
            "treatment" if p[2] == offset => {
                if record.1.replace(gini).is_some() {
                    return Err("duplicate treatment".into());
                }
            }
            "control" | "treatment" => {}
            _ => return Err("unrecognized arm".into()),
        }
    }
    if groups.len() < 2 {
        return Err("at least two pilot society pairs required".into());
    }
    groups
        .into_iter()
        .map(|(seed, (control, treatment))| {
            Ok(Pair {
                seed,
                horizon: spec.horizon,
                control: control.ok_or("unpaired control")?,
                treatment: treatment.ok_or("unpaired treatment")?,
            })
        })
        .collect()
}

pub fn terminal_pairs_file(path: &Path, spec: &PilotSpec) -> Result<Vec<Pair>, String> {
    terminal_pairs(&fs::read_to_string(path).map_err(|e| e.to_string())?, spec)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PrecisionPlan {
    pub pilot_societies: usize,
    pub pilot_mean_paired_delta: f64,
    pub pilot_sd_paired_delta: f64,
    /// Multiplies the pilot SD, so the planning variance is `sd_multiplier²` times the pilot's.
    pub sd_multiplier: f64,
    pub variance_multiplier: f64,
    pub conservative_planning_sd: f64,
    pub target_ci_half_width: f64,
    pub societies_for_95_ci_half_width: usize,
    pub minimum_mean_difference_of_interest: f64,
    pub societies_for_90pct_power_two_sided_5pct: usize,
    pub power_at_that_n: f64,
    pub illustrative_societies: usize,
    pub expected_95pct_ci_half_width_at_illustrative_n: f64,
    pub method: &'static str,
    pub scientific_status: &'static str,
}

fn positive_finite(n: f64) -> bool {
    n.is_finite() && n > 0.0
}

/// Standard normal CDF by Marsaglia's (2004) series, accurate to ~1e-15.
fn normal_cdf(x: f64) -> f64 {
    if x < -37.5 {
        return 0.0;
    }
    if x > 37.5 {
        return 1.0;
    }
    let q = x * x;
    let (mut sum, mut previous, mut term, mut k) = (x, 0.0, x, 1.0);
    while sum != previous {
        previous = sum;
        k += 2.0;
        term *= q / k;
        sum = previous + term;
    }
    0.5 + sum * (-0.5 * q - 0.918_938_533_204_672_8).exp()
}

/// P(T > c) for Student t with `df` degrees of freedom and noncentrality
/// `ncp`, i.e. E[1 − Φ(c·S − ncp)] with S = sqrt(χ²_df / df). Integrated by
/// Simpson's rule over S's density, normalized numerically.
fn t_upper_tail(c: f64, df: f64, ncp: f64) -> f64 {
    const INTERVALS: usize = 2_000;
    let spread = 12.0 / (2.0 * df).sqrt();
    let (lo, hi) = ((1.0 - spread).max(0.0), 1.0 + spread);
    let step = (hi - lo) / INTERVALS as f64;
    // Log-density of S up to a constant, shifted by its maximum to avoid overflow.
    let mode = ((df - 1.0) / df).sqrt();
    let log_density = |s: f64| {
        if s == 0.0 {
            if df == 1.0 { 0.0 } else { f64::NEG_INFINITY }
        } else {
            (df - 1.0) * s.ln() - df * s * s / 2.0
        }
    };
    let peak = log_density(mode.max(lo));
    let (mut mass, mut tail) = (0.0, 0.0);
    for i in 0..=INTERVALS {
        let s = lo + i as f64 * step;
        let weight = match i {
            0 => 1.0,
            _ if i == INTERVALS => 1.0,
            _ if i % 2 == 1 => 4.0,
            _ => 2.0,
        } * (log_density(s) - peak).exp();
        mass += weight;
        tail += weight * (1.0 - normal_cdf(c * s - ncp));
    }
    tail / mass
}

/// Quantile of the central t distribution for p in (0.5, 1), by bisection.
fn t_quantile(p: f64, df: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, 100.0);
    for _ in 0..60 {
        let mid = (lo + hi) / 2.0;
        if t_upper_tail(mid, df, 0.0) > 1.0 - p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

/// 95% two-sided paired-t confidence interval half-width for `n` societies.
fn t_half_width(n: usize, sd: f64) -> f64 {
    t_quantile(0.975, (n - 1) as f64) * sd / (n as f64).sqrt()
}

/// Exact power of a two-sided 5% one-sample (paired) t-test on `n`
/// differences when the true mean difference is `effect` SDs.
fn paired_t_power(n: usize, effect: f64) -> f64 {
    let df = (n - 1) as f64;
    let critical = t_quantile(0.975, df);
    let ncp = effect * (n as f64).sqrt();
    t_upper_tail(critical, df, ncp) + (1.0 - t_upper_tail(-critical, df, ncp))
}

/// Smallest n >= 2 meeting `ok`, searching up from a normal-theory start
/// (which the t-based answer is never far from).
fn smallest_n(start: f64, ok: impl Fn(usize) -> bool) -> Result<usize, String> {
    if !positive_finite(start) || start > MAX_SOCIETIES as f64 {
        return Err(format!(
            "required sample size is invalid or over {MAX_SOCIETIES}"
        ));
    }
    let mut n = (start.ceil() as usize).max(2);
    while !ok(n) {
        n += 1;
        if n > MAX_SOCIETIES {
            return Err(format!("required sample size is over {MAX_SOCIETIES}"));
        }
    }
    while n > 2 && ok(n - 1) {
        n -= 1;
    }
    Ok(n)
}

/// Paired design: the estimand is the mean of per-society treatment − control
/// differences, so precision and power use the one-sample t distribution of
/// those differences (n − 1 degrees of freedom) with SD `sd_multiplier ×` the
/// pilot SD. Assumes roughly normal differences; must be independently
/// reviewed before preregistration.
pub fn plan_precision(
    pairs: &[Pair],
    sd_multiplier: f64,
    ci_half_width: f64,
    mde: f64,
    illustrative_n: usize,
) -> Result<PrecisionPlan, String> {
    if pairs.len() < 3
        || !(sd_multiplier.is_finite() && sd_multiplier >= 1.0)
        || !positive_finite(ci_half_width)
        || !positive_finite(mde)
        || illustrative_n < 2
    {
        return Err("invalid precision-design inputs".into());
    }
    let (_, mean, sd) = delta_moments(pairs)?;
    let sd = sd.filter(|sd| positive_finite(*sd));
    let sd = sd.ok_or("zero or invalid pilot variability")?;
    let assumed = sd_multiplier * sd;
    let z = 1.959963984540054;
    let z_power = 1.2815515655446004;
    let precision_n = smallest_n((z * assumed / ci_half_width).powi(2), |n| {
        t_half_width(n, assumed) <= ci_half_width
    })?;
    let power_n = smallest_n(((z + z_power) * assumed / mde).powi(2), |n| {
        paired_t_power(n, mde / assumed) >= 0.9
    })?;
    Ok(PrecisionPlan {
        pilot_societies: pairs.len(),
        pilot_mean_paired_delta: mean,
        pilot_sd_paired_delta: sd,
        sd_multiplier,
        variance_multiplier: sd_multiplier * sd_multiplier,
        conservative_planning_sd: assumed,
        target_ci_half_width: ci_half_width,
        societies_for_95_ci_half_width: precision_n,
        minimum_mean_difference_of_interest: mde,
        societies_for_90pct_power_two_sided_5pct: power_n,
        power_at_that_n: paired_t_power(power_n, mde / assumed),
        illustrative_societies: illustrative_n,
        expected_95pct_ci_half_width_at_illustrative_n: t_half_width(illustrative_n, assumed),
        method: "paired (one-sample) t on per-society differences: t-quantile CI half-width and exact noncentral-t power",
        scientific_status: "Exploratory pilot-based planning heuristic under an assumed SD and roughly normal differences. Do not equate with empirical causal evidence, guarantee of confidence interval width, or registered power.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PILOT: PilotSpec = PilotSpec {
        actors: 20,
        initial: 10,
        offset: 1,
        horizon: 500,
    };

    #[test]
    fn t_distribution_matches_reference_values() {
        // Student t table quantiles.
        for (df, expected) in [(9.0, 2.262157), (29.0, 2.045230), (120.0, 1.979930)] {
            assert!((t_quantile(0.975, df) - expected).abs() < 1e-5, "df {df}");
        }
        // One-sample t-test sizes for 90% power (R pwr: n = 43.99 at d = 0.5,
        // n = 18.51 at d = 0.8).
        assert!(paired_t_power(44, 0.5) >= 0.9 && paired_t_power(43, 0.5) < 0.9);
        assert!(paired_t_power(19, 0.8) >= 0.9 && paired_t_power(18, 0.8) < 0.9);
    }

    #[test]
    fn t_sizes_exceed_normal_theory_for_small_samples() {
        // Deltas alternating 0.04/0.06: mean 0.05, SD sqrt(0.001 / 9).
        let p = (0..10)
            .map(|i| Pair {
                seed: i,
                horizon: 500,
                control: 0.0,
                treatment: if i % 2 == 0 { 0.04 } else { 0.06 },
            })
            .collect::<Vec<_>>();
        let x = plan_precision(&p, 1.0, 0.01, 0.02, 100).unwrap();
        assert!((x.pilot_mean_paired_delta - 0.05).abs() < 1e-12);
        assert!((x.pilot_sd_paired_delta - (0.001f64 / 9.0).sqrt()).abs() < 1e-12);
        // Normal theory would give 5; t with 4-6 degrees of freedom needs 7.
        assert_eq!(x.societies_for_95_ci_half_width, 7);
        assert_eq!(x.variance_multiplier, 1.0);
        assert!(plan_precision(&p, 0.5, 0.01, 0.02, 100).is_err());
        assert!(plan_precision(&p[..2], 1.0, 0.01, 0.02, 100).is_err());
    }

    #[test]
    fn archived_pilot_reproduces_documented_plan() {
        let csv = include_str!(
            "../experiments/001-emergence-of-inequality/results/replicated-74000/hiyal-trajectories.csv"
        );
        let pairs = terminal_pairs(csv, &PILOT).unwrap();
        assert_eq!(pairs.len(), 10);
        let x = plan_precision(&pairs, 2.0, 0.01, 0.02, 100).unwrap();
        assert!((x.pilot_mean_paired_delta - 0.058128571429).abs() < 1e-11);
        assert!((x.pilot_sd_paired_delta - 0.022417644694).abs() < 1e-11);
        assert_eq!(x.variance_multiplier, 4.0);
        assert_eq!(x.societies_for_95_ci_half_width, 80);
        assert_eq!(x.societies_for_90pct_power_two_sided_5pct, 55);
        assert!((x.power_at_that_n - 0.9013).abs() < 5e-4);
        assert!((x.expected_95pct_ci_half_width_at_illustrative_n - 0.00890).abs() < 5e-5);
    }

    #[test]
    fn trajectory_parser_respects_shared_control_and_seed_alignment() {
        let mut s = format!("{TRAJECTORY_HEADER}\n");
        for seed in [9, 10] {
            s.push_str(&format!("20,10,,500,{seed},control,500,0.1000,0.2,,\n"));
            s.push_str(&format!("20,10,1,500,{seed},treatment,500,0.1300,0.2,,\n"));
            s.push_str(&format!("20,10,10,500,{seed},treatment,500,0.1100,0.2,,\n"));
            // Earlier checkpoints, other horizons and populations are ignored.
            s.push_str(&format!("20,10,,500,{seed},control,250,0.9000,0.2,,\n"));
            s.push_str(&format!("20,10,1,500,{seed},treatment,125,0.9000,0.2,,\n"));
            s.push_str(&format!("20,10,1,100,{seed},treatment,100,0.9000,0.2,,\n"));
            s.push_str(&format!("40,10,1,500,{seed},treatment,500,0.9000,0.2,,\n"));
        }
        let pair = terminal_pairs(&s, &PILOT).unwrap();
        assert_eq!(pair.len(), 2);
        assert_eq!(pair[0].seed, 9);
        assert!((pair[0].delta() - 0.03).abs() < 1e-12);
        assert!((pair[1].delta() - 0.03).abs() < 1e-12);

        let fails = |rows: &str| terminal_pairs(&format!("{s}{rows}"), &PILOT).unwrap_err();
        assert_eq!(
            fails("20,10,,500,9,control,500,0.1,0.2,,\n"),
            "duplicate uniform control"
        );
        assert_eq!(
            fails("20,10,1,500,9,treatment,500,0.1,0.2,,\n"),
            "duplicate treatment"
        );
        assert_eq!(
            fails("20,10,,500,11,control,500,0.1,0.2,,\n"),
            "unpaired treatment"
        );
        assert_eq!(
            fails("20,10,1,500,11,treatment,500,0.1,0.2,,\n"),
            "unpaired control"
        );
        assert_eq!(
            fails("20,10,1,500,9,placebo,500,0.1,0.2,,\n"),
            "unrecognized arm"
        );
        assert_eq!(
            fails("20,10,1,500,9,treatment,500,1.5,0.2,,\n"),
            "invalid terminal gini"
        );
        assert!(terminal_pairs("seed,arm\n", &PILOT).is_err());
    }
}
