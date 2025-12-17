# Project Context: dbt-oxide

**Codename:** dbt-oxide
**Base Repository:** Fork of `dbt-labs/dbt-core` (v1.10.16)
**Architecture:** Strangler Fig Pattern (Python Shell, Rust Core)
**Goal:** 10x-100x performance improvement in compile/DAG resolution. 100% backward compatibility.

## Core Mandates

1.  **The Strangler Pattern:** We replace internal Python components with Rust implementations one by one. The Python API surface must remain identical to consumers.
2.  **Performance First:** Every Rust replacement must benchmark significantly faster than the Python equivalent.
3.  **Safety:** Use `pyo3` for bindings. Ensure thread safety when moving to parallel execution (Rayon).
4.  **No Regressions:** "Golden Master" tests must pass. Output SQL must be byte-for-byte identical to `dbt-core`.
5.  **Development Environment:**
    - Strictly use `uv` for Python virtual environment management and package installation.
    - Build backend: `maturin`.

## Roadmap Status

- [x] **Phase 0: Foundation & Infrastructure** (Current Focus)
    - Hybrid build environment (Maturin + PyO3).
    - Rust workspace in `src/dbt_rs`.
- [ ] **Phase 1: The Graph Engine**
    - Replace `networkx` with `petgraph`.
- [ ] **Phase 2: Zero-Copy Manifest**
    - Share Manifest state without serialization overhead.
- [ ] **Phase 3: The Compiler Engine**
    - Replace Jinja2 with `minijinja`.
- [ ] **Phase 4: Parallelization & Introspection**
    - Rayon integration and introspection separation.

## Development Protocols

- **Build Tool:** `maturin` is used for building Python wheels from Rust.
- **Testing:**
    - Rust: `cargo test`
    - Python: `uv run pytest`
    - Integration: `uv run dbt --version` (and verify oxide components are loaded)
