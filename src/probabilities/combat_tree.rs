use std::collections::btree_map::Values;
use std::collections::HashMap;

use crate::probabilities::combat_stats::{
    CombatStatus, CombatStatusAttribute,
    AttackStats, AttackStatsModifier,
    DefenseStats, DefenseStatsModifier
};

use crate::probabilities::partitions::generate_partitions_probabilities;

use super::combat_stats::StatType;
use super::dice::{DiceRoll, TestRollOutcome};

#[derive(Clone)]
pub struct CombatNode {
    pub status: CombatStatus,
    pub probability: f64,
    pub children: Vec<CombatNode>,
}

impl CombatNode {
    pub fn new(status: CombatStatus, probability: f64) -> CombatNode {
        CombatNode {
            status,
            probability,
            children: Vec::new(),
        }
    }

    fn from_attributes_and_probability(status_attrs: &HashMap<CombatStatusAttribute, u32>, probability: f64) -> CombatNode {
        let mut new_status = CombatStatus::from_status_attributes(status_attrs);
        CombatNode::new(new_status, probability)
    }
    
    pub fn add_child(&mut self, node: CombatNode) {
        self.children.push(node);
    }

    pub fn leaves(&self) -> Vec<&CombatNode> {
        if self.children.is_empty() {
            return vec![self];
        }

        let mut leaves = Vec::new();
        for child in &self.children {
            leaves.extend(child.leaves());
        }
        leaves
    }

    pub fn leaves_with_probability(&self) -> Vec<(&CombatNode, f64)> {
        self.leaves()
            .iter()
            .map(|node| (*node, node.probability))
            .collect()
    }
}


pub trait AttackRule {
    fn apply(
        &self,
        node: &CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
    ) -> Vec<CombatNode>;
}

pub trait DefenseRule {
    fn apply(
        &self,
        node: &CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
        defense_stats: &DefenseStats,
        defense_modifier: &DefenseStatsModifier,
    ) -> (Vec<CombatNode>, bool);
}

type AttackSequence<'a> = Vec<dyn AttackRule + 'a>;
type DefenseSequence<'a> = Vec<dyn DefenseRule + 'a>;


pub trait QuantityAttackRule {
    fn roll_input_stats(&self, attack_stats: &AttackStats, attack_modifier: &Option<AttackStatsModifier>) -> (StatType, Option<i32>);

    fn apply(       
        &self,
        node: &mut CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
    )-> Vec<CombatNode> {
        let (stat, modifier) = self.roll_input_stats(attack_stats, attack_modifier);
        match stat {
            StatType::Value(value) => vec![],
            StatType::DiceRoll(roll) => self.resolve_roll(roll),
        }
    }
}

pub trait TestAttackRule: AttackRule {
    fn has_critical(&self) -> bool;
    fn roll_input_stats(&self, attack_stats: &AttackStats, attack_modifier: &Option<AttackStatsModifier>) -> (StatType, Option<i32>);
    fn roll_input_quantiy(&self, status: &CombatStatus);

    fn outcome_probabilities(
        &self,
        attack_stats: &AttackStats,
        attack_modifier: &Option<AttackStatsModifier>,
    ) -> HashMap<TestRollOutcome, f64> {
        let (threshold, modifier) = self.roll_input_stats(attack_stats, attack_modifier);
        TestRollOutcome::output_probabilities(threshold, modifier, self.has_critical())
    }

    fn status_attributes(
        &self,
        raw_counts: &Vec<u32>,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
    ) -> HashMap<CombatStatusAttribute, u32>;

    fn apply(
        &self,
        node: &mut CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
    ) {

        let n_elements = self.roll_input_quantity(&node.status);
        let probabilities: Vec<f64> = Vec::new();
        let status_attribtues: Vec<CombatStatusAttribute> = Vec::new();

        self.outcome_probabilities(attack_stats, attack_modifier).iter().for_each(
            |(attr, proba)|{
                probabilities.push(*proba);
                status_attribtues.push(*attr)
            }
        );

        let partition_probas = generate_partitions_probabilities(n_elements, &probabilities);
        
        let children = Vec::new();
        for (partition, probability) in partition_probas {
            let attribute_counts =
                self.status_attributes(&partition, attack_stats, attack_modifier);
            children.push(
                CombatNode::from_attributes_and_probability(&attribute_counts,
                probability * node.probability)
            )
        }
    }
}

impl<T: BasicAttackRule> AttackRule for T {
    fn apply(
        &self,
        node: &mut CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
    ) {
        <Self as BasicAttackRule>::apply(&self, node, attack_stats, attack_modifier)
    }
}

pub trait BasicDefenseRule {
    fn probabilities(
        &self,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
        defense_stats: &DefenseStats,
        defense_modifier: &DefenseStatsModifier,
    ) -> (u32, Vec<f64>);

    fn counts(
        &self,
        raw_counts: &Vec<u32>,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
        defense_stats: &DefenseStats,
        defense_modifier: &DefenseStatsModifier,
    ) -> (Vec<u32>, Vec<OutputStatus>);

    fn apply(
        &self,
        node: &mut CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
        defense_stats: &DefenseStats,
        defense_modifier: &DefenseStatsModifier,
    ) {
        let (n_elements, probabilities) = self.probabilities(
            attack_stats,
            attack_modifier,
            defense_stats,
            defense_modifier,
        );
        let partition_probas = generate_partitions_probabilities(n_elements, &probabilities);
        for (partition, probability) in partition_probas {
            let (stat_counts, output_status) = self.counts(
                &partition,
                attack_stats,
                attack_modifier,
                defense_stats,
                defense_modifier,
            );
            node.add_child(new_node(
                probability * node.probability,
                &stat_counts,
                &output_status,
            ))
        }
    }
}

impl<T: BasicDefenseRule> DefenseRule for T {
    fn apply(
        &self,
        node: &mut CombatNode,
        attack_stats: &AttackStats,
        attack_modifier: &AttackStatsModifier,
        defense_stats: &DefenseStats,
        defense_modifier: &DefenseStatsModifier,
    ) {
        <Self as BasicDefenseRule>::apply(
            self,
            node,
            attack_stats,
            attack_modifier,
            defense_stats,
            defense_modifier,
        );
    }
}