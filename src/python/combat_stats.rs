use pyo3::{exceptions::PyValueError, prelude::*};
use crate::probabilities::combat_stats::{
    AttackStats, Characteristic, DefenseStats, RollModifier
};
use crate::probabilities::dice::DiceRoll;



/* impl TryFrom<PyAny> for Characteristic {
    type Error = PyErr;

    fn try_from(value: PyAny) -> Result<Self, Self::Error> {
        if let Ok(char_val) = value.extract::<i32>() {Ok(Characteristic::Value(char_val as u32))}
        else if let Ok(char_roll) = value.try_into() {Ok(Characteristic::DiceRoll(char_roll))}
        else {Err(PyValueError::new_err("Could not convert to Characteritic"))}
    }
}

impl TryFrom<&PyAny> for Characteristic {
    type Error = PyErr;

    fn try_from(value: &PyAny) -> Result<Self, Self::Error> {
        if let Ok(char_val) = value.extract::<i32>() {Ok(Characteristic::Value(char_val as u32))}
        else if let Ok(char_roll) = TryInto::try_into(value) {Ok(Characteristic::DiceRoll(char_roll))}
        else {Err(PyValueError::new_err("Could not convert to Characteritic"))}
    }
}
 */

#[pyclass(name="Characteristic")]
#[derive(Clone, Copy, Debug)]
pub struct CharacteristicPy {
    pub characteristic: Characteristic 
}

#[pymethods]
impl CharacteristicPy {
    #[new]
    fn new(value: &PyAny) -> PyResult<Self> {
        if let Ok(char_val) = value.extract::<i32>() {Ok(
            CharacteristicPy { characteristic: Characteristic::Value(char_val as u32)}
        )}
        else if let Ok(char_roll) = TryInto::try_into(value) {Ok(
            CharacteristicPy {
                characteristic: Characteristic::DiceRoll(char_roll)
            }
        )}
        else if let Ok(dice_str) = value.extract::<String>() {Ok(
            CharacteristicPy{
                characteristic: Characteristic::DiceRoll(DiceRoll::from_str(dice_str).unwrap())
            }
        )}
        else {Err(PyValueError::new_err("Could not convert to Characteritic"))}
    }
}

impl Into<Characteristic> for CharacteristicPy {
    fn into(self) -> Characteristic {
        self.characteristic
    }
}

impl Into<CharacteristicPy> for i32 {
    fn into(self) -> CharacteristicPy {
        CharacteristicPy {characteristic: Characteristic::Value(self as u32)}
    }
}

#[pyclass(name="AttackStats")]
#[derive(Clone, Debug)]
pub struct AttackStatsPy {
    pub attack_stats: AttackStats
}

fn extract_characteristic(value: &PyAny) -> Characteristic {
    if let Ok(char_val) = value.extract::<i32>() {Characteristic::Value(char_val as u32)}
    else if let Ok(dice_str) = value.extract::<String>() {Characteristic::DiceRoll(DiceRoll::from_str(dice_str).unwrap())}
    else if let Ok(char_roll) = TryInto::try_into(value) {Characteristic::DiceRoll(char_roll)}
    else if let Ok(charac) = value.extract::<CharacteristicPy>() {charac.into()}
    else {panic!("Could not convert to Characteritic")}
}

#[pymethods]
impl AttackStatsPy {
    #[new]
    fn new(attacks: &PyAny, to_hit: i32, to_wound: i32, rend: i32, damages: &PyAny) -> Self {
        AttackStatsPy {
            attack_stats: AttackStats {
                attacks: extract_characteristic(attacks).into(),
                to_hit: to_hit as u32,
                to_wound: to_wound as u32,
                rend: rend as u32,
                damages : extract_characteristic(damages).into()
            }
        }
    }
}

#[pyclass(name="DefenseStats")]
#[derive(Clone, Debug)]
pub struct DefenseStatsPy {
    pub defense_stats: DefenseStats
}

#[pymethods]
impl DefenseStatsPy {
    #[new]
    fn new(to_save: u32, ward: Option<u32>) -> Self {
        DefenseStatsPy {
            defense_stats: DefenseStats {
                to_save, ward
            }
        }
    }
}

#[pyclass(name="RollModifier")]
#[derive(Clone, Debug)]
pub struct RollModifierPy {
    pub roll_modifier: RollModifier
}

#[pymethods]
impl RollModifierPy {
    #[new]
    fn new(to_hit: i32, to_wound: i32, to_save: i32) -> Self{
        RollModifierPy {
            roll_modifier: RollModifier::new(to_hit, to_wound, to_save)
        }
    }
}

impl Into<AttackStats> for AttackStatsPy {
    fn into(self) -> AttackStats {
        self.attack_stats
    }
}

impl Into<DefenseStats> for DefenseStatsPy {
    fn into(self) -> DefenseStats {
        self.defense_stats
    }
}

impl Into<RollModifier> for RollModifierPy {
    fn into(self) -> RollModifier {
        self.roll_modifier
    }
}