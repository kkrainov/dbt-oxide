# Changelog

All notable changes to dbt-oxide are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) and uses [Conventional Commits](https://www.conventionalcommits.org/).

> dbt-oxide is a fork of dbt-core v1.10.16 with Rust-powered performance improvements.

## [Unreleased]

### Features

- Phase 1: Rust graph implementation (`DbtGraph`, `OxideGraph`)
- Phase 2: Zero-copy manifest infrastructure

### Infrastructure

- Migrated build system to maturin + PyO3
- Added CI workflows for Rust + Python testing
- Modernized Python tooling with ruff
