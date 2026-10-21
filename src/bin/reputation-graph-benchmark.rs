//! Bounded graph rejection benchmark. Each attempted seed is reported,
//! including failures; this is a *performance diagnostic*, not research data.
use hiyal::rng::Streams;
use serde::Serialize;
use social_emergence_lab::reputation::{Config, sample_regular_graph};
use std::{env, fs, path::Path, process, time::Instant};
#[derive(Serialize)]
struct Sample {
    seed: u64,
    graph: String,
    attempts: Option<usize>,
    elapsed_micros: u128,
    error: Option<String>,
}
#[derive(Serialize)]
struct Results<'a> {
    status: &'a str,
    actors: usize,
    degree: usize,
    seeds: Vec<u64>,
    attempt_cap: usize,
    attempted_graphs: usize,
    accepted_graphs: usize,
    failed_graphs: usize,
    /// Pairings tried over all graphs; a failed graph used the whole cap.
    total_attempts: u64,
    acceptance_per_attempt: f64,
    /// One-sided 95% lower bound on the per-attempt acceptance probability.
    acceptance_per_attempt_lower_95: f64,
    /// Implied 95% upper bound on one graph exhausting `attempt_cap`.
    cap_failure_probability_upper_95: f64,
    samples: Vec<Sample>,
    caution: &'a str,
}
fn run() -> Result<(), String> {
    let a = env::args().skip(1).collect::<Vec<_>>();
    if a.len() != 5 {
        return Err("usage: reputation-graph-benchmark <new-output.json> <actors> <degree> <first-seed> <count-1..500>".into());
    }
    let out = Path::new(&a[0]);
    if out.exists() {
        return Err("refusing to overwrite existing benchmark".into());
    }
    let parse = |i: usize| {
        a[i].parse::<u64>()
            .map_err(|_| format!("invalid integer: {}", a[i]))
    };
    let n = usize::try_from(parse(1)?).map_err(|e| e.to_string())?;
    let d = usize::try_from(parse(2)?).map_err(|e| e.to_string())?;
    let start = parse(3)?;
    let count = usize::try_from(parse(4)?).map_err(|e| e.to_string())?;
    if !(1..=500).contains(&count) {
        return Err("benchmark count must be 1..500".into());
    }
    let cfg = Config::default_for(n, d);
    cfg.validate()?;
    let mut samples = Vec::new();
    let mut seeds = Vec::new();
    for i in 0..count {
        let seed = start.checked_add(i as u64).ok_or("seed overflow")?;
        seeds.push(seed);
        let mut rng = Streams::new(format!("v1-graph-benchmark-{n}-{d}"), seed);
        for stream in ["setup/initial_graph", "setup/repair_graph"] {
            let t = Instant::now();
            let result = sample_regular_graph(&cfg, &mut rng, stream);
            let elapsed = t.elapsed().as_micros();
            match result {
                Ok(s) => samples.push(Sample {
                    seed,
                    graph: stream.into(),
                    attempts: Some(s.attempts),
                    elapsed_micros: elapsed,
                    error: None,
                }),
                Err(e) => samples.push(Sample {
                    seed,
                    graph: stream.into(),
                    attempts: None,
                    elapsed_micros: elapsed,
                    error: Some(e),
                }),
            }
        }
    }
    let failed = samples.iter().filter(|x| x.error.is_some()).count();
    let accepted = samples.len() - failed;
    let total_attempts: u64 = samples
        .iter()
        .map(|x| x.attempts.unwrap_or(cfg.graph_attempts) as u64)
        .sum();
    let lower = poisson_lower_95(accepted as u64) / total_attempts as f64;
    let report = Results {
        status: "EXPLORATORY_GRAPH_SAMPLER_BENCHMARK",
        actors: n,
        degree: d,
        seeds,
        attempt_cap: cfg.graph_attempts,
        attempted_graphs: samples.len(),
        accepted_graphs: accepted,
        failed_graphs: failed,
        total_attempts,
        acceptance_per_attempt: accepted as f64 / total_attempts as f64,
        acceptance_per_attempt_lower_95: lower,
        cap_failure_probability_upper_95: (cfg.graph_attempts as f64 * (-lower).ln_1p()).exp(),
        samples,
        caution: "Each attempt is an independent uniform stub pairing, so attempts are Bernoulli trials; the bounds use a Poisson approximation. Timings are not a controlled hardware benchmark. Report failed seeds without substitution.",
    };
    fs::write(
        out,
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    if failed > 0 {
        return Err(format!(
            "{failed} graph draws hit the bounded cap; record them, do not replace seeds"
        ));
    }
    Ok(())
}
/// One-sided 95% lower confidence bound on a Poisson mean after `k` events:
/// the λ with P(X >= k | λ) = 0.05, by bisection on the exact CDF.
fn poisson_lower_95(k: u64) -> f64 {
    if k == 0 {
        return 0.0;
    }
    let below_k = |lambda: f64| {
        let mut log_factorial = 0.0;
        let logs: Vec<f64> = (0..k)
            .map(|i| {
                if i > 0 {
                    log_factorial += (i as f64).ln();
                }
                -lambda + i as f64 * lambda.ln() - log_factorial
            })
            .collect();
        let top = logs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        top.exp() * logs.iter().map(|x| (x - top).exp()).sum::<f64>()
    };
    let (mut low, mut high) = (0.0, k as f64);
    for _ in 0..200 {
        let mid = 0.5 * (low + high);
        if below_k(mid) > 0.95 {
            low = mid;
        } else {
            high = mid;
        }
    }
    0.5 * (low + high)
}

#[cfg(test)]
mod tests {
    #[test]
    fn poisson_lower_bound_matches_chi_square_tables() {
        // chi2(0.05; 2k) / 2 for k = 1, 8, 80.
        for (k, expected) in [(1, 0.051293), (8, 3.981), (80, 65.29)] {
            let got = super::poisson_lower_95(k);
            assert!((got - expected).abs() < 1e-2 * expected, "{k}: {got}");
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("graph benchmark: {e}");
        process::exit(2);
    }
}
