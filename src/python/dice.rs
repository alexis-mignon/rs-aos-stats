use pyo3::exceptions::{PyValueError, PyNotImplementedError};
use pyo3::prelude::*;
use pyo3::PyClass;
use crate::probabilities::dice::DiceRoll;

//======================== Dice =========================
// Base class for all dice rolls

fn _wrap_type<T: PyClass>(py: Python, obj: impl Into<PyClassInitializer<T>>) -> PyResult<PyObject> {
    Ok(Py::new(py, obj).unwrap().to_object(py))
}

#[pyclass(name="DiceRoll", subclass)]
pub struct DiceRollPy;

#[pymethods]
impl DiceRollPy {
    #[new]
    fn new() -> Self {
       DiceRollPy
    }

    // Common interface for all variants
    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Err(PyNotImplementedError::new_err("Not implemented for base class"))
    }

    #[staticmethod]
    fn from_str(py: Python, dice_str: String) -> PyResult<PyObject>{
        let dice = DiceRoll::from_str(dice_str);
        match dice {
            Ok(DiceRoll::D3) => _wrap_type(py, D3::new()),
            Ok(DiceRoll::D6) => _wrap_type(py, D6::new()),
            Ok(DiceRoll::ND3(n)) => _wrap_type(py, ND3::new(n)),
            Ok(DiceRoll::ND6(n)) => _wrap_type(py, ND6::new(n)),
            Ok(DiceRoll::D3Plus(m)) => _wrap_type(py, D3Plus::new(m)),
            Ok(DiceRoll::D6Plus(m)) => _wrap_type(py, D6Plus::new(m)),
            Ok(DiceRoll::ND3Plus(n, m)) => _wrap_type(py, ND3Plus::new(n, m)),
            Ok(DiceRoll::ND6Plus(n, m)) => _wrap_type(py, ND6Plus::new(n, m)),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
}

// D6 class
#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct D6;

#[pymethods]
impl D6 {
    #[new]
    fn new() -> (Self, DiceRollPy) {
        (D6, DiceRollPy)
    }

    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::D6.values_and_probas())
    }
}



// D3 class
#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct D3;

#[pymethods]
impl D3 {
    #[new]
    fn new() -> (Self, DiceRollPy) {
        (D3, DiceRollPy)
    }

    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::D3.values_and_probas())
    }
}

// ND6 class
#[derive(Clone, Copy, Debug)]
#[pyclass(extends=DiceRollPy)]
pub struct ND6 {
    n: u32,
}

#[pymethods]
impl ND6 {
    #[new]
    fn new(n: u32) -> (Self, DiceRollPy) {
        (ND6 {n}, DiceRollPy)
    }

    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::ND3(self.n).values_and_probas())
    }
}


// ND3 class
#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct ND3 {
    n: u32
}

#[pymethods]
impl ND3 {
    #[new]
    fn new(n: u32) -> (Self, DiceRollPy) {
        (ND3 {n}, DiceRollPy)
    }

    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::ND3(self.n).values_and_probas())
    }
}

#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct D3Plus {
    m: u32
}

#[pymethods]
impl D3Plus {
    #[new]
    fn new(m: u32) -> (Self, DiceRollPy) {
        (D3Plus {m}, DiceRollPy)
    }
    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::D3Plus(self.m).values_and_probas())
    }
}

#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct D6Plus {
    m: u32
}

#[pymethods]
impl D6Plus {
    #[new]
    fn new(m: u32) -> (Self, DiceRollPy) {
        (D6Plus {m}, DiceRollPy)
    }
    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::D6Plus(self.m).values_and_probas())
    }
}



// ND6plus class
#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct ND6Plus {
    n: u32, m: u32
}

#[pymethods]
impl ND6Plus {
    #[new]
    fn new(n: u32, m: u32) -> (Self, DiceRollPy) {
        (ND6Plus {n, m}, DiceRollPy)
    }

    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::ND6Plus(self.n, self.m).values_and_probas())
    }
}

// ND3plus class
#[pyclass(extends=DiceRollPy)]
#[derive(Clone, Copy, Debug)]
pub struct ND3Plus {
    n: u32, m: u32
}

#[pymethods]
impl ND3Plus {
    #[new]
    fn new(n: u32, m: u32) -> (Self, DiceRollPy) {
        (ND3Plus {n, m}, DiceRollPy)
    }

    fn values_and_probas(&self) -> PyResult<Vec<(u32, f64)>> {
        Ok(DiceRoll::ND3Plus(self.n, self.m).values_and_probas())
    }
}

impl TryFrom<&PyAny> for DiceRoll {
    type Error = PyErr;

    fn try_from(value: &PyAny) -> Result<Self, Self::Error> {
        if let Ok(_d3) = value.extract::<D3>() {
            Ok(DiceRoll::D3)
        } else if let Ok(_d6) = value.extract::<D6>() {
            Ok(DiceRoll::D6)
        } else if let Ok(nd3) = value.extract::<ND3>() {
            Ok(DiceRoll::ND3(nd3.n))
        } else if let Ok(nd6) = value.extract::<ND6>() {
            Ok(DiceRoll::ND6(nd6.n))
        } else if let Ok(nd3plus) = value.extract::<ND3Plus>() {
            Ok(DiceRoll::ND3Plus(nd3plus.n, nd3plus.m))
        } else if let Ok(nd6plus) = value.extract::<ND6Plus>() {
            Ok(DiceRoll::ND6Plus(nd6plus.n, nd6plus.m))
        } else {
            Err(PyErr::new::<PyValueError, _>("Not implemented for this type"))
        }
    }
}

impl TryFrom<PyAny> for DiceRoll {
    type Error = PyErr;

    fn try_from(value: PyAny) -> Result<Self, Self::Error> {
        if let Ok(_d3) = value.extract::<D3>() {
            Ok(DiceRoll::D3)
        } else if let Ok(_d6) = value.extract::<D6>() {
            Ok(DiceRoll::D6)
        } else if let Ok(nd3) = value.extract::<ND3>() {
            Ok(DiceRoll::ND3(nd3.n))
        } else if let Ok(nd6) = value.extract::<ND6>() {
            Ok(DiceRoll::ND6(nd6.n))
        } else if let Ok(nd3plus) = value.extract::<ND3Plus>() {
            Ok(DiceRoll::ND3Plus(nd3plus.n, nd3plus.m))
        } else if let Ok(nd6plus) = value.extract::<ND6Plus>() {
            Ok(DiceRoll::ND6Plus(nd6plus.n, nd6plus.m))
        } else {
            Err(PyErr::new::<PyValueError, _>("Not implemented for this type"))
        }
    }
}


