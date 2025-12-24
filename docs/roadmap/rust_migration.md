# dbt-oxide: Python to Rust Migration Roadmap

**Project Goal:** Achieve 10x-100x performance improvement in `dbt compile`/DAG resolution while maintaining 100% backward compatibility with `dbt-core`.

**Architecture:** Strangler Fig Pattern (Python Shell, Rust Core)

---

## Related Documents

This roadmap provides an overview of the migration project. For detailed execution plans with step-by-step implementation guides, see:

| Phase | Detailed Plan |
|-------|---------------|
| Phase 1: Graph Engine | [phase_1_graph_engine.md](../plans/phase_1_graph_engine.md) |
| Phase 2: Zero-Copy Manifest | [phase_2_zero_copy_manifest.md](../plans/phase_2_zero_copy_manifest.md) |

> [!TIP]
> The `docs/plans/` directory contains granular implementation plans with code samples, test strategies, and verification checklists for each phase.

---

## Executive Summary

| Phase | Status | Performance Target | Risk |
|-------|--------|-------------------|------|
| **Phase 0: Foundation** |  Complete | - | Low |
| **Phase 1: Graph Engine** |  Complete | 100x faster toposort | Low |
| **Phase 2: Zero-Copy Manifest** |  Complete | Rust manifest storage | Medium |
| **Phase 2.5: Unified Data Layer** |  Complete | Direct graph-from-manifest | Low |
| **Phase 3.1: minijinja-py** |  Next | 3-5x faster Jinja | Low |
| **Phase 3.2: Rust Node Construction** |  Planned | Eliminate 5s validation | Medium |
| **Phase 3.3: Rust Manifest Ops** |  Planned | Fast lookups | Low |
| **Phase 4: Parallelization** |  Planned | Linear scaling with CPU cores | Medium |

---

## Phase 0: Foundation & Infrastructure 

**Status:** Complete

Established the hybrid build environment enabling Python/Rust interoperability.

### Completed Items

- [x] Maturin build configuration
- [x] PyO3 bindings setup
- [x] Rust workspace in `src/dbt_rs/`
- [x] Development environment with `uv`

---

## Phase 1: The Graph Engine 

**Status:** Complete
**Objective:** Replace NetworkX with high-performance Rust graph using `petgraph`.
**Result:** 100x improvement in topological sorting and cycle detection.

### Implementation Summary

| Component | File | LOC | Description |
|-----------|------|-----|-------------|
| Pure Rust Graph | `src/dbt_rs/src/graph.rs` | 695 | `OxideGraph` with all algorithms |
| PyO3 Wrapper | `src/dbt_rs/src/py_graph.rs` | 116 | `DbtGraph` Python class |
| Python Integration | `core/dbt/graph/graph.py` | - | Graph wrapper using Rust |
| Queue Refactor | `core/dbt/graph/queue.py` | - | Uses Rust toposort |

### Completed Checklist

#### Sub-Phase 1: Rust Implementation
- [x] Step 1.1: Foundation & Skeleton (`DbtGraph` struct, `petgraph` integration)
- [x] Step 1.2: Core Operations (`add_node`, `add_edge` with NetworkX parity)
- [x] Step 1.3: Traversal Algorithms (`ancestors`, `descendants` with edge filtering)
- [x] Step 1.4: Selection Algorithms (`select_children`, `select_parents`)
- [x] Step 1.5: Topological Sort (Kahn's algorithm with level grouping)

#### Sub-Phase 2: Python Integration
- [x] Step 2.1: Wrapper & Loading (Bulk load from manifest)
- [x] Step 2.2: Method Routing (Delegate hot paths to Rust)
- [x] Step 2.3: Queue Acceleration (Rust toposort in `_get_scores`)

#### Sub-Phase 3: Rust Exclusivity
- [x] Step 3.1: Enforce Rust Availability (Remove Python fallbacks)
- [x] Step 3.2: Clean Queue Logic (Delete Python toposort)

#### Sub-Phase 4: Complete NetworkX Removal
- [x] Step 4.1: Structural Parity (`remove_node`, `edges`, `find_cycles`, `subgraph`, `get_subset_graph`)
- [x] Step 4.2: Linker & Compiler Refactor (Direct Rust graph construction)
- [x] Step 4.3: Graph Wrapper & Queue Refactor (Encapsulate `DbtGraph`)
- [x] Step 4.4: Test Suite Updates
- [x] Step 4.5: Deprecation (NetworkX removed from hot paths)

### Key Rust Methods Implemented

```rust
// Core Operations
pub fn add_node(&mut self, id: String) -> String
pub fn add_edge(&mut self, source: &str, target: &str, edge_type: Option<String>) -> Result<(), String>
pub fn remove_node(&mut self, node: &str)

// Traversal
pub fn ancestors(&self, node: &str, limit: Option<usize>) -> HashSet<String>
pub fn descendants(&self, node: &str, limit: Option<usize>) -> HashSet<String>
pub fn select_children(&self, selected: &HashSet<String>, limit: Option<usize>) -> HashSet<String>
pub fn select_parents(&self, selected: &HashSet<String>, limit: Option<usize>) -> HashSet<String>

// Analysis
pub fn topological_sort_grouped(&self) -> Result<Vec<Vec<String>>, String>
pub fn find_cycle(&self) -> Option<Vec<(String, String)>>
pub fn subgraph(&self, nodes: &HashSet<String>) -> OxideGraph
pub fn get_subset_graph(&self, nodes: &HashSet<String>) -> OxideGraph
```

---

## Phase 2: Zero-Copy Manifest 

**Status:** Complete (Infrastructure)
**Objective:** Share complete Manifest state between Python and Rust without repeated serialization.
**Result:** Rust can store and query the full manifest; awaiting integration in Phase 2.5.

> [!NOTE]
> **Phase 2 vs Phase 2.5:** Phase 2 established the Rust manifest storage infrastructure. The manifest is synced to Rust but **not yet utilized** by any operations. Phase 2.5 will integrate the manifest with the graph engine, eliminating Python from hot paths entirely.

### Implementation Summary

| Component | File | LOC | Description |
|-----------|------|-----|-------------|
| Rust Manifest Schema | `src/dbt_rs/src/manifest.rs` | 267 | Full `OxideManifest` with all node types |
| PyO3 Bindings | `src/dbt_rs/src/py_manifest.rs` | 61 | `load_manifest`, `get_node_count`, etc. |
| Python Sync | `core/dbt/parser/manifest.py` | - | `_sync_manifest_to_rust()` at end of `load()` |

### Completed Checklist

#### Step 1: Rust Dependencies
- [x] Add `serde = { version = "1.0", features = ["derive"] }`
- [x] Add `serde_json = "1.0"`
- [x] Add `once_cell = "1.19"`

#### Step 2: Rust Manifest Schema (Core Structures)
- [x] `OxideDependsOn` struct
- [x] `OxideNodeConfig` struct
- [x] `OxideNode` struct (all fields from ManifestNode)
- [x] `OxideManifest::from_json_str()` method

#### Step 3: Rust Manifest Schema (Additional Types)
- [x] `OxideSource`, `OxideMacro`, `OxideExposure`
- [x] `OxideMetric`, `OxideGroup`
- [x] `OxideSemanticModel`, `OxideSavedQuery`, `OxideUnitTest`
- [x] `OxideManifestMetadata`

#### Step 4: Rust Unit Tests
- [x] `test_parse_empty_manifest`
- [x] `test_parse_single_node`
- [x] `test_parse_with_missing_optional_fields`
- [x] `test_parse_all_node_types`
- [x] `test_invalid_json_returns_error`

#### Step 5: PyO3 Bindings
- [x] `load_manifest(json_string: &str)` function
- [x] `get_node_count()` function
- [x] `get_node_dependencies(unique_id: &str)` function
- [x] Global `OnceCell<RwLock<OxideManifest>>` storage

#### Step 6-10: Integration & Verification
- [x] Python `_sync_manifest_to_rust()` in `ManifestLoader.load()`
- [x] All unit tests pass unchanged
- [x] `dbt parse` works with Rust sync
- [x] Performance benchmark established (~18.8s baseline)

### Current Data Flow

```
Python: Parse files → Build Manifest → manifest.to_json()
                                           ↓
Rust: dbt_rs.load_manifest(json) → OxideManifest in OnceCell<RwLock<>>
                                           ↓
                    [Phase 2.5 will connect this to OxideGraph]
```

### What's Next (Phase 2.5)

The Rust manifest is currently **stored but not queried**. Rather than adding temporary Python→Rust integration (which would be discarded), Phase 2.5 will:
1. Build `OxideGraph` directly from `OxideManifest` in Rust
2. Eliminate graph construction from Python entirely
3. Provide a unified data layer for Phase 3 (Compiler)

---

## Phase 2.5: Unified Data Layer 

**Status:** Complete
**Objective:** Integrate Manifest and Graph in Rust to build the graph directly from manifest dependencies.
**Result:** `build_graph_from_global_manifest()` implemented in Rust, used by `Linker`.

> [!NOTE]
> Graph construction now uses the global OxideManifest directly in Rust via `Graph.from_global_manifest()`. The `link_graph()` method in `Linker` leverages this for faster graph building.

### Completed Items

- [x] `OxideManifest::build_graph()` method in Rust
- [x] `build_graph_from_global_manifest()` PyO3 function
- [x] `Graph.from_global_manifest()` Python class method
- [x] `Linker.link_graph()` uses global manifest when available
- [x] Rust tests for graph-from-manifest
- [x] Python integration tests pass

### Implementation Summary

| Component | File | Description |
|-----------|------|-------------|
| Rust Graph Builder | `src/dbt_rs/src/manifest.rs` | `build_graph()` method |
| PyO3 Function | `src/dbt_rs/src/py_data_layer.rs` | `build_graph_from_global_manifest()` |
| Python Integration | `core/dbt/graph/graph.py` | `Graph.from_global_manifest()` |
| Linker Usage | `core/dbt/compilation.py` | `link_graph()` method |

---

## Phase 3: Hybrid Parsing Engine 

**Architecture:** Python + minijinja-py for Jinja, Rust for node construction and operations.

> [!NOTE]
> Based on profiling (2000 models, 16s total), the hybrid approach delivers maximum performance with minimal risk.

### Profiling Evidence

| Component | Current Time | Target | Strategy |
|-----------|--------------|--------|----------|
| Jinja rendering | 11.1s (69%) | 3-4s | minijinja-py |
| Node construction + validation | 5.3s (33%) | <0.5s | Rust structs |
| Manifest sync | 675ms | 0ms | Direct Rust storage |

---

## Phase 3.1: minijinja-py Integration 

**Status:** Next
**Objective:** Replace Jinja2 with minijinja-py (Rust-powered Python bindings).
**Performance Target:** 3-5x faster template rendering.
**Risk:** Low

> [!TIP]
> minijinja-py is the official Python binding for minijinja. It's a drop-in replacement that runs Jinja templates in Rust while keeping Python functions callable.

### Implementation

```python
# core/dbt/clients/jinja.py
from minijinja import Environment as MiniJinjaEnv

def create_dbt_jinja_env(manifest, config):
    env = MiniJinjaEnv()
    
    # Register context functions (these are Python, called from Rust)
    env.add_function("ref", create_ref_function(manifest))
    env.add_function("source", create_source_function(manifest))
    env.add_function("config", create_config_function())
    env.add_function("var", create_var_function(config))
    env.add_function("env_var", os.environ.get)
    
    # Adapter calls work automatically - they're Python functions
    env.add_function("adapter", lambda: adapter)
    
    return env
```

### Checklist

- [ ] Add `minijinja` to `pyproject.toml`
- [ ] Create `DbtMiniJinjaEnvironment` wrapper class
- [ ] Register core context functions (ref, source, config, var)
- [ ] Register adapter and other Python objects
- [ ] Replace `get_rendered()` to use minijinja
- [ ] Register project macros as templates
- [ ] Unit tests comparing output with Jinja2
- [ ] Performance benchmark

### Why This Works

minijinja-py handles Python function calls automatically:
- Known functions (ref, source, config) → registered Python callbacks
- Unknown functions → error (fail-fast, no silent issues)
- Adapter methods → Python object passed, all methods callable

---

## Phase 3.2: Rust Node Construction 

**Status:** Planned
**Objective:** Build OxideNode directly in Rust, eliminating Python dataclass + jsonschema overhead.
**Performance Target:** Eliminate 5.3s node construction time.
**Risk:** Medium

### Profiling Evidence

| Function | Time | Issue |
|----------|------|-------|
| `finalize_and_validate` | 3.4s | jsonschema validation |
| `_create_parsetime_node` | 2.3s | Python dataclass creation |
| `validate` (jsonschema) | 5.4s | Called 8001 times |

### Implementation

```python
# Instead of Python dataclass + validation
def _create_parsetime_node(self, block, path, config, fqn, ...):
    dct = { ... }  # Build dict as before
    
    # Call Rust to construct node directly in OxideManifest
    node_id = dbt_rs.create_node(
        name=dct["name"],
        raw_code=dct["raw_code"],
        path=dct["path"],
        package_name=dct["package_name"],
        resource_type=dct["resource_type"],
        config=dct["config"],
        fqn=dct["fqn"],
        depends_on=depends_on,
    )
    
    # Return lightweight Python wrapper
    return RustNodeWrapper(node_id)
```

```rust
// src/dbt_rs/src/node_builder.rs
#[pyfunction]
pub fn create_node(
    name: &str,
    raw_code: &str,
    path: &str,
    package_name: &str,
    resource_type: &str,
    config: HashMap<String, Value>,
    fqn: Vec<String>,
    depends_on: Vec<String>,
) -> PyResult<String> {
    let node = OxideNode {
        unique_id: format!("{}.{}.{}", resource_type, package_name, name),
        name: name.to_string(),
        // ... other fields
    };
    
    // Store directly in global manifest
    let manifest = get_global_manifest()?;
    let mut guard = manifest.write().unwrap();
    guard.nodes.insert(node.unique_id.clone(), node);
    
    Ok(node.unique_id)
}
```

### Checklist

- [ ] Add `create_node()` PyO3 function
- [ ] Create `RustNodeWrapper` Python class for backward compatibility
- [ ] Implement all node types (ModelNode, TestNode, SeedNode, etc.)
- [ ] Update parsers to use Rust node construction
- [ ] Add Rust tests (TDD)
- [ ] Verify all Python tests pass with wrappers

---

## Phase 3.3: Rust Manifest Operations 

**Status:** Planned
**Objective:** Move lookup operations from Python to Rust.
**Performance Target:** O(1) HashMap lookups, eliminate parent/child map rebuilding.
**Risk:** Low

### Operations to Migrate

| Operation | Current | Target |
|-----------|---------|--------|
| `resolve_ref()` | Python iteration | Rust HashMap |
| `resolve_source()` | Python iteration | Rust HashMap |
| `build_parent_and_child_maps()` | Python (called 3x) | Rust (build once) |
| `get_node()` | Python dict | Rust HashMap |

### Implementation

```rust
// src/dbt_rs/src/manifest_ops.rs
#[pyfunction]
pub fn resolve_ref(
    name: &str,
    package: Option<&str>,
    version: Option<&str>,
    current_project: &str,
) -> PyResult<Option<String>> {
    let manifest = get_global_manifest()?;
    let guard = manifest.read().unwrap();
    Ok(guard.find_node_by_name(name, package, version))
}

#[pyfunction]
pub fn get_parent_map() -> PyResult<HashMap<String, Vec<String>>> {
    let manifest = get_global_manifest()?;
    let guard = manifest.read().unwrap();
    Ok(guard.build_parent_map())
}
```

### Checklist

- [ ] Add `resolve_ref()` PyO3 function
- [ ] Add `resolve_source()` PyO3 function
- [ ] Add `get_parent_map()` / `get_child_map()` functions
- [ ] Update `Manifest` Python class to use Rust lookups
- [ ] Cache parent/child maps in Rust
- [ ] Add Rust tests (TDD)

---

### Expected Performance After Phase 3

| Component | Before | After Phase 3 |
|-----------|--------|---------------|
| Jinja rendering | 11.1s | **3-4s** |
| Node construction | 5.3s | **<0.5s** |
| Manifest sync | 675ms | **0ms** |
| **Total parse (2000 models)** | **~16s** | **~4s** |

---

## Phase 4: Parallelization & Introspection 

**Status:** Planned (Weeks 17+)
**Objective:** Multithreaded compilation and intelligent database introspection handling.
**Performance Target:** Linear scaling with CPU cores (8-core = 8x speedup).
**Risk:** Medium (GIL contention for Python trapdoor macros)

### Step 4.1: Rayon Integration

- [ ] Add `rayon` to Cargo.toml
- [ ] Expose `compile_batch(node_ids: Vec<String>) -> Vec<CompiledNode>`
- [ ] Use `rayon::par_iter` to render templates in parallel

```rust
use rayon::prelude::*;

pub fn compile_batch(&self, node_ids: Vec<String>) -> Vec<Result<String, Error>> {
    node_ids.par_iter()
        .map(|id| {
            let node = self.manifest.get_node(id)?;
            self.compile(&node.raw_code.unwrap_or_default())
        })
        .collect()
}
```

> [!NOTE]
> Pure Rust macros run in parallel. Macros hitting the "Python Trapdoor" will serialize on the GIL (bottleneck), but this is acceptable for the <5% of macros that require Python.

### Step 4.2: Introspection Separation

- [ ] Identify models requiring DB introspection (e.g., `adapter.get_columns_in_relation`)
- [ ] Create `IntrospectionRequirement` enum:

```rust
pub enum CompilationStatus {
    Complete(String),                    // Compiled SQL
    PendingIntrospection {
        node_id: String,
        required_relations: Vec<String>, // Tables to introspect
        partial_sql: String,             // SQL so far
    },
}
```

- [ ] Split compilation work:
  1. **Rust:** Compile all "pure" models (90% of DAG) in parallel
  2. **Rust:** Return list of `PendingIntrospection` nodes to Python
  3. **Python:** Execute SQL introspection queries (using existing Adapter)
  4. **Python:** Update context and call Rust to finish compiling

### Step 4.3: Python Integration for Introspection

- [ ] Add `dbt_rs.compile_batch_pure(node_ids) -> (compiled, pending)`
- [ ] Add `dbt_rs.complete_introspection(node_id, columns_data) -> compiled_sql`
- [ ] Integrate with existing `Compiler` in Python

### Step 4.4: GIL Optimization

- [ ] Track GIL acquisition time per trapdoor call
- [ ] Batch Python calls where possible
- [ ] Consider `pyo3-asyncio` for async Python calls (optional)

### Verification Checklist

- [ ] Parallel compilation produces identical results to sequential
- [ ] Introspection models compile correctly after Python provides data
- [ ] Performance scales linearly with available cores
- [ ] No race conditions or data corruption

---

## Testing Strategy

### Rust Tests

```bash
# Run pure Rust tests (no Python dependency)
cargo test --no-default-features

# Run with PyO3 (requires Python)
cargo test
```

### Python Tests

```bash
# Unit tests
uv run pytest tests/unit/ -v

# Integration tests
uv run pytest tests/functional/ -v

# Specific graph tests
uv run pytest tests/unit/graph/ -v
```

### Performance Benchmarks

```bash
# Prerequisites
cargo install hyperfine

# Run benchmark suite
uv run cargo run --manifest-path performance/runner/Cargo.toml -- sample \
    -p $PWD/performance/projects \
    -b $PWD/performance/baselines \
    -o $PWD/performance/results
```

---

## Risk Mitigation

| Risk | Phase | Mitigation |
|------|-------|------------|
| Schema drift between Python/Rust | 2, 2.5 | Use `#[serde(deny_unknown_fields)]` in dev |
| Jinja2 compatibility | 3 | Python trapdoor for unsupported features |
| GIL bottleneck | 4 | Batch Python calls, async where possible |
| Memory pressure | 2, 3 | Consider `rkyv` for zero-copy if needed |
| Test regression | All | Golden master tests, no assertion changes |

---

## Version Milestones

| Version | Phase | Key Features |
|---------|-------|--------------|
| v0.1.0 | 0-1 | Rust graph engine, NetworkX replaced |
| v0.1.x | 2 | Zero-copy manifest in Rust |
| v0.2.0 | 2.5-3 | Unified data layer, experimental compiler |
| v0.3.0 | 4 | Parallel compilation |
| v1.0.0 | All | Production-ready, full feature parity |

---

## File Structure Reference

```
src/dbt_rs/
├── Cargo.toml           # Rust dependencies
├── src/
│   ├── lib.rs           # PyO3 module registration
│   ├── graph.rs         # OxideGraph (Phase 1) 
│   ├── py_graph.rs      # DbtGraph PyO3 wrapper 
│   ├── manifest.rs      # OxideManifest (Phase 2) 
│   ├── py_manifest.rs   # Manifest PyO3 bindings 
│   ├── compiler.rs      # Compiler (Phase 3) 
│   └── py_compiler.rs   # Compiler PyO3 bindings 

core/dbt/
├── graph/
│   ├── graph.py         # Graph wrapper (uses DbtGraph) 
│   └── queue.py         # Uses Rust toposort 
├── compilation.py       # Linker (uses DbtGraph) 
└── parser/
    └── manifest.py      # _sync_manifest_to_rust() 
```

---

## Contributing

See [CONTRIBUTING.md](/CONTRIBUTING.md) for development setup.

**Quick Start:**

```bash
# Build Rust extension
uv run maturin develop

# Run tests
cargo test --no-default-features
uv run pytest tests/unit/ -v

# Verify integration
uv run dbt --version
```
