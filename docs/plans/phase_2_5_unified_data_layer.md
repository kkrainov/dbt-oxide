# Phase 2.5: Unified Data Layer - Implementation Plan

Build graph from manifest entirely in Rust. **Full replacement** of Python iteration.

## Design Principle

> [!IMPORTANT]
> **Single Source of Truth**
> 
> `Graph` class is the **ONLY** interface to Rust graph. No `dbt_rs.*` calls outside of `graph.py`.

> [!IMPORTANT]
> **Test-Driven Development (TDD)**
> 
> - **Rust:** Write tests FIRST, then implementation
> - **Python:** Verify existing interface tests pass WITHOUT modification
> - All Python unit tests must pass unchanged (backward compatibility guarantee)

---

## Rust Changes

#### [NEW] [data_layer.rs](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/src/dbt_rs/src/data_layer.rs)

```rust
use crate::graph::OxideGraph;
use crate::manifest::OxideManifest;

pub fn build_graph_from_manifest(manifest: &OxideManifest) -> OxideGraph {
    let mut graph = OxideGraph::new();
    
    // Add all nodes
    for id in manifest.sources.keys() { graph.add_node(id.clone()); }
    for id in manifest.nodes.keys() { graph.add_node(id.clone()); }
    for id in manifest.exposures.keys() { graph.add_node(id.clone()); }
    for id in manifest.metrics.keys() { graph.add_node(id.clone()); }
    for id in manifest.semantic_models.keys() { graph.add_node(id.clone()); }
    for id in manifest.saved_queries.keys() { graph.add_node(id.clone()); }
    for id in manifest.unit_tests.keys() { graph.add_node(id.clone()); }
    
    // Add edges from depends_on
    for (uid, n) in &manifest.nodes { for d in &n.depends_on.nodes { let _ = graph.add_edge(d, uid, None); } }
    for (uid, e) in &manifest.exposures { for d in &e.depends_on.nodes { let _ = graph.add_edge(d, uid, None); } }
    for (uid, m) in &manifest.metrics { for d in &m.depends_on.nodes { let _ = graph.add_edge(d, uid, None); } }
    for (uid, s) in &manifest.semantic_models { for d in &s.depends_on.nodes { let _ = graph.add_edge(d, uid, None); } }
    for (uid, q) in &manifest.saved_queries { for d in &q.depends_on.nodes { let _ = graph.add_edge(d, uid, None); } }
    for (uid, t) in &manifest.unit_tests { for d in &t.depends_on.nodes { let _ = graph.add_edge(d, uid, None); } }
    
    graph
}
```

#### [NEW] [py_data_layer.rs](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/src/dbt_rs/src/py_data_layer.rs)

```rust
#[pyfunction]
pub fn build_graph_from_manifest_json(json_string: &str) -> PyResult<DbtGraph> {
    let manifest = OxideManifest::from_json_str(json_string)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(DbtGraph::from_oxide_graph(build_graph_from_manifest(&manifest)))
}
```

#### [MODIFY] py_graph.rs, lib.rs

Add `from_oxide_graph()` factory and register new function.

---

## Python Changes

#### [MODIFY] [graph.py](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/core/dbt/graph/graph.py)

```python
class Graph:
    @classmethod
    def empty(cls) -> "Graph":
        return cls(dbt_rs.DbtGraph())

    @classmethod
    def from_manifest(cls, manifest: "Manifest") -> "Graph":
        json_str = manifest.writable_manifest().to_json()
        return cls(dbt_rs.build_graph_from_manifest_json(json_str))

    def find_cycle(self):
        return self.graph.find_cycle()
```

#### [MODIFY] [compilation.py](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/core/dbt/compilation.py)

```python
from dbt.graph.graph import Graph

class Linker:
    def __init__(self, data=None) -> None:
        if data is None:
            data = {}
        self.graph: Graph = Graph.empty()
        if data:
            raise NotImplementedError("Initializing Linker with data is not supported.")

    def link_graph(self, manifest: Manifest):
        self.graph = Graph.from_manifest(manifest)
        cycle = self.find_cycles()
        if cycle:
            raise RuntimeError(f"Found a cycle: {cycle}")

    def get_graph(self, manifest: Manifest) -> Graph:
        self.link_graph(manifest)
        return self.graph
```

**Removed:** `link_node()`, `dependency()`, `add_node()` iteration logic.

---

## Implementation Workflow (TDD)

### Step 1: Write Rust Tests FIRST

**File:** `src/dbt_rs/src/data_layer.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_graph_empty() {
        let manifest = OxideManifest::from_json_str(r#"{"nodes":{},"sources":{}}"#).unwrap();
        let graph = build_graph_from_manifest(&manifest);
        assert_eq!(graph.node_count(), 0);
    }
    
    #[test]
    fn test_build_graph_with_single_dependency() {
        let json = r#"{
            "nodes": {
                "model.test.a": {"unique_id":"model.test.a","name":"a","resource_type":"model","package_name":"test"},
                "model.test.b": {"unique_id":"model.test.b","name":"b","resource_type":"model","package_name":"test",
                                 "depends_on":{"nodes":["model.test.a"]}}
            },
            "sources": {}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);
        
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.ancestors("model.test.b", None).contains("model.test.a"));
    }
    
    #[test]
    fn test_build_graph_includes_all_manifest_types() {
        let json = r#"{
            "nodes": {"model.test.m": {"unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test"}},
            "sources": {"source.test.raw.tbl": {"unique_id":"source.test.raw.tbl","source_name":"raw","name":"tbl","package_name":"test"}},
            "exposures": {"exposure.test.e": {"unique_id":"exposure.test.e","name":"e"}},
            "metrics": {"metric.test.met": {"unique_id":"metric.test.met","name":"met"}},
            "semantic_models": {"semantic_model.test.sm": {"unique_id":"semantic_model.test.sm","name":"sm"}},
            "saved_queries": {"saved_query.test.sq": {"unique_id":"saved_query.test.sq","name":"sq"}},
            "unit_tests": {"unit_test.test.ut": {"unique_id":"unit_test.test.ut","name":"ut"}}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);
        assert_eq!(graph.node_count(), 7);
    }
}
```

**Run (should FAIL):**
```bash
cargo test --no-default-features test_build_graph
```

### Step 2: Implement Rust Code

Implement `build_graph_from_manifest()` until tests pass.

**Run:**
```bash
cargo test --no-default-features
```

### Step 3: Verify Python Interface (NO TEST CHANGES)

**Critical:** Existing tests must pass **WITHOUT modification**.

```bash
# These tests verify Graph interface hasn't changed
uv run pytest tests/unit/graph/test_graph.py -v

# Full unit test suite
uv run pytest tests/unit/ -v --tb=short
```

**Success criteria:** All tests pass without changing any assertions.

---

## Verification Checklist

- [ ] **Rust tests written FIRST** (Step 1)
- [ ] Rust tests initially FAIL
- [ ] `build_graph_from_manifest()` implemented
- [ ] `cargo test --no-default-features` passes
- [ ] Python `tests/unit/graph/test_graph.py` passes **unchanged**
- [ ] Full Python test suite passes **unchanged**
- [ ] No test assertions modified


## Files Changed

| File | Action |
|------|--------|
| `src/dbt_rs/src/data_layer.rs` | NEW |
| `src/dbt_rs/src/py_data_layer.rs` | NEW |
| `src/dbt_rs/src/py_graph.rs` | MODIFY |
| `src/dbt_rs/src/lib.rs` | MODIFY |
| `core/dbt/graph/graph.py` | MODIFY |
| `core/dbt/compilation.py` | MODIFY |
