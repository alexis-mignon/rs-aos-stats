use crate::probabilities::combat_stats::Characteristic;
use crate::probabilities::compute_engine::{CombatConfig, Rule};
use crate::probabilities::dice::DiceRoll;
use crate::probabilities::partitions::generate_partitions_probabilities;
use crate::probabilities::states::*;
use std::hash::Hash;

// ---------------------------------------------------------------------------
// Generic helper traits
// ---------------------------------------------------------------------------

/// Shared helper for rules modelled as "roll N identical dice and classify
/// each roll into a small number of outcome buckets".
pub(crate) trait TestRollRule<In, Out>: Rule<In, Out>
where
    In: Clone + Eq + Hash,
    Out: Clone + Eq + Hash,
{
    fn roll_count(&self, state: &In) -> u32;
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64>;
    fn build_state(&self, state: &In, counts: &[u32]) -> Out;

    fn apply_distribution(
        &self,
        state: &In,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(Out, f64)> {
        let probas = self.partition_prior(config);
        let nrolls = self.roll_count(state);
        let partitions = generate_partitions_probabilities(nrolls, &probas);
        let mut results = vec![];
        for (counts, proba) in partitions {
            let new_state = self.build_state(state, &counts);
            results.push((new_state, probability * proba));
        }
        results
    }
}

/// Base implementation for hit-like rules with three outcome buckets:
/// critical hit, normal hit, and failure.
///
/// Concrete rules only need to decide how the partition counts map to
/// `(hits, wounds, mortal_wounds)` via `result`.
pub(crate) trait BaseHitRule: TestRollRule<Initial, Hit> {
    fn hit_roll_count(&self, _state: &Initial) -> u32 {
        1
    }

    fn hit_partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let critical_proba = 1.0 / 6.0;
        let success_proba: f64 = (1..=6)
            .map(|roll| match roll {
                6 => 0.0,
                1 => 0.0,
                _ => {
                    (config.modifier.apply_to_hit_modifier(roll) >= config.attack_stats.to_hit)
                        as u32 as f64
                        / 6.0
                }
            })
            .sum();

        vec![
            critical_proba,
            success_proba,
            1.0 - success_proba - critical_proba,
        ]
    }

    fn result(&self, partition: &[u32]) -> (u32, u32, u32);

    fn hit_build_state(&self, _state: &Initial, counts: &[u32]) -> Hit {
        let (hits, wounds, mortal_wounds) = self.result(counts);
        Hit {
            hits,
            wounds,
            mortal_wounds,
        }
    }
}

// ---------------------------------------------------------------------------
// Hit rules: Initial → Hit
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct HitRule;

impl BaseHitRule for HitRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        (partition[0] + partition[1], 0, 0)
    }
}

impl TestRollRule<Initial, Hit> for HitRule {
    fn roll_count(&self, state: &Initial) -> u32 {
        BaseHitRule::hit_roll_count(self, state)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::hit_partition_prior(self, config)
    }
    fn build_state(&self, state: &Initial, counts: &[u32]) -> Hit {
        BaseHitRule::hit_build_state(self, state, counts)
    }
}

impl Rule<Initial, Hit> for HitRule {
    fn apply(&self, state: &Initial, probability: f64, config: &CombatConfig) -> Vec<(Hit, f64)> {
        TestRollRule::apply_distribution(self, state, probability, config)
    }
}

// ---------------------------------------------------------------------------
// CritMortalWoundRule: Initial → Hit
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct CritMortalWoundRule;

impl BaseHitRule for CritMortalWoundRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        (partition[1], 0, partition[0])
    }
}

impl TestRollRule<Initial, Hit> for CritMortalWoundRule {
    fn roll_count(&self, state: &Initial) -> u32 {
        BaseHitRule::hit_roll_count(self, state)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::hit_partition_prior(self, config)
    }
    fn build_state(&self, state: &Initial, counts: &[u32]) -> Hit {
        BaseHitRule::hit_build_state(self, state, counts)
    }
}

impl Rule<Initial, Hit> for CritMortalWoundRule {
    fn apply(&self, state: &Initial, probability: f64, config: &CombatConfig) -> Vec<(Hit, f64)> {
        TestRollRule::apply_distribution(self, state, probability, config)
    }
}

// ---------------------------------------------------------------------------
// CritAutoWoundRule: Initial → Hit
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct CritAutoWoundRule;

impl BaseHitRule for CritAutoWoundRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        (partition[1], partition[0], 0)
    }
}

impl TestRollRule<Initial, Hit> for CritAutoWoundRule {
    fn roll_count(&self, state: &Initial) -> u32 {
        BaseHitRule::hit_roll_count(self, state)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::hit_partition_prior(self, config)
    }
    fn build_state(&self, state: &Initial, counts: &[u32]) -> Hit {
        BaseHitRule::hit_build_state(self, state, counts)
    }
}

impl Rule<Initial, Hit> for CritAutoWoundRule {
    fn apply(&self, state: &Initial, probability: f64, config: &CombatConfig) -> Vec<(Hit, f64)> {
        TestRollRule::apply_distribution(self, state, probability, config)
    }
}

// ---------------------------------------------------------------------------
// CritDoubleHitRule: Initial → Hit
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct CritDoubleHitRule;

impl BaseHitRule for CritDoubleHitRule {
    fn result(&self, partition: &[u32]) -> (u32, u32, u32) {
        (2 * partition[0] + partition[1], 0, 0)
    }
}

impl TestRollRule<Initial, Hit> for CritDoubleHitRule {
    fn roll_count(&self, state: &Initial) -> u32 {
        BaseHitRule::hit_roll_count(self, state)
    }
    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        BaseHitRule::hit_partition_prior(self, config)
    }
    fn build_state(&self, state: &Initial, counts: &[u32]) -> Hit {
        BaseHitRule::hit_build_state(self, state, counts)
    }
}

impl Rule<Initial, Hit> for CritDoubleHitRule {
    fn apply(&self, state: &Initial, probability: f64, config: &CombatConfig) -> Vec<(Hit, f64)> {
        TestRollRule::apply_distribution(self, state, probability, config)
    }
}

// ---------------------------------------------------------------------------
// WoundRule: Hit → Wounded
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct WoundRule;

impl TestRollRule<Hit, Wounded> for WoundRule {
    fn roll_count(&self, state: &Hit) -> u32 {
        state.hits
    }

    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let success_proba: f64 = (1..=6)
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

    fn build_state(&self, state: &Hit, counts: &[u32]) -> Wounded {
        Wounded {
            wounds: counts[0] + state.wounds,
            mortal_wounds: state.mortal_wounds,
        }
    }
}

impl Rule<Hit, Wounded> for WoundRule {
    fn apply(&self, state: &Hit, probability: f64, config: &CombatConfig) -> Vec<(Wounded, f64)> {
        TestRollRule::apply_distribution(self, state, probability, config)
    }
}

// ---------------------------------------------------------------------------
// SaveRule: Wounded → Saved
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SaveRule;

impl TestRollRule<Wounded, Saved> for SaveRule {
    fn roll_count(&self, state: &Wounded) -> u32 {
        state.wounds
    }

    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let success_proba: f64 = (1..=6)
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

    fn build_state(&self, state: &Wounded, counts: &[u32]) -> Saved {
        Saved {
            unsaved_wounds: state.wounds - counts[0],
            mortal_wounds: state.mortal_wounds,
        }
    }
}

impl Rule<Wounded, Saved> for SaveRule {
    fn apply(&self, state: &Wounded, probability: f64, config: &CombatConfig) -> Vec<(Saved, f64)> {
        TestRollRule::apply_distribution(self, state, probability, config)
    }
}

// ---------------------------------------------------------------------------
// DamagesRule: Saved → Damaged
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DamagesRule;

impl DamagesRule {
    fn random_damages(roll: DiceRoll, num_wounds: u32) -> Vec<(u32, f64)> {
        crate::probabilities::convolution::convolve_n(&roll.values_and_probas(), num_wounds)
    }
}

impl Rule<Saved, Damaged> for DamagesRule {
    fn apply(&self, state: &Saved, probability: f64, config: &CombatConfig) -> Vec<(Damaged, f64)> {
        let num_wounds = state.unsaved_wounds + state.mortal_wounds;
        let damages_and_probas = match config.attack_stats.damages {
            Characteristic::Value(value) => vec![(value * num_wounds, 1.0)],
            Characteristic::DiceRoll(roll) => DamagesRule::random_damages(roll, num_wounds),
        };
        damages_and_probas
            .iter()
            .map(|(damages, proba)| (Damaged { damages: *damages }, probability * proba))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// WardRule: Damaged → Damaged
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct WardRule;

impl TestRollRule<Damaged, Damaged> for WardRule {
    fn roll_count(&self, state: &Damaged) -> u32 {
        state.damages
    }

    fn partition_prior(&self, config: &CombatConfig) -> Vec<f64> {
        let ward_value = config
            .defense_stats
            .ward
            .expect("WardRule requires ward save to be set");
        let success_proba: f64 = (1..=6)
            .map(|roll| match roll {
                1 => 0.0,
                _ => (roll >= ward_value) as u32 as f64 / 6.0,
            })
            .sum();

        vec![success_proba, 1.0 - success_proba]
    }

    fn build_state(&self, state: &Damaged, counts: &[u32]) -> Damaged {
        Damaged {
            damages: state.damages - counts[0],
        }
    }
}

impl Rule<Damaged, Damaged> for WardRule {
    fn apply(
        &self,
        state: &Damaged,
        probability: f64,
        config: &CombatConfig,
    ) -> Vec<(Damaged, f64)> {
        if config.defense_stats.ward.is_some() {
            TestRollRule::apply_distribution(self, state, probability, config)
        } else {
            vec![(*state, probability)]
        }
    }
}

// ---------------------------------------------------------------------------
// HitRuleVariant enum for convenience pipeline construction
// ---------------------------------------------------------------------------

/// Selects which hit rule variant to use in the standard pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitRuleVariant {
    Normal,
    CritAutoWound,
    CritMortalWound,
    CritDoubleHit,
}

impl std::str::FromStr for HitRuleVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(HitRuleVariant::Normal),
            "crit_auto_wound" => Ok(HitRuleVariant::CritAutoWound),
            "crit_mortal_wound" => Ok(HitRuleVariant::CritMortalWound),
            "crit_double_hit" => Ok(HitRuleVariant::CritDoubleHit),
            _ => Err(format!(
                "Invalid hit_rule_type '{}'. Must be one of: 'normal', 'crit_auto_wound', 'crit_mortal_wound', 'crit_double_hit'",
                s
            )),
        }
    }
}

/// Compute the exact damage probability distribution for a combat profile.
///
/// This is the main entry point. It runs the pipeline for a single attack,
/// then convolves the result N times for N attacks. For random attack counts
/// (dice), it marginalizes over possible values.
pub fn compute_damages(config: CombatConfig, hit_variant: HitRuleVariant) -> Vec<(u32, f64)> {
    use crate::probabilities::convolution::{convolve_n, mix};

    let single = compute_single_attack(&config, hit_variant);
    let attack_values = config.attack_stats.attacks.values_and_probas();

    let convolved: Vec<Vec<(u32, f64)>> = attack_values
        .iter()
        .map(|(n, _)| convolve_n(&single, *n))
        .collect();
    let components: Vec<(&[(u32, f64)], f64)> = convolved
        .iter()
        .zip(attack_values.iter())
        .map(|(dist, (_, p))| (dist.as_slice(), *p))
        .collect();
    mix(&components)
}

/// Run the pipeline for a single attack, returning the damage distribution.
fn compute_single_attack(config: &CombatConfig, hit_variant: HitRuleVariant) -> Vec<(u32, f64)> {
    use crate::probabilities::compute_engine::PipelineBuilder;

    let has_ward = config.defense_stats.ward.is_some();

    macro_rules! finish_pipeline {
        ($after_hits:expr, $config:expr, $has_ward:expr) => {{
            let after_damage = $after_hits
                .add_rule(WoundRule)
                .add_rule(SaveRule)
                .add_rule(DamagesRule);
            if $has_ward {
                after_damage.add_rule(WardRule).compute_damages(&$config)
            } else {
                after_damage.compute_damages(&$config)
            }
        }};
    }

    match hit_variant {
        HitRuleVariant::Normal => {
            finish_pipeline!(PipelineBuilder::new().add_rule(HitRule), config, has_ward)
        }
        HitRuleVariant::CritAutoWound => {
            finish_pipeline!(
                PipelineBuilder::new().add_rule(CritAutoWoundRule),
                config,
                has_ward
            )
        }
        HitRuleVariant::CritMortalWound => {
            finish_pipeline!(
                PipelineBuilder::new().add_rule(CritMortalWoundRule),
                config,
                has_ward
            )
        }
        HitRuleVariant::CritDoubleHit => {
            finish_pipeline!(
                PipelineBuilder::new().add_rule(CritDoubleHitRule),
                config,
                has_ward
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilities::combat_stats::{AttackStats, DefenseStats};

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

    fn proba_sum(results: &[(u32, f64)]) -> f64 {
        results.iter().map(|(_, p)| p).sum()
    }

    fn mean_damage(config: CombatConfig, variant: HitRuleVariant) -> f64 {
        compute_damages(config, variant)
            .iter()
            .map(|(d, p)| *d as f64 * p)
            .sum()
    }

    #[test]
    fn simple_fixed_attack() {
        let config = make_config(1, 2, 2, 0, 1, 7, None);
        let results = compute_damages(config, HitRuleVariant::Normal);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let p1 = results
            .iter()
            .find(|(d, _)| *d == 1)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        assert!((p1 - 25.0 / 36.0).abs() < 1e-10);
    }

    #[test]
    fn all_miss() {
        let config = make_config(1, 7, 7, 0, 1, 7, None);
        let results = compute_damages(config, HitRuleVariant::Normal);
        let p0: f64 = results
            .iter()
            .filter(|(d, _)| *d == 0)
            .map(|(_, p)| *p)
            .sum();
        assert!((p0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn crit_double_hit_max_damage() {
        let config = make_config(3, 4, 4, 0, 2, 7, None);
        let results = compute_damages(config, HitRuleVariant::CritDoubleHit);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert_eq!(max_damage, 12);
    }

    #[test]
    fn crit_auto_wound() {
        let config = make_config(1, 4, 7, 0, 1, 7, None);
        let results = compute_damages(config, HitRuleVariant::CritAutoWound);
        let p1 = results
            .iter()
            .find(|(d, _)| *d == 1)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        assert!((p1 - 1.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn crit_mortal_wound() {
        let config = make_config(1, 4, 4, 0, 1, 7, None);
        let results = compute_damages(config, HitRuleVariant::CritMortalWound);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert_eq!(max_damage, 1);
    }

    #[test]
    fn ward_save_reduces_damage() {
        let config = make_config(1, 2, 2, 0, 1, 7, Some(4));
        let results = compute_damages(config, HitRuleVariant::Normal);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let p1 = results
            .iter()
            .find(|(d, _)| *d == 1)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);
        assert!((p1 - 25.0 / 72.0).abs() < 1e-10);
    }

    #[test]
    fn probabilities_always_sum_to_one() {
        let config = make_config(3, 3, 4, 1, 2, 4, Some(6));
        let results = compute_damages(config, HitRuleVariant::Normal);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
    }

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
        let results = compute_damages(config, HitRuleVariant::Normal);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let max_damage = results.iter().map(|(d, _)| *d).max().unwrap();
        assert!(max_damage <= 3);
    }

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
        let results = compute_damages(config, HitRuleVariant::Normal);
        let total = proba_sum(&results);
        assert!((total - 1.0).abs() < 1e-10);
        let has_positive_damage = results.iter().any(|(d, p)| *d > 0 && *p > 0.0);
        assert!(has_positive_damage);
    }

    #[test]
    fn ward_rule_no_ward() {
        // WardRule with no ward configured should be a no-op
        let config = make_config(1, 2, 2, 0, 1, 7, None);
        let state = Damaged { damages: 1 };
        let result = <WardRule as Rule<Damaged, Damaged>>::apply(&WardRule, &state, 1.0, &config);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, state);
        assert!((result[0].1 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn crit_auto_wound_increases_mean_damage() {
        let config = make_config(10, 3, 3, 1, 1, 4, None);
        let normal = mean_damage(config, HitRuleVariant::Normal);
        let crit = mean_damage(config, HitRuleVariant::CritAutoWound);
        assert!(
            crit > normal,
            "CritAutoWound ({crit:.4}) should exceed normal ({normal:.4})"
        );
    }

    #[test]
    fn crit_mortal_wound_increases_mean_damage() {
        let config = make_config(10, 3, 3, 1, 1, 4, None);
        let normal = mean_damage(config, HitRuleVariant::Normal);
        let crit = mean_damage(config, HitRuleVariant::CritMortalWound);
        assert!(
            crit > normal,
            "CritMortalWound ({crit:.4}) should exceed normal ({normal:.4})"
        );
    }

    #[test]
    fn crit_double_hit_increases_mean_damage() {
        let config = make_config(10, 3, 3, 1, 1, 4, None);
        let normal = mean_damage(config, HitRuleVariant::Normal);
        let crit = mean_damage(config, HitRuleVariant::CritDoubleHit);
        assert!(
            crit > normal,
            "CritDoubleHit ({crit:.4}) should exceed normal ({normal:.4})"
        );
    }

    #[test]
    fn crit_rules_increase_damage_at_six_plus_to_hit() {
        let config = make_config(10, 6, 3, 0, 1, 4, None);
        let normal = mean_damage(config, HitRuleVariant::Normal);
        for (name, variant) in [
            ("CritAutoWound", HitRuleVariant::CritAutoWound),
            ("CritMortalWound", HitRuleVariant::CritMortalWound),
            ("CritDoubleHit", HitRuleVariant::CritDoubleHit),
        ] {
            let crit = mean_damage(config, variant);
            assert!(
                crit > normal,
                "{name} ({crit:.4}) should exceed normal ({normal:.4}) at 6+ to hit"
            );
        }
    }

    #[test]
    fn ward_reduces_mean_damage() {
        let profiles = [
            (
                make_config(10, 3, 3, 1, 1, 4, None),
                make_config(10, 3, 3, 1, 1, 4, Some(4)),
            ),
            (
                make_config(5, 2, 2, 0, 2, 5, None),
                make_config(5, 2, 2, 0, 2, 5, Some(5)),
            ),
            (
                make_config(3, 4, 4, 2, 3, 3, None),
                make_config(3, 4, 4, 2, 3, 3, Some(6)),
            ),
        ];
        for (config_no_ward, config_ward) in profiles {
            let without = mean_damage(config_no_ward, HitRuleVariant::Normal);
            let with = mean_damage(config_ward, HitRuleVariant::Normal);
            assert!(
                with < without,
                "Ward should reduce damage: {with:.4} >= {without:.4} for config {:?}",
                config_ward
            );
        }
    }

    #[test]
    fn stronger_ward_reduces_more() {
        let config_4plus = make_config(10, 3, 3, 1, 2, 4, Some(4));
        let config_5plus = make_config(10, 3, 3, 1, 2, 4, Some(5));

        let mean_4plus = mean_damage(config_4plus, HitRuleVariant::Normal);
        let mean_5plus = mean_damage(config_5plus, HitRuleVariant::Normal);
        assert!(
            mean_4plus < mean_5plus,
            "4+ ward ({mean_4plus:.4}) should reduce more than 5+ ({mean_5plus:.4})"
        );
    }
}
