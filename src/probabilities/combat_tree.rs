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
        let mut new_status = self.clone();
        new_status.attacks = attacks;
        new_status
    }

    pub fn with_hits(&self, hits: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.hits = hits;
        new_status
    }

    pub fn with_wounds(&self, wounds: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.wounds = wounds;
        new_status
    }

    pub fn with_mortal_wounds(&self, mortal_wounds: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.mortal_wounds = mortal_wounds;
        new_status
    }

    pub fn with_damages(&self, damages: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.damages = damages;
        new_status
    }

/*     pub fn with_attribute(&self, attribute: CombatStatusAttribute, value: u32) -> CombatStatus {
        match attribute {
            CombatStatusAttribute::Attacks => self.with_attacks(value),
            CombatStatusAttribute::Hits => self.with_hits(value),
            CombatStatusAttribute::Wounds => self.with_wounds(value),
            CombatStatusAttribute::MortalWounds => self.with_mortal_wounds(value),
            CombatStatusAttribute::Damages => self.with_damages(value)
        }
    } */
}

#[derive(Clone, Copy, Debug)]
pub enum CombatStatusAttribute {
    Attacks,
    Hits,
    Wounds,
    MortalWounds,
    Damages
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
            attack_stats: attack_stats,
            defense_stats: defense_stats,
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

    pub fn build(&mut self, sequence: &Vec<Box<dyn Rule>>) -> () {
        for rule in sequence {
            let leaves = self.root.leaves_mut();
            for leaf in leaves {
                //println!("{:?}: {:?}", rule, leaf.status);
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
