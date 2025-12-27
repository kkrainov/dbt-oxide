use crate::manifest::{OxideManifest, ManifestMetadata, OxideNode, OxideMacro, OxideSource, OxideExposure, OxideMetric, OxideGroup};
use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use pythonize::depythonize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

static MANIFEST: OnceCell<RwLock<OxideManifest>> = OnceCell::new();

/// Get reference to the global manifest (for internal use by other modules).
pub fn get_global_manifest() -> PyResult<&'static RwLock<OxideManifest>> {
    MANIFEST.get().ok_or_else(|| {
        pyo3::exceptions::PyRuntimeError::new_err(
            "Manifest not loaded. Call load_manifest() first.",
        )
    })
}

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
        Some(node) => Ok(node.depends_on().nodes.clone()),
        None => Err(pyo3::exceptions::PyKeyError::new_err(format!(
            "Node not found: {}",
            unique_id
        ))),
    }
}

#[pyfunction]
pub fn write_manifest_to_file(manifest_json: &str, path: &str) -> PyResult<()> {
    // Deserialize JSON to OxideManifest
    let manifest: OxideManifest = serde_json::from_str(manifest_json).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to parse manifest JSON: {}", e))
    })?;

    // Serialize to pretty JSON
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize manifest: {}", e))
    })?;

    // Write to file
    std::fs::write(path, json).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to write file: {}", e))
    })?;

    Ok(())
}

#[pyfunction]
pub fn serialize_manifest_to_json(manifest_json: &str) -> PyResult<String> {
    // Round-trip through Rust types to validate + normalize
    let manifest: OxideManifest = serde_json::from_str(manifest_json).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to parse manifest JSON: {}", e))
    })?;

    serde_json::to_string_pretty(&manifest).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize manifest: {}", e))
    })
}

#[pyfunction]
pub fn get_manifest_stats(
    manifest_json: &str,
) -> PyResult<std::collections::HashMap<String, usize>> {
    let manifest: OxideManifest = serde_json::from_str(manifest_json).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to parse manifest JSON: {}", e))
    })?;

    let mut stats = std::collections::HashMap::new();
    stats.insert("nodes".to_string(), manifest.nodes.len());
    stats.insert("sources".to_string(), manifest.sources.len());
    stats.insert("macros".to_string(), manifest.macros.len());
    stats.insert("docs".to_string(), manifest.docs.len());
    stats.insert("exposures".to_string(), manifest.exposures.len());
    stats.insert("metrics".to_string(), manifest.metrics.len());
    stats.insert("groups".to_string(), manifest.groups.len());

    Ok(stats)
}

pub fn register_manifest_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_manifest, m)?)?;
    m.add_function(wrap_pyfunction!(get_node_count, m)?)?;
    m.add_function(wrap_pyfunction!(get_node_dependencies, m)?)?;
    m.add_function(wrap_pyfunction!(write_manifest_to_file, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_manifest_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(get_manifest_stats, m)?)?;
    m.add_class::<PyOxideManifest>()?;
    Ok(())
}

/// Python wrapper around OxideManifest - Rust owns all data.
#[pyclass(name = "OxideManifest")]
pub struct PyOxideManifest {
    inner: OxideManifest,
}

#[pymethods]
impl PyOxideManifest {
    /// Create new OxideManifest with parameters (matching Python Manifest API)
    #[new]
    #[pyo3(signature = (nodes=None, sources=None, macros=None, docs=None, exposures=None, metrics=None, groups=None, selectors=None, disabled=None, files=None, semantic_models=None, unit_tests=None, saved_queries=None, fixtures=None, metadata=None))]
    pub fn new(
        nodes: Option<&PyDict>,
        sources: Option<&PyDict>,
        macros: Option<&PyDict>,
        docs: Option<&PyDict>,
        exposures: Option<&PyDict>,
        metrics: Option<&PyDict>,
        groups: Option<&PyDict>,
        selectors: Option<&PyDict>,
        disabled: Option<&PyDict>,
        files: Option<&PyDict>,
        semantic_models: Option<&PyDict>,
        unit_tests: Option<&PyDict>,
        saved_queries: Option<&PyDict>,
        fixtures: Option<&PyDict>,
        metadata: Option<&PyDict>,
    ) -> PyResult<Self> {
        // Depythonize each parameter with defaults
        let nodes_map: HashMap<String, OxideNode> = nodes
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse nodes: {}", e)))?
            .unwrap_or_default();

        let sources_map: HashMap<String, OxideSource> = sources
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse sources: {}", e)))?
            .unwrap_or_default();

        let macros_map: HashMap<String, OxideMacro> = macros
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse macros: {}", e)))?
            .unwrap_or_default();

        let docs_map: HashMap<String, Value> = docs
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse docs: {}", e)))?
            .unwrap_or_default();

        let exposures_map: HashMap<String, OxideExposure> = exposures
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse exposures: {}", e)))?
            .unwrap_or_default();

        let metrics_map: HashMap<String, OxideMetric> = metrics
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse metrics: {}", e)))?
            .unwrap_or_default();

        let groups_map: HashMap<String, OxideGroup> = groups
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse groups: {}", e)))?
            .unwrap_or_default();

        let disabled_map: HashMap<String, Vec<Value>> = disabled
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse disabled: {}", e)))?
            .unwrap_or_default();

        let metadata_obj: ManifestMetadata = metadata
            .map(|d| depythonize(d))
            .transpose()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse metadata: {}", e)))?
            .unwrap_or_default();

        let manifest = OxideManifest {
            nodes: nodes_map,
            sources: sources_map,
            macros: macros_map,
            docs: docs_map,
            exposures: exposures_map,
            metrics: metrics_map,
            groups: groups_map,
            selectors: HashMap::new(),
            disabled: disabled_map,
            metadata: metadata_obj,
            semantic_models: HashMap::new(),
            unit_tests: HashMap::new(),
            saved_queries: HashMap::new(),
        };

        Ok(Self { inner: manifest })
    }

    /// Load manifest from JSON string
    #[staticmethod]
    pub fn from_json(json_str: &str) -> PyResult<Self> {
        let manifest = OxideManifest::from_json_str(json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: manifest })
    }

    /// Build parent map: maps each node to its direct dependencies.
    pub fn build_parent_map(&self) -> std::collections::HashMap<String, Vec<String>> {
        self.inner.build_parent_map()
    }

    /// Build child map: maps each node to what depends on it.
    pub fn build_child_map(&self) -> std::collections::HashMap<String, Vec<String>> {
        self.inner.build_child_map()
    }

    /// Build group map: maps group names to their member nodes.
    pub fn build_group_map(&self) -> std::collections::HashMap<String, Vec<String>> {
        self.inner.build_group_map()
    }

    /// Number of nodes in manifest.
    #[getter]
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    /// Resolve a ref to its full node object.
    #[pyo3(signature = (source_node_id, target_model_name, target_model_package, target_model_version, current_project, node_package))]
    pub fn resolve_ref(
        &self,
        py: Python,
        source_node_id: Option<&str>,
        target_model_name: &str,
        target_model_package: Option<&str>,
        target_model_version: Option<i64>,
        current_project: &str,
        node_package: &str,
    ) -> PyResult<Option<PyObject>> {
        let node_opt = self.inner.resolve_ref(
            source_node_id,
            target_model_name,
            target_model_package,
            target_model_version,
            current_project,
            node_package,
        );
        
        match node_opt {
            Some(node) => {
                let py_obj = pythonize::pythonize(py, node)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Failed to serialize node: {}", e)
                    ))?;
                Ok(Some(py_obj))
            },
            None => Ok(None),
        }
    }

    /// Resolve a source to its full source object.
    #[pyo3(signature = (target_source_name, target_table_name, current_project, node_package))]
    pub fn resolve_source(
        &self,
        py: Python,
        target_source_name: &str,
        target_table_name: &str,
        current_project: &str,
        node_package: &str,
    ) -> PyResult<Option<PyObject>> {
        let source_opt = self.inner.resolve_source(
            target_source_name,
            target_table_name,
            current_project,
            node_package,
        );
        
        match source_opt {
            Some(source) => {
                let py_obj = pythonize::pythonize(py, source)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Failed to serialize source: {}", e)
                    ))?;
                Ok(Some(py_obj))
            },
            None => Ok(None),
        }
    }

    /// Resolve a doc to its full doc object.
    #[pyo3(signature = (name, package, current_project, node_package))]
    pub fn resolve_doc(
        &self,
        py: Python,
        name: &str,
        package: Option<&str>,
        current_project: &str,
        node_package: &str,
    ) -> PyResult<Option<PyObject>> {
        let doc_opt = self.inner.resolve_doc(name, package, current_project, node_package);
        
        match doc_opt {
            Some(doc) => {
                let py_obj = pythonize::pythonize(py, doc)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Failed to serialize doc: {}", e)
                    ))?;
                Ok(Some(py_obj))
            },
            None => Ok(None),
        }
    }

    /// Find macro by name with priority and package filtering.
    #[pyo3(signature = (name, root_project_name, internal_packages, package=None))]
    pub fn find_macro_by_name(
        &self,
        name: &str,
        root_project_name: &str,
        internal_packages: std::collections::HashSet<String>,
        package: Option<&str>,
    ) -> Option<String> {
        self.inner
            .find_macro_by_name(name, root_project_name, &internal_packages, package)
    }

    /// Find materialization macro by name with adapter inheritance.
    pub fn find_materialization_macro_by_name(
        &self,
        project_name: &str,
        materialization_name: &str,
        adapter_types: Vec<String>,
        internal_packages: std::collections::HashSet<String>,
        allow_package_override: bool,
    ) -> Option<String> {
        self.inner.find_materialization_macro_by_name(
            project_name,
            materialization_name,
            &adapter_types,
            &internal_packages,
            allow_package_override,
        )
    }

    /// Get macro by unique_id, returns the macro as a Python dict via pythonize.
    pub fn get_macro(&self, py: Python, unique_id: &str) -> PyResult<PyObject> {
        match self.inner.get_macro(unique_id) {
            Some(oxide_macro) => pythonize::pythonize(py, oxide_macro)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string())),
            None => Ok(py.None()),
        }
    }

    /// Get the adapter type from manifest metadata.
    pub fn get_adapter_type(&self) -> Option<String> {
        self.inner.get_adapter_type().map(|s| s.to_string())
    }
}
