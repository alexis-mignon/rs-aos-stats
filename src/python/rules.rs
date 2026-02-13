use crate::probabilities::rules::{
    AttackCharacteristicRule, CritAutoWoundRule, CritDoubleHitRule, CritMortalWoundRule,
    DamagesRule, HitRule, SaveRule, WardRule, WoundRule,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::probabilities::combat_tree::Rule;

#[pyclass(name = "HitRule")]
#[derive(Clone, Debug)]
pub struct HitRulePy;

#[pymethods]
impl HitRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<HitRulePy> for HitRule {
    fn from(_val: HitRulePy) -> Self {
        HitRule
    }
}

#[pyclass(name = "WoundRule")]
#[derive(Clone, Debug)]
pub struct WoundRulePy;

#[pymethods]
impl WoundRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<WoundRulePy> for WoundRule {
    fn from(_val: WoundRulePy) -> Self {
        WoundRule
    }
}

#[pyclass(name = "SaveRule")]
#[derive(Clone, Debug)]
pub struct SaveRulePy;

#[pymethods]
impl SaveRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<SaveRulePy> for SaveRule {
    fn from(_val: SaveRulePy) -> Self {
        SaveRule
    }
}

#[pyclass(name = "DamagesRule")]
#[derive(Clone, Debug)]
pub struct DamagesRulePy;

#[pymethods]
impl DamagesRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<DamagesRulePy> for DamagesRule {
    fn from(_val: DamagesRulePy) -> Self {
        DamagesRule
    }
}

#[pyclass(name = "AttackCharacteristicRule")]
#[derive(Clone, Debug)]
pub struct AttackCharacteristicRulePy;

#[pymethods]
impl AttackCharacteristicRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<AttackCharacteristicRulePy> for AttackCharacteristicRule {
    fn from(_val: AttackCharacteristicRulePy) -> Self {
        AttackCharacteristicRule
    }
}

#[pyclass(name = "WardRule")]
#[derive(Clone, Debug)]
pub struct WardRulePy;

#[pymethods]
impl WardRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<WardRulePy> for WardRule {
    fn from(_val: WardRulePy) -> Self {
        WardRule
    }
}

#[pyclass(name = "CritAutoWoundRule")]
#[derive(Clone, Debug)]
pub struct CritAutoWoundRulePy;

#[pymethods]
impl CritAutoWoundRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<CritAutoWoundRulePy> for CritAutoWoundRule {
    fn from(_val: CritAutoWoundRulePy) -> Self {
        CritAutoWoundRule
    }
}

#[pyclass(name = "CritMortalWoundRule")]
#[derive(Clone, Debug)]
pub struct CritMortalWoundRulePy;

#[pymethods]
impl CritMortalWoundRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<CritMortalWoundRulePy> for CritMortalWoundRule {
    fn from(_val: CritMortalWoundRulePy) -> Self {
        CritMortalWoundRule
    }
}

#[pyclass(name = "CritDoubleHitRule")]
#[derive(Clone, Debug)]
pub struct CritDoubleHitRulePy;

#[pymethods]
impl CritDoubleHitRulePy {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

impl From<CritDoubleHitRulePy> for CritDoubleHitRule {
    fn from(_val: CritDoubleHitRulePy) -> Self {
        CritDoubleHitRule
    }
}

pub fn extract_rule(rule: &Bound<'_, PyAny>) -> PyResult<Box<dyn Rule>> {
    if let Ok(rule) = rule.extract::<HitRulePy>() {
        let rule: HitRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<WoundRulePy>() {
        let rule: WoundRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<SaveRulePy>() {
        let rule: SaveRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<DamagesRulePy>() {
        let rule: DamagesRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<AttackCharacteristicRulePy>() {
        let rule: AttackCharacteristicRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<WardRulePy>() {
        let rule: WardRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<CritAutoWoundRulePy>() {
        let rule: CritAutoWoundRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<CritMortalWoundRulePy>() {
        let rule: CritMortalWoundRule = rule.into();
        Ok(Box::new(rule))
    } else if let Ok(rule) = rule.extract::<CritDoubleHitRulePy>() {
        let rule: CritDoubleHitRule = rule.into();
        Ok(Box::new(rule))
    } else {
        Err(PyValueError::new_err("Unknown rule type"))
    }
}

pub fn register_rules(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<HitRulePy>()?;
    m.add_class::<WoundRulePy>()?;
    m.add_class::<SaveRulePy>()?;
    m.add_class::<DamagesRulePy>()?;
    m.add_class::<AttackCharacteristicRulePy>()?;
    m.add_class::<WardRulePy>()?;
    m.add_class::<CritAutoWoundRulePy>()?;
    m.add_class::<CritMortalWoundRulePy>()?;
    m.add_class::<CritDoubleHitRulePy>()?;
    Ok(())
}
