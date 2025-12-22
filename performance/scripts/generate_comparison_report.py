#!/usr/bin/env python3
"""
Generate performance comparison report between vanilla dbt-core and dbt-oxide.

Reads JSON results from hyperfine benchmarks and creates a markdown report
with side-by-side comparisons showing performance deltas.
"""
import argparse
import json
from pathlib import Path
from typing import Dict, List, Tuple


def load_results(results_dir: Path) -> Dict[str, dict]:
    """Load all JSON result files from a directory."""
    results = {}
    for json_file in results_dir.glob("*.json"):
        with open(json_file) as f:
            data = json.load(f)
            # Extract metric name from filename (e.g., "parse___02_500_chain_models.json")
            metric_name = json_file.stem.split("___")[0]
            project_name = json_file.stem.split("___")[1] if "___" in json_file.stem else "unknown"
            key = f"{metric_name}___{project_name}"
            results[key] = data["results"][0] if "results" in data else data
    return results


def format_time(seconds: float) -> str:
    """Format time in seconds to human-readable string."""
    if seconds < 1:
        return f"{seconds * 1000:.0f}ms"
    return f"{seconds:.2f}s"


def calculate_delta(vanilla: float, oxide: float) -> Tuple[str, str, str]:
    """Calculate delta and percentage change."""
    delta = oxide - vanilla
    percent = (delta / vanilla) * 100 if vanilla > 0 else 0
    
    delta_str = f"{delta:+.2f}s" if abs(delta) >= 1 else f"{delta * 1000:+.0f}ms"
    percent_str = f"{percent:+.1f}%"
    
    # Text-based indicator
    if percent < -5:
        indicator = "++"  # Significantly faster
    elif percent < -1:
        indicator = "+"   # Faster
    elif percent > 5:
        indicator = "--"  # Significantly slower
    elif percent > 1:
        indicator = "-"   # Slower
    else:
        indicator = "="   # Negligible difference
    
    return delta_str, percent_str, indicator


def generate_report(
    vanilla_dir: Path,
    oxide_dir: Path,
    vanilla_version: str,
    output_file: Path
):
    """Generate markdown comparison report."""
    vanilla_results = load_results(vanilla_dir)
    oxide_results = load_results(oxide_dir)
    
    # Find common metrics
    common_keys = set(vanilla_results.keys()) & set(oxide_results.keys())
    
    if not common_keys:
        print("[WARN] No overlapping metrics found between vanilla and oxide results")
        return
    
    # Build report
    lines = [
        "# Performance Comparison Report",
        "",
        f"**Vanilla dbt-core:** {vanilla_version}  ",
        f"**dbt-oxide:** (current)",
        "",
        "## Summary",
        "",
        "| Metric | Vanilla | dbt-oxide | Δ | % Change | |",
        "|--------|---------|-----------|---|---------|-|"
    ]
    
    # Sort by metric name for consistent ordering
    for key in sorted(common_keys):
        vanilla = vanilla_results[key]
        oxide = oxide_results[key]
        
        metric_name, project_name = key.split("___")
        
        vanilla_mean = vanilla["mean"]
        oxide_mean = oxide["mean"]
        
        delta_str, percent_str, indicator = calculate_delta(vanilla_mean, oxide_mean)
        
        lines.append(
            f"| `{metric_name}` ({project_name}) | "
            f"{format_time(vanilla_mean)} | "
            f"{format_time(oxide_mean)} | "
            f"{delta_str} | "
            f"{percent_str} | "
            f"{indicator} |"
        )
    
    lines.extend([
        "",
        "## Legend",
        "",
        "- `++` Significantly faster (>5% improvement)",
        "- `+`  Faster (>1% improvement)", 
        "- `=`  Negligible difference (±1%)",
        "- `-`  Slower (>1% regression)",
        "- `--` Significantly slower (>5% regression)",
        "",
        "## Detailed Results",
        ""
    ])
    
    # Add detailed results for each metric
    for key in sorted(common_keys):
        vanilla = vanilla_results[key]
        oxide = oxide_results[key]
        
        metric_name, project_name = key.split("___")
        
        lines.extend([
            f"### `{metric_name}` - {project_name}",
            "",
            "| Measurement | Vanilla | dbt-oxide |",
            "|-------------|---------|-----------|",
            f"| Mean | {format_time(vanilla['mean'])} | {format_time(oxide['mean'])} |",
            f"| Std Dev | {format_time(vanilla['stddev'])} | {format_time(oxide['stddev'])} |",
            f"| Min | {format_time(vanilla['min'])} | {format_time(oxide['min'])} |",
            f"| Max | {format_time(vanilla['max'])} | {format_time(oxide['max'])} |",
            f"| User Time | {format_time(vanilla['user'])} | {format_time(oxide['user'])} |",
            f"| System Time | {format_time(vanilla['system'])} | {format_time(oxide['system'])} |",
            ""
        ])
    
    # Write report
    output_file.write_text("\n".join(lines))
    print(f"[OK] Report generated: {output_file}")


def main():
    parser = argparse.ArgumentParser(
        description="Generate performance comparison report"
    )
    parser.add_argument(
        "--vanilla",
        required=True,
        help="Directory containing vanilla dbt-core results"
    )
    parser.add_argument(
        "--oxide",
        required=True,
        help="Directory containing dbt-oxide results"
    )
    parser.add_argument(
        "--vanilla-version",
        default="unknown",
        help="Vanilla dbt-core version string"
    )
    parser.add_argument(
        "--output",
        "-o",
        required=True,
        help="Output markdown file"
    )
    
    args = parser.parse_args()
    
    generate_report(
        vanilla_dir=Path(args.vanilla),
        oxide_dir=Path(args.oxide),
        vanilla_version=args.vanilla_version,
        output_file=Path(args.output)
    )


if __name__ == "__main__":
    main()
