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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode;

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

    fn result(&self, partition: &Vec<u32>) -> (u32, u32, u32);
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
    fn result(&self, partition: &Vec<u32>) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
        let priors = rolls_probas.iter().map(|(_, proba)| *proba).collect();
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
        let success_proba = (1..=6).map(
            |roll| {
                match roll {
                    1 => 0.0,
                    _ => (roll >= config.defense_stats.ward.unwrap()) as u32 as f64 / 6.0
                }
            }
        ).sum();
    
        vec![success_proba, 1.0 - success_proba]

    }

    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
        if let Some(_) = node.config.defense_stats.ward {
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
    fn result(&self, partition: &Vec<u32>) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
    fn result(&self, partition: &Vec<u32>) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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
    fn result(&self, partition: &Vec<u32>) -> (u32, u32, u32) {
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
    fn build_node(&self, node: &CombatNode, counts: &Vec<u32>, probability: f64) -> CombatNode {
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

    fn proba_sum(results: &[(u32, f64)]) -> f64 {
        results.iter().map(|(_, p)| p).sum()
    }

    fn standard_sequence() -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(AttackCharacteristicRule),
            Box::new(HitRule),
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ]
    }

    #[test]
    fn simple_fixed_attack() {
        // 1 attack, 2+ hit, 2+ wound, no save (7+), 1 damage
        let config = make_config(1, 2, 2, 0, 1, 7, None);
        let results = compute_damages(config, &standard_sequence());
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        // With 2+ to hit (5/6 considering crit) and 2+ to wound (5/6), P(1 damage) = 25/36
        let p1 = results.iter().find(|(d, _)| *d == 1).map(|(_, p)| *p).unwrap_or(0.0);
        assert!((p1 - 25.0 / 36.0).abs() < 1e-10);
    }

    #[test]
    fn all_miss() {
        // to_hit = 7 and to_wound = 7: no damage possible
        let config = make_config(1, 7, 7, 0, 1, 7, None);
        let results = compute_damages(config, &standard_sequence());
        // All probability should be on damage = 0
        let p0: f64 = results.iter().filter(|(d, _)| *d == 0).map(|(_, p)| *p).sum();
        assert!((p0 - 1.0).abs() < 1e-10);
        // No non-zero damage entries
        let non_zero: f64 = results.iter().filter(|(d, _)| *d > 0).map(|(_, p)| *p).sum();
        assert!(non_zero.abs() < 1e-10);
    }

    #[test]
    fn crit_double_hit_max_damage() {
        // 3 attacks, 4+ hit, 4+ wound, no save, 2 damage, crit double hit
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
        // Max damage: all 3 crits = 6 hits, all wound = 6 wounds * 2 damage = 12
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert_eq!(max_damage, 12);
    }

    #[test]
    fn crit_auto_wound() {
        // 1 attack, 4+ hit, 7 wound (impossible without crit auto-wound)
        let config = make_config(1, 4, 7, 0, 1, 7, None);
        let sequence: Vec<Box<dyn Rule>> = vec![
            Box::new(AttackCharacteristicRule),
            Box::new(CritAutoWoundRule),
            Box::new(WoundRule),
            Box::new(SaveRule),
            Box::new(DamagesRule),
        ];
        let results = compute_damages(config, &sequence);
        // Only crits (1/6) auto-wound, everything else misses wound roll
        let p1 = results.iter().find(|(d, _)| *d == 1).map(|(_, p)| *p).unwrap_or(0.0);
        assert!((p1 - 1.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn crit_mortal_wound() {
        // 1 attack, 4+ hit, 4+ wound, no save, 1 damage
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
        // Crits (1/6) generate mortal wounds instead of hits
        // Max damage should be 1 (mortal wound from crit, or wound from normal hit)
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert_eq!(max_damage, 1);
    }

    #[test]
    fn ward_save_reduces_damage() {
        // 1 attack, 2+ hit, 2+ wound, no armor save, 1 damage, 4+ ward
        let config = make_config(1, 2, 2, 0, 1, 7, Some(4));
        let mut sequence = standard_sequence();
        sequence.push(Box::new(WardRule));
        let results = compute_damages(config, &sequence);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        // P(hit)*P(wound) = 25/36, then ward saves half => P(1 damage) = 25/72
        let p1 = results.iter().find(|(d, _)| *d == 1).map(|(_, p)| *p).unwrap_or(0.0);
        assert!((p1 - 25.0 / 72.0).abs() < 1e-10);
    }

    #[test]
    fn probabilities_always_sum_to_one() {
        let config = make_config(3, 3, 4, 1, 2, 4, Some(6));
        let mut sequence = standard_sequence();
        sequence.push(Box::new(WardRule));
        let results = compute_damages(config, &sequence);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
    }
}