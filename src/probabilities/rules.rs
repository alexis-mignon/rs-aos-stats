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
                    _ => (config.modifier.apply_to_wound_modifier(roll) >= config.defense_stats.to_save + config.attack_stats.rend) as u32 as f64 / 6.0
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
                (roll_values.iter().zip(counts).map(|(value, count)| (value * count)).sum(), *proba)
            }
        ).collect()
    }
}

impl Rule for DamagesRule {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode> {
        let num_wounds = node.status.wounds + node.status.mortal_wounds;
        //println!("{:?}", node.config.attack_stats);
        let damages_and_probas = match node.config.attack_stats.damages {
            Characteristic::Value(value) => vec![(value * num_wounds, 1.0)],
            Characteristic::DiceRoll(roll) => DamagesRule::_random_damages(roll, num_wounds)
        };
        //println!("{:?}", damages_and_probas);
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