# Phase 3.2: OxideManifest Full Alignment & Python Manifest Replacement

> **Goal:** Achieve full parity between OxideManifest and Python Manifest using TDD, then replace Python Manifest entirely.

## Status: 🟢 IN PROGRESS

---

## Test Layer Legend

| Icon | Layer | Location |
|------|-------|----------|
| 🦀 | Rust Unit Test | `src/dbt_rs/src/tests/*.rs` |
| 🔗 | PyO3 Binding | `src/dbt_rs/src/py_manifest.rs` |
| 🐍 | Python Test | `tests/unit/test_rust_manifest_builder.py` |

---

## Phase Overview

| Phase | Description | Status |
|-------|-------------|--------|
| 3.2.1 | Core Lookup Methods (resolve_ref, resolve_source, resolve_doc) | ✅ |
| 3.2.2 | Resource Type Support | ✅ |
| 3.2.3 | Macro & Materialization | ✅ |
| 3.2.4 | Disabled Node Support | ⬜ |
| 3.2.5 | Additional Methods (find_generate_macros, deepcopy) | ⬜ |
| 3.2.6 | Serialization & Write | ⬜ |
| 3.2.7 | Integration & Migration | ⬜ |
| 3.2.8 | Python Manifest Removal | ⬜ |
| **3.3** | **Full Macro Class Replacement** | ⬜ |

---

## Phase 3.2.1: Core Lookup Methods (TDD)

> Critical: `resolve_ref`, `resolve_source`, and `resolve_doc` need full implementation and tests

### Step 1: 🦀 Rust TDD

**RED Phase - Write Failing Rust Tests First**
- [x] **T.R.3.2.1.1**: `test_resolve_ref_by_name` (RED) ✅
- [x] **T.R.3.2.1.2**: `test_resolve_ref_by_name_and_package` (RED) ✅
- [x] **T.R.3.2.1.3**: `test_resolve_ref_by_name_package_version` (RED) ✅
- [x] **T.R.3.2.1.4**: `test_resolve_ref_not_found` (RED) ✅
- [x] **T.R.3.2.1.5**: `test_resolve_ref_multiple_matches` (RED) ✅
- [x] **T.R.3.2.1.6**: `test_resolve_source_simple` (RED) ✅
- [x] **T.R.3.2.1.7**: `test_resolve_source_not_found` (RED) ✅
- [x] **T.R.3.2.1.8**: `test_resolve_doc_by_name` (RED) ✅
- [x] **T.R.3.2.1.9**: `test_resolve_doc_not_found` (RED) ✅

**GREEN Phase - Make Rust Tests Pass**
- [x] **C.R.3.2.1.1**: Verify/fix `resolve_ref()` to pass tests (GREEN) ✅
- [x] **C.R.3.2.1.2**: Verify/fix `resolve_source()` to pass tests (GREEN) ✅
- [x] **C.R.3.2.1.3**: Verify/fix `resolve_doc()` to pass tests (GREEN) ✅

**Rust Verification**
- [x] **V.R.3.2.1.1**: `cargo test --no-default-features` all pass ✅

**REFACTOR Phase - Return Full Nodes Instead of unique_ids (RED → GREEN)**
- [x] **T.R.3.2.1.10**: Update tests to expect node references (RED) ✅
- [x] **C.R.3.2.1.4**: Change `resolve_ref` to return `Option<&OxideNode>` (GREEN) ✅
- [x] **C.R.3.2.1.5**: Change `resolve_source` to return `Option<&OxideSource>` (GREEN) ✅
- [x] **C.R.3.2.1.6**: Change `resolve_doc` to return `Option<&serde_json::Value>` (GREEN) ✅
- [x] **V.R.3.2.1.2**: Rust tests pass with new return types ✅

---

### Step 2: 🔗 PyO3 Bindings

- [x] **C.P.3.2.1.1**: `resolve_ref()` exposed in PyOxideManifest ✅
- [x] **C.P.3.2.1.2**: `resolve_source()` exposed in PyOxideManifest ✅
- [x] **C.P.3.2.1.3**: `resolve_doc()` exposed in PyOxideManifest ✅

---

### Step 3: 🐍 Python TDD

**RED Phase - Write Failing Python Tests First**
- [ ] **T.Y.3.2.1.1**: `test_resolve_ref_by_name` (RED)
- [ ] **T.Y.3.2.1.2**: `test_resolve_ref_by_name_and_package` (RED)
- [ ] **T.Y.3.2.1.3**: `test_resolve_ref_not_found` (RED)
- [ ] **T.Y.3.2.1.4**: `test_resolve_source_simple` (RED)
- [ ] **T.Y.3.2.1.5**: `test_resolve_source_not_found` (RED)

**GREEN Phase - Make Python Tests Pass**
- [ ] **C.Y.3.2.1.1**: Update `oxide_manifest.py` wrapper if needed (GREEN)

**Python Verification**
- [ ] **V.Y.3.2.1.1**: `uv run pytest test_rust_manifest_builder.py` all pass

---

### Phase 3.2.1 Final Verification

- [x] **V.3.2.1.1**: All Python tests pass (45/45 resolve tests) ✅

**Phase 3.2.1 Status: ✅ COMPLETE**

---

## Phase 3.2.2: Resource Type Support (TDD)

> Current tests only cover models. Need seeds, snapshots, tests, exposures, metrics.

### Step 1: 🦀 Rust TDD

**RED Phase**
- [ ] **T.R.3.2.2.1**: `test_manifest_with_seeds` (RED)
- [ ] **T.R.3.2.2.2**: `test_manifest_with_snapshots` (RED)
- [ ] **T.R.3.2.2.3**: `test_manifest_with_generic_tests` (RED)
- [ ] **T.R.3.2.2.4**: `test_manifest_with_singular_tests` (RED)
- [ ] **T.R.3.2.2.5**: `test_manifest_with_exposures` (RED)
- [ ] **T.R.3.2.2.6**: `test_manifest_with_metrics` (RED)
- [ ] **T.R.3.2.2.7**: `test_manifest_with_semantic_models` (RED)
- [ ] **T.R.3.2.2.8**: `test_build_parent_map_mixed_types` (RED)
- [ ] **T.R.3.2.2.9**: `test_build_child_map_mixed_types` (RED)

**GREEN Phase**
- [ ] **C.R.3.2.2.1**: Fix any deserialization issues (GREEN)

**Rust Verification**
- [ ] **V.R.3.2.2.1**: All Rust tests pass

---

### Step 2: 🐍 Python TDD

**RED Phase**
- [ ] **T.Y.3.2.2.1**: `test_manifest_with_seeds` (RED)
- [ ] **T.Y.3.2.2.2**: `test_build_maps_mixed_resource_types` (RED)

**GREEN Phase**
- [ ] **C.Y.3.2.2.1**: Update wrapper if needed (GREEN)

**Python Verification**
- [ ] **V.Y.3.2.2.1**: All Python tests pass

---

## Phase 3.2.3: Macro & Materialization Lookup (TDD) - FULL RUST IMPLEMENTATION

> **Strategy**: Replace entire `MacroMethods` mixin (225 lines) with Rust implementation.
> External dependencies (`get_adapter_package_names`, `get_adapter_type_names`, `get_flags`) passed from Python.

### Architecture Overview

**Python Classes to Replace in Rust:**
- `Locality` enum (Core/Imported/Root)
- `MacroCandidate` dataclass
- `MaterializationCandidate` dataclass  
- `CandidateList` sorted list
- `MacroMethods` mixin (all methods)
- `_get_locality()` function

**External Dependencies (Python → Rust as parameters):**
- `get_adapter_package_names(adapter_type)` → `HashSet<String>`
- `get_adapter_type_names(adapter_type)` → `Vec<String>` (adapter inheritance chain)
- `get_flags().require_explicit_package_overrides...` → `bool`

**Design Decisions:**
- ✅ Return `unique_id` only (Python fetches full Macro object)
- ✅ Deprecation warnings stay in Python wrapper
- ✅ Lazy build `_macros_by_name` index (match Python behavior)

---

### 🔴 RUST RED PHASE: Write Failing Tests First

#### New Test File: `src/dbt_rs/src/tests/macro_lookup.rs`

**Locality Tests:**
- [x] **T.R.3.2.3.1**: `test_locality_core` - dbt packages return Core
- [x] **T.R.3.2.3.2**: `test_locality_root` - root project returns Root
- [x] **T.R.3.2.3.3**: `test_locality_imported` - dependencies return Imported

**find_macro_by_name Tests:**
- [x] **T.R.3.2.3.4**: `test_find_macro_empty_manifest` - returns None
- [x] **T.R.3.2.3.5**: `test_find_macro_single` - finds single macro
- [x] **T.R.3.2.3.6**: `test_find_macro_priority_root_over_imported`
- [x] **T.R.3.2.3.7**: `test_find_macro_priority_imported_over_core`
- [x] **T.R.3.2.3.8**: `test_find_macro_with_package_filter`

**find_materialization_macro_by_name Tests:**
- [x] **T.R.3.2.3.9**: `test_materialization_macro_name_default`
- [x] **T.R.3.2.3.10**: `test_materialization_macro_name_adapter`
- [x] **T.R.3.2.3.11**: `test_find_materialization_empty`
- [x] **T.R.3.2.3.12**: `test_find_materialization_default_fallback`
- [x] **T.R.3.2.3.13**: `test_find_materialization_adapter_specific`
- [x] **T.R.3.2.3.14**: `test_find_materialization_core_protection`
- [x] **T.R.3.2.3.15**: `test_find_materialization_legacy_mode`

**RED Verification:**
- [x] **V.R.RED.3.2.3**: Run `cargo test --no-default-features macro_lookup` - expect compile errors or test failures

---

### 🟢 RUST GREEN PHASE: Implement to Pass Tests

#### New File: `src/dbt_rs/src/manifest/macro_lookup.rs`

- [x] **C.R.3.2.3.1**: Create `Locality` enum with `Ord` derive
- [x] **C.R.3.2.3.2**: Create `MacroCandidate` struct
- [x] **C.R.3.2.3.3**: Create `MaterializationCandidate` struct with specificity
- [x] **C.R.3.2.3.4**: Implement `get_locality()` function
- [x] **C.R.3.2.3.5**: Implement `get_materialization_macro_name()` function
- [x] **C.R.3.2.3.6**: Implement `find_macro_by_name()` in OxideManifest
- [x] **C.R.3.3.3.7**: Implement `find_materialization_macro_by_name()` in OxideManifest
- [x] **C.R.3.2.3.8**: Implement `apply_core_protection()` for builtin override logic

**GREEN Verification:**
- [x] **V.R.GREEN.3.2.3**: Run `cargo test --no-default-features macro_lookup` - all 15 tests pass

---

### 🔗 PyO3 Bindings (`src/dbt_rs/src/py_manifest.rs`)

- [x] **C.P.3.2.3.1**: Expose `find_macro_by_name(name, root_project, internal_packages, package)`
- [x] **C.P.3.2.3.2**: Expose `find_materialization_macro_by_name(project, name, adapter_types, internal_packages, allow_override)`

---

### 🔴 PYTHON RED PHASE: Verify Tests Fail Before Wrappers

Run existing tests against OxideManifest (should fail - methods not yet wired):
- [ ] **T.Y.RED.3.2.3.1**: `test_find_macro_by_name` - expect AttributeError
- [ ] **T.Y.RED.3.2.3.2**: `test_find_materialization_by_name` - expect AttributeError

---

### 🟢 PYTHON GREEN PHASE: Add Wrappers

#### Modify: `core/dbt/contracts/graph/oxide_manifest.py`

- [x] **C.Y.3.2.3.1**: Add `find_macro_by_name()` wrapper (calls adapter factory, delegates to Rust)
- [x] **C.Y.3.2.3.2**: Add `find_materialization_macro_by_name()` wrapper (handles deprecation warnings)

**GREEN Verification:**
- [x] **V.Y.GREEN.3.2.3.1**: `test_find_macro_by_name` (8 scenarios) - all pass
- [x] **V.Y.GREEN.3.2.3.2**: `test_find_materialization_by_name` (21 scenarios) - all pass  
- [x] **V.Y.GREEN.3.2.3.3**: `test_find_materialization_by_name_legacy` (21 scenarios) - all pass

---

### ✅ Final Verification

- [x] **V.3.2.3.1**: All 15 Rust unit tests pass
- [x] **V.3.2.3.2**: All 5 Python macro lookup tests pass
- [x] **V.3.2.3.3**: Code quality (`cargo fmt`, `cargo clippy`)

> **Implementation Note:** Phase 3.2.3 uses Path 2 (hybrid approach):
> - Rust `get_macro()` returns dict via `pythonize`
> - Python wrapper converts to `Macro` using `Macro.from_dict()`
> - Full Macro class replacement deferred to Phase 3.3

---

## Phase 3.2.4: Disabled Node Support (TDD)

> Python has extensive disabled node lookup tests (14+)

### 🦀 Rust Implementation

- [ ] **C.R.3.2.4.1**: Implement `find_disabled_by_name()` in OxideManifest

### 🦀 Rust Tests

- [ ] **T.R.3.2.4.1**: `test_disabled_lookup_by_name`
- [ ] **T.R.3.2.4.2**: `test_disabled_lookup_by_name_and_package`
- [ ] **T.R.3.2.4.3**: `test_disabled_lookup_not_found`
- [ ] **T.R.3.2.4.4**: `test_disabled_lookup_with_version`
- [ ] **T.R.3.2.4.5**: `test_disabled_lookup_multiple_matches`

### 🔗 PyO3 Bindings

- [ ] **C.P.3.2.4.1**: Expose `find_disabled_by_name()` in PyOxideManifest

### 🐍 Python Wrapper

- [ ] **C.Y.3.2.4.1**: Add `find_disabled_by_name()` wrapper method

### 🐍 Python Tests

- [ ] **T.Y.3.2.4.1**: `test_disabled_lookup_by_name`

### Verification

- [ ] **V.3.2.4.1**: All tests pass
- [ ] **V.3.2.4.2**: Matches Python behavior

---

## Phase 3.2.5: Additional Methods (TDD)

> Methods discovered from test failures: `find_generate_macros_by_name`
> 
> Note: `deepcopy()` removed - zero production callers, test-only method

### 🦀 Rust Tests

- [ ] **T.R.3.2.5.1**: `test_find_generate_macros_by_name` - find generate() macros

### 🔗 PyO3 Bindings

- [ ] **C.P.3.2.5.1**: Expose `find_generate_macros_by_name()` in PyOxideManifest

### 🐍 Python Wrapper

- [ ] **C.Y.3.2.5.1**: Add `find_generate_macros_by_name()` wrapper method

### Final Verification

- [ ] **V.3.2.5.1**: All related Python tests pass

---

## Phase 3.2.6: Serialization & Write (TDD)

> Manifest needs to write back to JSON format identical to Python

### 🦀 Rust Tests

- [ ] **T.R.3.2.5.1**: `test_write_manifest_to_file`
- [ ] **T.R.3.2.5.2**: `test_serialize_round_trip`
- [ ] **T.R.3.2.5.3**: `test_serialize_matches_python_format`

### 🔗 PyO3 Bindings

- [ ] **C.P.3.2.5.1**: Expose `to_json()` in PyOxideManifest
- [ ] **C.P.3.2.5.2**: Expose `write()` in PyOxideManifest

### 🐍 Python Wrapper

- [ ] **C.Y.3.2.5.1**: Add `to_json()` wrapper method
- [ ] **C.Y.3.2.5.2**: Add `write()` wrapper method

### 🐍 Python Tests

- [ ] **T.Y.3.2.5.1**: `test_write_manifest_to_file`
- [ ] **T.Y.3.2.5.2**: `test_round_trip_serialization`

### Verification

- [ ] **V.3.2.5.1**: Round-trip deserialization/serialization preserves data
- [ ] **V.3.2.5.2**: Output JSON matches Python format

---

## Phase 3.2.6: Integration & Migration

> Replace usage of Python Manifest with OxideManifest in dbt commands

### File Migration (by dependency order)

- [ ] **C.3.2.6.1**: Update `core/dbt/graph/selector.py`
- [ ] **C.3.2.6.2**: Update `core/dbt/compilation.py`
- [ ] **C.3.2.6.3**: Update `core/dbt/task/runnable.py`
- [ ] **C.3.2.6.4**: Update `core/dbt/task/run.py`
- [ ] **C.3.2.6.5**: Update `core/dbt/parser/manifest.py`
- [ ] **C.3.2.6.6**: Update remaining 30 files

### Integration Tests

- [ ] **V.3.2.6.1**: `dbt parse` works with OxideManifest
- [ ] **V.3.2.6.2**: `dbt run` works with OxideManifest
- [ ] **V.3.2.6.3**: `dbt ls` works with OxideManifest
- [ ] **V.3.2.6.4**: All functional tests pass

---

## Phase 3.2.7: Python Manifest Removal

> Final step: Remove Python Manifest class entirely

### Pre-removal Checklist

- [ ] **V.3.2.7.1**: All 75+ Python Manifest tests pass with OxideManifest
- [ ] **V.3.2.7.2**: All imports updated
- [ ] **V.3.2.7.3**: `DBT_USE_RUST_MANIFEST=1` is default
- [ ] **V.3.2.7.4**: Performance benchmarks pass

### Removal Steps

- [ ] **C.3.2.7.1**: Delete `get_manifest_class()` factory
- [ ] **C.3.2.7.2**: Rename `OxideManifest` → `Manifest`
- [ ] **C.3.2.7.3**: Delete `core/dbt/contracts/graph/manifest.py` (1844 lines)
- [ ] **C.3.2.7.4**: Update all imports
- [ ] **C.3.2.7.5**: Remove Python Manifest tests (keep OxideManifest tests)

### Final Verification

- [ ] **V.3.2.7.5**: Full test suite passes
- [ ] **V.3.2.7.6**: CI pipeline green
- [ ] **V.3.2.7.7**: Performance regression tests pass

---

## Naming Convention

| Prefix | Meaning |
|--------|---------|
| T.R.* | 🦀 Rust Test |
| T.Y.* | 🐍 Python Test |
| C.R.* | 🦀 Rust Implementation |
| C.P.* | 🔗 PyO3 Binding |
| C.Y.* | 🐍 Python Wrapper |
| V.* | Verification |

---

## Dependencies

- Phase 3.1.1 (Rust Manifest Serialization) ✅ COMPLETE
- PyO3 bindings established ✅
- OxideManifest wrapper pattern ✅

---

## Phase 3.3: Full Macro Class Replacement (Future)

> **Goal:** Replace Python `Macro` class entirely with Rust-backed `OxideMacro` exposed via PyO3.

### Current State (Path 2 - Hybrid)

- Rust `get_macro()` returns dict via `pythonize`
- Python wrapper converts to `Macro` using `Macro.from_dict()`
- Full compatibility but extra serialization overhead

### Target State (Path 1 - Full Rust)

- `OxideMacro` exposed as `#[pyclass]` with all fields
- All nested types (`MacroDependsOn`, `Docs`, `MacroArgument`) also `#[pyclass]`
- Production code imports `from dbt_rs import OxideMacro as Macro`
- Zero Python object construction overhead

### Implementation Steps

- [ ] **C.R.3.3.1**: Add `#[pyo3(get)]` to all OxideMacro fields (or `get_all`)
- [ ] **C.R.3.3.2**: Make `MacroDependsOn` a `#[pyclass]` with getters
- [ ] **C.R.3.3.3**: Make `Docs` a `#[pyclass]` with getters
- [ ] **C.R.3.3.4**: Make `MacroArgument` a `#[pyclass]` with getters
- [ ] **C.R.3.3.5**: Implement `same_contents()` method on OxideMacro
- [ ] **C.R.3.3.6**: Implement `depends_on_macros` property on OxideMacro
- [ ] **C.Y.3.3.1**: Update `oxide_manifest.py` to return OxideMacro directly
- [ ] **C.Y.3.3.2**: Alias `OxideMacro` as `Macro` in nodes.py
- [ ] **V.3.3.1**: All existing Macro tests pass with OxideMacro
- [ ] **V.3.3.2**: Production code works without changes

### Blocked By

- Phase 3.2.7 (Python Manifest Removal) should complete first
- Need to ensure all callers only use duck-typing

---

## Estimated Timeline

| Phase | Duration |
|-------|----------|
| 3.2.1 Core Lookups | 1 day |
| 3.2.2 Resource Types | 1 day |
| 3.2.3 Macros | 2 days ✅ |
| 3.2.4 Disabled | 1 day |
| 3.2.5 Serialization | 1 day |
| 3.2.6 Integration | 3 days |
| 3.2.7 Removal | 1 day |
| **3.3 Full Macro** | **2 days** |
| **Total** | **12 days** |
