use pyo3::prelude::*;
mod basic;

#[pymodule]
fn digital_logic_circuit_simulator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let basic_module = PyModule::new(m.py(), "basic")?;
    basic::register(&basic_module)?;
    m.add_submodule(&basic_module)?;
    Ok(())
}
