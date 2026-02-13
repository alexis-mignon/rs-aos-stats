use statrs::distribution::{Discrete, Multinomial};

fn generate_partitions_recursive(
    n_partitions: usize,
    _n_elements: u32,
    current_partition_index: usize,
    remaining_elements: u32,
    current_partition: &mut Vec<u32>,
    partitions: &mut Vec<Vec<u32>>,
) {
    if current_partition_index == n_partitions - 1 {
        current_partition[current_partition_index] = remaining_elements;
        partitions.push(current_partition.clone());
        return;
    }

    for i in 0..=remaining_elements {
        current_partition[current_partition_index] = i;
        generate_partitions_recursive(
            n_partitions,
            _n_elements,
            current_partition_index + 1,
            remaining_elements - i,
            current_partition,
            partitions,
        );
    }
}

// Generate all the possible partitions of a number of elements into a given number of partitions
pub fn generate_partitions(n_partitions: usize, n_elements: u32) -> Vec<Vec<u32>> {
    let mut partitions = Vec::new();
    let mut current_partition = vec![0; n_partitions];
    generate_partitions_recursive(
        n_partitions,
        n_elements,
        0,
        n_elements,
        &mut current_partition,
        &mut partitions,
    );
    partitions
}

pub fn generate_partitions_probabilities(
    n_elements: u32,
    probabilities: &[f64],
) -> Vec<(Vec<u32>, f64)> {
    let n_partitions = probabilities.len();
    let partitions = generate_partitions(n_partitions, n_elements);
    let mut partitions_probabilities = Vec::new();
    let multinomial = Multinomial::new(probabilities, n_elements as u64).unwrap();

    for partition in partitions {
        let partition_probability = multinomial.pmf(
            partition
                .iter()
                .map(|&x| x as u64)
                .collect::<Vec<u64>>()
                .as_slice(),
        );
        partitions_probabilities.push((partition, partition_probability));
    }
    partitions_probabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Distributing 2 elements into 3 bins should produce C(2+3-1, 3-1) = 6 partitions.
    #[test]
    fn partition_count() {
        let partitions = generate_partitions(3, 2);
        assert_eq!(partitions.len(), 6);
    }

    /// Every partition of 4 elements into 3 bins must have its parts sum to 4.
    #[test]
    fn partition_sums() {
        let partitions = generate_partitions(3, 4);
        for p in &partitions {
            let s: u32 = p.iter().sum();
            assert_eq!(s, 4);
        }
    }

    /// The multinomial probabilities over all partitions must sum to 1,
    /// ensuring the probability distribution is valid.
    #[test]
    fn probabilities_sum_to_one() {
        let probas = vec![0.5, 0.3, 0.2];
        let partitions = generate_partitions_probabilities(3, &probas);
        let total: f64 = partitions.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }
}
