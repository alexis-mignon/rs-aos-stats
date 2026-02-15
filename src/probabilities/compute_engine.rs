use crate::probabilities::combat_stats::{AttackStats, DefenseStats, RollModifier};
use std::collections::HashMap;
use std::fmt;

/// Snapshot of the combat pipeline at a given step.
///
/// The rules in `rules_impl` transform this struct: starting from all
/// counters at zero, each rule consumes some fields (e.g. `attacks` or
/// `wounds`) and produces others (e.g. `hits`, `mortal_wounds`, `damages`).
/// The engine tracks a probability distribution over these statuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    pub fn new_with_values(
        attacks: u32,
        hits: u32,
        wounds: u32,
        mortal_wounds: u32,
        damages: u32,
    ) -> CombatStatus {
        CombatStatus {
            attacks,
            hits,
            wounds,
            mortal_wounds,
            damages,
        }
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

/// Static configuration for a combat profile (stats + modifiers).
#[derive(Clone, Copy, Debug)]
pub struct CombatConfig {
    pub attack_stats: AttackStats,
    pub defense_stats: DefenseStats,
    pub modifier: RollModifier,
}

impl CombatConfig {
    pub fn new(attack_stats: AttackStats, defense_stats: DefenseStats) -> CombatConfig {
        CombatConfig {
            attack_stats,
            defense_stats,
            modifier: RollModifier::new_null(),
        }
    }

    pub fn new_with_modifiers(
        attack_stats: AttackStats,
        defense_stats: DefenseStats,
        modifier: RollModifier,
    ) -> CombatConfig {
        CombatConfig {
            attack_stats,
            defense_stats,
            modifier,
        }
    }
}

/// A single step in the combat sequence.
///
/// Given an input `(CombatStatus, probability)` and the immutable
/// `CombatConfig`, a rule branches into zero or more successor states
/// with associated probabilities. The engine is responsible for
/// aggregating identical `CombatStatus` values across branches.
pub trait Rule: fmt::Debug {
    fn apply(
        &self,
        status: &CombatStatus,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(CombatStatus, f64)>;
}
pub type CombatDist = HashMap<CombatStatus, f64>;

/// Apply a single rule to a distribution of combat statuses, returning the
/// new distribution. States that end up with identical `CombatStatus`
/// are merged by summing their probabilities.
fn apply_rule(
    dist: &CombatDist,
    config: CombatConfig,
    rule: &dyn Rule,
) -> CombatDist {
    let mut new_dist: CombatDist = HashMap::new();

    for (status, p) in dist.iter() {
        let children = rule.apply(status, *p, &config);

        for (child_status, child_p) in children {
            *new_dist.entry(child_status).or_insert(0.0) += child_p;
        }
    }

    new_dist
}

/// Apply a full sequence of rules to an initial configuration and
/// return the resulting distribution over combat statuses.
///
/// Conceptually this performs a dynamic-programming traversal of all
/// possible outcomes without building an explicit tree: we iteratively
/// push the probability mass through each rule and coalesce identical
/// states at every step.
pub fn apply_rules(config: CombatConfig, sequence: &Vec<Box<dyn Rule>>) -> CombatDist {
    // Start with a single initial state
    let mut dist: CombatDist = HashMap::new();
    dist.insert(CombatStatus::new(), 1.0);

    // Apply each rule iteratively, aggregating states with identical status
    for rule in sequence {
        dist = apply_rule(&dist, config, rule.as_ref());
    }

    dist
}

/// Convert a distribution over combat statuses into a marginal damage
/// distribution by aggregating on the `damages` field only.
pub fn damages_from_distribution(dist: &CombatDist) -> Vec<(u32, f64)> {
    let mut damage_map: HashMap<u32, f64> = HashMap::new();
    for (status, p) in dist.iter() {
        *damage_map.entry(status.damages).or_insert(0.0) += p;
    }

    let mut damages_probas_vec: Vec<(u32, f64)> =
        damage_map.into_iter().map(|(d, p)| (d, p)).collect();
    damages_probas_vec.sort_by(|a, b| a.0.cmp(&b.0));
    damages_probas_vec
}

pub fn compute_damages(config: CombatConfig, sequence: &Vec<Box<dyn Rule>>) -> Vec<(u32, f64)> {
    let dist = apply_rules(config, sequence);
    damages_from_distribution(&dist)
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
        let attack_stats =
            AttackStats::new(Characteristic::Value(5), 3, 4, 1, Characteristic::Value(2));
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

}
