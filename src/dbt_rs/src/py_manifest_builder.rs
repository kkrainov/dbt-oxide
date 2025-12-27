// PyO3 class for manifest building
// Provides Python interface to OxideManifest

use crate::manifest::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule};
use pythonize::depythonize;

/// Python-facing ManifestBuilder class
/// Wraps OxideManifest and provides methods for incremental building
#[pyclass(name = "ManifestBuilder")]
pub struct PyManifestBuilder {
    manifest: OxideManifest,
}

#[pymethods]
impl PyManifestBuilder {
    /// Create a new ManifestBuilder
    ///
    /// Args:
    ///     metadata: Optional dictionary containing manifest metadata
    #[new]
    #[pyo3(signature = (metadata=None))]
    pub fn new(metadata: Option<&PyDict>) -> PyResult<Self> {
        let manifest = match metadata {
            Some(meta) => {
                let metadata_rust: ManifestMetadata = depythonize(meta).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Failed to parse metadata: {}",
                        e
                    ))
                })?;
                OxideManifest::with_metadata(metadata_rust)
            }
            None => OxideManifest::default(),
        };
        Ok(Self { manifest })
    }

    /// Add a node to the manifest
    ///
    /// Args:
    ///     node: Dictionary containing node data
    pub fn add_node(&mut self, node: &PyDict) -> PyResult<()> {
        let node_rust: OxideNode = depythonize(node).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse node: {}", e))
        })?;
        self.manifest.add_node(node_rust);
        Ok(())
    }

    /// Add multiple nodes at once (batch operation)
    ///
    /// Args:
    ///     nodes: List of node dictionaries
    pub fn add_nodes(&mut self, nodes: &PyList) -> PyResult<()> {
        for item in nodes {
            let node_dict = item.downcast::<PyDict>()?;
            let node_rust: OxideNode = depythonize(node_dict).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to parse node in batch: {}",
                    e
                ))
            })?;
            self.manifest.add_node(node_rust);
        }
        Ok(())
    }

    /// Add a source to the manifest
    ///
    /// Args:
    ///     source: Dictionary containing source data
    pub fn add_source(&mut self, source: &PyDict) -> PyResult<()> {
        let source_rust: SourceDefinition = depythonize(source).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse source: {}",
                e
            ))
        })?;
        self.manifest.add_source(source_rust);
        Ok(())
    }

    /// Add a macro to the manifest
    ///
    /// Args:
    ///     macro_def: Dictionary containing macro data
    pub fn add_macro(&mut self, macro_def: &PyDict) -> PyResult<()> {
        let macro_rust: OxideMacro = depythonize(macro_def).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse macro: {}", e))
        })?;
        self.manifest.add_macro(macro_rust);
        Ok(())
    }

    /// Get the number of nodes in the manifest
    #[getter]
    pub fn node_count(&self) -> usize {
        self.manifest.node_count()
    }

    /// Get the number of sources in the manifest
    #[getter]
    pub fn source_count(&self) -> usize {
        self.manifest.sources.len()
    }

    /// Get the number of macros in the manifest
    #[getter]
    pub fn macro_count(&self) -> usize {
        self.manifest.macros.len()
    }
}

/// Register the PyManifestBuilder class in the module
pub fn register(m: &PyModule) -> PyResult<()> {
    m.add_class::<PyManifestBuilder>()?;
    Ok(())
}
