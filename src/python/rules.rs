/// Python wrappers for the Rust rule implementations.
///
/// Rule classes are kept as marker types for backward compatibility
/// (e.g. test_rules.py tests instantiation). The actual pipeline
/// is now driven by `compute_damages(config, hit_rule_type)`.
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

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

#[pyfunction(name = "build_standard_sequence")]
pub fn build_standard_sequence_py(
    py: Python<'_>,
    hit_rule_type: &str,
    has_ward: bool,
) -> PyResult<Vec<PyObject>> {
    let mut rules: Vec<PyObject> = Vec::new();

    rules.push(Py::new(py, AttackCharacteristicRulePy)?.into());

    match hit_rule_type {
        "normal" => {
            rules.push(Py::new(py, HitRulePy)?.into());
        }
        "crit_auto_wound" => {
            rules.push(Py::new(py, CritAutoWoundRulePy)?.into());
        }
        "crit_mortal_wound" => {
            rules.push(Py::new(py, CritMortalWoundRulePy)?.into());
        }
        "crit_double_hit" => {
            rules.push(Py::new(py, CritDoubleHitRulePy)?.into());
        }
        _ => {
            return Err(PyValueError::new_err(format!(
                "Invalid hit_rule_type '{}'. Must be one of: 'normal', 'crit_auto_wound', 'crit_mortal_wound', 'crit_double_hit'",
                hit_rule_type
            )));
        }
    }

    rules.push(Py::new(py, WoundRulePy)?.into());
    rules.push(Py::new(py, SaveRulePy)?.into());
    rules.push(Py::new(py, DamagesRulePy)?.into());

    if has_ward {
        rules.push(Py::new(py, WardRulePy)?.into());
    }

    Ok(rules)
}
