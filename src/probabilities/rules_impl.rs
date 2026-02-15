use crate::probabilities::combat_stats::Characteristic;
use crate::probabilities::compute_engine::{CombatConfig, CombatStatus, Rule};
use crate::probabilities::dice::DiceRoll;
use crate::probabilities::partitions::generate_partitions_probabilities;

#[derive(Clone, Debug)]
pub struct AttackCharacteristicRule;
/// Determines the number of attacks for the current profile.
///
/// If the characteristic is a fixed value we just forward it, otherwise
/// we expand the dice expression into an exact `(value, probability)`
/// distribution and branch the combat state accordingly.
impl Rule for AttackCharacteristicRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        let attack_num_stat = config.attack_stats.attacks;

        let values_and_probas = match attack_num_stat {
            Characteristic::Value(value) => vec![(value, 1.0)],
            Characteristic::DiceRoll(roll) => roll.values_and_probas(),
        };

        values_and_probas
            .iter()
            .map(|(value, proba)| (status.with_attacks(*value), probability * proba))
            .collect()
    }
}

/// Shared helper for rules that can be modelled as "roll N identical dice
/// and classify each roll into a small number of outcome buckets".
///
/// Examples: hit/wound/save tests, ward saves, and crit variants.
/// All of them only depend on the number of rolls, the per-roll outcome
/// probabilities and on how the final bucket counts update `CombatStatus`.
pub trait TestRollRule: Rule {
    fn roll_count(&self, status: &CombatStatus) -> u32;
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64>;
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus;

    /// Apply the multinomial distribution induced by this rule to a single
    /// `(CombatStatus, probability)` state.
    ///
    /// - `partition_prior` gives the probability of each outcome bucket for
    ///   one die (e.g. `[P(crit), P(normal hit), P(miss)]`).
    /// - `roll_count` says how many dice we roll.
    /// - `generate_partitions_probabilities` then enumerates all possible
    ///   bucket count vectors and their multinomial probabilities.
    /// - For each such partition we build a new `CombatStatus` and scale the
    ///   current state probability accordingly.
    fn apply_distribution(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        let probas = self.partition_prior(config);
        let nrolls = self.roll_count(status);
        let partitions = generate_partitions_probabilities(nrolls, &probas);
        let mut results = vec![];
        for (counts, proba) in partitions {
            let new_status = self.build_status(status, &counts);
            results.push((new_status, probability * proba));
        }
        results
    }
}

/// Base implementation for hit-like rules with three outcome buckets:
/// critical hit, normal hit and failure.
///
/// Concrete rules only need to decide how the partition counts map to
/// `(hits, wounds, mortal_wounds)` via `result`.
pub trait BaseHitRule: TestRollRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        status.attacks
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        // Per-roll probabilities for the three buckets:
        //  - index 0: critical hits (natural 6)
        //  - index 1: normal successful hits (meeting to-hit after modifiers)
        //  - index 2: failures.

        let critical_proba = 1.0 / 6.0;
        let success_proba = (1..=6)
            .map(|roll| {
                match roll {
                    6 => 0.0, // 6s are critical and will be counted separately
                    1 => 0.0,
                    _ => {
                        (config.modifier.apply_to_hit_modifier(roll) >= config.attack_stats.to_hit)
                            as u32 as f64
                            / 6.0
                    }
                }
            })
            .sum();

        vec![
            1.0 / 6.0,
            success_proba,
            1.0 - success_proba - critical_proba,
        ]
    }

    fn result(&self, partition: &[u32]) -> (u32, u32, u32);
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        let (hits, wounds, mortal_wounds) = self.result(counts);
        status
            .with_attacks(0)
            .with_hits(hits)
            .with_wounds(wounds)
            .with_mortal_wounds(mortal_wounds)
    }
}

#[derive(Clone, Debug)]
pub struct HitRule;

impl BaseHitRule for HitRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        (partition[0] + partition[1], 0, 0)
    }
}

impl TestRollRule for HitRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        BaseHitRule::roll_count(self, status)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::partition_prior(self, config)
    }
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        BaseHitRule::build_status(self, status, counts)
    }
}

impl Rule for HitRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        TestRollRule::apply_distribution(self, status, probability, config)
    }
}

#[derive(Clone, Debug)]
pub struct WoundRule;

impl TestRollRule for WoundRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        status.hits
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        // Two buckets for each wound roll:
        //  - index 0: successful wounds
        //  - index 1: failed wounds.
        let success_proba = (1..=6)
            .map(|roll| match roll {
                1 => 0.0,
                _ => {
                    (config.modifier.apply_to_wound_modifier(roll) >= config.attack_stats.to_wound)
                        as u32 as f64
                        / 6.0
                }
            })
            .sum();

        vec![success_proba, 1.0 - success_proba]
    }
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        status.with_hits(0).with_wounds(counts[0] + status.wounds)
    }
}

impl Rule for WoundRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        TestRollRule::apply_distribution(self, status, probability, config)
    }
}

#[derive(Clone, Debug)]
pub struct SaveRule;

impl TestRollRule for SaveRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        status.wounds
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        // Two buckets for each save roll:
        //  - index 0: successful saves
        //  - index 1: failed saves (become unsaved wounds).
        let success_proba = (1..=6)
            .map(|roll| match roll {
                1 => 0.0,
                _ => {
                    (config.modifier.apply_to_save_modifier(roll)
                        >= config.defense_stats.to_save + config.attack_stats.rend)
                        as u32 as f64
                        / 6.0
                }
            })
            .sum();

        vec![success_proba, 1.0 - success_proba]
    }
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        status.with_hits(0).with_wounds(status.wounds - counts[0])
    }
}

impl Rule for SaveRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        TestRollRule::apply_distribution(self, status, probability, config)
    }
}

#[derive(Clone, Debug)]
pub struct DamagesRule;

impl DamagesRule {
    fn _random_damages(roll: DiceRoll, num_wounds: u32) -> Vec<(u32, f64)> {
        // For each wound, damage is an independent draw from `roll`.
        //
        // If we have `num_wounds` such draws, the total damage is the sum of
        // `num_wounds` i.i.d. variables. We compute the exact distribution of
        // this sum by repeated convolution of the single-wound distribution
        // instead of enumerating all integer partitions, which would be
        // combinatorially expensive for large `num_wounds`.

        let n = num_wounds as usize;
        if n == 0 {
            return vec![(0, 1.0)];
        }

        let single = roll.values_and_probas();
        // Maximum damage from a single wound
        let max_single = single.iter().map(|(v, _)| *v as usize).max().unwrap_or(0);

        if max_single == 0 {
            return vec![(0, 1.0)];
        }

        let max_total = max_single * n;
        let mut dist = vec![0.0_f64; max_total + 1];
        dist[0] = 1.0;

        for _ in 0..n {
            let mut new_dist = vec![0.0_f64; max_total + 1];
            for (sum, &p_sum) in dist.iter().enumerate() {
                if p_sum == 0.0 {
                    continue;
                }
                for (d, p_d) in &single {
                    let new_sum = sum + *d as usize;
                    if new_sum <= max_total {
                        new_dist[new_sum] += p_sum * p_d;
                    }
                }
            }
            dist = new_dist;
        }

        let mut results: Vec<(u32, f64)> = dist
            .iter()
            .enumerate()
            .filter_map(|(damage, &p)| {
                if p > 0.0 {
                    Some((damage as u32, p))
                } else {
                    None
                }
            })
            .collect();

        // Ensure a stable, ascending order (useful for callers/tests).
        results.sort_by_key(|(d, _)| *d);
        results
    }
}

impl Rule for DamagesRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        let num_wounds = status.wounds + status.mortal_wounds;
        let damages_and_probas = match config.attack_stats.damages {
            Characteristic::Value(value) => vec![(value * num_wounds, 1.0)],
            Characteristic::DiceRoll(roll) => DamagesRule::_random_damages(roll, num_wounds),
        };
        damages_and_probas
            .iter()
            .map(|(damages, proba)| {
                (
                    status
                        .with_mortal_wounds(0)
                        .with_wounds(0)
                        .with_damages(*damages),
                    probability * proba,
                )
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct WardRule;

impl TestRollRule for WardRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        status.damages
    }

    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let ward_value = config
            .defense_stats
            .ward
            .expect("WardRule requires ward save to be set");
        // Two buckets for each ward roll:
        //  - index 0: successful ward (damage prevented)
        //  - index 1: failed ward (damage goes through).
        let success_proba = (1..=6)
            .map(|roll| match roll {
                1 => 0.0,
                _ => (roll >= ward_value) as u32 as f64 / 6.0,
            })
            .sum();

        vec![success_proba, 1.0 - success_proba]
    }

    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        status.with_damages(status.damages - counts[0])
    }
}

impl Rule for WardRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        if config.defense_stats.ward.is_some() {
            TestRollRule::apply_distribution(self, status, probability, config)
        } else {
            vec![]
        }
    }
}

#[derive(Clone, Debug)]
pub struct CritMortalWoundRule;

impl BaseHitRule for CritMortalWoundRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        // `partition[0]` = crits, `partition[1]` = normal hits.
        // Crits become mortal wounds, normal hits stay as hits.
        (partition[1], 0, partition[0])
    }
}

impl TestRollRule for CritMortalWoundRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        BaseHitRule::roll_count(self, status)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::partition_prior(self, config)
    }
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        BaseHitRule::build_status(self, status, counts)
    }
}

impl Rule for CritMortalWoundRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        TestRollRule::apply_distribution(self, status, probability, config)
    }
}

#[derive(Clone, Debug)]
pub struct CritAutoWoundRule;

impl BaseHitRule for CritAutoWoundRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        // `partition[0]` = crits, `partition[1]` = normal hits.
        // Crits skip the wound roll and go straight to the wound pool.
        (partition[1], partition[0], 0)
    }
}

impl TestRollRule for CritAutoWoundRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        BaseHitRule::roll_count(self, status)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::partition_prior(self, config)
    }
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        BaseHitRule::build_status(self, status, counts)
    }
}

impl Rule for CritAutoWoundRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        TestRollRule::apply_distribution(self, status, probability, config)
    }
}

#[derive(Clone, Debug)]
pub struct CritDoubleHitRule;

impl BaseHitRule for CritDoubleHitRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        // `partition[0]` = crits, `partition[1]` = normal hits.
        // Crits count as two hits each.
        (2 * partition[0] + partition[1], 0, 0)
    }
}

impl TestRollRule for CritDoubleHitRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {
        BaseHitRule::roll_count(self, status)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::partition_prior(self, config)
    }
    fn build_status(&self, status: &CombatStatus, counts: &[u32]) -> CombatStatus {
        BaseHitRule::build_status(self, status, counts)
    }
}

impl Rule for CritDoubleHitRule {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)> {
        TestRollRule::apply_distribution(self, status, probability, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilities::combat_stats::{AttackStats, DefenseStats};
    use crate::probabilities::compute_engine::compute_damages;

    /// Helper: build a CombatConfig with fixed-value attacks and damages.
    fn make_config(
        attacks: u32,
        to_hit: u32,
        to_wound: u32,
        rend: u32,
        damages: u32,
        to_save: u32,
        ward: Option<u32>,
    ) -> CombatConfig {
        CombatConfig::new(
            AttackStats::new(
                Characteristic::Value(attacks),
                to_hit,
                to_wound,
                rend,
                Characteristic::Value(damages),
            ),
            DefenseStats::new(to_save, ward),
        )
    }

    /// Helper: sum all probabilities in a damage distribution.
    fn proba_sum(results: &[(u32, f64)]) -> f64 {
        results.iter().map(|(_, p)| p).sum()
    }

    /// Helper: the standard combat sequence (attacks → hit → wound → save → damages).
    fn standard_sequence() -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(AttackCharacteristicRule),
            Box::new(HitRule),
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ]
    }

    /// End-to-end test with a simple profile: 1 attack, 2+ hit, 2+ wound,
    /// no save, 1 damage. Verifies the exact P(1 damage) = 25/36
    /// (5/6 hit chance including crits * 5/6 wound chance).
    #[test]
    fn simple_fixed_attack() {
        let config = make_config(1, 2, 2, 0, 1, 7, None);
        let results = compute_damages(config, &standard_sequence());
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let p1 = results
            .iter()
            .find(|(d, _)| *d == 1)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        assert!((p1 - 25.0 / 36.0).abs() < 1e-10);
    }

    /// When both to_hit and to_wound thresholds are impossible (7+),
    /// all probability must land on 0 damage. Note that crits (6) still
    /// hit, but the wound roll at 7+ blocks all damage.
    #[test]
    fn all_miss() {
        let config = make_config(1, 7, 7, 0, 1, 7, None);
        let results = compute_damages(config, &standard_sequence());
        let p0: f64 = results
            .iter()
            .filter(|(d, _)| *d == 0)
            .map(|(_, p)| *p)
            .sum();
        assert!((p0 - 1.0).abs() < 1e-10);
        let non_zero: f64 = results
            .iter()
            .filter(|(d, _)| *d > 0)
            .map(|(_, p)| *p)
            .sum();
        assert!(non_zero.abs() < 1e-10);
    }

    /// CritDoubleHitRule: critical rolls of 6 count as two hits instead of
    /// one. With 3 attacks and 2 damage each, the theoretical maximum is
    /// 3 crits * 2 hits * 2 damage = 12.
    #[test]
    fn crit_double_hit_max_damage() {
        let config = make_config(3, 4, 4, 0, 2, 7, None);
        let sequence: Vec<Box<dyn Rule>> = vec![
            Box::new(AttackCharacteristicRule),
            Box::new(CritDoubleHitRule),
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ];
        let results = compute_damages(config, &sequence);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert_eq!(max_damage, 12);
    }

    /// CritAutoWoundRule: crits bypass the wound roll entirely. With
    /// to_wound=7 (impossible), only crits (1/6) can deal damage, so
    /// P(1 damage) = 1/6.
    #[test]
    fn crit_auto_wound() {
        let config = make_config(1, 4, 7, 0, 1, 7, None);
        let sequence: Vec<Box<dyn Rule>> = vec![
            Box::new(AttackCharacteristicRule),
            Box::new(CritAutoWoundRule),
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ];
        let results = compute_damages(config, &sequence);
        let p1 = results
            .iter()
            .find(|(d, _)| *d == 1)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        assert!((p1 - 1.0 / 6.0).abs() < 1e-10);
    }

    /// CritMortalWoundRule: crits generate mortal wounds instead of normal
    /// hits. With 1 attack and 1 damage, the max damage is still 1
    /// (either via mortal wound from a crit, or via a normal wound).
    #[test]
    fn crit_mortal_wound() {
        let config = make_config(1, 4, 4, 0, 1, 7, None);
        let sequence: Vec<Box<dyn Rule>> = vec![
            Box::new(AttackCharacteristicRule),
            Box::new(CritMortalWoundRule),
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ];
        let results = compute_damages(config, &sequence);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert_eq!(max_damage, 1);
    }

    /// A 4+ ward save should halve the probability of taking damage.
    /// With P(wound) = 25/36 and a 4+ ward (50% chance to negate),
    /// P(1 damage) = 25/72.
    #[test]
    fn ward_save_reduces_damage() {
        let config = make_config(1, 2, 2, 0, 1, 7, Some(4));
        let mut sequence = standard_sequence();
        sequence.push(Box::new(WardRule));
        let results = compute_damages(config, &sequence);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let p1 = results
            .iter()
            .find(|(d, _)| *d == 1)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        assert!((p1 - 25.0 / 72.0).abs() < 1e-10);
    }

    /// Sanity check on a more complex profile (3 attacks, 3+ hit, 4+ wound,
    /// rend 1, 2 damage, 4+ save, 6+ ward): probabilities must sum to 1.
    #[test]
    fn probabilities_always_sum_to_one() {
        let config = make_config(3, 3, 4, 1, 2, 4, Some(6));
        let mut sequence = standard_sequence();
        sequence.push(Box::new(WardRule));
        let results = compute_damages(config, &sequence);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// Test AttackCharacteristicRule with DiceRoll (D3) instead of fixed value.
    /// With D3 attacks (1-3), we should get different possible damage outcomes.
    #[test]
    fn attack_characteristic_with_dice() {
        let config = CombatConfig::new(
            AttackStats::new(
                Characteristic::DiceRoll(DiceRoll::D3),
                2,
                2,
                0,
                Characteristic::Value(1),
            ),
            DefenseStats::new(7, None),
        );
        let results = compute_damages(config, &standard_sequence());
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        // With D3 attacks, max damage should be 3 (if all 3 attacks hit and wound)
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert!(max_damage <= 3);
    }

    /// Test DamagesRule with DiceRoll (D3) instead of fixed damage value.
    /// With 1 attack dealing D3 damage, outcomes should range from 0 to 3.
    #[test]
    fn damages_with_dice() {
        let config = CombatConfig::new(
            AttackStats::new(
                Characteristic::Value(1),
                2,
                2,
                0,
                Characteristic::DiceRoll(DiceRoll::D3),
            ),
            DefenseStats::new(7, None),
        );
        let results = compute_damages(config, &standard_sequence());
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        // With D3 damage, if the attack hits and wounds, damage should be 1-3
        let has_positive_damage = results.iter().any(|(d, p)| *d > 0 && *p > 0.0);
        assert!(has_positive_damage);
    }

    /// Test WardRule when no ward save is present. The rule should return
    /// an empty vector when config.defense_stats.ward is None.
    #[test]
    fn ward_rule_no_ward() {
        let config = make_config(1, 2, 2, 0, 1, 7, None);
        let ward_rule = WardRule;
        let status = CombatStatus::new_with_values(0, 0, 0, 0, 1);
        let result = ward_rule.apply(&status, 1.0, &config);
        // When no ward is present, WardRule should return an empty vector
        assert_eq!(result.len(), 0);
    }

    /// Helper: compute mean damage for a given config and sequence.
    fn mean_damage(config: CombatConfig, sequence: &Vec<Box<dyn Rule>>) -> f64 {
        compute_damages(config, sequence)
            .iter()
            .map(|(d, p)| *d as f64 * p)
            .sum()
    }

    /// Helper: build a crit sequence replacing HitRule with the given crit rule.
    fn crit_sequence(crit_rule: Box<dyn Rule>) -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(AttackCharacteristicRule),
            crit_rule,
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ]
    }

    /// CritAutoWoundRule should increase mean damage compared to normal hits.
    #[test]
    fn crit_auto_wound_increases_mean_damage() {
        let config = make_config(10, 3, 3, 1, 1, 4, None);
        let normal = mean_damage(config, &standard_sequence());
        let crit = mean_damage(config, &crit_sequence(Box::new(CritAutoWoundRule)));
        assert!(
            crit > normal,
            "CritAutoWound ({crit:.4}) should exceed normal ({normal:.4})"
        );
    }

    /// CritMortalWoundRule should increase mean damage compared to normal hits.
    #[test]
    fn crit_mortal_wound_increases_mean_damage() {
        let config = make_config(10, 3, 3, 1, 1, 4, None);
        let normal = mean_damage(config, &standard_sequence());
        let crit = mean_damage(config, &crit_sequence(Box::new(CritMortalWoundRule)));
        assert!(
            crit > normal,
            "CritMortalWound ({crit:.4}) should exceed normal ({normal:.4})"
        );
    }

    /// CritDoubleHitRule should increase mean damage compared to normal hits.
    #[test]
    fn crit_double_hit_increases_mean_damage() {
        let config = make_config(10, 3, 3, 1, 1, 4, None);
        let normal = mean_damage(config, &standard_sequence());
        let crit = mean_damage(config, &crit_sequence(Box::new(CritDoubleHitRule)));
        assert!(
            crit > normal,
            "CritDoubleHit ({crit:.4}) should exceed normal ({normal:.4})"
        );
    }

    /// All crit rules should increase mean damage even with a difficult
    /// to-hit roll (6+ means only crits land).
    #[test]
    fn crit_rules_increase_damage_at_six_plus_to_hit() {
        let config = make_config(10, 6, 3, 0, 1, 4, None);
        let normal = mean_damage(config, &standard_sequence());
        for (name, rule) in [
            (
                "CritAutoWound",
                Box::new(CritAutoWoundRule) as Box<dyn Rule>,
            ),
            (
                "CritMortalWound",
                Box::new(CritMortalWoundRule) as Box<dyn Rule>,
            ),
            (
                "CritDoubleHit",
                Box::new(CritDoubleHitRule) as Box<dyn Rule>,
            ),
        ] {
            let crit = mean_damage(config, &crit_sequence(rule));
            assert!(
                crit > normal,
                "{name} ({crit:.4}) should exceed normal ({normal:.4}) at 6+ to hit"
            );
        }
    }

    /// Ward save should strictly reduce mean damage across several profiles.
    #[test]
    fn ward_reduces_mean_damage() {
        let profiles = [
            make_config(10, 3, 3, 1, 1, 4, Some(4)),
            make_config(5, 2, 2, 0, 2, 5, Some(5)),
            make_config(3, 4, 4, 2, 3, 3, Some(6)),
        ];
        for config in profiles {
            let mut ward_sequence = standard_sequence();
            ward_sequence.push(Box::new(WardRule));

            let without = mean_damage(config, &standard_sequence());
            let with = mean_damage(config, &ward_sequence);
            assert!(
                with < without,
                "Ward should reduce damage: {with:.4} >= {without:.4} for config {:?}",
                config
            );
        }
    }

    /// Stronger ward saves (lower threshold) should reduce damage more.
    #[test]
    fn stronger_ward_reduces_more() {
        let config_4plus = make_config(10, 3, 3, 1, 2, 4, Some(4));
        let config_5plus = make_config(10, 3, 3, 1, 2, 4, Some(5));
        let mut ward_sequence = standard_sequence();
        ward_sequence.push(Box::new(WardRule));

        let mean_4plus = mean_damage(config_4plus, &ward_sequence);
        let mean_5plus = mean_damage(config_5plus, &ward_sequence);
        assert!(
            mean_4plus < mean_5plus,
            "4+ ward ({mean_4plus:.4}) should reduce more than 5+ ({mean_5plus:.4})"
        );
    }
}
