mod data_layer;
mod graph;
mod manifest;

#[cfg(feature = "extension-module")]
mod py_graph;

#[cfg(feature = "extension-module")]
mod py_manifest;

#[cfg(feature = "extension-module")]
mod py_data_layer;

#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
use py_graph::DbtGraph; // This line will likely become unused if register_graph_module is used.

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

    py_manifest::register_manifest_module(m)?;
    py_data_layer::register_data_layer_module(m)?;

    Ok(())
}
