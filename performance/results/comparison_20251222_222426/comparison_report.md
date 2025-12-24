# Performance Comparison Report

**Vanilla dbt-core:** 1.10.16  
**dbt-oxide:** (current)

## Summary

| Metric | Vanilla | dbt-oxide | Δ | % Change | |
|--------|---------|-----------|---|---------|-|
| `ls` (01_2000_simple_models) | 20.79s | 21.20s | +409ms | +2.0% | - |
| `ls` (02_500_chain_models) | 4.28s | 4.55s | +265ms | +6.2% | -- |
| `ls` (03_2000_chain_models) | 10.25s | 10.40s | +148ms | +1.4% | - |
| `ls_select_ancestors` (01_2000_simple_models) | 18.62s | 19.60s | +981ms | +5.3% | -- |
| `ls_select_ancestors` (02_500_chain_models) | 4.11s | 4.40s | +285ms | +6.9% | -- |
| `ls_select_ancestors` (03_2000_chain_models) | 9.46s | 9.85s | +389ms | +4.1% | - |
| `parse` (01_2000_simple_models) | 18.18s | 18.39s | +203ms | +1.1% | - |
| `parse` (02_500_chain_models) | 3.97s | 4.06s | +94ms | +2.4% | - |
| `parse` (03_2000_chain_models) | 8.97s | 10.00s | +1.03s | +11.5% | -- |

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
| Mean | 20.79s | 21.20s |
| Std Dev | 558ms | 614ms |
| Min | 20.39s | 20.76s |
| Max | 21.18s | 21.63s |
| User Time | 19.76s | 20.13s |
| System Time | 1.78s | 1.81s |

### `ls` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.28s | 4.55s |
| Std Dev | 6ms | 119ms |
| Min | 4.28s | 4.46s |
| Max | 4.29s | 4.63s |
| User Time | 3.57s | 3.78s |
| System Time | 226ms | 229ms |

### `ls` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 10.25s | 10.40s |
| Std Dev | 34ms | 63ms |
| Min | 10.23s | 10.36s |
| Max | 10.28s | 10.45s |
| User Time | 9.35s | 9.53s |
| System Time | 390ms | 366ms |

### `ls_select_ancestors` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 18.62s | 19.60s |
| Std Dev | 13ms | 350ms |
| Min | 18.61s | 19.35s |
| Max | 18.63s | 19.85s |
| User Time | 17.87s | 18.67s |
| System Time | 1.56s | 1.62s |

### `ls_select_ancestors` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 4.11s | 4.40s |
| Std Dev | 12ms | 211ms |
| Min | 4.10s | 4.25s |
| Max | 4.12s | 4.54s |
| User Time | 3.40s | 3.62s |
| System Time | 224ms | 209ms |

### `ls_select_ancestors` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 9.46s | 9.85s |
| Std Dev | 0ms | 257ms |
| Min | 9.46s | 9.67s |
| Max | 9.46s | 10.03s |
| User Time | 8.62s | 8.97s |
| System Time | 350ms | 350ms |

### `parse` - 01_2000_simple_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 18.18s | 18.39s |
| Std Dev | 48ms | 81ms |
| Min | 18.15s | 18.33s |
| Max | 18.22s | 18.44s |
| User Time | 17.27s | 17.62s |
| System Time | 1.69s | 1.60s |

### `parse` - 02_500_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 3.97s | 4.06s |
| Std Dev | 9ms | 82ms |
| Min | 3.96s | 4.00s |
| Max | 3.97s | 4.12s |
| User Time | 3.26s | 3.35s |
| System Time | 233ms | 206ms |

### `parse` - 03_2000_chain_models

| Measurement | Vanilla | dbt-oxide |
|-------------|---------|-----------|
| Mean | 8.97s | 10.00s |
| Std Dev | 7ms | 86ms |
| Min | 8.96s | 9.93s |
| Max | 8.97s | 10.06s |
| User Time | 8.12s | 8.69s |
| System Time | 361ms | 407ms |
