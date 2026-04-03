use crate::probabilities::combat_stats::{AttackStats, DefenseStats, RollModifier};
use crate::probabilities::states::{Damaged, Initial};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

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

/// A typed transition rule in the combat pipeline.
///
/// Given an input state `In` with its probability and the immutable
/// `CombatConfig`, the rule produces zero or more successor states of
/// type `Out` with associated probabilities.
pub trait Rule<In, Out>: fmt::Debug
where
    In: Clone + Eq + Hash,
    Out: Clone + Eq + Hash,
{
    fn apply(&self, state: &In, probability: f64, config: &CombatConfig) -> Vec<(Out, f64)>;
}

/// A probability distribution over states of type `S`.
pub type StateDist<S> = HashMap<S, f64>;

/// Apply a single typed rule to a distribution, producing a new distribution.
fn apply_transition<In, Out, R>(
    dist: &StateDist<In>,
    config: &CombatConfig,
    rule: &R,
) -> StateDist<Out>
where
    In: Clone + Eq + Hash,
    Out: Clone + Eq + Hash,
    R: Rule<In, Out>,
{
    let mut new_dist: StateDist<Out> = HashMap::new();
    for (state, p) in dist.iter() {
        let children = rule.apply(state, *p, config);
        for (child_state, child_p) in children {
            *new_dist.entry(child_state).or_insert(0.0) += child_p;
        }
    }
    new_dist
}

type PipelineFn<S> = Box<dyn Fn(&CombatConfig) -> StateDist<S>>;

/// A type-safe pipeline builder that chains rules with compile-time
/// enforcement of correct state transitions.
///
/// The type parameter `Current` tracks the current state type in the
/// pipeline. Rules can only be added if their input type matches
/// `Current`, and the resulting builder has `Next` as its state type.
pub struct PipelineBuilder<Current: Clone + Eq + Hash + 'static> {
    run: PipelineFn<Current>,
}

impl PipelineBuilder<Initial> {
    /// Create a new pipeline starting from the `Initial` state.
    pub fn new() -> Self {
        PipelineBuilder {
            run: Box::new(|_config| {
                let mut dist = HashMap::new();
                dist.insert(Initial, 1.0);
                dist
            }),
        }
    }
}

impl Default for PipelineBuilder<Initial> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Current: Clone + Eq + Hash + 'static> PipelineBuilder<Current> {
    /// Add a rule to the pipeline, transitioning from `Current` to `Next`.
    ///
    /// Misordering rules is a **compile error**: e.g. adding a
    /// `Rule<Hit, Wounded>` to a `PipelineBuilder<Initial>` won't type-check.
    pub fn add_rule<Next, R>(self, rule: R) -> PipelineBuilder<Next>
    where
        Next: Clone + Eq + Hash + 'static,
        R: Rule<Current, Next> + 'static,
    {
        let prev_run = self.run;
        PipelineBuilder {
            run: Box::new(move |config| {
                let dist = prev_run(config);
                apply_transition(&dist, config, &rule)
            }),
        }
    }

    /// Execute the pipeline and return the raw state distribution.
    pub fn run(&self, config: &CombatConfig) -> StateDist<Current> {
        (self.run)(config)
    }
}

impl PipelineBuilder<Damaged> {
    /// Execute the pipeline and extract the marginal damage distribution.
    ///
    /// This method is only available when the pipeline ends at the
    /// `Damaged` state, ensuring all combat steps have been applied.
    pub fn compute_damages(&self, config: &CombatConfig) -> Vec<(u32, f64)> {
        let dist = self.run(config);
        damages_from_dist(&dist)
    }
}

/// Marginalize a `Damaged` distribution into `(damage_value, probability)` pairs.
fn damages_from_dist(dist: &StateDist<Damaged>) -> Vec<(u32, f64)> {
    let mut damage_map: HashMap<u32, f64> = HashMap::new();
    for (state, p) in dist.iter() {
        *damage_map.entry(state.damages).or_insert(0.0) += p;
    }
    let mut result: Vec<(u32, f64)> = damage_map.into_iter().collect();
    result.sort_by_key(|(d, _)| *d);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilities::combat_stats::Characteristic;

    #[test]
    fn combat_config_constructors() {
        let attack_stats =
            AttackStats::new(Characteristic::Value(5), 3, 4, 1, Characteristic::Value(2));
        let defense_stats = DefenseStats::new(5, None);

        let config = CombatConfig::new(attack_stats, defense_stats);
        assert_eq!(config.modifier.to_hit, 0);
        assert_eq!(config.modifier.to_wound, 0);
        assert_eq!(config.modifier.to_save, 0);

        let modifier = RollModifier::new(1, -1, 0);
        let config = CombatConfig::new_with_modifiers(attack_stats, defense_stats, modifier);
        assert_eq!(config.modifier.to_hit, 1);
        assert_eq!(config.modifier.to_wound, -1);
    }

    #[test]
    fn pipeline_builder_seeds_initial() {
        let config = CombatConfig::new(
            AttackStats::new(Characteristic::Value(1), 3, 3, 0, Characteristic::Value(1)),
            DefenseStats::new(7, None),
        );
        let builder = PipelineBuilder::new();
        let dist = builder.run(&config);
        assert_eq!(dist.len(), 1);
        assert!((dist[&Initial] - 1.0).abs() < 1e-12);
    }
}
