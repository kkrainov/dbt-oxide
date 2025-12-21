use crate::manifest::OxideManifest;
use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::sync::RwLock;

static MANIFEST: OnceCell<RwLock<OxideManifest>> = OnceCell::new();

#[pyfunction]
pub fn load_manifest(json_string: &str) -> PyResult<()> {
    let manifest = OxideManifest::from_json_str(json_string)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    // Initialize or replace the manifest
    match MANIFEST.get() {
        Some(lock) => {
            let mut guard = lock
                .write()
                .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Lock poisoned"))?;
            *guard = manifest;
        }
        None => {
            MANIFEST.set(RwLock::new(manifest)).map_err(|_| {
                pyo3::exceptions::PyRuntimeError::new_err("Failed to initialize manifest")
            })?;
        }
    }
    Ok(())
}

#[pyfunction]
pub fn get_node_count() -> PyResult<usize> {
    let lock = MANIFEST
        .get()
        .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Manifest not loaded"))?;
    let manifest = lock
        .read()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Lock poisoned"))?;
    Ok(manifest.node_count())
}

#[pyfunction]
pub fn get_node_dependencies(unique_id: &str) -> PyResult<Vec<String>> {
    let lock = MANIFEST
        .get()
        .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Manifest not loaded"))?;
    let manifest = lock
        .read()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Lock poisoned"))?;

    match manifest.get_node(unique_id) {
        Some(node) => Ok(node.depends_on.nodes.clone()),
        None => Err(pyo3::exceptions::PyKeyError::new_err(format!(
            "Node not found: {}",
            unique_id
        ))),
    }
}

pub fn register_manifest_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_manifest, m)?)?;
    m.add_function(wrap_pyfunction!(get_node_count, m)?)?;
    m.add_function(wrap_pyfunction!(get_node_dependencies, m)?)?;
    Ok(())
}
