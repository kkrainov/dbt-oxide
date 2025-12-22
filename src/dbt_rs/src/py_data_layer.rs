use crate::data_layer::build_graph_from_manifest;
use crate::manifest::OxideManifest;
use crate::py_graph::DbtGraph;
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

pub fn register_data_layer_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_graph_from_manifest_json, m)?)?;
    Ok(())
}
