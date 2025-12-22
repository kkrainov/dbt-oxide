# Contributing to dbt-oxide

Thank you for your interest in contributing to dbt-oxide! This document provides guidelines for contributing to this high-performance Rust-accelerated fork of dbt-core.

---

## Table of Contents

1. [About dbt-oxide](#about-dbt-oxide)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Testing](#testing)
5. [Code Standards](#code-standards)
6. [Submitting Changes](#submitting-changes)
7. [Project Structure](#project-structure)

---

## About dbt-oxide

dbt-oxide is a performance-focused fork of [dbt-core](https://github.com/dbt-labs/dbt-core) that replaces critical Python components with Rust implementations using the Strangler Fig pattern.

**Key principles:**
- **100% backward compatibility** - existing dbt projects work without modification
- **Rust for performance** - graph algorithms, manifest parsing, data structures
- **Python for compatibility** - maintain the dbt API surface
- **Performance first** - every change should benchmark faster

---

## Getting Started

### Prerequisites

- **Python 3.9+** - dbt-oxide supports Python 3.9, 3.11, and 3.12
- **Rust 1.92+** - for building the Rust extensions
- **uv** - modern Python package manager (replaces pip/virtualenv)
- **PostgreSQL** (optional) - for running integration tests

### Installation

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/dbt-oxide.git
cd dbt-oxide

# Install uv (if not already installed)
pip install uv

# Install all dependencies (dbt-core, pytest, dbt-postgres, etc.)
uv sync --group dev

# Build and install the Rust extension
uv pip install -e .
```

### Verify Setup

```bash
# Check dbt-oxide version
uv run dbt --version

# Run Rust tests
cargo test --no-default-features --manifest-path src/dbt_rs/Cargo.toml

# Run Python unit tests
uv run pytest tests/unit -v
```

---

## Development Workflow

### 1. Fork and Branch

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/dbt-oxide.git
cd dbt-oxide

# Add upstream remote
git remote add upstream https://github.com/kkrainov/dbt-oxide.git

# Create a feature branch
git checkout -b feature/my-awesome-feature
```

### 2. Make Changes

**For Rust code:**
```bash
# Work in src/dbt_rs/
vim src/dbt_rs/src/your_file.rs

# Format code
cargo fmt --manifest-path src/dbt_rs/Cargo.toml

# Check with Clippy
cargo clippy --no-default-features --manifest-path src/dbt_rs/Cargo.toml -- -D warnings

# Run tests
cargo test --no-default-features --manifest-path src/dbt_rs/Cargo.toml
```

**For Python code:**
```bash
# Work in core/dbt/
vim core/dbt/your_file.py

# Format and lint code
uv run ruff check core/dbt --fix
uv run ruff format core/dbt

# Type check (optional)
uv run mypy core/dbt

# Rebuild Rust extension if you changed Rust code
uv pip install -e .

# Run Python tests
uv run pytest tests/unit -v
```

### 3. Sign Your Commits (DCO)

dbt-oxide requires **Developer Certificate of Origin (DCO)** sign-offs on all commits. This certifies that you have the right to submit the code.

```bash
# Sign your commits with -s flag
git commit -s -m "feat: add awesome feature"

# Or configure git to always sign
git config --global format.signoff true
```

**What this adds to your commit:**
```
feat: add awesome feature

Signed-off-by: Your Name <your.email@example.com>
```

This sign-off certifies you agree to the [Developer Certificate of Origin](DCO).

### 4. Keep Your Branch Updated

```bash
# Fetch upstream changes
git fetch upstream

# Rebase your branch
git rebase upstream/main

# Force push to your fork (if already pushed)
git push origin feature/my-awesome-feature --force-with-lease
```

---

## Testing

### Rust Tests

```bash
# Run all Rust tests
cargo test --no-default-features --manifest-path src/dbt_rs/Cargo.toml

# Run specific test
cargo test --no-default-features test_name --manifest-path src/dbt_rs/Cargo.toml

# Run with output
cargo test --no-default-features --manifest-path src/dbt_rs/Cargo.toml -- --nocapture
```

**Why `--no-default-features`?** This avoids PyO3 linking issues during pure Rust testing.

### Python Tests

```bash
# Run all unit tests
uv run pytest tests/unit

# Run specific test file
uv run pytest tests/unit/test_graph.py

# Run specific test
uv run pytest tests/unit/test_graph.py::TestGraph::test_ancestors -v

# Run with coverage
uv run pytest tests/unit --cov=dbt --cov-report=html
```

### Performance Tests

```bash
# Run performance benchmarks (requires hyperfine)
cargo install hyperfine
uv run cargo run --manifest-path performance/runner/Cargo.toml -- sample \
  -p $PWD/performance/projects \
  -b $PWD/performance/baselines \
  -o $PWD/performance/results
```

---

## Code Standards

### Rust Code

**Style:**
- Use `cargo fmt` (enforced in CI)
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Pass `cargo clippy` with `-D warnings`

**Rules:**
- No `.unwrap()` or `.expect()` in production code (use `?` or `match`)
- Borrow over clone unless necessary
- Use `thiserror` for errors
- Write doc comments for public APIs
- Add unit tests for new functions

**Example:**
```rust
/// Calculates ancestors of a node in the graph.
///
/// # Arguments
/// * `node` - The node to start from
///
/// # Returns
/// Set of ancestor node IDs
pub fn ancestors(&self, node: &str) -> Result<HashSet<String>> {
    self.graph.get_ancestors(node)
        .context("Failed to calculate ancestors")?
}
```

### Python Code

**Style:**
- Use `uv run ruff check core/dbt --fix` for linting (enforced in CI)
- Use `uv run ruff format core/dbt` for code formatting (enforced in CI)
- Write type hints for new/modified functions
- Follow existing code patterns (this is a wrapper layer)
- Keep wrapper logic minimal - delegate to Rust

**Pre-commit hooks:** Install with `pre-commit install`. Runs automatically on commit:
- `ruff` - Linting with auto-fix
- `ruff-format` - Code formatting
- `mypy` - Type checking
- Standard checks (yaml, json, trailing whitespace)

**Rules:**
- Don't modify existing unit tests unless changing behavior
- Don't import `dbt_rs` directly in core dbt code
- Convert Rust types to Python types at wrapper boundary
- Handle Rust errors and convert to `DbtRuntimeError`

**Example:**
```python
def ancestors(self, node: str) -> Set[str]:
    """Get ancestors of a node."""
    try:
        return self._rust_graph.ancestors(node)
    except Exception as e:
        raise DbtRuntimeError(f"Failed to get ancestors: {e}")
```

---

## Submitting Changes

### 1. Open a Pull Request

- **Title:** Use [Conventional Commits](https://www.conventionalcommits.org/) format
  - `feat: add graph parallel processing`
  - `fix: handle empty manifest edge case`
  - `docs: update Rust architecture section`

- **Description:** Fill out the PR template
  - Link to related issue
  - Describe the problem and solution
  - Include performance benchmarks if applicable

### 2. PR Checklist

Before submitting, ensure:

- [ ] Code follows style guidelines (cargo fmt, ruff check, ruff format, type hints)
- [ ] All tests pass locally
- [ ] Commits are signed with DCO (`git commit -s`)
- [ ] Rust code passes clippy
- [ ] Python code has type annotations
- [ ] Added tests for new functionality
- [ ] Updated documentation if needed

### 3. Code Review Process

1. **CI checks** - Must pass (Rust checks, Python tests, DCO)
2. **Code review** - A maintainer will review your code
3. **Feedback** - Address any requested changes
4. **Approval** - Once approved, a maintainer will merge

**Review time:** We aim to review PRs within 1 week.

---

## Release Process

### Version Numbering

dbt-oxide follows [Semantic Versioning](https://semver.org/):
- **MAJOR.MINOR.PATCH** (e.g., `0.2.0`)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Creating a Release

Releases are created by maintainers through GitHub Releases:

1. **Ensure CHANGELOG.md is up to date**
   ```bash
   git cliff --tag v0.2.0 --prepend CHANGELOG.md
   git commit -m "chore: update changelog for v0.2.0"
   ```

2. **Create and push tag**
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```

3. **GitHub Actions will automatically:**
   - Build wheels for all platforms
   - Create GitHub Release with binaries
   - Publish to PyPI (if configured)

### Changelog is Auto-Generated

Thanks to **git-cliff**, the changelog is generated from commit messages. This is why following [Conventional Commits](https://www.conventionalcommits.org/) format is important!

**Example:**
```
feat: add parallel graph processing
fix: handle empty manifest edge case
docs: update architecture documentation
```

These commits automatically become changelog entries:
- `feat:` → **Features** section
- `fix:` → **Bug Fixes** section
- `docs:` → **Documentation** section

---

## Project Structure

```
dbt-oxide/
├── core/dbt/           # Python code (wrapper layer)
│   ├── graph/          # Graph wrapper (uses dbt_rs)
│   ├── parser/         # Manifest parser (uses dbt_rs)
│   └── ...
├── src/dbt_rs/         # Rust code (core engine)
│   ├── src/
│   │   ├── graph.rs    # Graph algorithms
│   │   ├── manifest.rs # Manifest parsing
│   │   └── ...
│   └── Cargo.toml
├── tests/
│   ├── unit/           # Python unit tests
│   └── functional/     # Integration tests
├── performance/        # Performance benchmarks
└── docs/
    ├── roadmap/        # Development roadmap
    └── plans/          # Detailed implementation plans
```

---

## Communication

- **Issues:** Report bugs or request features via [GitHub Issues](https://github.com/kkrainov/dbt-oxide/issues)
- **Discussions:** Ask questions in [GitHub Discussions](https://github.com/kkrainov/dbt-oxide/discussions)
- **Questions:** For quick questions, open a discussion

---

## Additional Resources

- [AGENTS.md](AGENTS.md) - Detailed development protocols
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [docs/roadmap/rust_migration.md](docs/roadmap/rust_migration.md) - Migration roadmap
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [PyO3 Guide](https://pyo3.rs/) - Rust-Python bindings

---

## License

By contributing to dbt-oxide, you agree that your contributions will be licensed under the Apache License 2.0.

---

**Thank you for contributing to dbt-oxide! 🦀⚡**
