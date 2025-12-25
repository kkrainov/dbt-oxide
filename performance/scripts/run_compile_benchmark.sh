#!/bin/bash
# Performance comparison script: dbt compile benchmark
# Measures template rendering performance (minijinja vs Jinja2)
#
# This script automatically starts PostgreSQL via Docker if not running.
#
# Usage: ./run_compile_benchmark.sh [vanilla_version] [project_name] [runs]
#   vanilla_version: dbt-core version to test against (default: 1.10.16)
#   project_name: Which project to benchmark (default: 01_2000_simple_models)
#   runs: Number of benchmark runs (default: 3)

set -e

DBT_VANILLA_VERSION="${1:-1.10.16}"
PROJECT_NAME="${2:-01_2000_simple_models}"
RUNS="${3:-3}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="$PWD/performance/results/compile_benchmark_$TIMESTAMP"
PROJECT_DIR="$PWD/performance/projects/$PROJECT_NAME"
CONTAINER_NAME="dbt-perf-postgres"

echo "=== dbt compile Benchmark: vanilla dbt $DBT_VANILLA_VERSION vs dbt-oxide ==="
echo "Project: $PROJECT_NAME"
echo "Runs: $RUNS"
echo ""

# Function to cleanup PostgreSQL container on exit
cleanup() {
    if [ "$POSTGRES_STARTED" = "true" ]; then
        echo ""
        echo "Cleaning up PostgreSQL container..."
        docker stop "$CONTAINER_NAME" >/dev/null 2>&1 || true
        docker rm "$CONTAINER_NAME" >/dev/null 2>&1 || true
    fi
}
trap cleanup EXIT

# Check if PostgreSQL is running, if not start it via Docker
POSTGRES_STARTED="false"
echo "Checking PostgreSQL availability..."

if pg_isready -h localhost -p 5432 -q 2>/dev/null; then
    echo "PostgreSQL already running on localhost:5432"
else
    echo "PostgreSQL not running. Starting via Docker..."
    
    # Check if Docker is available
    if ! command -v docker &> /dev/null; then
        echo "ERROR: Docker is not installed. Please install Docker or start PostgreSQL manually."
        exit 1
    fi
    
    # Remove existing container if it exists
    docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true
    
    # Start PostgreSQL container with credentials matching profiles.yml
    docker run -d --name "$CONTAINER_NAME" \
        -e POSTGRES_USER=dummy \
        -e POSTGRES_PASSWORD=dummy_password \
        -e POSTGRES_DB=dummy \
        -p 5432:5432 \
        postgres:15 >/dev/null
    
    POSTGRES_STARTED="true"
    
    # Wait for PostgreSQL to be ready
    echo "Waiting for PostgreSQL to start..."
    for i in {1..30}; do
        if pg_isready -h localhost -p 5432 -q 2>/dev/null; then
            echo "PostgreSQL is ready!"
            break
        fi
        if [ $i -eq 30 ]; then
            echo "ERROR: PostgreSQL failed to start within 30 seconds"
            exit 1
        fi
        sleep 1
    done
fi
echo ""

# Create results directory
mkdir -p "$RESULTS_DIR"

# Create temporary venv for vanilla dbt-core
echo "Creating temporary venv for vanilla dbt-core $DBT_VANILLA_VERSION..."
VANILLA_VENV=$(mktemp -d)

# Cleanup function extended for venv
cleanup_all() {
    cleanup
    rm -rf "$VANILLA_VENV" 2>/dev/null || true
}
trap cleanup_all EXIT

# Install vanilla dbt-core using uv
uv venv "$VANILLA_VENV"
source "$VANILLA_VENV/bin/activate"
uv pip install "dbt-core==$DBT_VANILLA_VERSION" dbt-postgres >/dev/null 2>&1

# Verify installation
echo ""
echo "Vanilla dbt version:"
dbt --version | head -3
echo ""

# ==== VANILLA BENCHMARK ====
echo "=== Running vanilla dbt-core compile benchmark ==="
cd "$PROJECT_DIR"

# Parse first (required for compile)
echo "Parsing project..."
dbt clean --profiles-dir ../../project_config/ > /dev/null 2>&1 || true
dbt parse --no-version-check --profiles-dir ../../project_config/ > /dev/null 2>&1

# Run compile benchmark
echo "Running compile benchmark ($RUNS runs)..."
hyperfine \
    --warmup 1 \
    --runs "$RUNS" \
    --prepare "dbt parse --no-version-check --profiles-dir ../../project_config/" \
    "dbt compile --no-version-check --profiles-dir ../../project_config/" \
    --export-json "$RESULTS_DIR/vanilla_compile.json" \
    2>&1 | grep -E "(Benchmark|Time|Range)" || true

deactivate
cd - > /dev/null

# ==== DBT-OXIDE BENCHMARK ====
echo ""
echo "=== Running dbt-oxide compile benchmark ==="
cd "$PROJECT_DIR"

# Parse first
echo "Parsing project..."
uv run dbt clean --profiles-dir ../../project_config/ > /dev/null 2>&1 || true
uv run dbt parse --no-version-check --profiles-dir ../../project_config/ > /dev/null 2>&1

# Run compile benchmark
echo "Running compile benchmark ($RUNS runs)..."
hyperfine \
    --warmup 1 \
    --runs "$RUNS" \
    --prepare "uv run dbt parse --no-version-check --profiles-dir ../../project_config/" \
    "uv run dbt compile --no-version-check --profiles-dir ../../project_config/" \
    --export-json "$RESULTS_DIR/oxide_compile.json" \
    2>&1 | grep -E "(Benchmark|Time|Range)" || true

cd - > /dev/null

# ==== GENERATE REPORT ====
echo ""
echo "=== Generating comparison report ==="

# Extract times from JSON
VANILLA_MEAN=$(jq '.results[0].mean' "$RESULTS_DIR/vanilla_compile.json")
VANILLA_STDDEV=$(jq '.results[0].stddev' "$RESULTS_DIR/vanilla_compile.json")
OXIDE_MEAN=$(jq '.results[0].mean' "$RESULTS_DIR/oxide_compile.json")
OXIDE_STDDEV=$(jq '.results[0].stddev' "$RESULTS_DIR/oxide_compile.json")

# Calculate speedup
SPEEDUP=$(echo "scale=2; $VANILLA_MEAN / $OXIDE_MEAN" | bc)
IMPROVEMENT=$(echo "scale=1; (1 - $OXIDE_MEAN / $VANILLA_MEAN) * 100" | bc)

# Generate report
cat > "$RESULTS_DIR/report.md" << EOF
# dbt compile Performance Comparison

**Date:** $(date)
**Project:** $PROJECT_NAME
**Runs:** $RUNS

## Results

| Version | Mean Time | Std Dev |
|---------|-----------|---------|
| Vanilla dbt $DBT_VANILLA_VERSION | ${VANILLA_MEAN}s | ±${VANILLA_STDDEV}s |
| dbt-oxide (minijinja) | ${OXIDE_MEAN}s | ±${OXIDE_STDDEV}s |

## Summary

- **Speedup:** ${SPEEDUP}x faster
- **Improvement:** ${IMPROVEMENT}% reduction in compile time

### Interpretation

The \`dbt compile\` command exercises the template rendering path where
minijinja replaces Jinja2 for \`capture_macros=False\` scenarios.

This benchmark validates the Phase 3.1 minijinja integration and measures
the actual performance improvement in real dbt workloads.
EOF

echo ""
echo "============================================"
echo "           BENCHMARK RESULTS"
echo "============================================"
echo ""
echo "Vanilla dbt $DBT_VANILLA_VERSION: ${VANILLA_MEAN}s (±${VANILLA_STDDEV}s)"
echo "dbt-oxide (minijinja):  ${OXIDE_MEAN}s (±${OXIDE_STDDEV}s)"
echo ""
echo "Speedup: ${SPEEDUP}x faster"
echo "Improvement: ${IMPROVEMENT}% reduction"
echo ""
echo "Full report: $RESULTS_DIR/report.md"
