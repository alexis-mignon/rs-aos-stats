use crate::probabilities::combat_stats::{AttackStats,DefenseStats, RollModifier};
use std::collections::HashMap;
use std::fmt;


#[derive(Clone, Copy, Debug)]
pub struct CombatStatus {
    pub attacks: u32,
    pub hits: u32,
    pub wounds: u32,
    pub mortal_wounds: u32,
    pub damages: u32,
}

impl CombatStatus {
    pub fn new() -> CombatStatus {
        CombatStatus {
            attacks: 0,
            hits: 0,
            wounds: 0,
            mortal_wounds: 0,
            damages: 0,
        }
    }

    pub fn new_with_values(attacks: u32, hits: u32, wounds: u32, mortal_wounds: u32, damages: u32) -> CombatStatus {
        CombatStatus{attacks, hits, wounds, mortal_wounds, damages}
    }

    pub fn with_attacks(&self, attacks: u32) -> CombatStatus {
        let mut new_status = *self;
        new_status.attacks = attacks;
        new_status
    }

    pub fn with_hits(&self, hits: u32) -> CombatStatus {
        let mut new_status = *self;
        new_status.hits = hits;
        new_status
    }

    pub fn with_wounds(&self, wounds: u32) -> CombatStatus {
        let mut new_status = *self;
        new_status.wounds = wounds;
        new_status
    }

    pub fn with_mortal_wounds(&self, mortal_wounds: u32) -> CombatStatus {
        let mut new_status = *self;
        new_status.mortal_wounds = mortal_wounds;
        new_status
    }

    pub fn with_damages(&self, damages: u32) -> CombatStatus {
        let mut new_status = *self;
        new_status.damages = damages;
        new_status
    }
}

impl Default for CombatStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CombatConfig {
    pub attack_stats: AttackStats,
    pub defense_stats: DefenseStats,
    pub modifier: RollModifier
}

impl CombatConfig {
    pub fn new(
        attack_stats: AttackStats,
        defense_stats: DefenseStats,
    ) -> CombatConfig {
        CombatConfig {
            attack_stats,
            defense_stats,
            modifier: RollModifier::new_null()
        }
    }

    pub fn new_with_modifiers(
        attack_stats: AttackStats,
        defense_stats: DefenseStats,
        modifier: RollModifier
    ) -> CombatConfig {
        CombatConfig {attack_stats, defense_stats, modifier}
    }
}

#[derive(Clone, Debug)]
pub struct CombatNode {
    pub status: CombatStatus,
    pub config: CombatConfig,
    pub probability: f64,
    pub children: Vec<CombatNode>,
}

impl CombatNode {
    pub fn new(status: CombatStatus, config: CombatConfig, probability: f64) -> CombatNode {
        CombatNode {
            status,
            config,
            probability,
            children: Vec::new(),
        }
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

    pub fn leaves_mut(&mut self) -> Vec<&mut CombatNode> {
        if self.children.is_empty() {
            return vec![self];
        }

        let mut leaves = Vec::new();
        for child in self.children.iter_mut() {
            leaves.extend(child.leaves_mut());
        }
        leaves
    }

    pub fn apply_rule(&mut self, rule: &dyn Rule) {
        let children = rule.apply(self);
        for child in children {
            self.add_child(child)
        }
    }
}

pub trait Rule: fmt::Debug {
    fn apply(&self, node: &CombatNode) -> Vec<CombatNode>;
}


pub struct CombatTree {
    root: CombatNode
}

impl CombatTree {
    pub fn new(config: CombatConfig) -> CombatTree {
        CombatTree {
            root: CombatNode::new(
                CombatStatus::new(),
                config,
                1.0
            )
        }
    }

    pub fn build(&mut self, sequence: &Vec<Box<dyn Rule>>) {
        for rule in sequence {
            let leaves = self.root.leaves_mut();
            for leaf in leaves {
                leaf.apply_rule(rule.as_ref());
            }
        }
    }

    pub fn retrieve_damages_probas(&self) -> Vec<(u32, f64)> {
        let damages_probas: Vec<(u32, f64)> = self.root.leaves().iter().map(
            |node| (node.status.damages, node.probability)
        ).collect();


        let mut damages_proba_grouped = HashMap::new();
        for (value, proba) in damages_probas {
            let entry = damages_proba_grouped.entry(value).or_insert(0.0);
            *entry += proba;
        }

        let mut damages_probas_vec: Vec<(u32, f64)> = damages_proba_grouped
            .iter().map(|(value, proba)| (*value, *proba))
            .collect();
        damages_probas_vec.sort_by(|a, b| a.0.cmp(&b.0));
        damages_probas_vec
    }
}


pub fn compute_damages(config: CombatConfig, sequence: &Vec<Box<dyn Rule>>) -> Vec<(u32, f64)> {
    let mut tree = CombatTree::new(config);
    tree.build(sequence);
    tree.retrieve_damages_probas()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilities::combat_stats::Characteristic;

    /// Test CombatStatus::new creates a zero-initialized status.
    #[test]
    fn combat_status_new() {
        let status = CombatStatus::new();
        assert_eq!(status.attacks, 0);
        assert_eq!(status.hits, 0);
        assert_eq!(status.wounds, 0);
        assert_eq!(status.mortal_wounds, 0);
        assert_eq!(status.damages, 0);
    }

    /// Test CombatStatus::new_with_values sets all fields correctly.
    #[test]
    fn combat_status_new_with_values() {
        let status = CombatStatus::new_with_values(5, 3, 2, 1, 4);
        assert_eq!(status.attacks, 5);
        assert_eq!(status.hits, 3);
        assert_eq!(status.wounds, 2);
        assert_eq!(status.mortal_wounds, 1);
        assert_eq!(status.damages, 4);
    }

    /// Test all CombatStatus builder methods (with_*).
    #[test]
    fn combat_status_builder_methods() {
        let status = CombatStatus::new();

        // Test with_attacks
        let modified = status.with_attacks(10);
        assert_eq!(modified.attacks, 10);
        assert_eq!(modified.hits, 0);

        // Test with_hits
        let modified = status.with_hits(5);
        assert_eq!(modified.hits, 5);
        assert_eq!(modified.attacks, 0);

        // Test with_wounds
        let modified = status.with_wounds(3);
        assert_eq!(modified.wounds, 3);

        // Test with_mortal_wounds
        let modified = status.with_mortal_wounds(2);
        assert_eq!(modified.mortal_wounds, 2);

        // Test with_damages
        let modified = status.with_damages(8);
        assert_eq!(modified.damages, 8);
    }

    /// Test CombatConfig constructors.
    #[test]
    fn combat_config_constructors() {
        let attack_stats = AttackStats::new(
            Characteristic::Value(5),
            3, 4, 1,
            Characteristic::Value(2)
        );
        let defense_stats = DefenseStats::new(5, None);

        // Test new (without modifiers)
        let config = CombatConfig::new(attack_stats, defense_stats);
        assert_eq!(config.modifier.to_hit, 0);
        assert_eq!(config.modifier.to_wound, 0);
        assert_eq!(config.modifier.to_save, 0);

        // Test new_with_modifiers
        let modifier = RollModifier::new(1, -1, 0);
        let config = CombatConfig::new_with_modifiers(attack_stats, defense_stats, modifier);
        assert_eq!(config.modifier.to_hit, 1);
        assert_eq!(config.modifier.to_wound, -1);
    }

    /// Test CombatNode creation and child management.
    #[test]
    fn combat_node_children() {
        let attack_stats = AttackStats::new(
            Characteristic::Value(5),
            3, 4, 1,
            Characteristic::Value(2)
        );
        let defense_stats = DefenseStats::new(5, None);
        let config = CombatConfig::new(attack_stats, defense_stats);

        let mut node = CombatNode::new(CombatStatus::new(), config, 1.0);
        assert_eq!(node.children.len(), 0);

        // Add a child
        let child = CombatNode::new(CombatStatus::new(), config, 0.5);
        node.add_child(child);
        assert_eq!(node.children.len(), 1);
    }

    /// Test leaves() returns the node itself when there are no children.
    #[test]
    fn combat_node_leaves_no_children() {
        let attack_stats = AttackStats::new(
            Characteristic::Value(5),
            3, 4, 1,
            Characteristic::Value(2)
        );
        let defense_stats = DefenseStats::new(5, None);
        let config = CombatConfig::new(attack_stats, defense_stats);

        let node = CombatNode::new(CombatStatus::new(), config, 1.0);
        let leaves = node.leaves();
        assert_eq!(leaves.len(), 1);
    }
}
