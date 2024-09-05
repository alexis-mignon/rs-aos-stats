use pyo3::prelude::*;
use crate::probabilities::rules::{
    HitRule, WoundRule, SaveRule, DamagesRule, AttackCharacteristicRule,
    WardRule, CritAutoWoundRule, CritMortalWoundRule, CritDoubleHitRule
};

use crate::probabilities::combat_tree::Rule;


#[pyclass(name="HitRule")]
#[derive(Clone, Debug)]
pub struct HitRulePy;

#[pymethods]
impl HitRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<HitRule> for HitRulePy {
    fn into(self) -> HitRule {
        HitRule
    }
}

#[pyclass(name="WoundRule")]
#[derive(Clone, Debug)]
pub struct WoundRulePy;

#[pymethods]
impl WoundRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<WoundRule> for WoundRulePy {
    fn into(self) -> WoundRule {
        WoundRule
    }
}

#[pyclass(name="SaveRule")]
#[derive(Clone, Debug)]
pub struct SaveRulePy;

#[pymethods]
impl SaveRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<SaveRule> for SaveRulePy {
    fn into(self) -> SaveRule {
        SaveRule
    }
}

#[pyclass(name="DamagesRule")]
#[derive(Clone, Debug)]
pub struct DamagesRulePy;

#[pymethods]
impl DamagesRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<DamagesRule> for DamagesRulePy {
    fn into(self) -> DamagesRule {
        DamagesRule
    }
}

#[pyclass(name="AttackCharacteristicRule")]
#[derive(Clone, Debug)]
pub struct AttackCharacteristicRulePy;

#[pymethods]
impl AttackCharacteristicRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<AttackCharacteristicRule> for AttackCharacteristicRulePy {
    fn into(self) -> AttackCharacteristicRule {
        AttackCharacteristicRule
    }
}

#[pyclass(name="WardRule")]
#[derive(Clone, Debug)]
pub struct WardRulePy;

#[pymethods]
impl WardRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<WardRule> for WardRulePy {
    fn into(self) -> WardRule {
        WardRule
    }
}

#[pyclass(name="CritAutoWoundRule")]
#[derive(Clone, Debug)]
pub struct CritAutoWoundRulePy;


#[pymethods]
impl CritAutoWoundRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<CritAutoWoundRule> for CritAutoWoundRulePy {
    fn into(self) -> CritAutoWoundRule {
        CritAutoWoundRule
    }
}

#[pyclass(name="CritMortalWoundRule")]
#[derive(Clone, Debug)]
pub struct CritMortalWoundRulePy;

#[pymethods]
impl CritMortalWoundRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<CritMortalWoundRule> for CritMortalWoundRulePy {
    fn into(self) -> CritMortalWoundRule {
        CritMortalWoundRule
    }
}

#[pyclass(name="CritDoubleHitRule")]
#[derive(Clone, Debug)]
pub struct CritDoubleHitRulePy;

#[pymethods]
impl CritDoubleHitRulePy {
    #[new]
    fn new() -> Self {Self {}}
}

impl Into<CritDoubleHitRule> for CritDoubleHitRulePy {
    fn into(self) -> CritDoubleHitRule {
        CritDoubleHitRule
    }
}


impl From<&PyAny> for Box<dyn Rule> {
    fn from(rule: &PyAny) -> Box<dyn Rule> {
        if let Ok(rule) = rule.extract::<HitRulePy>() {
            let rule: HitRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<WoundRulePy>() {
            let rule: WoundRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<SaveRulePy>() {
            let rule: SaveRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<DamagesRulePy>() {
            let rule: DamagesRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<AttackCharacteristicRulePy>() {
            let rule: AttackCharacteristicRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<WardRulePy>() {
            let rule: WardRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<CritAutoWoundRulePy>() {
            let rule: CritAutoWoundRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<CritMortalWoundRulePy>() {
            let rule: CritMortalWoundRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else if let Ok(rule) = rule.extract::<CritDoubleHitRulePy>() {
            let rule: CritDoubleHitRule = rule.into();
            let rule: Box<dyn Rule> = Box::new(rule);
            rule
        }
        else {
            panic!("Unknown rule type")
        }
    }
}



