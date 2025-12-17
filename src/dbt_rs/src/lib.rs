use pyo3::prelude::*;

/// Returns the Rust version of the dbt-oxide extension.
#[pyfunction]
fn rust_version() -> PyResult<String> {
    Ok("0.1.0".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn dbt_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_version, m)?)?;
    Ok(())
}
