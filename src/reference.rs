//! Independent *structural* reference of the event journal.
//! Does not invoke Hiyal reducers, policy weighting, or metric implementation.
//! It validates recipients/accounting/metrics, not the RNG's causal draws.
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

/// Same per-record cap as Hiyal's journal reader.
const MAX_RECORD_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub struct ReferenceOutcome {
    pub wealth: Vec<u64>,
    pub opportunities: u64,
}

/// Replays only the first `verified_bytes` of the journal, i.e. exactly the
/// prefix Hiyal verified, so concurrent appends or a torn tail are ignored.
/// `opportunities` counts accepted events in that prefix only.
pub fn replay(
    journal: &Path,
    initial: &[u64],
    verified_bytes: u64,
) -> Result<ReferenceOutcome, String> {
    if initial.len() < 2 || initial.iter().any(|w| *w == 0) {
        return Err("reference requires >=2 positive initial endowments".into());
    }
    let file = File::open(journal).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() < verified_bytes {
        return Err("journal is shorter than the verified prefix".into());
    }
    let mut wealth = initial.to_vec();
    let mut opportunities = 0u64;
    let mut reader = BufReader::new(file.take(verified_bytes));
    let mut raw = Vec::new();
    for line_number in 1.. {
        raw.clear();
        let read = (&mut reader)
            .take(MAX_RECORD_BYTES + 1)
            .read_until(b'\n', &mut raw)
            .map_err(|e| e.to_string())?;
        if read == 0 {
            break;
        }
        if raw.pop() != Some(b'\n') {
            return Err(format!("unterminated or oversized record at {line_number}"));
        }
        let line = std::str::from_utf8(&raw).map_err(|e| format!("record {line_number}: {e}"))?;
        if line.is_empty() {
            return Err(format!("empty journal record at {line_number}"));
        }
        let record: serde_json::Value =
            serde_json::from_str(line).map_err(|e| format!("record {line_number}: {e}"))?;
        let event = record.get("outcome").ok_or("missing outcome")?;
        let kind = event
            .get("outcome")
            .and_then(serde_json::Value::as_str)
            .ok_or("missing tagged outcome kind")?;
        match kind {
            "accepted" => {
                let recipient = event
                    .get("action")
                    .and_then(|a| a.get("recipient"))
                    .and_then(serde_json::Value::as_u64)
                    .ok_or("missing integer recipient")?;
                let index = usize::try_from(recipient).map_err(|e| e.to_string())?;
                let slot = wealth
                    .get_mut(index)
                    .ok_or("reference: recipient outside society")?;
                *slot = slot.checked_add(1).ok_or("reference: wealth overflow")?;
                opportunities = opportunities
                    .checked_add(1)
                    .ok_or("reference: accepted event overflow")?;
            }
            "rejected" | "control" => {} // Engine verifies cause/hash; this model counts only accepted opportunity events.
            other => return Err(format!("unexpected outcome kind: {other}")),
        }
    }
    Ok(ReferenceOutcome {
        wealth,
        opportunities,
    })
}

/// Sorted weighted-sum Gini, algebraically distinct from pairwise absolute differences.
pub fn reference_gini(input: &[u64]) -> Result<f64, String> {
    if input.is_empty() {
        return Err("empty vector".into());
    }
    let mut sorted = input.to_vec();
    sorted.sort_unstable();
    let sum = sorted.iter().try_fold(0u128, |s, v| {
        s.checked_add(*v as u128).ok_or("total overflow")
    })?;
    if sum == 0 {
        return Err("zero sum".into());
    }
    let n = sorted.len() as f64;
    let weighted = sorted.iter().enumerate().try_fold(0i128, |s, (i, v)| {
        let coeff = 2i128 * (i as i128 + 1) - (sorted.len() as i128 + 1);
        s.checked_add(coeff * (*v as i128))
            .ok_or("weighted sum overflow")
    })?;
    Ok(weighted as f64 / (n * sum as f64))
}

/// Share held by the ceil(n/5) highest balances, computed from a histogram of
/// balances (taking whole tie groups, then part of the boundary group) rather
/// than by sorting. The definition itself is pinned by hand-computed cases.
pub fn reference_top_share(input: &[u64]) -> Result<f64, String> {
    if input.is_empty() {
        return Err("empty vector".into());
    }
    let mut histogram = std::collections::BTreeMap::<u64, u128>::new();
    for &value in input {
        *histogram.entry(value).or_default() += 1;
    }
    let total: u128 = histogram.iter().map(|(v, c)| u128::from(*v) * c).sum();
    if total == 0 {
        return Err("zero sum".into());
    }
    let mut remaining = (input.len() as u128).div_ceil(5);
    let mut highest = 0u128;
    for (&value, &count) in histogram.iter().rev() {
        let taken = count.min(remaining);
        highest += u128::from(value) * taken;
        remaining -= taken;
        if remaining == 0 {
            break;
        }
    }
    Ok(highest as f64 / total as f64)
}

/// Unbiased integer in `0..bound` by rejection sampling, shared by the
/// reference simulator and the bootstrap.
pub(crate) fn unbiased_below(bound: u64, mut next: impl FnMut() -> u64) -> u64 {
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let draw = next();
        if draw >= threshold {
            return draw % bound;
        }
    }
}

/// Independent stochastic model for **exploratory** parameter sensitivity.
/// Its SplitMix64 random stream is intentionally different from Hiyal's
/// ChaCha20 profile: compare statistical patterns, never byte-identical runs.
pub fn simulate_reference(
    seed: u64,
    actors: usize,
    initial: u64,
    offset: u64,
    steps: u64,
    linear: bool,
) -> Result<ReferenceOutcome, String> {
    if !(2..=1000).contains(&actors) || initial == 0 || offset == 0 || steps > 100_000 {
        return Err("reference simulation parameters out of bounds".into());
    }
    let mut wealth = vec![initial; actors];
    let mut state = seed;
    for _ in 0..steps {
        // Direct cumulative weighted lottery; separate from production select_ticket.
        let weights = wealth
            .iter()
            .map(|&v| {
                if linear {
                    v.checked_add(offset).ok_or("weight overflow")
                } else {
                    Ok(1)
                }
            })
            .collect::<Result<Vec<u64>, _>>()?;
        let total = weights
            .iter()
            .try_fold(0u64, |s, w| s.checked_add(*w).ok_or("weight sum overflow"))?;
        let mut cursor = unbiased_below(total, || splitmix64(&mut state));
        let mut recipient = None;
        for (idx, &weight) in weights.iter().enumerate() {
            if cursor < weight {
                recipient = Some(idx);
                break;
            }
            cursor -= weight;
        }
        let idx = recipient.ok_or("reference lottery exhausted")?;
        wealth[idx] = wealth[idx]
            .checked_add(1)
            .ok_or("reference wealth overflow")?;
    }
    Ok(ReferenceOutcome {
        wealth,
        opportunities: steps,
    })
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod sensitivity_tests {
    use super::*;
    #[test]
    fn reference_simulation_conserves_resources_and_seeded_replays() {
        let a = simulate_reference(17, 20, 10, 1, 1000, true).unwrap();
        let b = simulate_reference(17, 20, 10, 1, 1000, true).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.wealth.iter().sum::<u64>(), 1200);
        assert_eq!(a.opportunities, 1000);
        let c = simulate_reference(17, 20, 10, 1, 1000, false).unwrap();
        assert_eq!(c.wealth.iter().sum::<u64>(), 1200);
    }
    #[test]
    fn zero_step_reference_is_equal_and_safe() {
        let a = simulate_reference(2, 10, 3, 1, 0, false).unwrap();
        assert_eq!(a.wealth, vec![3; 10]);
        assert_eq!(reference_gini(&a.wealth).unwrap(), 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gini, top_quintile_share};
    #[test]
    fn independent_formula_matches_known_cases_and_pairwise_gini() {
        let cases = [
            vec![1, 1, 1, 1],
            vec![1, 2, 3, 4],
            vec![5, 5, 1, 1, 20],
            vec![10, 20],
            vec![0, 0, 10],
        ];
        for values in cases {
            let a = gini(&values).unwrap();
            let b = reference_gini(&values).unwrap();
            assert!((a - b).abs() < 1e-12, "{values:?}: {a} vs {b}");
            assert!(
                (top_quintile_share(&values).unwrap() - reference_top_share(&values).unwrap())
                    .abs()
                    < 1e-12
            );
            let mut reversed = values.clone();
            reversed.reverse();
            assert_eq!(
                reference_gini(&values).unwrap(),
                reference_gini(&reversed).unwrap()
            );
        }
    }
    #[test]
    fn exhaustive_permutation_small_societies() {
        for a in 0..5u64 {
            for b in 0..5u64 {
                for c in 0..5u64 {
                    if a + b + c == 0 {
                        continue;
                    }
                    let expected = reference_gini(&[a, b, c]).unwrap();
                    for p in [
                        [a, b, c],
                        [a, c, b],
                        [b, a, c],
                        [b, c, a],
                        [c, a, b],
                        [c, b, a],
                    ] {
                        assert!((reference_gini(&p).unwrap() - expected).abs() < 1e-12);
                        assert!((gini(&p).unwrap() - expected).abs() < 1e-12);
                    }
                }
            }
        }
    }
    #[test]
    fn independent_replay_counts_only_accepted_events() {
        let file =
            std::env::temp_dir().join(format!("reference-journal-{}.jsonl", std::process::id()));
        std::fs::write(
            &file,
            concat!(
                "{\"outcome\":{\"outcome\":\"accepted\",\"action\":{\"recipient\":1}}}\n",
                "{\"outcome\":{\"outcome\":\"control\"}}\n",
                "{\"outcome\":{\"outcome\":\"rejected\"}}\n",
                "{\"outcome\":{\"outcome\":\"accepted\",\"action\":{\"recipient\":1}}}\n",
            ),
        )
        .unwrap();
        let outcome = replay(
            &file,
            &[10, 10, 10],
            std::fs::metadata(&file).unwrap().len(),
        )
        .unwrap();
        assert_eq!(outcome.wealth, vec![10, 12, 10]);
        assert_eq!(outcome.opportunities, 2);
        std::fs::remove_file(file).unwrap();
    }
    #[test]
    fn replay_stops_at_verified_prefix_and_rejects_torn_records() {
        let file =
            std::env::temp_dir().join(format!("reference-prefix-{}.jsonl", std::process::id()));
        let first = "{\"outcome\":{\"outcome\":\"accepted\",\"action\":{\"recipient\":0}}}\n";
        let appended = "{\"outcome\":{\"outcome\":\"accepted\",\"action\":{\"recipient\":1}}}\n";
        std::fs::write(&file, format!("{first}{appended}")).unwrap();
        let outcome = replay(&file, &[10, 10], first.len() as u64).unwrap();
        assert_eq!(outcome.wealth, vec![11, 10]);
        assert_eq!(outcome.opportunities, 1);
        assert!(replay(&file, &[10, 10], first.len() as u64 - 1).is_err());
        assert!(replay(&file, &[10, 10], 10_000).is_err());
        std::fs::remove_file(file).unwrap();
    }
    #[test]
    fn top_share_matches_hand_computed_definition() {
        // ceil(n/5) top balances: n=5 -> 1, n=7 -> 2, n=12 -> 3, ties split.
        assert_eq!(reference_top_share(&[1, 1, 1, 1, 6]).unwrap(), 0.6);
        assert_eq!(reference_top_share(&[1, 1, 1, 1, 1, 2, 3]).unwrap(), 0.5);
        let twelve = [1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 5];
        assert_eq!(reference_top_share(&twelve).unwrap(), 12.0 / 21.0);
        assert_eq!(
            reference_top_share(&[5, 5, 5, 1, 1, 1, 1]).unwrap(),
            10.0 / 19.0
        );
        for values in [twelve.to_vec(), vec![5, 5, 5, 1, 1, 1, 1], vec![0, 0, 9]] {
            assert_eq!(
                top_quintile_share(&values).unwrap(),
                reference_top_share(&values).unwrap()
            );
        }
    }
}
