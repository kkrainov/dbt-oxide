# Performance Comparison Report

**Vanilla dbt-core:** 1.10.16  
**dbt-oxide:** (current)

## Summary

| Metric | Vanilla | dbt-oxide | Δ | % Change | |
|--------|---------|-----------|---|---------|-|
| `ls` (01_2000_simple_models) | 21.36s | 21.04s | -321ms | -1.5% | + |
| `ls` (02_500_chain_models) | 4.43s | 4.16s | -261ms | -5.9% | ++ |
| `ls` (03_2000_chain_models) | 10.49s | 10.14s | -343ms | -3.3% | + |
| `ls_select_ancestors` (01_2000_simple_models) | 19.26s | 19.71s | +445ms | +2.3% | - |
| `ls_select_ancestors` (02_500_chain_models) | 4.23s | 3.96s | -270ms | -6.4% | ++ |
| `ls_select_ancestors` (03_2000_chain_models) | 9.52s | 9.33s | -187ms | -2.0% | + |
| `parse` (01_2000_simple_models) | 18.80s | 18.68s | -121ms | -0.6% | = |
| `parse` (02_500_chain_models) | 4.09s | 3.97s | -123ms | -3.0% | + |
| `parse` (03_2000_chain_models) | 9.09s | 8.91s | -176ms | -1.9% | + |

## Legend

- `++` Significantly faster (>5% improvement)
- `+`  Faster (>1% improvement)
- `=`  Negligible difference (±1%)
- `-`  Slower (>1% regression)
- `--` Significantly slower (>5% regression)

## Detailed Results

### `ls` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 21.36s | 21.04s |
| Std Dev | 484ms | 202ms |
| Min | 21.02s | 20.90s |
| Max | 21.70s | 21.18s |
| User Time | 20.30s | 20.03s |
| System Time | 1.87s | 1.74s |

### `ls` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.43s | 4.16s |
| Std Dev | 32ms | 29ms |
| Min | 4.40s | 4.14s |
| Max | 4.45s | 4.19s |
| User Time | 3.68s | 3.50s |
| System Time | 247ms | 193ms |

### `ls` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 10.49s | 10.14s |
| Std Dev | 135ms | 211ms |
| Min | 10.39s | 9.99s |
| Max | 10.58s | 10.29s |
| User Time | 9.57s | 9.22s |
| System Time | 406ms | 368ms |

### `ls_select_ancestors` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 19.26s | 19.71s |
| Std Dev | 56ms | 116ms |
| Min | 19.22s | 19.63s |
| Max | 19.30s | 19.79s |
| User Time | 18.36s | 18.83s |
| System Time | 1.64s | 1.63s |

### `ls_select_ancestors` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.23s | 3.96s |
| Std Dev | 52ms | 5ms |
| Min | 4.19s | 3.96s |
| Max | 4.27s | 3.96s |
| User Time | 3.50s | 3.30s |
| System Time | 244ms | 176ms |

### `ls_select_ancestors` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 9.52s | 9.33s |
| Std Dev | 39ms | 51ms |
| Min | 9.49s | 9.29s |
| Max | 9.55s | 9.37s |
| User Time | 8.69s | 8.54s |
| System Time | 340ms | 310ms |

### `parse` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 18.80s | 18.68s |
| Std Dev | 79ms | 55ms |
| Min | 18.75s | 18.64s |
| Max | 18.86s | 18.72s |
| User Time | 17.84s | 17.90s |
| System Time | 1.80s | 1.58s |

### `parse` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.09s | 3.97s |
| Std Dev | 8ms | 10ms |
| Min | 4.08s | 3.96s |
| Max | 4.09s | 3.97s |
| User Time | 3.35s | 3.29s |
| System Time | 255ms | 199ms |

### `parse` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 9.09s | 8.91s |
| Std Dev | 50ms | 28ms |
| Min | 9.05s | 8.89s |
| Max | 9.12s | 8.93s |
| User Time | 8.22s | 8.11s |
| System Time | 374ms | 321ms |
