<p align="center">
  <img src="https://raw.githubusercontent.com/dbt-labs/dbt-core/fa1ea14ddfb1d5ae319d5141844910dd53ab2834/etc/dbt-oxide.png" alt="dbt oxide logo" width="750"/>
</p>

# dbt-oxide

**High-performance dbt fork with Rust-accelerated core**

dbt-oxide is a performance-focused fork of [dbt-core](https://github.com/dbt-labs/dbt-core) that replaces critical Python components with Rust implementations, delivering **significant performance improvements** in compile and DAG resolution while maintaining **100% backward compatibility**.

> This is a fork of dbt-core v1.10.16. Original work: Copyright 2021 dbt Labs, Inc. | Rust enhancements: Copyright 2025 Kirill Krainov

---

## What is dbt?

[dbt](https://www.getdbt.com/) enables data analysts and engineers to transform their data using the same practices that software engineers use to build applications. dbt-oxide extends dbt with a high-performance Rust core.

![architecture](https://github.com/dbt-labs/dbt-core/blob/202cb7e51e218c7b29eb3b11ad058bd56b7739de/etc/dbt-transform.png)

## Why dbt-oxide?

### Performance Goals

- **⚡ Significantly faster** compile and DAG resolution
- **🦀 Rust-powered** graph algorithms and manifest parsing  
- **🔄 Zero-copy** data structures via PyO3
- **📊 Parallel** execution with Rayon

### Architecture: Strangler Fig Pattern

dbt-oxide uses the [Strangler Fig pattern](https://martinfowler.com/bliki/StranglerFigApplication.html) to gradually replace Python internals with Rust:

```
┌───────────────────────────────────────┐
│     Python Shell (API Surface)        │
│   100% compatible with dbt-core       │
├───────────────────────────────────────┤
│         Rust Core Engine              │
│  • Graph algorithms (networkx → Rust) │
│  • Manifest parsing (JSON → Rust)     │
│  • Zero-copy data structures          │
└───────────────────────────────────────┘
```

**Key principle:** The Python API remains identical. Existing dbt projects, adapters, and packages work without modification.

---

## Installation

### From PyPI (recommended)

```bash
pip install dbt-oxide
```

### From Source

```bash
# Clone the repository
git clone https://github.com/kkrainov/dbt-oxide.git
cd dbt-oxide

# Install uv (modern Python package manager)
pip install uv

# Install dependencies and build Rust extension
uv sync
uv run maturin develop --release
```

### Verify Installation

```bash
dbt --version
# Should show dbt-oxide version
```

---

## Usage

dbt-oxide is a drop-in replacement for dbt-core. All existing dbt commands work:

```bash
# Initialize a project
dbt init my_project

# Run models
dbt run

# Test your data
dbt test

# Build documentation
dbt docs generate
dbt docs serve
```

**No changes to your existing dbt projects are required.**

---

## Project Status

dbt-oxide is under active development. See [docs/roadmap/rust_migration.md](docs/roadmap/rust_migration.md) for detailed status.

**Completed:**
- ✅ Phase 1: Rust graph implementation (replacing networkx)
- ✅ Phase 2: Zero-copy manifest infrastructure

**In Progress:**
- 🚧 Phase 2.5: Integration of Rust graph with manifest
- 🚧 Phase 3: Compiler engine optimization

---

## Contributing

We welcome contributions! dbt-oxide uses:
- **uv** for Python dependency management
- **maturin** for building the Rust extension
- **DCO** for contributor sign-offs (no CLA required)

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

### Quick Start for Contributors

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/dbt-oxide.git
cd dbt-oxide

# Install dependencies
pip install uv
uv sync

# Build Rust extension
uv run maturin develop

# Run tests
cargo test --no-default-features  # Rust tests
uv run pytest tests/unit           # Python tests

# Sign your commits
git commit -s -m "feat: your awesome feature"
```

---

## Documentation

- [Architecture Overview](ARCHITECTURE.md) - System design and Rust integration
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Roadmap](docs/roadmap/rust_migration.md) - Development phases
- [Upstream Tracking](UPSTREAM_TRACKING.md) - Fork maintenance policy

---

## License

Licensed under the Apache License 2.0. See [LICENSE.md](LICENSE.md) for details.

This project is a derivative work of [dbt-core](https://github.com/dbt-labs/dbt-core):
- **Original work:** Copyright 2021 dbt Labs, Inc.
- **Rust enhancements:** Copyright 2025 Kirill Krainov

See [NOTICE](NOTICE) for full attribution.

---

## Acknowledgments

dbt-oxide builds upon the excellent work of the dbt Labs team and the broader dbt community. We are grateful for:
- The original dbt-core architecture and ecosystem
- The PyO3 project for Rust-Python interoperability
- All contributors to the dbt community

**This is an independent fork and is not affiliated with or endorsed by dbt Labs.**
