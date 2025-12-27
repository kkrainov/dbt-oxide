# Rust-Native Manifest (Phase C) - Complete Python Replacement

**Goal:** Complete Rust implementation of manifest with zero Python legacy code remaining.

> [!IMPORTANT]
> **Architecture Mandate:**
> - Rust owns all manifest data, operations, and serialization
> - Python only gets thin wrappers for Jinja context access
> - No redundant operations between Python and Rust
> - Incremental verification: verify each feature before removing Python equivalent

---

## Rust-First Architecture (Complete)

### What Moves to Rust (Complete)

| Category | Methods | Count |
|----------|---------|-------|
| **Lookups** | resolve_ref, resolve_source, resolve_metric, resolve_doc, resolve_saved_query | 5 |
| **Lookup Classes** | DocLookup, SourceLookup, RefableLookup, MetricLookup, DisabledLookup, AnalysisLookup | 6 |
| **Maps** | build_parent_and_child_maps, build_group_map, build_macro_child_map | 3 |
| **Serialization** | write, writable_manifest, to_json | 3 |
| **Node Access** | expect, get_node, nodes, sources, macros, docs, exposures, metrics | 8 |
| **Macro Methods** | find_macro_by_name, find_generate_macro_by_name, get_macros_by_name | 3 |

### What Python Exposes (Minimal API)

Python only exposes thin wrappers needed for Jinja context:

```python
# Minimal Python API - thin wrappers around Rust
class OxideManifest:
    """Python interface to Rust manifest - no data storage."""
    
    # Jinja context requirements
    def ref(self, name, package=None, version=None) -> Dict:
        return dbt_rs.resolve_ref(name, package, version)
    
    def source(self, source_name, table_name) -> Dict:
        return dbt_rs.resolve_source(source_name, table_name)
    
    def doc(self, name, package=None) -> str:
        return dbt_rs.resolve_doc(name, package)
    
    # Node iteration (for tasks that iterate over nodes)
    def get_nodes(self) -> Iterator[Dict]:
        return dbt_rs.get_nodes_iterator()
    
    # Serialization (delegates to Rust)
    def write(self, path: str):
        dbt_rs.write_manifest(path)
```

### Python Files to Delete After Complete Migration

| File | Lines | Reason |
|------|-------|--------|
| `contracts/graph/manifest.py` | 1813 | Entire Manifest class replaced |
| `parser/manifest.py` | 500+ | Parser builds directly in Rust |
| Parts of `compilation.py` | ~200 | Linker uses Rust graph |

---

## Incremental Verification Strategy

Each feature is:
1. **Tested** - Rust unit test written first (TDD)
2. **Implemented** - Rust implementation makes test pass
3. **Verified** - Python test calls Rust and compares output
4. **Replaced** - Python code removed, Rust is sole implementation

```mermaid
graph LR
    A[Write Rust Test] --> B[Implement in Rust]
    B --> C[Verify vs Python]
    C --> D[Delete Python Code]
    D --> E[Next Feature]
```

---

## Feature Gap Analysis

### Current Rust Implementation vs Required

| Component | Current | Required | Gap |
|-----------|---------|----------|-----|
| OxideNode fields | ~10 | 47 | 37 fields |
| OxideNodeConfig fields | 2 | 29 | 27 fields |
| OxideMacro fields | 5 | 15 | 10 fields |
| OxideSource fields | 6 | 31 | 25 fields |
| Top-level manifest | 10 | 16 | 6 fields |

### Missing Node Fields (47 total for Model)

**Note:** Model has 47 total fields, but 3 are excluded during serialization (44 serialized).

```
access, alias, build_path, checksum, columns, compiled_path,
config, constraints, contract, created_at, database, depends_on,
deprecation_date, description, doc_blocks, docs, fqn, group,
language, latest_version, meta, metrics, name, original_file_path,
package_name, patch_path, path, primary_key, raw_code, refs,
relation_name, resource_type, schema, sources, tags, time_spine,
unique_id, unrendered_config, version, access, compiled_code,
compiled, extra_ctes_injected, extra_ctes, defer_relation
```

**Fields excluded during serialization (when `artifact=True`):**
- `_pre_injected_sql` (private, never serialized)
- `config_call_dict` (excluded)
- `unrendered_config_call_dict` (excluded)
- `defer_relation` (excluded in Model context)

**Actual serialized fields:** 44 (47 total - 3 excluded)

### Missing Config Fields (29 total)

```
access, alias, batch_size, begin, column_types, concurrent_batches,
contract, database, docs, enabled, event_time, freshness, full_refresh,
grants, group, incremental_strategy, lookback, materialized, meta,
on_configuration_change, on_schema_change, packages, persist_docs,
post-hook, pre-hook, quoting, schema, tags, unique_key
```

### Complete Type Inventory (60+ Types)

#### Base Types
| Type | Fields | File |
|------|--------|------|
| `BaseResource` | name, resource_type, package_name, path, original_file_path, unique_id | base.py |
| `GraphResource` | extends BaseResource + fqn | base.py |
| `FileHash` | name, checksum | base.py |
| `Docs` | show, node_color | base.py |

#### Component Types (components.py)
| Type | Fields |
|------|--------|
| `MacroDependsOn` | macros: List[str] |
| `DependsOn` | nodes, macros |
| `RefArgs` | name, package, version |
| `ColumnConfig` | meta, tags |
| `ColumnInfo` | name, description, meta, data_type, constraints, quote, config, tags, granularity, doc_blocks |
| `InjectedCTE` | id, sql |
| `Contract` | enforced, alias_types, checksum |
| `Quoting` | database, schema, identifier, column |
| `Time` | count, period |
| `FreshnessThreshold` | warn_after, error_after, filter |
| `HasRelationMetadata` | database, schema |
| `DeferRelation` | alias, relation_name, resource_type, name, description, compiled_code, meta, tags, config |
| `ParsedResource` | 17 fields (tags, description, columns, meta, group, docs, patch_path, build_path, unrendered_config, created_at, config_call_dict, unrendered_config_call_dict, relation_name, raw_code, doc_blocks) |
| `CompiledResource` | extends ParsedResource + 12 fields (language, refs, sources, metrics, depends_on, compiled_path, compiled, compiled_code, extra_ctes_injected, extra_ctes, contract) |

#### Config Types (config.py)
| Type | Fields |
|------|--------|
| `ContractConfig` | enforced, alias_types |
| `Hook` | sql, transaction, index |
| `NodeAndTestConfig` | enabled, alias, schema, database, tags, meta, group |
| `NodeConfig` | extends NodeAndTestConfig + 22 fields |
| `TestConfig` | extends NodeAndTestConfig + severity, store_failures, store_failures_as, where, limit, fail_calc, warn_if, error_if |

#### Node Types (8 in ManifestResource)
| Type | Base | Extra Fields | File |
|------|------|--------------|------|
| `Model` | CompiledResource | access, config, constraints, version, latest_version, deprecation_date, defer_relation, primary_key, time_spine (47 total, 44 serialized) | model.py |
| `Seed` | **ParsedResource** | config, root_path, depends_on, defer_relation (30 total fields) | seed.py |
| `Snapshot` | CompiledResource | config (SnapshotConfig), defer_relation | snapshot.py |
| `GenericTest` | CompiledResource | column_name, file_key_name, config (TestConfig), attached_node, test_metadata | generic_test.py |
| `SingularTest` | CompiledResource | config (TestConfig) | singular_test.py |
| `Analysis` | CompiledResource | - | analysis.py |
| `HookNode` | CompiledResource | index | hook.py |
| `SqlOperation` | CompiledResource | - | sql_operation.py |

#### Resource Types
| Type | Fields | File |
|------|--------|------|
| `SourceDefinition` | 31 fields (quoting, loaded_at_field, freshness, external, columns, source_meta, tags, config, etc.) | source_definition.py |
| `SourceConfig` | enabled, event_time, freshness, loaded_at_field, loaded_at_query, meta, tags |
| `ExternalTable` | location, file_format, row_format, tbl_properties, partitions |
| `ExternalPartition` | name, description, data_type, meta |
| `Macro` | 15 fields (macro_sql, depends_on, description, meta, docs, patch_path, arguments, created_at, supported_languages, etc.) | macro.py |
| `MacroArgument` | name, type, description |
| `Exposure` | type, owner, resource_type, description, label, maturity, meta, tags, config, unrendered_config, url, depends_on, refs, sources, metrics, created_at | exposure.py |
| `ExposureConfig` | enabled, tags, meta |
| `Owner` | email, name | owner.py |
| `Group` | name, owner | group.py |
| `Documentation` | block_contents | documentation.py |

#### Metric Types (metric.py - 10+ nested)
| Type | Fields |
|------|--------|
| `Metric` | 20 fields |
| `MetricConfig` | enabled, group, meta |
| `MetricTypeParams` | measure, input_measures, numerator, denominator, expr, window, metrics, etc. |
| `MetricInputMeasure` | name, filter, alias, join_to_timespine, fill_nulls_with |
| `MetricTimeWindow` | count, granularity |
| `MetricInput` | name, filter, alias, offset_window, offset_to_grain |
| `ConversionTypeParams` | base_measure, conversion_measure, entity, calculation, window, constant_properties |
| `CumulativeTypeParams` | window, grain_to_date, period_agg, metric |
| `MetricAggregationParams` | semantic_model, agg, agg_params, agg_time_dimension, non_additive_dimension, expr |

#### SemanticModel Types (semantic_model.py - 15+ nested)
| Type | Fields |
|------|--------|
| `SemanticModel` | 15+ fields (model, node_relation, entities, dimensions, measures, etc.) |
| `SemanticModelConfig` | enabled, group, meta |
| `NodeRelation` | alias, schema_name, database, relation_name |
| `Dimension` | name, type, description, label, is_partition, type_params, expr, config |
| `DimensionTypeParams` | time_granularity, validity_params |
| `Entity` | name, type, description, label, role, expr, config |
| `Measure` | name, agg, description, label, create_metric, expr, agg_params, non_additive_dimension, agg_time_dimension, config |
| `Defaults` | agg_time_dimension |

#### SavedQuery Types (saved_query.py)
| Type | Fields |
|------|--------|
| `SavedQuery` | 14 fields |
| `SavedQueryConfig` | enabled, group, meta, export_as, schema, cache |
| `QueryParams` | metrics, group_by, where, order_by, limit |
| `Export` | name, config, unrendered_config |
| `ExportConfig` | export_as, schema_name, alias, database |

#### UnitTest Types (unit_test_definition.py)
| Type | Fields |
|------|--------|
| `UnitTestDefinition` | 10 fields |
| `UnitTestConfig` | tags, meta, enabled |
| `UnitTestInputFixture` | input, rows, format, fixture |
| `UnitTestOutputFixture` | rows, format, fixture |
| `UnitTestOverrides` | macros, vars, env_vars |
| `UnitTestNodeVersions` | include, exclude |

#### WritableManifest (16 top-level fields)
| Field | Type |
|-------|------|
| `nodes` | Mapping[UniqueID, ManifestResource] |
| `sources` | Mapping[UniqueID, SourceDefinition] |
| `macros` | Mapping[UniqueID, Macro] |
| `docs` | Mapping[UniqueID, Documentation] |
| `exposures` | Mapping[UniqueID, Exposure] |
| `metrics` | Mapping[UniqueID, Metric] |
| `groups` | Mapping[UniqueID, Group] |
| `selectors` | Mapping[UniqueID, Any] |
| `disabled` | Mapping[UniqueID, List[DisabledManifestResource]] |
| `parent_map` | Dict[str, List[str]] |
| `child_map` | Dict[str, List[str]] |
| `group_map` | Dict[str, List[str]] |
| `saved_queries` | Mapping[UniqueID, SavedQuery] |
| `semantic_models` | Mapping[UniqueID, SemanticModel] |
| `metadata` | ManifestMetadata |
| `unit_tests` | Mapping[UniqueID, UnitTestDefinition] |

#### ManifestMetadata (7 fields)
| Field | Type |
|-------|------|
| `dbt_schema_version` | str |
| `project_name` | Optional[str] |
| `project_id` | Optional[str] |
| `user_id` | Optional[UUID] |
| `send_anonymous_usage_stats` | Optional[bool] |
| `adapter_type` | Optional[str] |
| `quoting` | Optional[Quoting] |

---

## TDD Implementation Approach

> [!IMPORTANT]
> **Test-Driven Development:** All tests are written BEFORE implementation.
> Tests initially fail, then implementation makes them pass.

---

## Phase T.0: Write All Tests First (RED Phase) ✓ COMPLETE

> [!IMPORTANT]
> **151 tests written** (exceeds 133 target). All tests import from `crate::manifest`.
> Tests correctly **FAIL** for ~60 unimplemented types. This is proper TDD RED phase.
>
> **Status:** RED phase complete. Ready for GREEN phase implementation.

### T.0.1 Base Types (4 tests)
- [x] `test_serialize_base_resource`
- [x] `test_serialize_graph_resource`
- [x] `test_serialize_file_hash`
- [x] `test_serialize_docs`

### T.0.2 Component Types (14 tests)
- [x] `test_serialize_macro_depends_on`
- [x] `test_serialize_depends_on`
- [x] `test_serialize_ref_args`
- [x] `test_serialize_column_config`
- [x] `test_serialize_column_info_all_10_fields`
- [x] `test_serialize_injected_cte`
- [x] `test_serialize_contract`
- [x] `test_serialize_quoting`
- [x] `test_serialize_time`
- [x] `test_serialize_freshness_threshold`
- [x] `test_serialize_has_relation_metadata`
- [x] `test_serialize_defer_relation`
- [x] `test_serialize_parsed_resource`
- [x] `test_serialize_compiled_resource`

### T.0.3 Config Types (10 tests)
- [x] `test_serialize_contract_config`
- [x] `test_serialize_hook`
- [x] `test_serialize_node_and_test_config`
- [x] `test_serialize_node_config_all_22_fields`
- [x] `test_serialize_node_config_defaults`
- [x] `test_serialize_test_config`
- [x] `test_serialize_model_config`
- [x] `test_serialize_seed_config`
- [x] `test_serialize_snapshot_config`
- [x] `test_serialize_source_config`

### T.0.4 Node Types (16 tests)
- [x] `test_serialize_model_minimal`
- [x] `test_serialize_model_all_fields`
- [x] `test_serialize_model_with_time_spine`
- [x] `test_serialize_model_with_constraints`
- [x] `test_serialize_seed`
- [x] `test_serialize_seed_with_root_path`
- [x] `test_serialize_snapshot`
- [x] `test_serialize_snapshot_check_strategy`
- [x] `test_serialize_snapshot_timestamp_strategy`
- [x] `test_serialize_generic_test`
- [x] `test_serialize_generic_test_with_test_metadata`
- [x] `test_serialize_singular_test`
- [x] `test_serialize_analysis`
- [x] `test_serialize_hook_node`
- [x] `test_serialize_sql_operation`
- [x] `test_serialize_compiled_vs_uncompiled`

### T.0.5 Source Types (8 tests)
- [x] `test_serialize_source_definition_full`
- [x] `test_serialize_source_definition_minimal`
- [x] `test_serialize_external_table`
- [x] `test_serialize_external_partition`
- [x] `test_serialize_source_config`
- [x] `test_serialize_source_with_freshness`
- [x] `test_serialize_source_with_external`
- [x] `test_serialize_source_with_columns`

### T.0.6 Macro Types (4 tests)
- [x] `test_serialize_macro`
- [x] `test_serialize_macro_with_arguments`
- [x] `test_serialize_macro_argument`
- [x] `test_serialize_macro_with_supported_languages`

### T.0.7 Exposure Types (4 tests)
- [x] `test_serialize_exposure`
- [x] `test_serialize_exposure_with_owner`
- [x] `test_serialize_exposure_config`
- [x] `test_serialize_owner`

### T.0.8 Metric Types (12 tests)
- [x] `test_serialize_metric`
- [x] `test_serialize_metric_config`
- [x] `test_serialize_metric_type_params`
- [x] `test_serialize_metric_input_measure`
- [x] `test_serialize_metric_time_window`
- [x] `test_serialize_metric_input`
- [x] `test_serialize_conversion_type_params`
- [x] `test_serialize_cumulative_type_params`
- [x] `test_serialize_metric_aggregation_params`
- [x] `test_serialize_constant_property_input`
- [x] `test_serialize_metric_simple`
- [x] `test_serialize_metric_derived`

### T.0.9 SemanticModel Types (12 tests)
- [x] `test_serialize_semantic_model`
- [x] `test_serialize_semantic_model_config`
- [x] `test_serialize_node_relation`
- [x] `test_serialize_dimension`
- [x] `test_serialize_dimension_type_params`
- [x] `test_serialize_dimension_validity_params`
- [x] `test_serialize_entity`
- [x] `test_serialize_measure`
- [x] `test_serialize_defaults`
- [x] `test_serialize_semantic_layer_element_config`
- [x] `test_serialize_semantic_model_with_entities`
- [x] `test_serialize_semantic_model_with_measures`

### T.0.10 SavedQuery Types (6 tests)
- [x] `test_serialize_saved_query`
- [x] `test_serialize_saved_query_config`
- [x] `test_serialize_query_params`
- [x] `test_serialize_export`
- [x] `test_serialize_export_config`
- [x] `test_serialize_saved_query_cache`

### T.0.11 UnitTest Types (8 tests)
- [x] `test_serialize_unit_test_definition`
- [x] `test_serialize_unit_test_config`
- [x] `test_serialize_unit_test_input_fixture`
- [x] `test_serialize_unit_test_output_fixture`
- [x] `test_serialize_unit_test_overrides`
- [x] `test_serialize_unit_test_node_versions`
- [x] `test_serialize_unit_test_with_csv_format`
- [x] `test_serialize_unit_test_with_sql_format`

### T.0.12 Other Resource Types (4 tests)
- [x] `test_serialize_documentation`
- [x] `test_serialize_group`
- [x] `test_serialize_selectors`
- [x] `test_serialize_disabled_resources`

### T.0.13 Manifest Level (10 tests)
- [x] `test_serialize_empty_manifest`
- [x] `test_serialize_manifest_metadata_all_fields`
- [x] `test_serialize_writable_manifest`
- [x] `test_build_parent_map`
- [x] `test_build_child_map`
- [x] `test_build_group_map`
- [x] `test_serialize_manifest_with_all_node_types`
- [x] `test_serialize_manifest_with_all_resource_types`
- [x] `test_manifest_round_trip`
- [x] `test_manifest_schema_version`

### T.0.14 Output Compatibility (8 tests)
- [x] `test_json_key_ordering_alphabetical` (Parity with Python confirmed)
- [x] `test_null_vs_omit_none`
- [x] `test_datetime_iso8601_format`
- [x] `test_uuid_lowercase`
- [x] `test_float_precision_created_at`
- [x] `test_enum_serialization`
- [x] `test_optional_fields_omitted`
- [x] `test_empty_collections_serialized`

### T.0.15 Python Integration Tests (6 tests)
- [x] `test_rust_manifest_json_matches_python`
- [x] `test_rust_write_file_matches_python_byte_for_byte`
- [x] `test_partial_parse_reads_rust_manifest`
- [x] `test_performance_2000_nodes_under_500ms`
- [x] `test_performance_10000_nodes_under_2s`
- [x] `test_memory_usage_under_2x_manifest_size`

### T.0.16 Lookup Operations (15 tests)
- [x] `test_resolve_ref_simple`
- [x] `test_resolve_ref_with_package`
- [x] `test_resolve_ref_with_version`
- [x] `test_resolve_ref_not_found`
- [x] `test_resolve_source_simple`
- [x] `test_resolve_source_with_package`
- [x] `test_resolve_source_not_found`
- [x] `test_resolve_metric`
- [x] `test_resolve_doc`
- [x] `test_resolve_saved_query`
- [x] `test_find_macro_by_name`
- [x] `test_find_generate_macro_by_name`
- [x] `test_get_macros_by_name`
- [x] `test_disabled_lookup`
- [x] `test_analysis_lookup`

### T.0.17 Node Iteration (6 tests)
- [x] `test_get_nodes_iterator`
- [x] `test_get_sources_iterator`
- [x] `test_get_macros_iterator`
- [x] `test_filter_nodes_by_resource_type`
- [x] `test_filter_nodes_by_package`
- [x] `test_external_node_unique_ids`

---

## Phase C.1: Schema Expansion (Rust Structs) ✅ COMPLETE

#### C.1.1 Core Components ✅ COMPLETE
- [x] Add `OxideFileHash` struct (make T.0.1 tests pass)
- [x] Add `OxideDocs` struct
- [x] Add `OxideContract` struct
- [x] Add `OxideColumnInfo` struct
- [x] Add `OxideRefArgs` struct
- [x] Add `OxideQuoting` struct

#### C.1.2 Config Types ✅ COMPLETE
- [x] Expand `OxideNodeConfig` (make T.0.2 tests pass)
- [x] Add `OxideModelConfig`
- [x] Add `OxideTestConfig`
- [x] Add `OxideSeedConfig`
- [x] Add `OxideSnapshotConfig`

#### C.1.3 Node Types (Full Schema) ✅ COMPLETE
- [x] Add ParsedResource base struct (30 fields)
- [x] Add CompiledResource base struct (40 fields)
- [x] Add `OxideModel` (47 fields total, 44 serialized)
- [x] Add `OxideSeed` (30 fields - extends ParsedResource NOT CompiledResource)
- [x] Add `OxideSnapshot`
- [x] Add `OxideGenericTest`
- [x] Add `OxideSingularTest`
- [x] Add `OxideAnalysis`
- [x] Add `OxideHookNode`

#### C.1.4 Other Resource Types ✅ COMPLETE
- [x] Expand `OxideSource` (make T.0.4 tests pass)
- [x] Expand `OxideMacro`
- [x] Add `OxideDocumentation`
- [x] Expand `OxideExposure`
- [x] Expand `OxideMetric`
- [x] Expand `OxideSemanticModel` (placeholder)
- [x] Expand `OxideSavedQuery` (placeholder)
- [x] Expand `OxideUnitTest` (placeholder)

#### C.1.5 Manifest Metadata ✅ COMPLETE
- [x] Expand `OxideManifestMetadata` (make T.0.5 tests pass)
- [x] Add `OxideManifest` with all fields

---

## Phase C.2: Serialization Implementation ✅ COMPLETE

#### C.2.1 Basic Serialization
- [x] Implement `OxideManifest::to_json_str()` (Will be part of implementation)
- [x] Implement `OxideManifest::write_to_file()`
- [x] Configure serde for alphabetical key ordering (Research confirmed not required, validated parity)
- [x] Handle `#[serde(skip_serializing_if)]`

#### C.2.2 Parent/Child Maps
- [x] Implement `build_parent_map()`
- [x] Implement `build_child_map()`
- [x] Implement `build_group_map()`

#### C.2.3 Special Handling
- [x] Handle datetime format
- [x] Handle UUID serialization
- [x] Handle float precision
- [x] Handle omit_none behavior

---

## Phase C.3: PyO3 Bindings ✅ COMPLETE

- [x] Add `write_manifest_to_file()` - Implemented in `py_manifest.rs`
- [x] Add `serialize_manifest_to_json()` - Implemented in `py_manifest.rs`
- [x] Add `get_manifest_stats()` - Implemented in `py_manifest.rs`
- [x] Register all functions in PyO3 module
- [x] Test functions from Python REPL - All working

---

## Phase C.4: Python Parsing + Rust Operations (PHASE 1) ⚙️ IN PROGRESS

> [!IMPORTANT]
> **Architecture**: Python keeps ALL parsing. Rust handles manifest data storage and operations.
> **No JSON serialization**: Use PyO3 native types (`pythonize` crate) for Python↔Rust conversion.

### Strategy: Separate OxideManifest Class (Option B)

> [!IMPORTANT]
> **Architecture Decision**: Create a separate `OxideManifest` Python wrapper class that delegates to Rust.
> Do NOT embed `_rust_builder` in the existing `Manifest` class. This matches the successful `Graph` migration pattern.

**Benefits:**
- No dual state (Rust is sole source of truth)
- Clean rollback (just use old Manifest)
- Matches existing Graph wrapper pattern
- Clear migration path (replace imports file-by-file)

**What Stays in Python `Manifest`** (unchanged):
- ✅ File scanning, parsing (MacroParser, ModelParser, etc.)
- ✅ PartialParsing class (unchanged)
- ✅ Jinja rendering, YAML parsing

**New `OxideManifest` Class** (Rust-backed):
- 🦀 Manifest data storage (delegates to Rust OxideManifest)
- 🦀 Map building (parent, child, group)
- 🦀 Manifest serialization (write)
- 🦀 Lookup operations (resolve_ref, resolve_source)

---

### C.4.1: Rust ManifestBuilder Class (TDD)

> [!TIP]
> Using `#[pyclass]` matches existing `DbtGraph` pattern. No global state needed.

#### Test Cases (RED Phase First!)

```rust
// src/dbt_rs/src/tests/manifest_builder.rs

#[test]
fn test_new_manifest_builder() {
    let builder = PyManifestBuilder::new(None).unwrap();
    assert_eq!(builder.node_count(), 0);
}

#[test]
fn test_add_single_node() {
    let mut builder = PyManifestBuilder::new(None).unwrap();
    // Add node via dict simulation
    assert_eq!(builder.node_count(), 1);
}

#[test]
fn test_add_multiple_nodes() {
    let mut builder = PyManifestBuilder::new(None).unwrap();
    // Add 100 nodes
    assert_eq!(builder.node_count(), 100);
}
```

#### Implementation Steps

**Week 1: Foundation**
- [x] **T.C4.1.1**: Write test `test_new_manifest_builder` (RED) ✅
- [x] **C.4.1.1**: Implement `PyManifestBuilder` class (GREEN) ✅
  ```rust
  #[pyclass(name = "ManifestBuilder")]
  pub struct PyManifestBuilder {
      manifest: OxideManifest,
  }

  #[pymethods]
  impl PyManifestBuilder {
      #[new]
      #[pyo3(signature = (metadata=None))]
      pub fn new(metadata: Option<&PyDict>) -> PyResult<Self> {
          let manifest = match metadata {
              Some(meta) => OxideManifest::with_metadata(depythonize(meta)?),
              None => OxideManifest::default(),
          };
          Ok(Self { manifest })
      }
      
      #[getter]
      pub fn node_count(&self) -> usize {
          self.manifest.node_count()
      }
  }
  ```
- [x] **T.C4.1.2**: Write test `test_add_single_node` (RED) ✅
- [x] **C.4.1.2**: Add `add_node(&mut self, node: &PyDict)` method (GREEN) ✅
- [x] **T.C4.1.3**: Write test `test_add_multiple_nodes` (RED) ✅
- [x] **C.4.1.3**: Add `add_nodes(&mut self, nodes: &PyList)` method (GREEN) ✅
- [x] **C.4.1.4**: Add `add_source()`, `add_macro()` methods ✅
- [x] **C.4.1.5**: Add `pythonize` crate to Cargo.toml ✅
- [x] **C.4.1.6**: Register class in module: `m.add_class::<PyManifestBuilder>()?` ✅
- [x] **V.C4.1.1**: Run Rust tests: `cargo test --no-default-features manifest_builder` ✅

**Week 2: OxideManifest Python Wrapper (Option B)**

> [!IMPORTANT]
> Create a **separate** `OxideManifest` class. Do NOT modify the existing `Manifest` class.

- [x] **C.4.2.1**: Create `core/dbt/contracts/graph/oxide_manifest.py` (OxideManifest wrapper) ✅
- [x] **C.4.2.2**: Add factory function `get_manifest_class()` to `__init__.py` ✅
- [x] **C.4.2.3**: Remove `_rust_builder` field from `Manifest` class (revert previous changes) ✅
- [x] **C.4.2.4**: Add `PyOxideManifest` class to Rust PyO3 bindings ✅
- [x] **C.4.2.5**: Expose `from_json()`, `build_parent_map()`, `build_child_map()` in PyO3 ✅
- [x] **V.C4.2.1**: Test OxideManifest wrapper independently ✅
- [x] **V.C4.2.2**: Verify `get_manifest_class()` returns correct class based on flag ✅

---

### C.4.3: Build Parent/Child Maps (TDD)

#### Test Cases (RED Phase)

```rust
// src/dbt_rs/src/tests/manifest_maps.rs

#[test]
fn test_build_parent_map_simple() {
    // Given: model_a → model_b (ref)
    // When: build_parent_map()
    // Then: parent_map[model_a] = [model_b]
}

#[test]
fn test_build_child_map_simple() {
    // Given: model_a → model_b
    // When: build_child_map()
    // Then: child_map[model_b] = [model_a]
}

#[test]
fn test_build_maps_multiple_refs() {
    // Given: model_c refs model_a and model_b
    // When: build maps
    // Then: parent_map[model_c] = [model_a, model_b]
}

#[test]
fn test_build_maps_with_sources() {
    // Given: model refs source
    // When: build maps
    // Then: parent_map includes source
}

#[test]
fn test_build_maps_transitive() {
    // Given: A → B → C (chain)
    // When: build maps
    // Then: All relationships captured
}
```

#### Implementation Steps

**Week 3: Map Building**
- [x] **T.C4.3.1**: Write tests for parent/child maps (RED) ✅ COMPLETE
- [x] **C.4.3.1**: Implement `OxideManifest::build_parent_map()` in Rust ✅ COMPLETE
- [x] **C.4.3.2**: Implement `OxideManifest::build_child_map()` in Rust ✅ COMPLETE
- [x] **C.4.3.3**: Expose methods in `PyOxideManifest` (already done via C.4.2.5) ✅
- [x] **C.4.3.4**: OxideManifest wrapper delegates to Rust (Option B - no modification of existing Manifest class) ✅
- [x] **V.C4.3.1**: Run Rust tests for map building ✅
- [x] **V.C4.3.2**: Test with model dependencies in Python ✅
- [x] **V.C4.3.3**: Verify Rust maps match expected output ✅

---

### C.4.4: Build Group Map (TDD)

#### Test Cases (RED Phase)

```rust
#[test]
fn test_build_group_map_empty() {
    // Given: No groups configured
    // When: build_group_map()
    // Then: Empty map
}

#[test]
fn test_build_group_map_single_group() {
    // Given: model with config.group = "analytics"
    // When: build_group_map()
    // Then: group_map["analytics"] = [model.unique_id]
}

#[test]
fn test_build_group_map_multiple_models() {
    // Given: 3 models in "analytics" group
    // When: build_group_map()
    // Then: All 3 in group_map["analytics"]
}
```

#### Implementation Steps

**Week 3 (continued)**
- [x] **T.C4.4.1**: Write group map tests (RED) ✅
- [x] **C.4.4.1**: Implement `OxideManifest::build_group_map()` in Rust ✅
- [x] **C.4.4.2**: Expose via PyOxideManifest (Option B) ✅
- [x] **C.4.4.3**: OxideManifest wrapper delegates to Rust ✅
- [x] **V.C4.4.1**: Test with Python - groups correctly mapped ✅
- [x] **V.C4.4.2**: Verified group map matches expected output ✅

---

### C.4.5: Write Manifest (TDD)

#### Test Cases (RED Phase)

```rust
#[test]
fn test_write_manifest_to_file() {
    // Given: Manifest with nodes
    // When: write_manifest_py(path)
    // Then: JSON file created, valid JSON
}

#[test]
fn test_write_manifest_contains_all_nodes() {
    // Given: 10 nodes added
    // When: write to file
    // Then: JSON contains all 10 nodes
}

#[test]
fn test_write_manifest_pretty_print() {
    // Given: Manifest
    // When: write_manifest_py()
    // Then: JSON is formatted (pretty printed)
}
```

#### Implementation Steps

**Week 4: Write & Finalize**
- [ ] **T.C4.5.1**: Write manifest write tests (RED)
- [ ] **C.4.5.1**: Implement `write_manifest_py(path: &str)` in Rust
  ```rust
  #[pyfunction]
  fn write_manifest_py(path: &str) -> PyResult<()> {
      let builder = MANIFEST_BUILDER.read().unwrap();
      let manifest = builder.as_ref().unwrap();
      let json = serde_json::to_string_pretty(&*manifest)?;
      std::fs::write(path, json)?;
      Ok(())
  }
  ```
- [ ] **C.4.5.2**: Modify Python `Manifest.write()`:
  ```python
  def write(self, path):
      if self._rust_enabled:
          dbt_rs.write_manifest_py(path)
          fire_event(ArtifactWritten(...))
      else:
          # Python fallback
  ```
- [ ] **V.C4.5.1**: Test manifest write with sample project
- [ ] **V.C4.5.2**: Compare Rust JSON vs Python JSON (should be identical)
- [ ] **V.C4.5.3**: Verify JSON is valid (parse with `jq`)

---

### C.4.6: Validation & Integration Testing

- [ ] **V.C4.6.1**: Run all existing Python tests: `uv run pytest tests/unit/contracts/graph/test_manifest.py`
- [ ] **V.C4.6.2**: Test with `DBT_USE_RUST_MANIFEST=0` (Python fallback)
- [ ] **V.C4.6.3**: Test with `DBT_USE_RUST_MANIFEST=1` (Rust path)
- [ ] **V.C4.6.4**: Compare results (should be identical)
- [ ] **V.C4.6.5**: Test with real projects:
  - [ ] jaffle_shop
  - [ ] Small user project
  - [ ] Medium user project
- [ ] **V.C4.6.6**: Benchmark: Measure parse time with Rust vs Python
- [ ] **V.C4.6.7**: Memory test: Verify <2x Python memory usage
- [ ] **V.C4.6.8**: Create validation mode:
  ```python
  if os.getenv("DBT_VALIDATE_RUST_MANIFEST") == "1":
      rust_result = dbt_rs.get_parent_map()
      python_result = self._build_parent_map_python()
      assert rust_result == python_result, "Rust/Python mismatch!"
  ```

---

## Phase C.4.1: Rust Partial Parse (PHASE 2) 📋 PLANNED

> [!NOTE]
> Phase 2 moves PartialParsing logic to Rust. Python parsing remains unchanged.

### Strategy

Rust handles file change detection, state validation, and incremental manifest building. Python parsers still parse changed files, but Rust determines WHICH files to parse.

**What Moves to Rust**:
- 🦀 File change detection (hash comparison)
- 🦀 State validation (version, vars, profile checks)
- 🦀 Saved manifest storage (MessagePack)
- 🦀 Incremental manifest building

**What Stays Python**:
- ✅ File parsing (after Rust determines changed files)
- ✅ All parsers (no changes)

---

### C.4.1.1: File Change Detection (TDD)

#### Test Cases (RED Phase)

```rust
// src/dbt_rs/src/tests/partial_parse_file_diff.rs

#[test]
fn test_detect_no_changes() {
    // Given: Saved and current files identical
    // When: analyze()
    // Then: FileDiff { changed: [], added: [], deleted: [] }
}

#[test]
fn test_detect_changed_file() {
    // Given: model.sql checksum changed
    // When: analyze()
    // Then: FileDiff { changed: ["model.sql"], ... }
}

#[test]
fn test_detect_added_file() {
    // Given: new_model.sql exists, not in saved
    // When: analyze()
    // Then: FileDiff { added: ["new_model.sql"], ... }
}

#[test]
fn test_detect_deleted_file() {
    // Given: old_model.sql in saved, not in current
    // When: analyze()
    // Then: FileDiff { deleted: ["old_model.sql"], ... }
}

#[test]
fn test_file_hash_calculation() {
    // Given: File content
    // When: calculate_checksum()
    // Then: SHA256 hash matches expected
}
```

#### Implementation Steps

**Week 1: File Diff**
- [ ] **T.C4.1.1.1**: Write file diff tests (RED)
- [ ] **C.4.1.1.1**: Implement `FileSnapshot` struct
  ```rust
  #[derive(Serialize, Deserialize)]
  pub struct FileSnapshot {
      pub path: PathBuf,
      pub checksum: String,  // SHA256
      pub modified_time: SystemTime,
  }
  ```
- [ ] **C.4.1.1.2**: Implement `calculate_checksum(path: &Path)` using SHA256
- [ ] **C.4.1.1.3**: Implement `FileDiffAnalyzer::analyze()` 
- [ ] **C.4.1.1.4**: Add PyO3 wrapper `get_file_diff(project_root: &str)`
- [ ] **V.C4.1.1.1**: Run Rust tests
- [ ] **V.C4.1.1.2**: Test with real project file changes

---

### C.4.1.2: State Validation (TDD)

#### Test Cases (RED Phase)

```rust
// src/dbt_rs/src/tests/partial_parse_state.rs

#[test]
fn test_state_validation_success() {
    // Given: Same version, vars, profile
    // When: can_partial_parse()
    // Then: Ok(())
}

#[test]
fn test_state_validation_version_mismatch() {
    // Given: Different dbt version
    // When: can_partial_parse()
    // Then: Err("dbt version changed")
}

#[test]
fn test_state_validation_vars_changed() {
    // Given: Different project vars hash
    // When: can_partial_parse()
    // Then: Err("project vars changed")
}

#[test]
fn test_state_validation_profile_changed() {
    // Given: Different profile hash
    // When: can_partial_parse()
    // Then: Err("profile changed")
}
```

#### Implementation Steps

**Week 2: State Check**
- [ ] **T.C4.1.2.1**: Write state validation tests (RED)
- [ ] **C.4.1.2.1**: Implement `ManifestState` struct
  ```rust
  #[derive(Serialize, Deserialize)]
  pub struct ManifestState {
      pub dbt_version: String,
      pub vars_hash: String,
      pub profile_hash: String,
  }
  ```
- [ ] **C.4.1.2.2**: Implement `ManifestState::can_partial_parse()`
- [ ] **C.4.1.2.3**: Add PyO3 wrapper `can_partial_parse(project_root, state)`
- [ ] **V.C4.1.2.1**: Test all state change scenarios

---

### C.4.1.3: Saved Manifest Storage (TDD)

#### Test Cases (RED Phase)

```rust
// src/dbt_rs/src/tests/partial_parse_storage.rs

#[test]
fn test_save_manifest_to_msgpack() {
    // Given: Manifest
    // When: save_manifest()
    // Then: File created at target/partial_parse.msgpack
}

#[test]
fn test_load_manifest_from_msgpack() {
    // Given: Saved msgpack file
    // When: load_manifest()
    // Then: Manifest loaded, matches original
}

#[test]
fn test_load_nonexistent_manifest() {
    // Given: No saved file
    // When: load_manifest()
    // Then: Ok(None)
}
```

#### Implementation Steps

**Week 3: Storage**
- [ ] **T.C4.1.3.1**: Write storage tests (RED)
- [ ] **C.4.1.3.1**: Add `rmp-serde` crate for MessagePack
- [ ] **C.4.1.3.2**: Implement `PartialParseStorage::save_manifest()`
- [ ] **C.4.1.3.3**: Implement `PartialParseStorage::load_manifest()`
- [ ] **C.4.1.3.4**: Add PyO3 wrappers for save/load
- [ ] **V.C4.1.3.1**: Verify msgpack files created correctly
- [ ] **V.C4.1.3.2**: Verify load matches save

---

### C.4.1.4: Incremental Manifest Builder (TDD)

#### Test Cases (RED Phase)

```rust
// src/dbt_rs/src/tests/partial_parse_incremental.rs

#[test]
fn test_incremental_builder_remove_deleted() {
    // Given: Saved manifest, file deleted
    // When: build()
    // Then: Nodes from deleted file removed
}

#[test]
fn test_incremental_builder_keep_unchanged() {
    // Given: Saved manifest, no changes
    // When: build()
    // Then: All nodes preserved
}

#[test]
fn test_incremental_builder_remove_changed() {
    // Given: File changed
    // When: build()
    // Then: Old nodes removed (ready for re-parse)
}
```

#### Implementation Steps

**Week 4: Incremental**
- [ ] **T.C4.1.4.1**: Write incremental tests (RED)
- [ ] **C.4.1.4.1**: Implement `IncrementalManifestBuilder`
- [ ] **C.4.1.4.2**: Implement `OxideManifest::remove_nodes_by_file()`
- [ ] **C.4.1.4.3**: Add PyO3 wrapper `init_partial_parse_builder()`
- [ ] **V.C4.1.4.1**: Test incremental building

---

### C.4.1.5: Python Integration

**Week 5: Integration**
- [ ] **C.4.1.5.1**: Modify `ManifestLoader.load()` to check `can_partial_parse()`
- [ ] **C.4.1.5.2**: If can partial parse:
  - [ ] Call `get_file_diff()`
  - [ ] Call `init_partial_parse_builder()`
  - [ ] Parse only changed files
- [ ] **C.4.1.5.3**: Call `save_partial_parse_manifest()` after build
- [ ] **V.C4.1.5.1**: Test partial parse with file changes
- [ ] **V.C4.1.5.2**: Verify incremental parse faster than full
- [ ] **V.C4.1.5.3**: Compare Rust partial parse vs Python partial parse results

---

### Success Criteria

**Phase 1 (C.4)**:
- ✅ All Python tests pass with Rust enabled
- ✅ Maps byte-for-byte identical to Python
- ✅ Memory <2x Python
- ✅ No JSON serialization overhead

**Phase 2 (C.4.1)**:
- ✅ Partial parse correctly detects changes
- ✅ Incremental manifest matches full parse
- ✅ Faster than Python partial parse
- ✅ MessagePack compatible with Python


#### Phase 4A: `resolve_ref()` - RED Phase (Tests First)

**Existing Rust Test**: `test_resolve_ref_simple()` in `logic.rs` (PASSING)

**Additional Tests Needed** (from Python `test_manifest.py`):
```rust
// src/dbt_rs/src/tests/logic.rs

#[test]
fn test_resolve_ref_with_package() {
    // Test resolving ref with explicit package
    // Python: test_resolve_ref with package="dep"
}

#[test]
fn test_resolve_ref_with_version() {
    // Test resolving versioned models
    // Python: test_resolve_ref with version=1
}

#[test]
fn test_resolve_ref_not_found() {
    // Test None return when ref doesn't exist
    // Python: test_resolve_ref with expected=None
}

#[test]
fn test_resolve_ref_ambiguous() {
    // Test error when multiple matches without package
    // Python: test_resolve_ref_ambiguous_resource_name_across_packages
    // Should return error or panic
}

#[test]
fn test_resolve_ref_cross_package() {
    // Test finding refs in other packages
    // Python: multiple test cases with root/dep packages
}
```

**Python Tests to Match**: Lines 1795-1840 in `test_manifest.py`

#### Phase 4B: `resolve_ref()` - GREEN Phase (Implementation)

- [ ] Implement package search priority (current → node → None)
- [ ] Implement version matching for versioned models
- [ ] Implement ambiguous ref detection and error handling
- [ ] Add PyO3 wrapper: `resolve_ref_py()`
- [ ] Replace Python `Manifest.resolve_ref()` with Rust call
- [ ] Test: `pytest tests/unit/contracts/graph/test_manifest.py -k test_resolve_ref`

---

#### Phase 4C: `resolve_source()` - RED Phase (Tests First)

**Existing Rust Test**: `test_resolve_source_simple()` in `logic.rs` (PASSING)

**Additional Tests Needed**:
```rust
#[test]
fn test_resolve_source_with_package() {
    // Test resolving source with explicit package
}

#[test]
fn test_resolve_source_not_found() {
    // Test None return when source doesn't exist
    // Wrong source name or table name
}

#[test]
fn test_resolve_source_cross_package() {
    // Test finding sources in other packages
}

#[test]
fn test_resolve_source_wrong_table() {
    // Test None when table name doesn't match
}
```

**Python Tests to Match**: Lines 1924-1940 in `test_manifest.py`

#### Phase 4D: `resolve_source()` - GREEN Phase (Implementation)

- [ ] Implement source name + table name matching
- [ ] Implement package search priority
- [ ] Handle missing sources gracefully
- [ ] Add PyO3 wrapper: `resolve_source_py()`
- [ ] Replace Python `Manifest.resolve_source()` with Rust call
- [ ] Test: `pytest tests/unit/contracts/graph/test_manifest.py -k test_resolve_source`

---

#### Phase 4E: `resolve_doc()` - RED Phase (Tests First)

**Existing Rust Test**: `test_resolve_doc_simple()` in `logic.rs` (#[ignore] - NOT IMPLEMENTED)

**Tests Needed**:
```rust
#[test]
fn test_resolve_doc_simple() {
    // Basic doc resolution in same package
}

#[test]
fn test_resolve_doc_with_package() {
    // Explicit package specified
}

#[test]
fn test_resolve_doc_not_found() {
    // None when doc doesn't exist
}

#[test]
fn test_resolve_doc_cross_package() {
    // Finding docs in other packages
}
```

**Python Tests to Match**: Lines 1979-1991 in `test_manifest.py`

#### Phase 4F: `resolve_doc()` - GREEN Phase (Implementation)

- [ ] Implement doc name matching
- [ ] Implement package search priority
- [ ] Remove `unimplemented!()` from Rust code
- [ ] Add PyO3 wrapper: `resolve_doc_py()`
- [ ] Replace Python `Manifest.resolve_doc()` with Rust call  
- [ ] Test: `pytest tests/unit/contracts/graph/test_manifest.py -k test_resolve_doc`
- [ ] Unignore Rust test: Remove `#[ignore]` from `test_resolve_doc_simple()`

---

#### Phase 4G: `resolve_metric()` - RED/GREEN (Lower Priority)

**Note**: No dedicated Python tests found. Implementation similar to `resolve_doc()`.

- [ ] RED: Write Rust tests for metric resolution
- [ ] GREEN: Implement metric resolution logic
- [ ] Add PyO3 wrapper
- [ ] Replace Python implementation

### Verification
- [ ] All existing Python manifest tests pass with flag OFF (Python)
- [ ] All existing Python manifest tests pass with flag ON (Rust)
- [ ] No behavior changes, only performance improvements

---

## Phase C.5: Verification, Migration & Python Removal

### C.5.1 Verification Checkpoints
- [ ] All 133 Rust tests pass (T.0.1 - T.0.17)
- [ ] All 75 existing Python manifest tests pass
- [ ] JSON output byte-for-byte identical to Python
- [ ] Benchmark 2000 nodes < 500ms
- [ ] Benchmark 10000 nodes < 2s

### C.5.2 Python Code Removal (Incremental)

> [!CAUTION]
> Each removal requires all related tests passing FIRST.

#### Phase C.5.2.1 - Remove Lookup Classes (after T.0.16 passes)
- [ ] Delete `DocLookup` class from manifest.py
- [ ] Delete `SourceLookup` class
- [ ] Delete `RefableLookup` class
- [ ] Delete `MetricLookup` class
- [ ] Delete `SavedQueryLookup` class
- [ ] Delete `DisabledLookup` class
- [ ] Delete `AnalysisLookup` class
- [ ] Update imports in 38 consuming files

#### Phase C.5.2.2 - Remove Map Building (after parent/child map tests pass)
- [ ] Delete `build_parent_and_child_maps()` method
- [ ] Delete `build_group_map()` method
- [ ] Delete `build_macro_child_map()` method

#### Phase C.5.2.3 - Remove Resolve Methods (after T.0.16 passes)
- [ ] Delete `resolve_ref()` method
- [ ] Delete `resolve_source()` method
- [ ] Delete `resolve_metric()` method
- [ ] Delete `resolve_doc()` method
- [ ] Delete `resolve_saved_query()` method

#### Phase C.5.2.4 - Remove MacroMethods (after macro lookup tests pass)
- [ ] Delete `MacroMethods` class
- [ ] Delete `find_macro_by_name()`
- [ ] Delete `find_generate_macro_by_name()`
- [ ] Delete `_find_macros_by_name()`

#### Phase C.5.2.5 - Remove Serialization (after serialization tests pass)
- [ ] Delete `writable_manifest()` method
- [ ] Delete `write()` method
- [ ] Delete `to_dict()` usage for manifest
- [ ] Delete `WritableManifest` class (keep for schema validation only?)

#### Phase C.5.2.6 - Remove Manifest Class Core
- [ ] Replace `Manifest` class with `OxideManifest` wrapper
- [ ] Update all 38 importing files
- [ ] Delete node storage (nodes, sources, macros dicts)
- [ ] Delete metadata storage

#### Phase C.5.2.7 - Final Cleanup
- [ ] Delete `contracts/graph/manifest.py` (1813 lines)
- [ ] Clean up `parser/manifest.py` (remove Python manifest building)
- [ ] Update `compilation.py` linker
- [ ] Update AGENTS.md with new architecture
- [ ] Delete `_sync_manifest_to_rust()` call

### C.5.3 Feature Flag (Optional)
- [ ] Add `DBT_USE_RUST_MANIFEST=1` environment variable
- [ ] Fallback to Python manifest if flag is unset
- [ ] Remove flag after 1 release cycle

## Test Cases

### Existing Python Tests to Pass (from test_manifest.py)

These **75 existing tests** MUST pass without modification after Rust integration:

| Test Class | Key Tests | What They Verify |
|------------|-----------|------------------|
| `ManifestTest` | `test_no_nodes`, `test_nested_nodes`, `test_build_flat_graph`, `test_no_nodes_with_metadata` | Empty manifest, serialization, parent/child maps, metadata |
| `MixedManifestTest` | `test_merge_from_artifact` | Compiled nodes, defer_relation |
| `TestManifestSearch` | `test_find_macro_by_name`, `test_find_materialization_by_name` | Macro lookup |
| `TestDisabledLookup` | `test_find`, `test_find_wrong_version` | Disabled node handling |
| `TestManifestFindNodeFromRefOrSource` | `test_find_node_from_ref_or_source` | Ref/source resolution |

### Required Node Keys (46 fields - from REQUIRED_PARSED_NODE_KEYS)

```python
REQUIRED_PARSED_NODE_KEYS = {
    "alias", "tags", "config", "unique_id", "refs", "sources", "metrics",
    "meta", "depends_on", "database", "schema", "name", "resource_type",
    "group", "package_name", "path", "original_file_path", "raw_code",
    "language", "description", "primary_key", "columns", "fqn", "build_path",
    "compiled_path", "patch_path", "docs", "doc_blocks", "checksum",
    "unrendered_config", "unrendered_config_call_dict", "created_at",
    "config_call_dict", "relation_name", "contract", "access", "version",
    "latest_version", "constraints", "deprecation_date", "defer_relation",
    "time_spine", "batch"
}
```

### Rust Unit Tests (New)

| Test | Description |
|------|-------------|
| `test_serialize_empty_manifest` | Empty manifest JSON matches Python |
| `test_serialize_model_all_fields` | Model with all 46 fields |
| `test_serialize_model_minimal` | Model with only required fields |
| `test_serialize_compiled_node` | compiled, extra_ctes_injected, compiled_code |
| `test_serialize_macro` | Macro with depends_on, arguments |
| `test_serialize_source` | SourceDefinition with loader, freshness |
| `test_serialize_exposure` | Exposure with owner, refs |
| `test_serialize_metric` | Metric with type_params |
| `test_serialize_semantic_model` | SemanticModel with node_relation |
| `test_serialize_saved_query` | SavedQuery with query_params |
| `test_serialize_unit_test` | UnitTestDefinition with fixtures |
| `test_serialize_generic_test` | GenericTestNode with test_metadata |
| `test_parent_child_maps` | parent_map, child_map generation |
| `test_disabled_nodes` | disabled dict serialization |
| `test_metadata_fields` | All 12 metadata fields |

### Output Compatibility Tests

| Test | Description |
|------|-------------|
| `test_json_key_ordering` | Keys sorted alphabetically (serde default) |
| `test_null_vs_omit` | `omit_none=True` behavior |
| `test_datetime_format` | `2018-02-14T09:15:13Z` format |
| `test_uuid_format` | Lowercase UUID strings |
| `test_float_precision` | `created_at` timestamp precision |

### Integration Tests (Python calling Rust)

| Test | Description |
|------|-------------|
| `test_rust_write_matches_python` | Compare file output byte-for-byte |
| `test_partial_parse_compatibility` | Manifest can be re-read by Python |
| `test_2000_nodes_performance` | < 500ms serialization |

### Test Fixtures (from utils/manifest.py)

Pre-built fixtures to port to Rust tests:
- `make_model()` - creates ModelNode
- `make_seed()` - creates SeedNode  
- `make_source()` - creates SourceDefinition
- `make_macro()` - creates Macro
- `make_generic_test()` - creates GenericTestNode
- `make_exposure()` - creates Exposure
- `make_metric()` - creates Metric
- `make_semantic_model()` - creates SemanticModel
- `make_saved_query()` - creates SavedQuery
- `make_unit_test()` - creates UnitTestDefinition

---

## Integration Points

### New PyO3 Functions

```rust
#[pyfunction]
fn write_manifest_to_file(path: &str) -> PyResult<()>;

#[pyfunction]
fn add_node(unique_id: &str, name: &str, ...) -> PyResult<()>;

#[pyfunction]
fn get_manifest_json() -> PyResult<String>;
```

### Python Changes

| File | Change |
|------|--------|
| `manifest.py` | Replace `write()` to call Rust |
| `manifest.py` | Remove `_sync_manifest_to_rust()` |
| `base.py` | Call `dbt_rs.add_node()` during parsing |

---

## Decision Required

> [!IMPORTANT]
> **Option A (Full Schema)** requires ~2-3 weeks but gives complete type safety.
> **Option B (Passthrough)** requires ~1 week but less maintainable.
>
> Which approach do you prefer?

> [!WARNING]
> **Breaking Change Risk:** If Rust writes manifest.json, it must be 
> byte-for-byte compatible with Python output for partial parsing to work.
>
> We need comprehensive output comparison tests before deployment.
