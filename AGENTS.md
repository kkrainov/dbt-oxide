# Project Context: dbt-oxide

**Codename:** dbt-oxide
**Base Repository:** Fork of `dbt-labs/dbt-core` (v1.10.16)
**Architecture:** Strangler Fig Pattern (Python Shell, Rust Core)
**Goal:** Substantial performance improvement in compile/DAG resolution through Rust-powered graph algorithms and zero-copy data structures. 100% backward compatibility.

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
- **Rust Code Quality:** **ALWAYS run these after making Rust code changes:**
  - `cargo fmt --all` - Format code (CI enforces this)
  - `cargo clippy --no-default-features -- -D warnings` - Lint code (CI enforces this)
- **Testing:**
    - Rust: `cargo test --no-default-features` (Tests Pure Rust logic only, bypassing PyO3 linking issues)
    - Python: `uv run pytest`
    - Integration: `uv run dbt --version` (and verify oxide components are loaded)
    - Performance: `performance/runner` (Use the `performance/` framework for regression testing)
        - **Prerequisites:** `hyperfine` must be installed (e.g., `cargo install hyperfine`).
        - **Command:** `uv run cargo run --manifest-path performance/runner/Cargo.toml -- sample -p $PWD/performance/projects -b $PWD/performance/baselines -o $PWD/performance/results`
- **Python Code Quality:**
  - `uv run ruff check core/dbt` - Lint code (CI enforces this)
  - `uv run ruff format core/dbt` - Format code (CI enforces this)
  - `uv run mypy core/dbt` - Type checking (optional, IDE integrated)
  - `pre-commit run --all-files` - Run all pre-commit hooks locally


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

---

## Git Workflow (Fork Repository)

This project is a **fork** of `dbt-labs/dbt-core`. We have two remotes configured:

| Remote | Repository | Purpose |
|--------|------------|---------|
| `origin` | `kkrainov/dbt-oxide` | **Our fork** - push all changes here |
| `upstream` | `dbt-labs/dbt-core` | Upstream dbt-core - read-only, for syncing updates |

### Critical Rules

> [!CAUTION]
> **NEVER push to upstream.** Always push to `origin` (the fork).
> **NEVER pull from upstream into main.** Only sync upstream manually when needed.

### Standard Git Operations

**Committing changes:**
```bash
# Stage ALL related files (don't forget implementation files!)
git add <all-related-files>

# Commit with conventional message
git commit -m "<type>: <short description>

<detailed body if needed>"
```

**Pushing to fork:**
```bash
# Push current branch to fork
git push origin HEAD

# Push main to fork
git push origin main
```

**Pulling latest from fork (not upstream!):**
```bash
# Fetch from fork
git fetch origin

# Reset to match fork's main
git reset --hard origin/main
```

**Syncing from upstream (rare, manual only):**
```bash
# Only when explicitly updating from dbt-core
git fetch upstream
git merge upstream/main --no-commit
# Resolve conflicts, then commit
```

### Commit Message Convention

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>: <short description>

<optional body with details>
```

**Types:**
| Type | Use For |
|------|---------|
| `feat` | New features, implementations (Rust structs, Python methods) |
| `fix` | Bug fixes |
| `docs` | Documentation only (roadmaps, plans, README) |
| `refactor` | Code restructuring without behavior change |
| `test` | Adding or modifying tests |
| `chore` | Build, CI, tooling changes |
| `perf` | Performance improvements |

**Examples:**

```bash
# Feature implementation
git commit -m "feat: implement Phase 2 Zero-Copy Manifest infrastructure

Rust implementation (src/dbt_rs/):
- manifest.rs: OxideManifest with full schema
- py_manifest.rs: PyO3 bindings with load_manifest(), get_node_count()
- Thread-safe global storage with OnceCell<RwLock<>>

Python implementation (core/dbt/parser/manifest.py):
- _sync_manifest_to_rust() method in ManifestLoader
- Eager sync at end of load() with fail-fast error handling"

# Documentation update
git commit -m "docs: clarify Phase 2 implementation status"

# Bug fix
git commit -m "fix: handle missing optional fields in OxideNode deserialization"
```

### Changelog Generation

dbt-oxide uses [git-cliff](https://git-cliff.org/) to automatically generate changelogs from conventional commits.

**How it works:**
1. Commits following conventional format are parsed automatically
2. Changelog entries are grouped by type (Features, Bug Fixes, etc.)
3. Run `git cliff --output CHANGELOG.md` to regenerate the changelog

**Manual changelog update:**
```bash
# Update changelog for unreleased changes
git cliff --unreleased --prepend CHANGELOG.md

# Generate changelog for a new version
git cliff --tag v0.2.0 --prepend CHANGELOG.md
```

**CI Integration:** Changelog is automatically updated on releases via GitHub Actions.

> [!TIP]
> Write meaningful commit messages - they become your changelog entries!

### Commit Checklist

Before committing, verify:
1. ✅ All related files are staged (implementation + docs + tests)
2. ✅ Commit message follows conventional format
3. ✅ Body describes Rust AND Python changes if both modified
4. ✅ No references to future phases (focus on what's done)
5. ✅ Tests pass: `cargo test --no-default-features` and `uv run pytest`
