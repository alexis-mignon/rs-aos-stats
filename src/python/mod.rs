mod dice;
mod combat_stats;
mod combat_tree;
mod rules;

use pyo3::prelude::*;
use crate::python::dice::{DiceRollPy, D3, D6, ND3, ND6, ND3Plus, ND6Plus};
use crate::python::combat_stats::{CharacteristicPy, AttackStatsPy, DefenseStatsPy, RollModifierPy};
use crate::python::combat_tree::{CombatConfigPy, compute_damages_py};
use crate::python::rules::{HitRulePy, WoundRulePy, SaveRulePy, DamagesRulePy, AttackCharacteristicRulePy};


#[pymodule]
fn rs_aos_stats(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DiceRollPy>()?;  // Now it's called DiceRoll in Python
    m.add_class::<D6>()?;
    m.add_class::<D3>()?;
    m.add_class::<ND6>()?;
    m.add_class::<ND3>()?;
    m.add_class::<ND6Plus>()?;
    m.add_class::<ND3Plus>()?;
    // Add combat stats objects
    m.add_class::<CharacteristicPy>()?;
    m.add_class::<AttackStatsPy>()?;
    m.add_class::<DefenseStatsPy>()?;
    m.add_class::<RollModifierPy>()?;
    // Add combat trees functions
    m.add_class::<CombatConfigPy>()?;
    m.add_function(wrap_pyfunction!(compute_damages_py, m)?)?;
    // Rules
    m.add_class::<HitRulePy>()?;
    m.add_class::<WoundRulePy>()?;
    m.add_class::<SaveRulePy>()?;
    m.add_class::<DamagesRulePy>()?;
    m.add_class::<AttackCharacteristicRulePy>()?;
    Ok(())
}
