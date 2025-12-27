# Rust Architecture Guidelines (dbt_rs)

## Core Architecture

### Component Separation

```
┌─────────────────────────────────────────────────────────────┐
│                     Python Layer                            │
│    Manifest, ManifestLoader, Graph (wrappers only)          │
└──────────────────────────┬──────────────────────────────────┘
                           │ PyO3 Bindings
┌──────────────────────────▼──────────────────────────────────┐
│                      Rust Layer                             │
│  ┌──────────────┐    ┌─────────────┐    ┌──────────────┐   │
│  │ OxideManifest│    │  OxideGraph │    │ PyO3 Adapters│   │
│  │  (data)      │ ──▶│ (traversal) │◀── │ (bindings)   │   │
│  └──────────────┘    └─────────────┘    └──────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

**OxideManifest** = Data storage (nodes, sources, metadata, parent/child maps)
**OxideGraph** = Graph traversal algorithms (ancestors, descendants, topological sort)
**PyO3 Adapters** = Python bindings (DbtGraph, ManifestBuilder)

### Key Principle: Separation of Concerns

**Manifest stores data, Graph handles traversal.**

- `build_parent_map()` and `build_child_map()` are simple iterations over `depends_on`
- They do NOT create or use OxideGraph internally
- Graph is a separate component created from Manifest JSON when traversal is needed

## Module Structure

```
src/dbt_rs/src/
├── lib.rs              # PyO3 module exports
├── graph.rs            # OxideGraph implementation
├── py_graph.rs         # DbtGraph PyO3 wrapper
├── py_manifest_builder.rs  # ManifestBuilder PyO3 wrapper
├── manifest/
│   ├── mod.rs          # Module re-exports
│   ├── resources.rs    # OxideManifest + helper types
│   └── types.rs        # Node types (OxideModel, OxideNode, etc.)
└── tests/
    ├── mod.rs          # Test module registry
    ├── helpers.rs      # Test builders (ModelBuilder, etc.)
    ├── logic.rs        # Unit tests for manifest logic
    └── manifest_maps.rs # Tests for parent/child map building
```

## Testing Guidelines

### Two-Layer Testing

1. **Pure Rust Tests** (`cargo test --no-default-features`)
   - Test OxideGraph, OxideManifest logic
   - Use test helpers (ModelBuilder, SourceBuilder)
   - No Python dependency

2. **Python Integration Tests** (`uv run pytest`)
   - Test PyO3 bindings
   - Test Python ↔ Rust interop

### Test Helpers

Use builders for consistent test setup:

```rust
use super::helpers::{ModelBuilder, SourceBuilder};

let model = ModelBuilder::new("my_model", "my_pkg")
    .depends_on(vec!["model.my_pkg.other"])
    .with_group("analytics")
    .build();
```

## Code Quality

Before committing:
```bash
cargo fmt --all
cargo clippy --no-default-features -- -D warnings
cargo test --no-default-features
```

### Import Best Practices

**All `use` statements MUST be at module level (top of file), never inside functions.**

```rust
// ✅ CORRECT - imports at top of file
use crate::manifest::{OxideManifest, OxideMacro};
use pythonize::depythonize;
use std::collections::HashMap;

pub fn my_function() {
    let map: HashMap<String, OxideMacro> = HashMap::new();
}

// ❌ WRONG - imports inside function
pub fn my_function() {
    use std::collections::HashMap;  // BAD!
    use pythonize::depythonize;     // BAD!
    let map: HashMap<String, i32> = HashMap::new();
}
```

This follows Rust best practices and keeps code maintainable.

## PyO3 Patterns

### Returning Python Iterables

```rust
#[pymethods]
impl DbtGraph {
    fn nodes(&self) -> HashSet<String> {
        self.inner.nodes()  // PyO3 auto-converts to Python set
    }
}
```

### Error Handling

```rust
pub fn add_node(&mut self, node: &PyDict) -> PyResult<()> {
    let node_rust: OxideNode = depythonize(node)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Failed to parse node: {}", e)
        ))?;
    self.manifest.add_node(node_rust);
    Ok(())
}
```

## Common Patterns

### Parent/Child Map Building (Option A)

Simple iteration matching Python - NO graph building:

```rust
pub fn build_parent_map(&self) -> HashMap<String, Vec<String>> {
    let mut parent_map = HashMap::new();
    for (unique_id, node) in &self.nodes {
        parent_map.insert(unique_id.clone(), node.depends_on().nodes.clone());
    }
    // Add sources, exposures, metrics...
    parent_map
}
```

### Accessing Node Dependencies

Use the `depends_on()` method for nodes (returns DependsOn struct):
```rust
let deps = node.depends_on().nodes;  // Returns Vec<String>
```

For exposures/metrics, access field directly:
```rust
let deps = exposure.depends_on.nodes.clone();
```
