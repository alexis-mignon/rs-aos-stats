use pyo3::prelude::*;


use crate::probabilities::combat_tree::{
    CombatConfig, compute_damages, Rule
};

use super::combat_stats::{
    AttackStatsPy, DefenseStatsPy, RollModifierPy
};

#[pyclass(name="CombatConfig")]
#[derive(Clone, Debug)]
pub struct CombatConfigPy {
    pub config: CombatConfig
}

#[pymethods]
impl CombatConfigPy {
    #[new]
    fn new(
        attack_stats: AttackStatsPy,
        defense_stats: DefenseStatsPy,
        roll_modifier: Option<RollModifierPy>
    ) -> Self {

        if let Some(modifier) = roll_modifier {
            CombatConfigPy {
                config: CombatConfig::new_with_modifiers(attack_stats.attack_stats, defense_stats.defense_stats, modifier.roll_modifier)
            }
        }
        else {
            CombatConfigPy {
                config: CombatConfig::new(attack_stats.attack_stats, defense_stats.defense_stats)
            }
        }
    }
}

impl Into<CombatConfig> for CombatConfigPy {
    fn into(self) -> CombatConfig {
        self.config
    }
}


#[pyfunction(name="compute_damages")]
pub fn compute_damages_py(config: CombatConfigPy, sequence: Vec<&PyAny>) -> Vec<(u32, f64)> {
    let rule_sequence: Vec<Box<dyn Rule>> = sequence.iter().map(|rule| Into::<Box<dyn Rule>>::into(*rule)).collect();
    compute_damages(config.into(), &rule_sequence)
}


