# Open Source Publication Plan

> **Status:** Ready for implementation  
> **Created:** 2025-12-21  
> **Goal:** Publish dbt-oxide as a public open-source fork of dbt-core with PyPI releases

---

## Overview

This plan prepares dbt-oxide (fork of dbt-core v1.10.16) for public open-source release with:
- Proper Apache 2.0 attribution
- Developer Certificate of Origin (DCO) for contributions
- Simplified CI/CD workflows
- Automated PyPI releases via Trusted Publishers

---

## Phase 1: Attribution & Legal

- [ ] **Create `NOTICE`** - dual copyright attribution
- [ ] **Create `DCO`** - Developer Certificate of Origin
- [ ] **Keep `LICENSE.md`** - unchanged Apache 2.0

### NOTICE File Content

```text
dbt-oxide
Copyright 2025-present Kirill Krainov

This project is a derivative work based on dbt-core.
Original work: https://github.com/dbt-labs/dbt-core
Copyright 2021 dbt Labs, Inc.
Licensed under the Apache License 2.0.

Modifications include:
- Rust implementation of core graph algorithms (src/dbt_rs/)
- Rust implementation of manifest parsing
- Performance-optimized data structures using PyO3 bindings
```

### DCO File Content

```text
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.

Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

---

## Phase 2: GitHub Workflows

### 2.1 Delete dbt Labs Workflows (18 files)

- [ ] `release.yml`
- [ ] `nightly-release.yml`
- [ ] `backport.yml`
- [ ] `cut-release-branch.yml`
- [ ] `release-branch-tests.yml`
- [ ] `artifact-reviews.yml`
- [ ] `check-artifact-changes.yml`
- [ ] `bot-changelog.yml`
- [ ] `changelog-existence.yml`
- [ ] `community-label.yml`
- [ ] `triage-labels.yml`
- [ ] `auto-respond-bug-reports.yml`
- [ ] `docs-issue.yml`
- [ ] `stale.yml`
- [ ] `repository-cleanup.yml`
- [ ] `model_performance.yml`
- [ ] `schema-check.yml`
- [ ] `structured-logging-schema-check.yml`

### 2.2 Keep & Modify

- [ ] **Delete `main.yml`** - replace with `ci.yml`
- [ ] **Keep `build-wheels.yml`** - already uses maturin ✅

### 2.3 Create New Workflows

- [ ] **Create `ci.yml`** - Rust + Python CI
- [ ] **Create `release.yml`** - PyPI publishing

---

## Workflow Implementations

### ci.yml

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  rust-checks:
    name: Rust checks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - name: Format check
        run: cargo fmt --all -- --check
        working-directory: src/dbt_rs
      - name: Clippy
        run: cargo clippy --no-default-features -- -D warnings
        working-directory: src/dbt_rs
      - name: Tests
        run: cargo test --no-default-features
        working-directory: src/dbt_rs

  python-tests:
    name: Python ${{ matrix.python-version }}
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ['3.9', '3.11', '3.12']
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}
      - name: Install uv
        run: pip install uv
      - name: Install dependencies
        run: uv sync
      - name: Build Rust extension
        run: uv run maturin develop --release
        working-directory: core
      - name: Run tests
        run: uv run pytest tests/unit -x

  dco:
    name: DCO Check
    runs-on: ubuntu-latest
    if: github.event_name == 'pull_request'
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: dkhamsing/dco-check@v1
```

### release.yml

```yaml
name: Release to PyPI

on:
  push:
    tags:
      - 'v*.*.*'
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release (e.g., 0.1.0)'
        required: true

permissions:
  contents: write
  id-token: write

jobs:
  build-wheels:
    name: Build wheels on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            target: x86_64
          - os: macos-latest
            target: x86_64 aarch64
          - os: windows-latest
            target: x86_64
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      - uses: PyO3/maturin-action@v1
        with:
          working-directory: core
          target: ${{ matrix.target }}
          args: --release --out dist
          manylinux: auto
      - uses: actions/upload-artifact@v4
        with:
          name: wheels-${{ matrix.os }}
          path: core/dist/*.whl

  build-sdist:
    name: Build source distribution
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: PyO3/maturin-action@v1
        with:
          working-directory: core
          command: sdist
          args: --out dist
      - uses: actions/upload-artifact@v4
        with:
          name: sdist
          path: core/dist/*.tar.gz

  publish:
    name: Publish to PyPI
    needs: [build-wheels, build-sdist]
    runs-on: ubuntu-latest
    environment: release
    steps:
      - uses: actions/download-artifact@v4
        with:
          path: dist
          merge-multiple: true
      - uses: pypa/gh-action-pypi-publish@release/v1

  github-release:
    name: Create GitHub Release
    needs: [publish]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with:
          path: dist
          merge-multiple: true
      - uses: softprops/action-gh-release@v1
        with:
          files: dist/*
          generate_release_notes: true
```

---

## Phase 3: Documentation

- [ ] **Rewrite `README.md`** - dbt-oxide identity, installation, fork notice
- [ ] **Rewrite `CONTRIBUTING.md`** - uv, maturin, DCO workflow
- [ ] **Update `ARCHITECTURE.md`** - add Rust architecture section
- [ ] **Rewrite `SECURITY.md`** - security policy
- [ ] **Create `UPSTREAM_TRACKING.md`** - document fork point

---

## Phase 4: GitHub Configuration

- [ ] **Update `.github/CODEOWNERS`** - replace @dbt-labs with @kkrainov
- [ ] **Update `.github/pull_request_template.md`** - dbt-oxide specific
- [ ] **Update issue templates** - remove dbt-core references
- [ ] **Update `.github/dependabot.yml`** - add Rust dependencies

### CODEOWNERS Content

```
# dbt-oxide Code Owners
*                     @kkrainov
/src/dbt_rs/          @kkrainov
/core/dbt/            @kkrainov
```

---

## Phase 5: Verification & Setup

- [ ] **Run local tests** - `cargo test`, `uv run pytest`
- [ ] **Test wheel build** - `maturin build --release`
- [ ] **Push to GitHub** - verify CI passes
- [ ] **Make repo public**
- [ ] **Configure PyPI Trusted Publisher**
- [ ] **Set up branch protection** for `main`
- [ ] **Create `release` environment** in GitHub
- [ ] **Test release** - tag `v0.1.0-alpha`

---

## Role-Based Access Model

| Role | Who | Permissions | Enforced By |
|------|-----|-------------|-------------|
| Contributor | Anyone | Open PRs with DCO | DCO check |
| Maintainer | Trusted devs | Review & merge | CODEOWNERS + branch protection |
| Releaser | Admin only | Tag releases | Environment + tag protection |

---

## Release Checklist

```bash
# 1. Update version
vim Cargo.toml       # version = "X.Y.Z"
vim pyproject.toml   # version = "X.Y.Z"

# 2. Commit
git add -A
git commit -s -m "chore: bump version to X.Y.Z"
git push origin main

# 3. Tag and release
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z  # Triggers release workflow
```

---

## PyPI Setup: Trusted Publishers

### One-Time Configuration

1. Go to [pypi.org](https://pypi.org) → Account → Publishing → Add pending publisher
2. Configure:
   - Owner: `kkrainov`
   - Repository: `dbt-oxide`
   - Workflow: `release.yml`
   - Environment: `release`

### GitHub Environment Setup

1. Repo → Settings → Environments → New: `release`
2. Add required reviewer: `kkrainov`
3. Deployment branches: Protected branches only
