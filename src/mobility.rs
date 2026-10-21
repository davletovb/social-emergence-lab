//! Rank movement between non-identical time checkpoints.
//! Average (mid-)ranks make ties permutation invariant.
pub fn midranks(values: &[u64]) -> Result<Vec<f64>, String> {
    if values.len() < 2 {
        return Err("need at least two actors".into());
    }
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by_key(|&id| values[id]);
    let mut ranks = vec![0.0; values.len()];
    let mut start = 0;
    while start < order.len() {
        let mut end = start + 1;
        while end < order.len() && values[order[start]] == values[order[end]] {
            end += 1;
        }
        let midpoint = (start + end - 1) as f64 / 2.0;
        for k in start..end {
            ranks[order[k]] = midpoint;
        }
        start = end;
    }
    Ok(ranks)
}

/// Spearman rank persistence, via Pearson correlation of tie-aware average ranks.
/// Undefined if either checkpoint has no rank variation.
pub fn rank_persistence(early: &[u64], late: &[u64]) -> Result<Option<f64>, String> {
    if early.len() != late.len() {
        return Err("actor populations changed".into());
    }
    let x = midranks(early)?;
    let y = midranks(late)?;
    let mx = x.iter().sum::<f64>() / x.len() as f64;
    let my = y.iter().sum::<f64>() / y.len() as f64;
    let (mut cov, mut vx, mut vy) = (0.0, 0.0, 0.0);
    for (&a, &b) in x.iter().zip(y.iter()) {
        let dx = a - mx;
        let dy = b - my;
        cov += dx * dy;
        vx += dx * dx;
        vy += dy * dy;
    }
    if vx == 0.0 || vy == 0.0 {
        return Ok(None);
    }
    Ok(Some((cov / (vx * vy).sqrt()).clamp(-1.0, 1.0)))
}

/// Descriptive average absolute fractional rank movement, 0 for no movement.
/// A fully tied baseline can yield movements even without identifiable ranks;
/// callers therefore use a noninitial checkpoint with unequal balances.
pub fn mean_rank_movement(early: &[u64], late: &[u64]) -> Result<f64, String> {
    if early.len() != late.len() {
        return Err("actor populations changed".into());
    }
    let x = midranks(early)?;
    let y = midranks(late)?;
    Ok(x.iter()
        .zip(y.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f64>()
        / (x.len() * (x.len() - 1)) as f64)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tie_midrank_and_inverse() {
        assert_eq!(midranks(&[1, 1, 4, 9]).unwrap(), vec![0.5, 0.5, 2.0, 3.0]);
        assert_eq!(
            rank_persistence(&[1, 2, 3], &[3, 2, 1]).unwrap(),
            Some(-1.0)
        );
        assert_eq!(rank_persistence(&[1, 2, 3], &[1, 2, 3]).unwrap(), Some(1.0));
        assert!(rank_persistence(&[1, 1, 1], &[1, 2, 3]).unwrap().is_none());
        assert_eq!(mean_rank_movement(&[1, 2, 3], &[1, 2, 3]).unwrap(), 0.0);
    }
    #[test]
    fn concurrent_permutation_preserves_both_metrics() {
        let a = [1, 3, 3, 8, 12];
        let b = [7, 1, 5, 5, 9];
        let p = [3, 0, 4, 2, 1];
        let x = rank_persistence(&a, &b).unwrap();
        let y = rank_persistence(&p.map(|i| a[i]), &p.map(|i| b[i])).unwrap();
        assert_eq!(x, y);
        assert!(
            (mean_rank_movement(&a, &b).unwrap()
                - mean_rank_movement(&p.map(|i| a[i]), &p.map(|i| b[i])).unwrap())
            .abs()
                < 1e-14
        );
    }
}
