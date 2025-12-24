# dbt-oxide Profiling Report

**Date:** 2025-12-23  
**Test Project:** 03_2000_chain_models (2000 models with ref() dependencies)  
**Command:** `dbt parse`  
**dbt-oxide version:** 1.10.16 + Phase 2.5

---

## Executive Summary

Total execution time: **15.98 seconds**

The profiling reveals that:
1. **Parsing dominates** - 69% of time is spent parsing SQL files
2. **Rust deserialization is efficient** - Only 10ms to load manifest in Rust
3. **Python serialization is the overhead** - 675ms spent preparing data for Rust

---

## Performance Breakdown

### Top Functions by Cumulative Time

| Rank | Function | Time | % | Description |
|------|----------|------|---|-------------|
| 1 | `parse_project` | 11.13s | 69.7% | Parsing all SQL/YAML files |
| 2 | `validate` (jsonschema) | 5.39s | 33.7% | Schema validation |
| 3 | `update_parsed_node_config` | 5.45s | 34.1% | Config resolution |
| 4 | `parse_versioned_tests` | 4.22s | 26.4% | Test parsing |
| 5 | `build_config_dict` | 4.26s | 26.6% | Config building |
| 6 | `call_macro` | 2.91s | 18.2% | Jinja macro execution |
| 7 | `write_manifest` | 1.97s | 12.3% | Writing manifest.json |
| 8 | `json.dump` | 1.38s | 8.7% | JSON serialization |

### Rust/Oxide-Specific Timing

| Function | Time | % | Location |
|----------|------|---|----------|
| `_sync_manifest_to_rust` | **675ms** | 4.2% | manifest.py:531 |
| `writable_manifest` | 794ms | 5.0% | manifest.py:1206 |
| `build_flat_graph` | 144ms | 0.9% | manifest.py:950 |
| `dbt_rs.load_manifest` | **10ms** | 0.06% | Rust extension |

---

## Key Insights

### 1. Rust Deserialization is NOT the Bottleneck

```
Python serialization:  ~665ms (writable_manifest + json.dumps)
Rust deserialization:   ~10ms (load_manifest in Rust)
```

The Rust side is **66x faster** than the Python serialization overhead.

### 2. Where Time is Actually Spent

```
Parsing SQL/YAML:     11.1s  (69%)
Schema validation:     5.4s  (34%)
Config resolution:     5.4s  (34%)
Writing artifacts:     2.0s  (12%)
Rust sync:            0.7s   (4%)
```

### 3. build_flat_graph is Minor

At only 144ms (0.9%), optimizing `build_flat_graph` with lazy serialization would save minimal time for `dbt parse`.

---

## Serialization Overhead Analysis

The `_sync_manifest_to_rust` function breakdown:

```python
# In manifest.py:531
def _sync_manifest_to_rust(self):
    writable = self.manifest.writable_manifest()  # ~400ms
    json_str = json.dumps(writable.to_dict(...))  # ~265ms
    dbt_rs.load_manifest(json_str)                # ~10ms
```

### Optimization Opportunities

| Optimization | Potential Savings | Complexity |
|--------------|-------------------|------------|
| Use `orjson` instead of `json.dumps` | ~200ms | Low |
| Cache `writable_manifest()` result | ~400ms | Medium |
| Skip sync for small projects | 675ms | Low |
| Direct PyO3 object binding | 665ms | High |

---

## Comparison: dbt-oxide Overhead

For `dbt parse` (15.98s total):
- Rust sync overhead: 675ms (**4.2%** of total)
- Acceptable for large projects

For `dbt ls` (typically ~4-5s):
- Rust sync overhead: 675ms (**15-17%** of total)
- Significant impact - optimization needed

---

## Recommendations

### Immediate (Low Effort)

1. **Skip Rust sync for small projects** (< 500 nodes)
   - Projects under 500 nodes don't benefit from Rust graph operations
   - Saves 675ms for small projects

2. **Use orjson for JSON serialization**
   - 3-5x faster than standard `json.dumps`
   - Simple drop-in replacement

### Medium-Term

3. **Lazy flat_graph serialization**
   - Only serialize when Jinja actually accesses `graph.nodes`
   - Most commands don't use it

4. **Cache writable_manifest result**
   - Called twice during load() - cache the result

### Long-Term (Phase 3+)

5. **Direct PyO3 object binding**
   - Eliminate JSON serialization entirely
   - Pass Python objects directly to Rust

---

## Raw Profile Data

Profile saved to: `performance/results/profile_parse.prof`

To view interactively:
```bash
uv run snakeviz performance/results/profile_parse.prof
```

---

## Appendix: Top 20 Functions by Internal Time

| Function | Time | Calls | Description |
|----------|------|-------|-------------|
| `_iterencode_dict` | 794ms | 5.3M | JSON encoding |
| `descend` | 779ms | 617K | JSON schema validation |
| `evolve` | 573ms | 545K | Validator evolution |
| `read` | 445ms | 4.2K | File I/O |
| `__getitem__` (os) | 371ms | 1.3M | Environment access |
| `__attrs_post_init__` | 309ms | 561K | Attrs initialization |
| `__iter__` | 279ms | 1.3M | Iterator protocol |
| `type` (keywords) | 277ms | 495K | Type checking |
| `isinstance` | 260ms | 6.0M | Type checking |
| `dict.get` | 250ms | 5.7M | Dictionary access |
