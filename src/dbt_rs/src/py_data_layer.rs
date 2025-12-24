use crate::data_layer::build_graph_from_manifest;
use crate::manifest::OxideManifest;
use crate::py_graph::DbtGraph;
use crate::py_manifest::get_global_manifest;
use pyo3::prelude::*;

/// Build a DbtGraph from manifest JSON.
/// Returns a new DbtGraph that can be used by Python Graph wrapper.
#[pyfunction]
pub fn build_graph_from_manifest_json(json_string: &str) -> PyResult<DbtGraph> {
    let manifest = OxideManifest::from_json_str(json_string)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let oxide_graph = build_graph_from_manifest(&manifest);
    Ok(DbtGraph::from_oxide_graph(oxide_graph))
}

/// Build a DbtGraph from the globally loaded manifest.
#[pyfunction]
pub fn build_graph_from_global_manifest() -> PyResult<DbtGraph> {
    let manifest_lock = get_global_manifest()?;
    let manifest = manifest_lock
        .read()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Lock poisoned"))?;

    let oxide_graph = build_graph_from_manifest(&manifest);
    Ok(DbtGraph::from_oxide_graph(oxide_graph))
}

pub fn register_data_layer_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_graph_from_manifest_json, m)?)?;
    m.add_function(wrap_pyfunction!(build_graph_from_global_manifest, m)?)?;
    Ok(())
}
