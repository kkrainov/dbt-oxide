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

## Documentation Structure

Project documentation is organized into two directories:

### `docs/roadmap/`

High-level roadmaps describing project initiatives. Each roadmap covers:
- What has been implemented
- Current work in progress
- Future plans and milestones

Start here to understand the "why" and "what" of a project.

### `docs/plans/`

Detailed execution plans for specific phases or features. Each plan includes:
- Step-by-step implementation guides
- Code samples and architecture decisions
- Test strategies and verification checklists

Consult these for the "how" when implementing a phase.

> [!NOTE]
> Always read the relevant roadmap first for context, then the detailed plan for implementation specifics.



## Code Comments Best Practices

- **Minimalism:** Only add comments if the code logic is not self-explanatory.
- **Current State Only:** Comments must describe the *current* implementation. Do not reference deprecated approaches, previous versions, or "old" logic.
- **No Cross-References:** Avoid comments like "See function X for details".
## Development Protocols

- **Build Tool:** `maturin` is used for building Python wheels from Rust.
- **Testing:**
    - Rust: `cargo test --no-default-features` (Tests Pure Rust logic only, bypassing PyO3 linking issues)
    - Python: `uv run pytest`
    - Integration: `uv run dbt --version` (and verify oxide components are loaded)
    - Performance: `performance/runner` (Use the `performance/` framework for regression testing)
        - **Prerequisites:** `hyperfine` must be installed (e.g., `cargo install hyperfine`).
        - **Command:** `uv run cargo run --manifest-path performance/runner/Cargo.toml -- sample -p $PWD/performance/projects -b $PWD/performance/baselines -o $PWD/performance/results`


## Rust Testing Strategy

To avoid PyO3 linking errors during `cargo test`, we strictly separate logic into two layers:
1.  **Pure Rust (`OxideGraph`):** Contains all business logic (graph algos, data structures). Uses standard Rust types. **Unit tested here.**
2.  **Python Adapter (`DbtGraph`):** A thin wrapper handling PyO3 conversions. Tested via Python integration tests.

**Rule:** `cargo test` must only test the Pure Rust layer and should run without requiring a Python shared library if possible.

## Python Testing Strategy (Strict)

**CRITICAL RULE:** Changing existing Python unit tests is **STRONGLY PROHIBITED** unless a change in behavior or contract is explicitly intended and documented.
- **Contract First:** If your implementation causes a test failure, your implementation is wrong. You must fix the code to match the existing test expectations.
- **No "Fixing" Tests:** Do not modify assertions to match your new output (e.g., changing `set` to `list`).
- **Adapt Implementation:** If a test fails due to a type mismatch (e.g. `EdgeView` vs `list`), you MUST modify the implementation to support the test's expectation (e.g. implementing `__eq__` to handle lists), rather than changing the test.
- **Legacy Compatibility:** We must maintain bug-for-bug compatibility with `networkx` unless explicitly deciding to fix a bug (which requires approval).

## Python Best Practices (Strangler Pattern)

The Python codebase is a **shell layer** that wraps Rust implementations. All external interfaces must remain unchanged.

### Core Principles

1. **Preserve Original Signatures:** Python wrappers must expose the exact same method signatures, return types, and behavior as the original implementation.
2. **Hide Rust Internals:** Other `dbt-core` components must never import from `dbt_rs` directly. They interact only with Python wrappers (e.g., `dbt.graph.Graph`, not `dbt_rs.DbtGraph`).
3. **Encapsulation:** Rust types should be converted to Python-native types at the wrapper boundary (e.g., `HashSet<String>` → `set[str]`).

### Do's and Don'ts

| ✅ Do | ❌ Don't |
|-------|----------|
| Wrap Rust calls in existing Python classes | Expose `dbt_rs.*` types in public APIs |
| Convert Rust return types to match original signatures | Change method signatures to "simplify" integration |
| Handle Rust errors and convert to `DbtRuntimeError` | Let Rust panics propagate to callers |
| Keep wrapper logic minimal (delegate to Rust) | Add business logic in wrapper layers |

### Example Pattern

```python
# Good: Python wrapper hides Rust implementation
class Graph:
    def __init__(self):
        self._rust_graph = dbt_rs.DbtGraph()  # Private
    
    def ancestors(self, node: str) -> Set[str]:  # Same signature as before
        return self._rust_graph.ancestors(node)  # Delegate to Rust

# Bad: Exposing Rust types directly
from dbt_rs import DbtGraph  # Don't do this in core modules
```


## Rust Best Practices & Guidelines

### 1. Core Philosophy: Type-Driven Development
In Rust, the type system is your primary documentation and correctness enforcer.

- **Make Invalid States Unrepresentable:** Design structs and enums such that a user cannot construct an invalid instance.
    - *Bad:* `struct Connection { state: String, ... }` where state can be "connected" or "disconnected".
    - *Good:* `enum ConnectionState { Connected, Disconnected }`.
- **The "New Type" Idiom:** Strictly separate domain concepts even if they share the underlying type.
    - *Code:* `struct UserId(u64);` and `struct ProductId(u64);`. This prevents accidentally passing a product ID to a function expecting a user ID.

### 2. Ownership & Borrowing (The "Rusty" Way)
- **Prefer Borrowing over Cloning:** Avoid `.clone()` unless you explicitly need shared ownership or a distinct copy. Cloning is expensive; references are free.
- **Use Cow (Clone-on-Write):** When you might return a reference or an owned value, use `std::borrow::Cow`. It avoids allocation until mutation is absolutely necessary.
- **Interior Mutability is a Last Resort:** Do not use `RefCell` or `Mutex` just to "shut up" the borrow checker. Re-architect your data flow first. Only use them when runtime synchronization is strictly required.

### 3. Error Handling
Rust has no exceptions. Errors are values.

- **Library Code vs. Application Code:**
    - *Libraries:* Use `thiserror` to derive custom error enums. Errors should be precise so users can match on them.
    - *Applications (CLI/Web):* Use `anyhow` for easy error propagation. It handles backtraces and context (`.context("Failed to read config")`) gracefully.
- **Never unwrap() in Production:**
    - *Rule:* Strict ban on `.unwrap()` and `.expect()` in non-test code.
    - *Alternative:* Use `?` propagation, `unwrap_or`, `unwrap_or_else`, or `match`.
- **Panic is for Bugs Only:** Only panic if the program has reached an impossible state (e.g., an index out of bounds on a fixed logic array).

### 4. Function Signatures & API Design
- **Accept &str / Return String:** If a function reads a string, take `&str`. If it creates one, return `String`.
- **Use impl Trait for Arguments:**
    - *Instead of:* `fn process(path: &PathBuf)`
    - *Use:* `fn process(path: impl AsRef<Path>)`
    - *Why:* This allows the caller to pass `String`, `&str`, `Path`, or `PathBuf` without manual conversion.
- **Builder Pattern:** For structs with more than 3 distinct configuration options, implement the Builder Pattern (or use the `derive_builder` crate) instead of a constructor with many arguments.

### 5. Async Rust (Tokio/Async-std)
- **Send + Sync:** Ensure data types shared across await points implement `Send`.
- **Avoid Blocking in Async:** Never call strict blocking functions (like `std::thread::sleep` or heavy CPU computation) inside an async function. It blocks the executor thread. Use `tokio::task::spawn_blocking` for CPU-heavy work.
- **Pinning:** Understand that Futures must be pinned to be polled. If passing futures around, you may need `Box::pin`.

### 6. Testing Strategy
- **Unit Tests:** Co-located in the same file inside `#[cfg(test)] mod tests { ... }`. Test private logic here.
- **Integration Tests:** In `tests/` directory. Treat the crate as a black box.
- **Doc Tests:** Examples in documentation comments (`///`) are compiled and run as tests. This guarantees documentation never goes stale.
- **Property-Based Testing:** For critical logic, use `proptest`. It generates thousands of random inputs to find edge cases.

### 7. The "Elite" Toolset (Standard Dependencies)
Don't reinvent the wheel. Use the "blessed" crates that the community has standardized on:
- **Serialization:** `serde` (with derive feature).
- **Async Runtime:** `tokio`.
- **HTTP Client:** `reqwest`.
- **CLI Arguments:** `clap` (derive mode).
- **Logging:** `tracing` (structured logging) + `tracing-subscriber`.
- **Static Variables:** `once_cell` or `lazy_static`.
- **Error Handling:** `thiserror` (libs), `anyhow` (apps).

### 8. Formatting & Linting (Strict Mode)
- **Clippy is Law:** The code must pass `cargo clippy -- -D warnings`. This treats all linter warnings as errors.
- **Format:** `cargo fmt` must be applied.
