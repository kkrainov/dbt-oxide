# dbt-oxide Performance Profiling Guide

**Purpose:** Systematic approach to identify performance bottlenecks in dbt-oxide's hybrid Python/Rust architecture.

---

## Executive Summary

After Phase 2.5 (Unified Data Layer), dbt-oxide shows **worse performance** than vanilla dbt-core:
- `dbt ls`: 2-11% slower
- `dbt ls --select`: 1.5-9.5% slower  
- `dbt parse`: ~1% slower

This guide provides profiling strategies to identify the root causes.

---

## Critical Finding: Double Serialization Bottleneck

> [!CAUTION]
> **Immediate Issue Identified**
> 
> The current implementation performs **double JSON serialization** of the manifest:
> 1. `_sync_manifest_to_rust()` in [manifest.py](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/core/dbt/parser/manifest.py#L531-559): Serializes manifest to JSON → Load into Rust storage
> 2. `link_graph()` in [compilation.py](file:///Users/kirillkrainov/apps/pet_projects/dbt-oxide/core/dbt/compilation.py#L189-195): **Repeats the same serialization** → Build graph from JSON
>
> This is a **major design flaw** that defeats the purpose of Phase 2 (Zero-Copy Manifest).

### Data Flow (Current - Inefficient)

```
Python: Manifest.writable_manifest() → to_dict() → json.dumps() [500+ ms for large projects]
    ↓ 
Rust: dbt_rs.load_manifest() → OxideManifest in OnceCell
    ↓
Python: Manifest.writable_manifest() → to_dict() → json.dumps() [500+ ms REPEATED!]
    ↓
Rust: dbt_rs.build_graph_from_manifest_json() → DbtGraph
```

### Data Flow (Expected - Efficient)

```
Python: Manifest.writable_manifest() → to_dict() → json.dumps() [Once]
    ↓ 
Rust: dbt_rs.load_manifest() → OxideManifest in OnceCell
    ↓
Rust: dbt_rs.build_graph_from_global_manifest() → DbtGraph [Uses stored manifest]
```

---

## Profiling Tools

### 1. py-spy (Recommended for PyO3)

**Best for:** Hybrid Python/Rust applications. Low overhead, no code changes required.

```bash
# Install
pip install py-spy

# Profile dbt parse (sampling profiler)
py-spy record -o profile.svg --native -- uv run dbt parse

# Profile dbt ls
py-spy record -o profile_ls.svg --native -- uv run dbt ls

# Top-like view (real-time)
py-spy top -- uv run dbt parse
```

The `--native` flag is crucial - it shows time spent in Rust/C extensions.

### 2. cProfile + snakeviz (Python Focused)

**Best for:** Detailed function-level analysis in Python code.

```bash
# Run with cProfile
uv run python -m cProfile -o dbt_profile.prof -m dbt parse

# Visualize
uv run pip install snakeviz
uv run snakeviz dbt_profile.prof
```

### 3. time.perf_counter Instrumentation

**Best for:** Targeted measurement of specific code sections.

```python
import time

start = time.perf_counter()
# Code to measure
elapsed_ms = (time.perf_counter() - start) * 1000
print(f"Operation took {elapsed_ms:.1f}ms")
```

### 4. Scalene (CPU + Memory)

**Best for:** Understanding memory allocation patterns alongside CPU usage.

```bash
pip install scalene
scalene --cpu --memory -- uv run dbt parse
```

---

## Profiling Targets

### High Priority

| Component | File | Method | Expected Bottleneck |
|-----------|------|--------|---------------------|
| Manifest Serialization | `manifest.py` | `writable_manifest().to_dict()` | JSON serialization overhead |
| Graph Building | `compilation.py` | `link_graph()` | Double serialization |
| Rust Deserialization | `py_data_layer.rs` | `build_graph_from_manifest_json()` | serde_json parsing |

### Medium Priority

| Component | File | Method | Potential Issue |
|-----------|------|--------|-----------------|
| Graph Operations | `graph.py` | Various | PyO3 call overhead |
| Test Edge Construction | `compilation.py` | `add_test_edges_*()` | Graph iteration patterns |

---

## Instrumented Profiling Script

Create `performance/scripts/profile_dbt.py`:

```python
#!/usr/bin/env python
"""Profile dbt operations with detailed timing."""

import cProfile
import pstats
import time
import sys
import os

# Add project root to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'core'))


def profile_parse():
    """Profile dbt parse with timing breakdowns."""
    import dbt.main as dbt_main
    
    # Monkey-patch key functions for timing
    timings = {}
    
    # Profile ManifestLoader.load
    from dbt.parser.manifest import ManifestLoader
    original_load = ManifestLoader.load
    
    def timed_load(self):
        start = time.perf_counter()
        result = original_load(self)
        timings['manifest_load'] = (time.perf_counter() - start) * 1000
        return result
    
    ManifestLoader.load = timed_load
    
    # Profile _sync_manifest_to_rust
    original_sync = ManifestLoader._sync_manifest_to_rust
    
    def timed_sync(self):
        start = time.perf_counter()
        result = original_sync(self)
        timings['rust_sync'] = (time.perf_counter() - start) * 1000
        return result
    
    ManifestLoader._sync_manifest_to_rust = timed_sync
    
    # Profile Linker.link_graph
    from dbt.compilation import Linker
    original_link = Linker.link_graph
    
    def timed_link(self, manifest):
        start = time.perf_counter()
        result = original_link(self, manifest)
        timings['link_graph'] = (time.perf_counter() - start) * 1000
        return result
    
    Linker.link_graph = timed_link
    
    # Run dbt parse
    sys.argv = ['dbt', 'parse']
    
    profiler = cProfile.Profile()
    profiler.enable()
    
    try:
        dbt_main.main()
    except SystemExit:
        pass
    
    profiler.disable()
    
    # Print timing summary
    print("\n" + "="*60)
    print("TIMING BREAKDOWN")
    print("="*60)
    for name, ms in sorted(timings.items(), key=lambda x: -x[1]):
        print(f"  {name}: {ms:.1f}ms")
    
    # Print top functions
    print("\n" + "="*60)
    print("TOP 20 FUNCTIONS BY TIME")
    print("="*60)
    stats = pstats.Stats(profiler)
    stats.strip_dirs()
    stats.sort_stats('cumulative')
    stats.print_stats(20)


if __name__ == '__main__':
    profile_parse()
```

---

## Recommended Investigation Steps

### Step 1: Confirm Double Serialization

Add timing to both serialization points:

```python
# In manifest.py _sync_manifest_to_rust():
start_serialize = time.perf_counter()
writable = self.manifest.writable_manifest()
json_str = json.dumps(writable.to_dict(omit_none=False, context={"artifact": True}))
serialize_ms = (time.perf_counter() - start_serialize) * 1000
print(f"[_sync_manifest_to_rust] Serialize: {serialize_ms:.1f}ms, JSON size: {len(json_str)//1024}KB")
```

```python
# In compilation.py link_graph():
start_serialize = time.perf_counter()
writable = manifest.writable_manifest()
json_str = json.dumps(writable.to_dict(omit_none=False, context={"artifact": True}))
serialize_ms = (time.perf_counter() - start_serialize) * 1000
print(f"[link_graph] Serialize: {serialize_ms:.1f}ms, JSON size: {len(json_str)//1024}KB")
```

### Step 2: Profile with py-spy

```bash
cd /Users/kirillkrainov/apps/pet_projects/dbt-oxide

# Profile against largest test project
cd performance/projects/03_2000_chain_models
py-spy record -o ../../../profile_parse.svg --native -- uv run dbt parse
```

### Step 3: Compare Vanilla vs dbt-oxide

```bash
# Create comparison profiles
# Vanilla dbt-core
workon vanilla-dbt  # or activate vanilla venv
py-spy record -o profile_vanilla.svg --native -- dbt parse

# dbt-oxide
uv run py-spy record -o profile_oxide.svg --native -- uv run dbt parse
```

---

## Optimization Roadmap

Based on profiling findings:

### Immediate Fix (High Impact)

**Eliminate double serialization** by modifying `link_graph()` to use stored manifest:

```python
# compilation.py - PROPOSED FIX
def link_graph(self, manifest: Manifest):
    # Use Rust function that builds graph from already-loaded manifest
    self.graph = Graph.from_global_manifest()  # No JSON serialization!
    cycle = self.find_cycles()
    if cycle:
        raise RuntimeError(f"Found a cycle: {cycle}")
```

This requires adding `build_graph_from_global_manifest()` to Rust:

```rust
// py_data_layer.rs
#[pyfunction]
pub fn build_graph_from_global_manifest() -> PyResult<DbtGraph> {
    let manifest_guard = GLOBAL_MANIFEST.get()
        .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Manifest not loaded"))?
        .read()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Lock poisoned"))?;
    
    let graph = build_graph_from_manifest(&manifest_guard);
    Ok(DbtGraph::from_oxide_graph(graph))
}
```

### Medium-Term Optimizations

1. **Replace JSON with direct PyO3 object binding** (eliminates serialization entirely)
2. **Use `orjson` instead of `json.dumps`** (3-5x faster JSON serialization)
3. **Lazy manifest sync** (only sync when Rust operations are needed)

---

## Benchmarking Commands

```bash
# Full comparison suite
./performance/scripts/run_comparison.sh

# Quick single-project benchmark
cd performance/projects/03_2000_chain_models
hyperfine --warmup 1 --runs 3 'uv run dbt parse' 'uv run dbt ls'

# Compare specific operations
hyperfine --warmup 1 \
    -n 'vanilla' 'dbt parse' \
    -n 'oxide' 'uv run dbt parse'
```

---

## References

- [py-spy GitHub](https://github.com/benfred/py-spy) - Sampling profiler for Python
- [PyO3 Performance Guide](https://pyo3.rs/v0.22.6/performance) - Official PyO3 optimization guide
- [orjson](https://github.com/ijl/orjson) - Fast JSON serialization for Python
