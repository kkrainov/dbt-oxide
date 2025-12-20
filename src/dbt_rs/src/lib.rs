mod graph;

#[cfg(feature = "extension-module")]
mod py_graph;

#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
use py_graph::DbtGraph;

/// Returns the Rust version of the dbt-oxide extension.
#[cfg(feature = "extension-module")]
#[pyfunction]
fn rust_version() -> PyResult<String> {
    Ok("0.1.0".to_string())
}

/// A Python module implemented in Rust.
#[cfg(feature = "extension-module")]
#[pymodule]
fn dbt_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_version, m)?)?;
    
    m.add_class::<DbtGraph>()?;
    
    Ok(())
}
