mod combat_stats;
mod combat_tree;
mod dice;
mod rules;

use crate::python::combat_stats::{
    AttackStatsPy, CharacteristicPy, DefenseStatsPy, RollModifierPy,
};
use crate::python::combat_tree::{compute_damages_py, CombatConfigPy};
use crate::python::dice::{D3Plus, D6Plus, DiceRollPy, ND3Plus, ND6Plus, D3, D6, ND3, ND6};
use crate::python::rules::{build_standard_sequence_py, register_rules};
use pyo3::prelude::*;

#[pymodule]
fn rs_aos_stats(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DiceRollPy>()?; // Now it's called DiceRoll in Python
    m.add_class::<D6>()?;
    m.add_class::<D3>()?;
    m.add_class::<ND6>()?;
    m.add_class::<ND3>()?;
    m.add_class::<ND6Plus>()?;
    m.add_class::<ND3Plus>()?;
    m.add_class::<D3Plus>()?;
    m.add_class::<D6Plus>()?;
    // Add combat stats objects
    m.add_class::<CharacteristicPy>()?;
    m.add_class::<AttackStatsPy>()?;
    m.add_class::<DefenseStatsPy>()?;
    m.add_class::<RollModifierPy>()?;
    // Add combat trees functions
    m.add_class::<CombatConfigPy>()?;
    m.add_function(wrap_pyfunction!(compute_damages_py, m)?)?;
    m.add_function(wrap_pyfunction!(build_standard_sequence_py, m)?)?;
    // Rules
    register_rules(_py, m)?;
    Ok(())
}
