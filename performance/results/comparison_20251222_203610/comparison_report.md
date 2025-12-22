# Performance Comparison Report

**Vanilla dbt-core:** 1.10.16  
**dbt-oxide:** (current)

## Summary

| Metric | Vanilla | dbt-oxide | Δ | % Change | |
|--------|---------|-----------|---|---------|-|
| `ls` (01_2000_simple_models) | 19.86s | 21.47s | +1.61s | +8.1% | -- |
| `ls` (02_500_chain_models) | 4.29s | 4.39s | +97ms | +2.3% | - |
| `ls` (03_2000_chain_models) | 10.23s | 11.40s | +1.17s | +11.5% | -- |
| `ls_select_ancestors` (01_2000_simple_models) | 18.53s | 20.28s | +1.75s | +9.5% | -- |
| `ls_select_ancestors` (02_500_chain_models) | 4.09s | 4.15s | +62ms | +1.5% | - |
| `ls_select_ancestors` (03_2000_chain_models) | 9.39s | 10.19s | +792ms | +8.4% | -- |
| `parse` (01_2000_simple_models) | 18.04s | 18.35s | +311ms | +1.7% | - |
| `parse` (02_500_chain_models) | 4.01s | 3.96s | -50ms | -1.3% | + |
| `parse` (03_2000_chain_models) | 9.08s | 9.18s | +97ms | +1.1% | - |

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
| Mean | 19.86s | 21.47s |
| Std Dev | 109ms | 52ms |
| Min | 19.79s | 21.44s |
| Max | 19.94s | 21.51s |
| User Time | 19.01s | 20.61s |
| System Time | 1.62s | 1.61s |

### `ls` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.29s | 4.39s |
| Std Dev | 73ms | 17ms |
| Min | 4.24s | 4.37s |
| Max | 4.34s | 4.40s |
| User Time | 3.57s | 3.69s |
| System Time | 217ms | 191ms |

### `ls` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 10.23s | 11.40s |
| Std Dev | 177ms | 34ms |
| Min | 10.10s | 11.38s |
| Max | 10.35s | 11.42s |
| User Time | 9.29s | 10.38s |
| System Time | 408ms | 450ms |

### `ls_select_ancestors` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 18.53s | 20.28s |
| Std Dev | 0ms | 637ms |
| Min | 18.53s | 19.83s |
| Max | 18.53s | 20.73s |
| User Time | 17.77s | 19.42s |
| System Time | 1.52s | 1.60s |

### `ls_select_ancestors` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.09s | 4.15s |
| Std Dev | 33ms | 74ms |
| Min | 4.07s | 4.10s |
| Max | 4.11s | 4.20s |
| User Time | 3.38s | 3.49s |
| System Time | 218ms | 176ms |

### `ls_select_ancestors` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 9.39s | 10.19s |
| Std Dev | 36ms | 147ms |
| Min | 9.37s | 10.08s |
| Max | 9.42s | 10.29s |
| User Time | 8.55s | 9.32s |
| System Time | 322ms | 343ms |

### `parse` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 18.04s | 18.35s |
| Std Dev | 653ms | 71ms |
| Min | 17.57s | 18.30s |
| Max | 18.50s | 18.40s |
| User Time | 17.18s | 17.60s |
| System Time | 1.64s | 1.53s |

### `parse` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.01s | 3.96s |
| Std Dev | 83ms | 44ms |
| Min | 3.95s | 3.92s |
| Max | 4.06s | 3.99s |
| User Time | 3.27s | 3.26s |
| System Time | 235ms | 198ms |

### `parse` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 9.08s | 9.18s |
| Std Dev | 95ms | 35ms |
| Min | 9.01s | 9.15s |
| Max | 9.14s | 9.20s |
| User Time | 8.18s | 8.36s |
| System Time | 380ms | 324ms |
