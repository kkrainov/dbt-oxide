#!/bin/bash
# Performance comparison script: vanilla dbt-core vs dbt-oxide
# 
# Usage: ./run_comparison.sh [vanilla_version]
#   vanilla_version: dbt-core version to test against (default: 1.10.16)

set -e

DBT_VANILLA_VERSION="${1:-1.10.16}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="$PWD/performance/results/comparison_$TIMESTAMP"
VANILLA_DIR="$RESULTS_DIR/vanilla"
OXIDE_DIR="$RESULTS_DIR/oxide"

echo "=== Performance Comparison: vanilla dbt $DBT_VANILLA_VERSION vs dbt-oxide ==="

# Create results directories
mkdir -p "$VANILLA_DIR" "$OXIDE_DIR"

# Create temporary venv for vanilla dbt-core
echo ""
echo "Creating temporary venv for vanilla dbt-core $DBT_VANILLA_VERSION..."
VANILLA_VENV=$(mktemp -d)
trap "rm -rf $VANILLA_VENV" EXIT

# Install vanilla dbt-core using uv
uv venv "$VANILLA_VENV"
source "$VANILLA_VENV/bin/activate"
uv pip install "dbt-core==$DBT_VANILLA_VERSION" dbt-postgres hyperfine

# Verify installation
echo ""
echo "Vanilla dbt version:"
dbt --version | head -5

# Run vanilla benchmarks
echo ""
echo "=== Running vanilla dbt-core benchmarks ==="
# Temporarily change METRICS to use bare 'dbt' commands
RUNNER_DIR="$PWD/performance/runner"
VANILLA_FS="$RUNNER_DIR/src/fs_vanilla_temp.rs"
cp "$RUNNER_DIR/src/fs.rs" "$VANILLA_FS"

# Create vanilla version with bare dbt commands
sed 's/uv run dbt/dbt/g' "$VANILLA_FS" > "$RUNNER_DIR/src/fs.rs"

# Build and run vanilla tests
cargo run --manifest-path "$RUNNER_DIR/Cargo.toml" -- sample \
    -p "$PWD/performance/projects" \
    -b "$PWD/performance/baselines" \
    -o "$VANILLA_DIR"

# Restore original fs.rs
mv "$VANILLA_FS" "$RUNNER_DIR/src/fs.rs"

deactivate

# Run dbt-oxide benchmarks
echo ""
echo "=== Running dbt-oxide benchmarks ==="
uv run cargo run --manifest-path performance/runner/Cargo.toml -- sample \
    -p "$PWD/performance/projects" \
    -b "$PWD/performance/baselines" \
    -o "$OXIDE_DIR"

# Generate comparison report
echo ""
echo "=== Generating comparison report ==="
python performance/scripts/generate_comparison_report.py \
    --vanilla "$VANILLA_DIR" \
    --oxide "$OXIDE_DIR" \
    --vanilla-version "$DBT_VANILLA_VERSION" \
    --output "$RESULTS_DIR/comparison_report.md"

echo ""
echo "[OK] Comparison complete!"
echo "Results: $RESULTS_DIR/comparison_report.md"
cat "$RESULTS_DIR/comparison_report.md"
