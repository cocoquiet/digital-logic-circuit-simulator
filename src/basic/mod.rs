use pyo3::prelude::*;
use pyo3::types::PyAny;

#[derive(Debug, Clone, Copy)]
enum BoolOrInt {
    Bool(bool),
}

impl<'source> FromPyObject<'source> for BoolOrInt {
    fn extract_bound(obj: &Bound<'source, PyAny>) -> PyResult<Self> {
        if let Ok(b) = obj.extract::<bool>() {
            Ok(BoolOrInt::Bool(b))
        } else if let Ok(i) = obj.extract::<u8>() {
            match i {
                0 => Ok(BoolOrInt::Bool(false)),
                1 => Ok(BoolOrInt::Bool(true)),
                _ => Err(pyo3::exceptions::PyValueError::new_err(
                    "Expected a boolean or 0/1 as input",
                )),
            }
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected a boolean or 0/1 as input",
            ))
        }
    }
}

impl BoolOrInt {
    fn as_bool(self) -> bool {
        match self {
            BoolOrInt::Bool(b) => b,
        }
    }
}

#[pyfunction]
fn gate_and(a: BoolOrInt, b: BoolOrInt) -> bool {
    a.as_bool() && b.as_bool()
}

#[pyfunction]
fn gate_or(a: BoolOrInt, b: BoolOrInt) -> bool {
    a.as_bool() || b.as_bool()
}

#[pyfunction]
fn gate_not(a: BoolOrInt) -> bool {
    !a.as_bool()
}

#[pyfunction]
fn gate_nand(a: BoolOrInt, b: BoolOrInt) -> bool {
    !(a.as_bool() && b.as_bool())
}

#[pyfunction]
fn gate_nor(a: BoolOrInt, b: BoolOrInt) -> bool {
    !(a.as_bool() || b.as_bool())
}

#[pyfunction]
fn gate_xor(a: BoolOrInt, b: BoolOrInt) -> bool {
    a.as_bool() ^ b.as_bool()
}

#[pyfunction]
fn gate_xnor(a: BoolOrInt, b: BoolOrInt) -> bool {
    !(a.as_bool() ^ b.as_bool())
}

#[pymodule]
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gate_and, m)?)?;
    m.add_function(wrap_pyfunction!(gate_or, m)?)?;
    m.add_function(wrap_pyfunction!(gate_not, m)?)?;
    m.add_function(wrap_pyfunction!(gate_nand, m)?)?;
    m.add_function(wrap_pyfunction!(gate_nor, m)?)?;
    m.add_function(wrap_pyfunction!(gate_xor, m)?)?;
    m.add_function(wrap_pyfunction!(gate_xnor, m)?)?;
    Ok(())
}
