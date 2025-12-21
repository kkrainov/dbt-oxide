# Phase 2: Zero-Copy Manifest - Implementation Plan

**Objective:** Efficiently share the complete Manifest state between Python and Rust without serialization overhead on every call.

**Performance Target:** Near-instant context access for Rust components.

**Risk:** Medium-High (Requires strict schema alignment with full Python parity).

**Profiling Insight:** Phase 1 profiling showed serialization accounts for **60% of parse time** (~10s). This phase directly targets that bottleneck.

---

## Core Requirements

> [!IMPORTANT]
> **Full Python Parity Required**
> 
> The Rust manifest must be fully compatible with the Python Manifest, including:
> - All fields with exact type mapping
> - All method signatures (for wrapped access)
> - All unit tests pass **without modifying test assertions**

### Key Decisions Made

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Sync Trigger** | Eager (at `ManifestLoader.load()`) | Predictable timing, fail-fast, no hidden latency |
| **Error Handling** | Strict (raise exception) | Immediate failure detection, clear debugging |
| **Field Coverage** | Full parity | All tests pass unchanged |

---

## Why Python Builds First (Staged Architecture)

> [!NOTE]
> **This is a stepping-stone architecture.** Phase 2 keeps Python parsing to minimize risk while establishing Rust data structures. Phase 3 will move parsing entirely to Rust.

### Current Reality: What Python's `_build_manifest()` Does

```
┌─────────────────────────────────────────────────────────────────┐
│               ManifestLoader._build_manifest()                  │
│                                                                 │
│  1. Read files from disk (SQL, YAML, properties)    ~1.5s      │
│  2. Parse Jinja templates (extract refs/sources)    ~2.0s      │
│  3. Resolve dependencies between nodes              ~0.2s      │
│  4. Construct Python node objects                   ~0.5s      │
│  5. Serialize to manifest.json (mashumaro)          ~11.0s ◄── │
│                                                                 │
│  Total: ~15-19 seconds                                         │
└─────────────────────────────────────────────────────────────────┘
```

### Phase 2 Goal: Eliminate Repeated Serialization

Phase 2 does NOT move parsing to Rust. Instead, it:
1. **Keeps Python parsing** (steps 1-4 unchanged)
2. **One-time sync** to Rust after build (~100-200ms overhead)
3. **Eliminates repeated serialization** during subsequent access

**Why this matters:** During `dbt compile` and `dbt run`, the manifest is accessed thousands of times. Without Phase 2, each access may trigger Python dict lookups. With Phase 2, Rust provides O(1) HashMap access.

### The Roadmap to Full Rust Parsing

```
Phase 2 (This Plan)           Phase 3 (Compiler)           Phase 4+ (Full Rust)
──────────────────────        ─────────────────────        ─────────────────────
Python: Read files            Python: Read files           Rust: Read files
Python: Parse Jinja      →    Rust: Parse minijinja   →    Rust: Parse minijinja
Python: Build Manifest        Rust: Build Manifest         Rust: Build Manifest
Python → Rust: Sync           (No sync needed)             (No Python involved)
Rust: Store & Access          Rust: Store & Access         Rust: Store & Access
```

| Phase | Parsing | Storage | Risk | Expected Speedup |
|-------|---------|---------|------|------------------|
| **2** | Python | Rust | Low | 1-2s (reduce repeated access) |
| **3** | Rust (Jinja) | Rust | Medium | 5-8s (eliminate Jinja2) |
| **4** | Rust (All) | Rust | High | 10-15s (full Rust pipeline) |

### Why Not Skip to Phase 3?

Moving parsing to Rust requires:
1. **Replacing Jinja2 with minijinja** - breaks all template tests
2. **Rewriting file parsers** - YAML, SQL extraction
3. **Changing how refs/sources resolve** - core dbt logic

Phase 2 is **low-risk preparation**:
- Establishes `OxideManifest` schema (reused in Phase 3)
- Validates Rust can hold all manifest data
- All Python tests pass unchanged
- Creates foundation for incremental migration

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Python (dbt-core)                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Manifest (Python Dataclass)                │   │
│  │  nodes: Dict[str, ManifestNode]                         │   │
│  │  sources: Dict[str, SourceDefinition]                   │   │
│  │  macros: Dict[str, Macro]                               │   │
│  │  ... (all 20+ fields)                                   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           │                                     │
│            ManifestLoader.load() completes                      │
│                           │                                     │
│                           ▼ One-time sync (eager)               │
│               manifest.writable_manifest().to_json()            │
│                           │                                     │
│                           ▼ dbt_rs.load_manifest(json)          │
│                           │                                     │
│        ┌──────────────────┴───────────────────┐                │
│        │     DbtRuntimeError on failure       │                │
│        └──────────────────────────────────────┘                │
└───────────────────────────┼─────────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────┼─────────────────────────────────────┐
│                     Rust (dbt_rs)                               │
│                           │                                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │           OxideManifest (Rust Struct)                   │   │
│  │  Full field parity with Python Manifest                 │   │
│  │                                                          │   │
│  │  Stored in: OnceCell<RwLock<OxideManifest>>             │   │
│  └─────────────────────────────────────────────────────────┘   │
│                           │                                     │
│                           ▼ Zero-copy access                    │
│               Fast lookups, graph operations                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Python Manifest Fields (Full Parity)

Based on analysis of `core/dbt/contracts/graph/manifest.py`:

### Primary Data Fields

```python
@dataclass
class Manifest:
    # Node collections (must map completely)
    nodes: MutableMapping[str, ManifestNode]           # ~8000 nodes in large projects
    sources: MutableMapping[str, SourceDefinition] 
    macros: MutableMapping[str, Macro]
    docs: MutableMapping[str, Documentation]
    exposures: MutableMapping[str, Exposure]
    metrics: MutableMapping[str, Metric]
    groups: MutableMapping[str, Group]
    selectors: MutableMapping[str, Any]
    files: MutableMapping[str, AnySourceFile]
    disabled: MutableMapping[str, List[GraphMemberNode]]
    env_vars: MutableMapping[str, str]
    semantic_models: MutableMapping[str, SemanticModel]
    unit_tests: MutableMapping[str, UnitTestDefinition]
    saved_queries: MutableMapping[str, SavedQuery]
    fixtures: MutableMapping[str, UnitTestFileFixture]
    
    # Metadata
    metadata: ManifestMetadata
    flat_graph: Dict[str, Any]
    state_check: ManifestStateCheck
    source_patches: MutableMapping[SourceKey, SourcePatch]
```

### Lookup Classes (Lazy Initialized, Not Serialized)

These are **NOT** serialized to Rust - they are Python-side caching mechanisms:

- `DocLookup`, `SourceLookup`, `RefableLookup`
- `MetricLookup`, `SavedQueryLookup`, `SemanticModelByMeasureLookup`
- `DisabledLookup`, `AnalysisLookup`, `SingularTestLookup`

### ManifestNode Types (All Must Be Supported)

| Node Type | Python Class | Key Fields |
|-----------|--------------|------------|
| Model | `ModelNode` | unique_id, name, resource_type, fqn, package_name, depends_on, config, raw_code, compiled_code, database, schema, alias |
| Test | `GenericTestNode`, `SingularTestNode` | test_metadata, attached_node |
| Seed | `SeedNode` | checksum, root_path |
| Snapshot | `SnapshotNode` | strategy |
| Analysis | `AnalysisNode` | - |
| Hook | `HookNode` | index |
| Source | `SourceDefinition` | source_name, source_description, tables |
| Macro | `Macro` | macro_sql, arguments |
| Exposure | `Exposure` | type, maturity, owner |

---

## Sub-Phase 2.1: Rust Manifest Schema (Full Parity)

### Step 2.1.1: Add Dependencies

#### [MODIFY] [Cargo.toml](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/src/dbt_rs/Cargo.toml)

```toml
[dependencies]
pyo3 = { version = "0.20" }
petgraph = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
once_cell = "1.19"
```

---

### Step 2.1.2: Create Manifest Module with Full Schema

#### [NEW] [manifest.rs](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/src/dbt_rs/src/manifest.rs)

The schema must mirror the Python `WritableManifest` exactly since that's what gets serialized.

**Strategy:** Use `serde(default)` extensively to handle missing optional fields gracefully.

---

### Step 2.1.3: Implement Rust Structs

Full schema design (~500 LOC expected) covering:

1. **Core Data Structures:**
   - `OxideManifest` - top level
   - `OxideNode` - all ManifestNode types via tagged union
   - `OxideSource` - SourceDefinition
   - `OxideMacro` - Macro
   - `OxideExposure`, `OxideMetric`, `OxideGroup`
   - `OxideSemanticModel`, `OxideSavedQuery`, `OxideUnitTest`

2. **Supporting Structures:**
   - `OxideDependsOn` - nodes/macros dependencies
   - `OxideNodeConfig` - materialized, enabled, etc.
   - `OxideFileHash` - checksum
   - `OxideManifestMetadata` - dbt_version, adapter_type, etc.

---

## Sub-Phase 2.2: Python Integration

### Step 2.2.1: Eager Sync in ManifestLoader

#### [MODIFY] [manifest.py](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/core/dbt/parser/manifest.py)

Add sync call at end of `load()` method:

```python
def load(self) -> Manifest:
    # ... existing load logic ...
    
    # Sync to Rust (eager, fail-fast)
    self._sync_manifest_to_rust(manifest)
    
    return manifest

def _sync_manifest_to_rust(self, manifest: Manifest) -> None:
    """Serialize manifest to Rust for zero-copy access."""
    import dbt_rs
    
    try:
        writable = manifest.writable_manifest()
        json_str = writable.to_json()  # Uses mashumaro serialization
        dbt_rs.load_manifest(json_str)
    except Exception as e:
        raise DbtRuntimeError(
            f"Failed to sync manifest to Rust engine: {e}"
        ) from e
```

### Step 2.2.2: Strict Error Handling

On Rust parse failure, raise `DbtRuntimeError` immediately. No graceful degradation - if Rust can't parse the manifest, it indicates a schema mismatch that must be fixed.

---

## Sub-Phase 2.3: Performance Optimization Checkpoint

### Step 2.3.1: Benchmark JSON Parsing

After implementation, measure deserialization time:

```rust
pub fn load_manifest(json_string: &str) -> PyResult<()> {
    use std::time::Instant;
    let start = Instant::now();
    
    let manifest = OxideManifest::from_json_str(json_string)?;
    
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 500 {
        eprintln!(
            "dbt-oxide WARNING: Manifest load took {}ms (threshold: 500ms)",
            elapsed.as_millis()
        );
    }
    
    // ... store manifest
}
```

> [!NOTE]
> If JSON parsing exceeds 500ms for 10k nodes, investigate `rkyv` for zero-copy deserialization in Phase 2.5.

---

## Verification Plan

### Automated Tests

#### 1. Rust Unit Tests

```bash
cd src/dbt_rs
cargo test --no-default-features
```

**Tests to implement:**
- `test_parse_empty_manifest`
- `test_parse_full_manifest_sample`
- `test_all_node_types_parse`
- `test_missing_optional_fields`
- `test_invalid_json_error`

#### 2. Existing Python Unit Tests (MUST PASS UNCHANGED)

```bash
uv run pytest tests/unit/parser/test_manifest.py -v
```

**Key tests:**
- `test_partial_parse_file_path`
- `test_profile_hash_change`
- `test_partial_parse_by_version`
- `test_write_perf_info`
- `test_reset`

#### 3. Full Test Suite

```bash
uv run pytest tests/unit/ -v --tb=short
```

**Success Criteria:** All existing tests pass without modification to assertions.

### Manual Verification

#### 1. Parse Command Test

```bash
cd performance/projects/01_2000_simple_models
uv run dbt parse --no-version-check --profiles-dir ../../project_config/
```

**Verify:**
- No errors during parse
- Log shows "Manifest synced to Rust in Xms"
- Output is identical to non-Rust version

#### 2. Performance Benchmark

```bash
hyperfine --warmup 1 --runs 3 \
  'uv run dbt parse --no-version-check --profiles-dir ../../project_config/'
```

**Compare against baseline:** ~18.8s (Phase 1)

---

## Files Changed Summary

| File | Action | Description |
|------|--------|-------------|
| `src/dbt_rs/Cargo.toml` | MODIFY | Add serde, serde_json, once_cell |
| `src/dbt_rs/src/manifest.rs` | NEW | Full OxideManifest schema (~500 LOC) |
| `src/dbt_rs/src/py_manifest.rs` | NEW | PyO3 bindings (~100 LOC) |
| `src/dbt_rs/src/lib.rs` | MODIFY | Register manifest module |
| `core/dbt/parser/manifest.py` | MODIFY | Add `_sync_manifest_to_rust()` |

---

## Success Criteria

1. ✅ All existing unit tests pass **without assertion changes**
2. ✅ Rust can parse a 2000-node manifest JSON in <500ms
3. ✅ `dbt parse` completes successfully with Rust sync enabled
4. ✅ Schema errors raise `DbtRuntimeError` immediately
5. ✅ Performance baseline established for Phase 3

---

## Implementation Checklist

### Step 1: Rust Dependencies
- [x] Add `serde = { version = "1.0", features = ["derive"] }` to Cargo.toml
- [x] Add `serde_json = "1.0"` to Cargo.toml
- [x] Add `once_cell = "1.19"` to Cargo.toml
- [x] Run `cargo build` to verify dependencies resolve

### Step 2: Rust Manifest Schema (Core Structures)
- [x] Create `src/dbt_rs/src/manifest.rs`
- [x] Implement `OxideDependsOn` struct
- [x] Implement `OxideNodeConfig` struct
- [x] Implement `OxideNode` struct (all fields from ManifestNode)
- [x] Implement `OxideManifest` struct (nodes, sources, macros, exposures)
- [x] Implement `OxideManifest::from_json_str()` method
- [x] Add `mod manifest;` to lib.rs

### Step 3: Rust Manifest Schema (Additional Types)
- [x] Implement `OxideSource` struct
- [x] Implement `OxideMacro` struct
- [x] Implement `OxideExposure` struct
- [x] Implement `OxideMetric` struct
- [x] Implement `OxideGroup` struct
- [x] Implement `OxideSemanticModel` struct
- [x] Implement `OxideSavedQuery` struct
- [x] Implement `OxideUnitTest` struct
- [x] Implement `OxideManifestMetadata` struct

### Step 4: Rust Unit Tests (TDD)
- [x] Test: `test_parse_empty_manifest`
- [x] Test: `test_parse_single_node`
- [x] Test: `test_parse_with_missing_optional_fields`
- [x] Test: `test_parse_all_node_types`
- [x] Test: `test_invalid_json_returns_error`
- [x] Run `cargo test --no-default-features` - all pass

### Step 5: PyO3 Bindings
- [x] Create `src/dbt_rs/src/py_manifest.rs`
- [x] Implement `load_manifest(json_string: &str)` function
- [x] Implement `get_node_count()` function
- [x] Implement `get_node_dependencies(unique_id: &str)` function
- [x] Add global `OnceCell<RwLock<OxideManifest>>` storage
- [x] Implement `register_manifest_module()` function
- [x] Add `mod py_manifest;` to lib.rs
- [x] Register manifest module in `#[pymodule]` function

### Step 6: Build & Verify Rust
- [x] Run `maturin develop` to build Python extension
- [x] Run `uv run python -c "import dbt_rs; print(dir(dbt_rs))"` - verify new functions
- [x] Run `cargo test --no-default-features` - all tests pass

### Step 7: Python Integration
- [x] Add `_sync_manifest_to_rust()` method to `ManifestLoader`
- [x] Call sync at end of `ManifestLoader.load()` method
- [x] Add `DbtRuntimeError` on sync failure
- [x] Add timing log: "Manifest synced to Rust in Xms"

### Step 8: Python Tests
- [x] Run `uv run pytest tests/unit/parser/test_manifest.py -v` - all pass
- [x] Run `uv run pytest tests/unit/ -v --tb=short` - all pass
- [x] Verify no test assertions were modified

### Step 9: Integration Testing
- [x] Run `uv run dbt parse --no-version-check` on 2000-model project
- [x] Verify "Manifest synced to Rust" log appears
- [x] Verify no errors during parse
- [x] Verify manifest.json output is identical

### Step 10: Performance Benchmark
- [x] Run `hyperfine` benchmark on parse command
- [x] Record baseline time (expected: ~18.8s)
- [x] Measure Rust sync overhead (target: <200ms)
- [x] Document results in walkthrough

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Schema drift | Use `#[serde(deny_unknown_fields)]` in development to catch new fields |
| Large JSON string | Consider streaming JSON parser if memory becomes issue |
| Thread safety | RwLock allows concurrent reads, single writer |
| Circular references | JSON doesn't support references; full copies expected |

---

## Completion Status

> [!IMPORTANT]
> **Phase 2 Status: Infrastructure Complete**
> 
> As of 2025-12-21, Phase 2 infrastructure is fully implemented:
> - ✅ Rust manifest schema (`OxideManifest`) with all node types
> - ✅ Python sync at end of `ManifestLoader.load()`
> - ✅ Rust storage in `OnceCell<RwLock<OxideManifest>>`
> - ✅ Performance baseline established (~18.8s for 2000 models)
>
> **What's NOT implemented:** The Rust manifest is currently **stored but not queried**. The PyO3 functions `get_node_count()` and `get_node_dependencies()` exist but are not called from Python.

### Architectural Decision: Proceed to Phase 2.5

Rather than adding temporary Python→Rust manifest lookups (which would be discarded), we proceed directly to **Phase 2.5: Unified Data Layer**, which will:

1. Build `OxideGraph` directly from `OxideManifest` in Rust
2. Eliminate Python graph construction entirely
3. Provide a foundation for Phase 3 (Compiler) where Rust needs both graph and manifest

**Rationale:**
- Avoids throwaway integration code
- Single FFI call replaces many individual lookups
- Memory efficiency via shared references in Rust
- Aligns with Strangler Fig pattern (replace subsystems, not add layers)

See [rust_migration.md](../roadmap/rust_migration.md#phase-25-unified-data-layer-new-) for Phase 2.5 implementation details.
