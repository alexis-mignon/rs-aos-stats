use crate::probabilities::combat_stats::{AttackStats, Characteristic, DefenseStats, RollModifier};
use crate::probabilities::dice::DiceRoll;
use pyo3::{exceptions::PyValueError, prelude::*};

fn require_non_negative(field_name: &str, value: i32) -> PyResult<u32> {
    if value < 0 {
        Err(PyValueError::new_err(format!(
            "{field_name} must be non-negative"
        )))
    } else {
        Ok(value as u32)
    }
}

#[pyclass(name = "Characteristic")]
#[derive(Clone, Copy, Debug)]
pub struct CharacteristicPy {
    pub characteristic: Characteristic,
}

#[pymethods]
impl CharacteristicPy {
    #[new]
    fn new(value: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(char_val) = value.extract::<i32>() {
            Ok(CharacteristicPy {
                characteristic: Characteristic::Value(require_non_negative(
                    "characteristic value",
                    char_val,
                )?),
            })
        } else if let Ok(char_roll) = TryInto::try_into(&value) {
            Ok(CharacteristicPy {
                characteristic: Characteristic::DiceRoll(char_roll),
            })
        } else if let Ok(dice_str) = value.extract::<String>() {
            Ok(CharacteristicPy {
                characteristic: Characteristic::DiceRoll(
                    DiceRoll::from_str(&dice_str)
                        .map_err(|e| PyValueError::new_err(e.to_string()))?,
                ),
            })
        } else {
            Err(PyValueError::new_err("Could not convert to Characteristic"))
        }
    }
}

impl From<CharacteristicPy> for Characteristic {
    fn from(val: CharacteristicPy) -> Self {
        val.characteristic
    }
}

#[pyclass(name = "AttackStats")]
#[derive(Clone, Debug)]
pub struct AttackStatsPy {
    pub attack_stats: AttackStats,
}

fn extract_characteristic(value: &Bound<'_, PyAny>) -> PyResult<Characteristic> {
    if let Ok(char_val) = value.extract::<i32>() {
        Ok(Characteristic::Value(require_non_negative(
            "characteristic value",
            char_val,
        )?))
    } else if let Ok(dice_str) = value.extract::<String>() {
        Ok(Characteristic::DiceRoll(
            DiceRoll::from_str(&dice_str).map_err(|e| PyValueError::new_err(e.to_string()))?,
        ))
    } else if let Ok(char_roll) = TryInto::try_into(value) {
        Ok(Characteristic::DiceRoll(char_roll))
    } else if let Ok(charac) = value.extract::<CharacteristicPy>() {
        Ok(charac.into())
    } else {
        Err(PyValueError::new_err("Could not convert to Characteristic"))
    }
}

#[pymethods]
impl AttackStatsPy {
    #[new]
    fn new(
        attacks: Bound<'_, PyAny>,
        to_hit: i32,
        to_wound: i32,
        rend: i32,
        damages: Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        Ok(AttackStatsPy {
            attack_stats: AttackStats {
                attacks: extract_characteristic(&attacks)?,
                to_hit: require_non_negative("to_hit", to_hit)?,
                to_wound: require_non_negative("to_wound", to_wound)?,
                rend: require_non_negative("rend", rend)?,
                damages: extract_characteristic(&damages)?,
            },
        })
    }
}

#[pyclass(name = "DefenseStats")]
#[derive(Clone, Debug)]
pub struct DefenseStatsPy {
    pub defense_stats: DefenseStats,
}

#[pymethods]
impl DefenseStatsPy {
    #[new]
    #[pyo3(signature = (to_save, ward=None))]
    fn new(to_save: u32, ward: Option<u32>) -> Self {
        DefenseStatsPy {
            defense_stats: DefenseStats { to_save, ward },
        }
    }
}

#[pyclass(name = "RollModifier")]
#[derive(Clone, Debug)]
pub struct RollModifierPy {
    pub roll_modifier: RollModifier,
}

#[pymethods]
impl RollModifierPy {
    #[new]
    fn new(to_hit: i32, to_wound: i32, to_save: i32) -> Self {
        RollModifierPy {
            roll_modifier: RollModifier::new(to_hit, to_wound, to_save),
        }
    }
}

impl From<AttackStatsPy> for AttackStats {
    fn from(val: AttackStatsPy) -> Self {
        val.attack_stats
    }
}

impl From<DefenseStatsPy> for DefenseStats {
    fn from(val: DefenseStatsPy) -> Self {
        val.defense_stats
    }
}

impl From<RollModifierPy> for RollModifier {
    fn from(val: RollModifierPy) -> Self {
        val.roll_modifier
    }
}
