use std::collections::HashMap;

/// Sparse probability distribution over non-negative integer values.
/// Sorted by value, probabilities sum to 1.0.
pub type DamageDist = Vec<(u32, f64)>;

/// N-fold self-convolution of a distribution.
///
/// Computes the distribution of the sum of N i.i.d. random variables,
/// each distributed according to `dist`.
///
/// - n=0 returns `[(0, 1.0)]` (identity for addition)
/// - n=1 returns a copy of the input
pub fn convolve_n(dist: &[(u32, f64)], n: u32) -> DamageDist {
    if n == 0 {
        return vec![(0, 1.0)];
    }
    if n == 1 {
        return dist.to_vec();
    }

    let max_single = dist.iter().map(|(v, _)| *v as usize).max().unwrap_or(0);
    if max_single == 0 {
        return vec![(0, 1.0)];
    }

    let n = n as usize;
    let max_total = max_single * n;
    let mut current = vec![0.0_f64; max_total + 1];
    current[0] = 1.0;

    for _ in 0..n {
        let mut next = vec![0.0_f64; max_total + 1];
        for (sum, &p_sum) in current.iter().enumerate() {
            if p_sum == 0.0 {
                continue;
            }
            for &(value, p_value) in dist {
                let new_sum = sum + value as usize;
                if new_sum <= max_total {
                    next[new_sum] += p_sum * p_value;
                }
            }
        }
        current = next;
    }

    current
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p > 0.0)
        .map(|(d, p)| (d as u32, p))
        .collect()
}

/// Weighted mixture of distributions.
///
/// Computes `P(d) = Σ_i weight_i * P_i(d)`. Used for marginalizing
/// over random attack counts.
pub fn mix(components: &[(&[(u32, f64)], f64)]) -> DamageDist {
    let mut combined: HashMap<u32, f64> = HashMap::new();
    for &(dist, weight) in components {
        for &(value, prob) in dist {
            *combined.entry(value).or_insert(0.0) += weight * prob;
        }
    }
    let mut result: DamageDist = combined.into_iter().filter(|(_, p)| *p > 0.0).collect();
    result.sort_by_key(|(d, _)| *d);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sums_to_one(dist: &[(u32, f64)]) {
        let total: f64 = dist.iter().map(|(_, p)| p).sum();
        assert!(
            (total - 1.0).abs() < 1e-10,
            "Distribution sums to {total}, expected 1.0"
        );
    }

    #[test]
    fn convolve_n_zero_returns_identity() {
        let dist = vec![(1, 0.5), (2, 0.5)];
        let result = convolve_n(&dist, 0);
        assert_eq!(result, vec![(0, 1.0)]);
    }

    #[test]
    fn convolve_n_one_returns_input() {
        let dist = vec![(1, 0.5), (2, 0.5)];
        let result = convolve_n(&dist, 1);
        assert_eq!(result, dist);
    }

    #[test]
    fn convolve_n_two_coin_flips() {
        // Two fair coin flips: 0 or 1 with equal probability
        let coin = vec![(0, 0.5), (1, 0.5)];
        let result = convolve_n(&coin, 2);
        assert_sums_to_one(&result);
        // P(0) = 0.25, P(1) = 0.5, P(2) = 0.25
        assert_eq!(result.len(), 3);
        assert!((result[0].1 - 0.25).abs() < 1e-10);
        assert!((result[1].1 - 0.50).abs() < 1e-10);
        assert!((result[2].1 - 0.25).abs() < 1e-10);
    }

    #[test]
    fn convolve_n_three_d6() {
        // 3d6: each die is uniform over 1..=6
        let d6: Vec<(u32, f64)> = (1..=6).map(|v| (v, 1.0 / 6.0)).collect();
        let result = convolve_n(&d6, 3);
        assert_sums_to_one(&result);
        // Min = 3, Max = 18
        assert_eq!(result.first().unwrap().0, 3);
        assert_eq!(result.last().unwrap().0, 18);
    }

    #[test]
    fn convolve_n_all_zero_damage() {
        let dist = vec![(0, 1.0)];
        let result = convolve_n(&dist, 5);
        assert_eq!(result, vec![(0, 1.0)]);
    }

    #[test]
    fn mix_single_component() {
        let dist = vec![(1, 0.5), (2, 0.5)];
        let result = mix(&[(&dist, 1.0)]);
        assert_eq!(result, dist);
    }

    #[test]
    fn mix_weighted_combination() {
        let a = vec![(0, 1.0)]; // always 0
        let b = vec![(1, 1.0)]; // always 1
        let result = mix(&[(&a, 0.3), (&b, 0.7)]);
        assert_sums_to_one(&result);
        assert_eq!(result.len(), 2);
        assert!((result[0].1 - 0.3).abs() < 1e-10); // P(0) = 0.3
        assert!((result[1].1 - 0.7).abs() < 1e-10); // P(1) = 0.7
    }

    #[test]
    fn mix_overlapping_values() {
        let a = vec![(1, 0.5), (2, 0.5)];
        let b = vec![(1, 0.5), (3, 0.5)];
        let result = mix(&[(&a, 0.5), (&b, 0.5)]);
        assert_sums_to_one(&result);
        // P(1) = 0.5*0.5 + 0.5*0.5 = 0.5, P(2) = 0.25, P(3) = 0.25
        let p1 = result.iter().find(|(d, _)| *d == 1).unwrap().1;
        let p2 = result.iter().find(|(d, _)| *d == 2).unwrap().1;
        let p3 = result.iter().find(|(d, _)| *d == 3).unwrap().1;
        assert!((p1 - 0.5).abs() < 1e-10);
        assert!((p2 - 0.25).abs() < 1e-10);
        assert!((p3 - 0.25).abs() < 1e-10);
    }
}
