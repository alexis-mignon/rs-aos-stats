use pyo3::prelude::*;

/// Python-facing bindings for the dynamic-programming combat engine.
///
/// This exposes `CombatConfig` and `compute_damages` to Python code,
/// which is used by the API tests and higher-level helpers.
use crate::probabilities::compute_engine::{compute_damages, CombatConfig, Rule};

use super::rules::extract_rule;

use super::combat_stats::{AttackStatsPy, DefenseStatsPy, RollModifierPy};

#[pyclass(name = "CombatConfig")]
#[derive(Clone, Debug)]
pub struct CombatConfigPy {
    pub config: CombatConfig,
}

#[pymethods]
impl CombatConfigPy {
    #[new]
    #[pyo3(signature = (attack_stats, defense_stats, roll_modifier=None))]
    fn new(
        attack_stats: AttackStatsPy,
        defense_stats: DefenseStatsPy,
        roll_modifier: Option<RollModifierPy>,
    ) -> Self {
        if let Some(modifier) = roll_modifier {
            CombatConfigPy {
                config: CombatConfig::new_with_modifiers(
                    attack_stats.attack_stats,
                    defense_stats.defense_stats,
                    modifier.roll_modifier,
                ),
            }
        } else {
            CombatConfigPy {
                config: CombatConfig::new(attack_stats.attack_stats, defense_stats.defense_stats),
            }
        }
    }
}

impl From<CombatConfigPy> for CombatConfig {
    fn from(val: CombatConfigPy) -> Self {
        val.config
    }
}

#[pyfunction(name = "compute_damages")]
pub fn compute_damages_py(
    config: CombatConfigPy,
    sequence: Vec<Bound<'_, PyAny>>,
) -> PyResult<Vec<(u32, f64)>> {
    let rule_sequence: Vec<Box<dyn Rule>> = sequence
        .iter()
        .map(|rule| extract_rule(rule))
        .collect::<PyResult<Vec<_>>>()?;
    Ok(compute_damages(config.into(), &rule_sequence))
}
