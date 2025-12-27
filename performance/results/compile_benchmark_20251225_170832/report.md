# dbt compile Performance Comparison

**Date:** Thu Dec 25 17:15:49 CET 2025
**Project:** 01_2000_simple_models
**Runs:** 3

## Results

| Version | Mean Time | Std Dev |
|---------|-----------|---------|
| Vanilla dbt 1.10.16 | 42.25104027348001s | ±0.2855009685380022s |
| dbt-oxide (minijinja) | 43.717917157473345s | ±1.1647915843889096s |

## Summary

- **Speedup:** .96x faster
- **Improvement:** 0% reduction in compile time

### Interpretation

The `dbt compile` command exercises the template rendering path where
minijinja replaces Jinja2 for `capture_macros=False` scenarios.

This benchmark validates the Phase 3.1 minijinja integration and measures
the actual performance improvement in real dbt workloads.
