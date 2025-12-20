use std::collections::HashSet;
use pyo3::prelude::*;
use crate::graph::OxideGraph;

#[pyclass]
pub struct DbtGraph {
    inner: OxideGraph,
}

impl Default for DbtGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl DbtGraph {
    #[new]
    pub fn new() -> Self {
        DbtGraph {
            inner: OxideGraph::new(),
        }
    }

    pub fn __len__(&self) -> usize {
        self.inner.node_count()
    }

    pub fn number_of_nodes(&self) -> usize {
        self.inner.node_count()
    }

    pub fn number_of_edges(&self) -> usize {
        self.inner.edge_count()
    }
    
    pub fn get_edge_weight(&self, source: String, target: String) -> Option<String> {
        self.inner.get_edge_weight(&source, &target).cloned()
    }

    pub fn add_node(&mut self, id: String) -> String {
        self.inner.add_node(id)
    }

    #[pyo3(signature = (source, target, edge_type=None))]
    pub fn add_edge(&mut self, source: String, target: String, edge_type: Option<String>) -> PyResult<()> {
        self.inner.add_edge(&source, &target, edge_type)
            .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }

    pub fn descendants(&self, node: String, limit: Option<usize>) -> HashSet<String> {
        self.inner.descendants(&node, limit)
    }

    pub fn ancestors(&self, node: String, limit: Option<usize>) -> HashSet<String> {
        self.inner.ancestors(&node, limit)
    }

    pub fn select_children(&self, selected: HashSet<String>, limit: Option<usize>) -> HashSet<String> {
        self.inner.select_children(&selected, limit)
    }

    pub fn select_parents(&self, selected: HashSet<String>, limit: Option<usize>) -> HashSet<String> {
        self.inner.select_parents(&selected, limit)
    }

    pub fn topological_sort_grouped(&self) -> PyResult<Vec<Vec<String>>> {
        self.inner.topological_sort_grouped()
            .map_err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>)
    }

    pub fn remove_node(&mut self, node: String) {
        self.inner.remove_node(&node);
    }

    pub fn nodes(&self) -> HashSet<String> {
        self.inner.nodes()
    }

    pub fn edges(&self) -> Vec<(String, String)> {
        self.inner.edges()
    }

    pub fn in_degree(&self, node: String) -> Option<usize> {
        self.inner.in_degree(&node)
    }

    pub fn out_degree(&self, node: String) -> Option<usize> {
        self.inner.out_degree(&node)
    }

    pub fn successors(&self, node: String) -> HashSet<String> {
        self.inner.successors(&node)
    }

    pub fn predecessors(&self, node: String) -> HashSet<String> {
        self.inner.predecessors(&node)
    }

    pub fn subgraph(&self, nodes: HashSet<String>) -> DbtGraph {
        DbtGraph {
            inner: self.inner.subgraph(&nodes),
        }
    }

    pub fn get_subset_graph(&self, nodes: HashSet<String>) -> DbtGraph {
        DbtGraph {
            inner: self.inner.get_subset_graph(&nodes),
        }
    }

    pub fn find_cycle(&self) -> Option<Vec<(String, String)>> {
        self.inner.find_cycle()
    }
}
