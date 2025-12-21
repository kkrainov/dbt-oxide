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
| **Phase 0: Foundation** | ✅ Complete | - | Low |
| **Phase 1: Graph Engine** | ✅ Complete | 100x faster toposort | Low |
| **Phase 2: Zero-Copy Manifest** | ✅ Complete (Infra) | Rust manifest storage | Medium |
| **Phase 2.5: Unified Data Layer** | 🔜 Next | Direct graph-from-manifest | Low |
| **Phase 3: Compiler Engine** | 🔲 Planned | 20x faster template rendering | High |
| **Phase 4: Parallelization** | 🔲 Planned | Linear scaling with CPU cores | Medium |

---

## Phase 0: Foundation & Infrastructure ✅

**Status:** Complete

Established the hybrid build environment enabling Python/Rust interoperability.

### Completed Items

- [x] Maturin build configuration
- [x] PyO3 bindings setup
- [x] Rust workspace in `src/dbt_rs/`
- [x] Development environment with `uv`

---

## Phase 1: The Graph Engine ✅

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

## Phase 2: Zero-Copy Manifest ✅

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

## Phase 2.5: Unified Data Layer (NEW) 🔲

**Status:** Planned
**Objective:** Integrate Manifest and Graph in Rust to build the graph directly from manifest dependencies.
**Performance Target:** Eliminate Python graph construction overhead.
**Risk:** Low (builds on existing infrastructure)

> [!IMPORTANT]
> This phase connects the two existing Rust modules (Graph + Manifest) to create a unified data layer where the graph is built directly from manifest dependency data.

### Step 2.5.1: Graph-from-Manifest Builder

- [ ] Add method `OxideManifest::build_graph(&self) -> OxideGraph`
- [ ] Iterate over `manifest.nodes` and extract `depends_on.nodes`
- [ ] Create edges for each dependency relationship
- [ ] Handle edge types (data vs. test dependencies)

**Implementation Plan:**

```rust
// In src/dbt_rs/src/manifest.rs
impl OxideManifest {
    pub fn build_graph(&self) -> OxideGraph {
        let mut graph = OxideGraph::new();
        
        // Add all nodes first
        for unique_id in self.nodes.keys() {
            graph.add_node(unique_id.clone());
        }
        
        // Add edges from depends_on
        for (unique_id, node) in &self.nodes {
            for dep_id in &node.depends_on.nodes {
                graph.add_edge(dep_id, unique_id, None).ok();
            }
        }
        
        graph
    }
}
```

### Step 2.5.2: Python Integration

- [ ] Add `dbt_rs.build_graph_from_manifest() -> DbtGraph` PyO3 function
- [ ] Modify `Linker` to optionally use Rust-built graph
- [ ] Add feature flag `DBT_OXIDE_UNIFIED_GRAPH=1`

### Step 2.5.3: Tests

- [ ] Rust test: `test_build_graph_from_manifest`
- [ ] Rust test: `test_graph_dependencies_match_manifest`
- [ ] Python integration test: Compare Rust-built graph with Python-built graph

### Step 2.5.4: Migration

- [ ] Verify graph parity between Python and Rust construction
- [ ] Replace Python graph construction with Rust
- [ ] Remove Python graph building code

### Expected Benefits

1. **Single source of truth:** Graph and manifest are always in sync
2. **Reduced memory:** No duplicate data structures in Python
3. **Faster initialization:** Graph construction in Rust is ~10x faster
4. **Foundation for Phase 3:** Compiler can query graph + node data without Python

---

## Phase 3: The Compiler Engine 🔲

**Status:** Planned (Weeks 11-16)
**Objective:** Rewrite the Jinja rendering pipeline using `minijinja`.
**Performance Target:** 20x faster template rendering.
**Risk:** High (Jinja compatibility)

> [!CAUTION]
> This is the "Holy Grail" of performance but carries the highest risk due to Jinja2 compatibility requirements. Many dbt projects use advanced Jinja features.

### Step 3.1: The Minijinja Environment

- [ ] Add `minijinja` to Cargo.toml
- [ ] Create `struct Compiler` in Rust
- [ ] Implement the `ref` context function in Rust:

```rust
fn ref(target_name: &str) -> String {
    // Look up target in OxideManifest
    // Return the relation_name (database.schema.alias)
}
```

- [ ] Implement `source` context function (similar to `ref`)
- [ ] Implement `config` context function
- [ ] Implement `var` context function

### Step 3.2: Template Loading

- [ ] Pass raw SQL content from Python to Rust
- [ ] Register all Macros as minijinja templates
- [ ] Parse `macros/*.sql` using regex or dbt-extractor to split into blocks
- [ ] Add macros to `minijinja::Environment`

**Implementation Notes:**

```rust
use minijinja::{Environment, context};

pub struct Compiler {
    env: Environment<'static>,
    manifest: Arc<OxideManifest>,
}

impl Compiler {
    pub fn new(manifest: Arc<OxideManifest>) -> Self {
        let mut env = Environment::new();
        
        // Register ref function
        env.add_function("ref", |name: String| -> String {
            // lookup in manifest.nodes
        });
        
        Self { env, manifest }
    }
    
    pub fn compile(&self, raw_sql: &str) -> Result<String, Error> {
        let template = self.env.template_from_str(raw_sql)?;
        template.render(context!{})
    }
}
```

### Step 3.3: The "Python Trapdoor" (Callback System)

- [ ] Detect macros using Python-specific logic (e.g., `modules.datetime`, `run_query`)
- [ ] Implement Rust callback for unknown functions:

```rust
fn python_trapdoor(macro_name: &str, args: Vec<Value>) -> String {
    Python::with_gil(|py| {
        let jinja_module = py.import("dbt.clients.jinja")?;
        let result = jinja_module.call_method1(
            "render_macro_in_python",
            (macro_name, args)
        )?;
        result.extract::<String>()
    })
}
```

- [ ] Return string result to minijinja
- [ ] Log/track trapdoor usage for optimization

### Step 3.4: Release v0.2

- [ ] Create opt-in flag: `DBT_OXIDE_COMPILER=1`
- [ ] Document known limitations
- [ ] Run compatibility tests against dbt-core test suite
- [ ] Performance benchmark against Python Jinja2

### Verification Checklist

- [ ] All models compile with identical SQL output
- [ ] Macros with refs/sources work correctly
- [ ] Custom macros work via Python trapdoor
- [ ] No regressions in existing tests

---

## Phase 4: Parallelization & Introspection 🔲

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
│   ├── graph.rs         # OxideGraph (Phase 1) ✅
│   ├── py_graph.rs      # DbtGraph PyO3 wrapper ✅
│   ├── manifest.rs      # OxideManifest (Phase 2) ✅
│   ├── py_manifest.rs   # Manifest PyO3 bindings ✅
│   ├── compiler.rs      # Compiler (Phase 3) 🔲
│   └── py_compiler.rs   # Compiler PyO3 bindings 🔲

core/dbt/
├── graph/
│   ├── graph.py         # Graph wrapper (uses DbtGraph) ✅
│   └── queue.py         # Uses Rust toposort ✅
├── compilation.py       # Linker (uses DbtGraph) ✅
└── parser/
    └── manifest.py      # _sync_manifest_to_rust() ✅
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
