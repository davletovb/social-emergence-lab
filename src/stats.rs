//! Paired-society analysis. Agents and time points are *not* independent observations.
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Clone, Debug)]
pub struct Pair {
    pub seed: u64,
    pub horizon: u64,
    pub control: f64,
    pub treatment: f64,
}
impl Pair {
    pub fn delta(&self) -> f64 {
        self.treatment - self.control
    }
}

/// Strictly parse the runner's fixed-column CSV, pairing by seed rather than row order.
pub fn read_pairs(path: &Path) -> Result<Vec<Pair>, String> {
    let input = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut lines = input.lines();
    if lines.next() != Some("seed,arm,attempts,opportunities,gini,top_quintile_share,chain_head") {
        return Err("unrecognized results.csv schema".into());
    }
    let mut groups: BTreeMap<u64, (Option<(u64, f64)>, Option<(u64, f64)>)> = BTreeMap::new();
    for (row, line) in lines.enumerate() {
        let parts: Vec<_> = line.split(',').collect();
        if parts.len() != 7 {
            return Err(format!("row {} has wrong field count", row + 2));
        }
        let seed = parts[0].parse::<u64>().map_err(|_| "invalid seed")?;
        let attempts = parts[2].parse::<u64>().map_err(|_| "invalid attempts")?;
        let opportunities = parts[3]
            .parse::<u64>()
            .map_err(|_| "invalid opportunities")?;
        let gini = parts[4].parse::<f64>().map_err(|_| "invalid Gini")?;
        if !gini.is_finite()
            || !(0.0..=1.0).contains(&gini)
            || attempts != opportunities
            || opportunities == 0
        {
            return Err(format!("row {}: invalid or incomplete outcome", row + 2));
        }
        let group = groups.entry(seed).or_default();
        let cell = match parts[1] {
            "control" => &mut group.0,
            "treatment" => &mut group.1,
            _ => return Err("unexpected arm".into()),
        };
        if cell.replace((opportunities, gini)).is_some() {
            return Err("duplicate arm/seed".into());
        }
    }
    if groups.is_empty() {
        return Err("empty results.csv".into());
    }
    groups
        .into_iter()
        .map(|(seed, (control, treatment))| {
            let (h0, c) = control.ok_or("unpaired control")?;
            let (h1, t) = treatment.ok_or("unpaired treatment")?;
            if h0 != h1 {
                return Err("arms have mismatched horizons".into());
            }
            Ok(Pair {
                seed,
                horizon: h0,
                control: c,
                treatment: t,
            })
        })
        .collect()
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PairedSummary {
    pub pairs: usize,
    pub horizon: u64,
    pub mean_control: f64,
    pub mean_treatment: f64,
    pub mean_delta: f64,
    pub sample_sd_delta: Option<f64>,
    pub bootstrap_percentile_95: Option<[f64; 2]>,
    pub bootstrap_resamples: usize,
    pub bootstrap_seed: u64,
    pub interpretation: &'static str,
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
struct XorShift64(u64);
impl XorShift64 {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn index(&mut self, n: usize) -> usize {
        // Equal-support rejection sampler, avoiding the bootstrap modulo mapping bias.
        let bound = n as u64;
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let v = self.next();
            if v >= threshold {
                return (v % bound) as usize;
            }
        }
    }
}
pub fn summarize(
    pairs: &[Pair],
    resamples: usize,
    bootstrap_seed: u64,
) -> Result<PairedSummary, String> {
    if pairs.is_empty() {
        return Err("no matched pairs".into());
    }
    if !(100..=100_000).contains(&resamples) {
        return Err("bootstrap resamples outside 100..=100000".into());
    }
    if bootstrap_seed == 0 {
        return Err("bootstrap seed must be nonzero".into());
    }
    let horizon = pairs[0].horizon;
    if pairs.iter().any(|p| p.horizon != horizon) {
        return Err("mixed terminal horizons".into());
    }
    let control = mean(&pairs.iter().map(|p| p.control).collect::<Vec<_>>());
    let treatment = mean(&pairs.iter().map(|p| p.treatment).collect::<Vec<_>>());
    let deltas: Vec<f64> = pairs.iter().map(Pair::delta).collect();
    let delta = mean(&deltas);
    let sample_sd_delta = (deltas.len() > 1).then(|| {
        (deltas.iter().map(|d| (d - delta).powi(2)).sum::<f64>() / (deltas.len() - 1) as f64).sqrt()
    });
    // With very small n, report descriptive means, not misleading bootstrap CI.
    let ci = if pairs.len() < 10 {
        None
    } else {
        let mut generator = XorShift64(bootstrap_seed);
        let mut replicate_means = Vec::with_capacity(resamples);
        for _ in 0..resamples {
            let sum = (0..pairs.len())
                .map(|_| deltas[generator.index(pairs.len())])
                .sum::<f64>();
            replicate_means.push(sum / pairs.len() as f64);
        }
        replicate_means.sort_by(f64::total_cmp);
        let lower = ((resamples - 1) as f64 * 0.025).floor() as usize;
        let upper = ((resamples - 1) as f64 * 0.975).ceil() as usize;
        Some([replicate_means[lower], replicate_means[upper]])
    };
    Ok(PairedSummary {
        pairs: pairs.len(),
        horizon,
        mean_control: control,
        mean_treatment: treatment,
        mean_delta: delta,
        sample_sd_delta,
        bootstrap_percentile_95: ci,
        bootstrap_resamples: if ci.is_some() { resamples } else { 0 },
        bootstrap_seed,
        interpretation: "Exploratory paired-society bootstrap interval; not a preregistered hypothesis test or empirical causal effect",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paired_bootstrap_is_reproducible_and_positive_for_dominant_treatment() {
        let pairs = (0..40)
            .map(|i| Pair {
                seed: i,
                horizon: 100,
                control: 0.10,
                treatment: 0.15 + i as f64 / 10000.0,
            })
            .collect::<Vec<_>>();
        let a = summarize(&pairs, 2000, 123).unwrap();
        let b = summarize(&pairs, 2000, 123).unwrap();
        assert_eq!(a.bootstrap_percentile_95, b.bootstrap_percentile_95);
        let [lo, hi] = a.bootstrap_percentile_95.unwrap();
        assert!(lo > 0.05 && lo < hi && hi < 0.055);
    }
    #[test]
    fn refuses_false_precision_on_small_or_mixed_samples() {
        let p = Pair {
            seed: 1,
            horizon: 100,
            control: 0.1,
            treatment: 0.2,
        };
        assert!(
            summarize(std::slice::from_ref(&p), 1000, 1)
                .unwrap()
                .bootstrap_percentile_95
                .is_none()
        );
        assert!(summarize(&[p.clone(), Pair { horizon: 200, ..p }], 1000, 1).is_err());
    }
    #[test]
    fn rejects_duplicate_or_unpaired_rows() {
        let file = std::env::temp_dir().join(format!("paired-results-{}.csv", std::process::id()));
        std::fs::write(&file,"seed,arm,attempts,opportunities,gini,top_quintile_share,chain_head\n1,control,2,2,0.1,0.2,hash\n").unwrap();
        assert!(read_pairs(&file).is_err());
        std::fs::remove_file(file).unwrap();
    }
}
