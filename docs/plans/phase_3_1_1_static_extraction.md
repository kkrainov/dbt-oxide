# Phase 3.1.1: Rust-Native Static Extraction

**Objective:** Replace dbt-extractor with unified `dbt_rs.static_extract()` using TDD.

---

## TDD Approach

> [!IMPORTANT]  
> **Tests first, then implementation.** All test cases below must be written in Rust before implementation.

### Development Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│                      TDD Cycle (per feature)                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   1. Write failing Rust tests for feature                       │
│   2. Run `cargo test` → verify tests fail                       │
│   3. Implement minimal code to pass tests                       │
│   4. Run `cargo test` → verify tests pass                       │
│   5. Refactor while keeping tests green                         │
│   6. Move to next feature                                       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Comprehensive Test Case Inventory

### 1. ref() Extraction — 15 Test Cases

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| R01 | `{{ ref('model') }}` | `{name: "model", pkg: None, ver: None}` | Basic |
| R02 | `{{ ref('pkg', 'model') }}` | `{name: "model", pkg: Some("pkg"), ver: None}` | With package |
| R03 | `{{ ref('model', v=3) }}` | `{name: "model", pkg: None, ver: Some("3")}` | Version int |
| R04 | `{{ ref('model', version=3) }}` | `{name: "model", pkg: None, ver: Some("3")}` | Version kwarg |
| R05 | `{{ ref('model', v='latest') }}` | `{name: "model", pkg: None, ver: Some("latest")}` | Version string |
| R06 | `{{ ref('pkg', 'model', v=2) }}` | `{name: "model", pkg: Some("pkg"), ver: Some("2")}` | All args |
| R07 | `{{ ref("double_quotes") }}` | `{name: "double_quotes", pkg: None, ver: None}` | Double quotes |
| R08 | `{{ ref( 'spaced' ) }}` | `{name: "spaced", pkg: None, ver: None}` | Whitespace |
| R09 | Multiple refs in template | All refs extracted | Multi-ref |
| R10 | `{{ ref() }}` | Error: no args | Invalid |
| R11 | `{{ ref('a', 'b', 'c') }}` | Error: too many | Invalid |
| R12 | `{{ ref(kwarg='bad') }}` | Error: wrong kwarg | Invalid |
| R13 | `{{ ref(['list']) }}` | Error: wrong type | Invalid |
| R14 | `{{ ref(variable) }}` | Fallback to runtime | Dynamic |
| R15 | `{% for m in x %}{{ ref(m) }}{% endfor %}` | Fallback to runtime | Dynamic loop |

---

### 2. source() Extraction — 10 Test Cases

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| S01 | `{{ source('src', 'table') }}` | `["src", "table"]` | Basic |
| S02 | `{{ source(source_name='src', table_name='tbl') }}` | `["src", "tbl"]` | Kwargs |
| S03 | `{{ source('src', table_name='tbl') }}` | `["src", "tbl"]` | Mixed |
| S04 | `{{ source("src", "table") }}` | `["src", "table"]` | Double quotes |
| S05 | Multiple sources | All extracted | Multi-source |
| S06 | `{{ source('one') }}` | Error: 1 arg | Invalid |
| S07 | `{{ source('a', 'b', 'c') }}` | Error: 3 args | Invalid |
| S08 | `{{ source(True, False) }}` | Error: wrong type | Invalid |
| S09 | `{{ source(source_name='s', BAD='t') }}` | Error: bad kwarg | Invalid |
| S10 | `{{ source(source_name='kwarg', 'positional') }}` | Error: kwargs before positional | Invalid |

---

### 3. config() Extraction — 12 Test Cases

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| C01 | `{{ config(materialized='table') }}` | `[("materialized", "table")]` | String |
| C02 | `{{ config(enabled=True) }}` | `[("enabled", true)]` | Boolean |
| C03 | `{{ config(enabled=False) }}` | `[("enabled", false)]` | Boolean false |
| C04 | `{{ config(tags=['a', 'b']) }}` | `[("tags", ["a", "b"])]` | List |
| C05 | `{{ config(meta={'k': 'v'}) }}` | `[("meta", {"k": "v"})]` | Dict |
| C06 | `{{ config(a='x', b='y') }}` | Multiple configs | Multi-kwarg |
| C07 | Nested structures | Correct extraction | Complex |
| C08 | `{{ config('positional') }}` | Error: non-kwarg | Invalid |
| C09 | `{{ config(True) }}` | Error: non-kwarg | Invalid |
| C10 | `{{ config(pre_hook='x') }}` | Excluded | Hooks excluded |
| C11 | `{{ config(post_hook='x') }}` | Excluded | Hooks excluded |
| C12 | `{{ config(pre-hook='x') }}` | Excluded | Hooks excluded |

---

### 4. macro_calls Extraction — 14 Test Cases

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| M01 | `{{ my_macro() }}` | `["my_macro"]` | Basic |
| M02 | `{{ my_macro(arg1, arg2) }}` | `["my_macro"]` | With args |
| M03 | `{{ pkg.macro_name() }}` | `["pkg.macro_name"]` | Package prefix |
| M04 | `{{ adapter.dispatch('name')() }}` | `["name"]` | Dispatch pattern |
| M05 | `{{ adapter.dispatch('name', 'pkg')() }}` | `["name", "pkg.name"]` | Dispatch with ns |
| M06 | `{{ adapter.dispatch('n', macro_namespace='pkg')() }}` | `["n", "pkg.n"]` | Dispatch kwarg |
| M07 | Multiple macros | All extracted | Multi-macro |
| M08 | `{{ ref('x') }}` | `[]` (not a macro) | Builtin excluded |
| M09 | `{{ source('a', 'b') }}` | `[]` (not a macro) | Builtin excluded |
| M10 | `{{ var('x') }}` | `[]` (not a macro) | Builtin excluded |
| M11 | `{{ config(x='y') }}` | `[]` (not a macro) | Builtin excluded |
| M12 | `{{ env_var('x') }}` | `[]` (not a macro) | Builtin excluded |
| M13 | `{{ return(nested()) }}` | `["nested"]` | Nested call |
| M14 | `{{ is_incremental() }}` | `["is_incremental"]` | Common macro |

---

### 5. Error & Edge Cases — 10 Test Cases

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| E01 | `{{ ref(` (unclosed) | Parse error, fallback | Malformed |
| E02 | `{{ True` (unclosed) | Parse error, fallback | Malformed |
| E03 | `{{` (unclosed) | Parse error, fallback | Malformed |
| E04 | `{% expression %}` | Not supported, fallback | Statement |
| E05 | `{% if x %}{{ ref('y') }}{% endif %}` | Extract ref | Conditional |
| E06 | `{{ [ref('x')] }}` | Nested function error | Nested |
| E07 | `{{ config(x=ref('y')) }}` | Nested function error | Nested |
| E08 | Plain text no Jinja | Empty extraction | No Jinja |
| E09 | SQL with embedded Jinja | Extract all | Mixed content |
| E10 | `{# comment #}` | Ignored | Comment |

---

## Implementation Checklist

### Phase 1: Test Infrastructure
- [ ] Create `src/dbt_rs/src/jinja_extractor.rs` skeleton
- [ ] Add tree-sitter dependencies to Cargo.toml
- [ ] Write all 61 test cases (failing)
- [ ] Verify `cargo test` fails with expected errors

### Phase 2: Implementation (TDD)
- [ ] Implement ref extraction → pass R01-R09
- [ ] Implement ref error handling → pass R10-R15
- [ ] Implement source extraction → pass S01-S10
- [ ] Implement config extraction → pass C01-C12
- [ ] Implement macro_calls extraction → pass M01-M14
- [ ] Implement error handling → pass E01-E10

### Phase 3: Python Integration
- [ ] Create `py_jinja_extractor.rs` bindings
- [ ] Remove dbt-extractor from pyproject.toml
- [ ] Update parser/models.py
- [ ] Update parser/unit_tests.py  
- [ ] Update clients/jinja_static.py
- [ ] Update parser/base.py

### Phase 4: Verification
- [ ] All 61 Rust tests pass
- [ ] All existing Python tests pass
- [ ] Performance benchmark shows improvement
- [ ] Golden master test passes

---

## Files to Create

| File | Purpose |
|------|---------|
| `src/dbt_rs/src/jinja_extractor.rs` | Core extraction logic with tests |
| `src/dbt_rs/src/py_jinja_extractor.rs` | PyO3 bindings |

## Files to Modify

| File | Change |
|------|--------|
| `src/dbt_rs/Cargo.toml` | Add tree-sitter deps |
| `src/dbt_rs/src/lib.rs` | Register new modules |
| `pyproject.toml` | Remove dbt-extractor |
| `core/dbt/parser/models.py` | Use dbt_rs.static_extract() |
| `core/dbt/parser/unit_tests.py` | Use dbt_rs.static_extract() |
| `core/dbt/clients/jinja_static.py` | Use dbt_rs.static_extract() |
| `core/dbt/parser/base.py` | Add static extraction for macros |
