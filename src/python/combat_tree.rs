use pyo3::prelude::*;

use crate::probabilities::compute_engine::CombatConfig;
use crate::probabilities::rules_impl::{compute_damages, HitRuleVariant};

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
    hit_rule_type: &str,
) -> PyResult<Vec<(u32, f64)>> {
    let variant: HitRuleVariant = hit_rule_type
        .parse()
        .map_err(|e: String| pyo3::exceptions::PyValueError::new_err(e))?;
    Ok(compute_damages(config.into(), variant))
}
