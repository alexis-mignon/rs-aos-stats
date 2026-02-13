use crate::probabilities::combat_stats::Characteristic;
use crate::probabilities::combat_tree::{CombatNode, CombatStatus, CombatConfig, Rule};
use crate::probabilities::partitions::generate_partitions_probabilities;
use crate::probabilities::dice::DiceRoll;

#[derive(Clone, Debug)]
pub struct AttackCharacteristicRule;
/// Dertmines the number of attacks
impl Rule for AttackCharacteristicRule {
    fn apply(
        &self,
        node: &CombatNode
    )-> Vec<CombatNode> {
        let attack_num_stat = node.config.attack_stats.attacks;

        let values_and_probas = match attack_num_stat {
            Characteristic::Value(value) => vec![(value, 1.0)],
            Characteristic::DiceRoll(roll) => roll.values_and_probas()
        };

        values_and_probas.iter().map(
            |(value, proba)| CombatNode::new(node.status.with_attacks(*value), node.config, node.probability * proba)
        ).collect()
    }
}

pub trait TestRollRule : Rule {
    fn roll_count(&self, status: &CombatStatus) -> u32;
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64>;
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode;

    fn apply(
        &self,
        node: &CombatNode
    )-> Vec<CombatNode>{
        let probas = self.partition_prior(&node.config);
        let nrolls = self.roll_count(&node.status);
        let partitions = generate_partitions_probabilities(nrolls, &probas);
        let mut nodes = vec![];
        for (counts, proba) in partitions {
            let new_node = self.build_node(node, &counts, proba);
            nodes.push(new_node);
        }
        nodes
    }
}

pub trait BaseHitRule : TestRollRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {status.attacks}
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        // Compute the probability of success
        
        let critical_proba = 1.0 / 6.0;
        let success_proba = (1..=6).map(
            |roll| {
                match roll {
                    6 => 0.0, // 6s are critical and will be counted separately
                    1 => 0.0,
                    _ => (config.modifier.apply_to_hit_modifier(roll) >= config.attack_stats.to_hit) as u32 as f64 / 6.0
                }
            }
        ).sum();
    
        vec![1.0 / 6.0, success_proba, 1.0 - success_proba - critical_proba ]
    }

    fn result(&self, partition: &[u32]) -> (u32, u32, u32);
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        let (hits, wounds, mortal_wounds) = self.result(counts);
        CombatNode::new(
            node.status
            .with_attacks(0)
            .with_hits(hits)
            .with_wounds(wounds)
            .with_mortal_wounds(mortal_wounds),
            node.config,
            node.probability * probability
        )
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
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        BaseHitRule::build_node(self, node, counts, probability)
    }

}

impl Rule for HitRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        TestRollRule::apply(self, node)
    }
}

#[derive(Clone, Debug)]
pub struct WoundRule;

impl TestRollRule for WoundRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {status.hits}
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let success_proba = (1..=6).map(
            |roll| {
                match roll {
                    1 => 0.0,
                    _ => (config.modifier.apply_to_wound_modifier(roll) >= config.attack_stats.to_wound) as u32 as f64 / 6.0
                }
            }
        ).sum();
    
        vec![success_proba, 1.0 - success_proba]

    }
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        CombatNode::new(
            node.status
                .with_hits(0)
                .with_wounds(counts[0] + node.status.wounds),
            node.config,
            probability * node.probability
        )
    }
}

impl Rule for WoundRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        TestRollRule::apply(self, node)
    }
}

#[derive(Clone, Debug)]
pub struct SaveRule;

impl TestRollRule for SaveRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {status.wounds}
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let success_proba = (1..=6).map(
            |roll| {
                match roll {
                    1 => 0.0,
                    _ => (config.modifier.apply_to_save_modifier(roll) >= config.defense_stats.to_save + config.attack_stats.rend) as u32 as f64 / 6.0
                }
            }
        ).sum();
    
        vec![success_proba, 1.0 - success_proba]

    }
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        CombatNode::new(
            node.status
                .with_hits(0)
                .with_wounds(node.status.wounds - counts[0]),
            node.config,
            probability * node.probability
        )
    }
}

impl Rule for SaveRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        TestRollRule::apply(self, node)
    }
}

#[derive(Clone, Debug)]
pub struct DamagesRule;

impl DamagesRule {
    fn _random_damages(roll: DiceRoll, num_wounds: u32) -> Vec<(u32, f64)> {
        let rolls_probas = roll.values_and_probas();
        let priors: Vec<f64> = rolls_probas.iter().map(|(_, proba)| *proba).collect();
        let roll_values: Vec<u32> = rolls_probas.iter().map(|(value, _)| *value).collect();
        let partitions = generate_partitions_probabilities(num_wounds, &priors);
        partitions.iter().map(
            |(counts, proba)| {
                (roll_values.iter().zip(counts).map(|(value, count)| value * count).sum(), *proba)
            }
        ).collect()
    }
}

impl Rule for DamagesRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        let num_wounds = node.status.wounds + node.status.mortal_wounds;
        let damages_and_probas = match node.config.attack_stats.damages {
            Characteristic::Value(value) => vec![(value * num_wounds, 1.0)],
            Characteristic::DiceRoll(roll) => DamagesRule::_random_damages(roll, num_wounds)
        };
        damages_and_probas.iter().map(
            |(damages, proba)| {
                CombatNode::new(
                    node.status
                        .with_mortal_wounds(0)
                        .with_wounds(0)
                        .with_damages(*damages),
                    node.config,
                    proba * node.probability
                )
            }
        ).collect()
    }
}


#[derive(Clone, Debug)]
pub struct WardRule;

impl TestRollRule for WardRule {
    fn roll_count(&self, status: &CombatStatus) -> u32 {status.damages}

    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let ward_value = config.defense_stats.ward.expect("WardRule requires ward save to be set");
        let success_proba = (1..=6).map(
            |roll| {
                match roll {
                    1 => 0.0,
                    _ => (roll >= ward_value) as u32 as f64 / 6.0
                }
            }
        ).sum();
    
        vec![success_proba, 1.0 - success_proba]

    }

    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        CombatNode::new(
            node.status
                .with_damages(node.status.damages - counts[0]),
            node.config,
            probability * node.probability
        )
    }

}

impl Rule for WardRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        if node.config.defense_stats.ward.is_some() {
            TestRollRule::apply(self, node)
        }
        else {
            vec![]
        }
    }
}

#[derive(Clone, Debug)]
pub struct CritMortalWoundRule;

impl BaseHitRule for CritMortalWoundRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        BaseHitRule::build_node(self, node, counts, probability)
    }
}

impl Rule for CritMortalWoundRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        TestRollRule::apply(self, node)
    }
}

#[derive(Clone, Debug)]
pub struct CritAutoWoundRule;

impl BaseHitRule for CritAutoWoundRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        BaseHitRule::build_node(self, node, counts, probability)
    }
}

impl Rule for CritAutoWoundRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        TestRollRule::apply(self, node)
    }
}

#[derive(Clone, Debug)]
pub struct CritDoubleHitRule;

impl BaseHitRule for CritDoubleHitRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &[u32], probability: f64) -> CombatNode {
        BaseHitRule::build_node(self, node, counts, probability)
    }
}

impl Rule for CritDoubleHitRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        TestRollRule::apply(self, node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilities::combat_stats::{AttackStats, DefenseStats};
    use crate::probabilities::combat_tree::compute_damages;

    /// Helper: build a CombatConfig with fixed-value attacks and damages.
    fn make_config(attacks: u32, to_hit: u32, to_wound: u32, rend: u32, damages: u32, to_save: u32, ward: Option<u32>) -> CombatConfig {
        CombatConfig::new(
            AttackStats::new(
                Characteristic::Value(attacks),
                to_hit, to_wound, rend,
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
        let p1 = results.iter().find(|(d, _)| *d == 1).map(|(_, p)| *p).unwrap_or(0.0);
        assert!((p1 - 25.0 / 36.0).abs() < 1e-10);
    }

    /// When both to_hit and to_wound thresholds are impossible (7+),
    /// all probability must land on 0 damage. Note that crits (6) still
    /// hit, but the wound roll at 7+ blocks all damage.
    #[test]
    fn all_miss() {
        let config = make_config(1, 7, 7, 0, 1, 7, None);
        let results = compute_damages(config, &standard_sequence());
        let p0: f64 = results.iter().filter(|(d, _)| *d == 0).map(|(_, p)| *p).sum();
        assert!((p0 - 1.0).abs() < 1e-10);
        let non_zero: f64 = results.iter().filter(|(d, _)| *d > 0).map(|(_, p)| *p).sum();
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
        let p1 = results.iter().find(|(d, _)| *d == 1).map(|(_, p)| *p).unwrap_or(0.0);
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
        let p1 = results.iter().find(|(d, _)| *d == 1).map(|(_, p)| *p).unwrap_or(0.0);
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
                2, 2, 0,
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
                2, 2, 0,
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
        let node = CombatNode::new(
            CombatStatus::new_with_values(0, 0, 0, 0, 1),
            config,
            1.0,
        );
        let ward_rule = WardRule;
        let result = Rule::apply(&ward_rule, &node);
        // When no ward is present, WardRule should return an empty vector
        assert_eq!(result.len(), 0);
    }
}