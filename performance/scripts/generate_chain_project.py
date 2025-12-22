#!/usr/bin/env python3
"""
Generate a dbt test project with configurable ref() dependency chains.

This creates a project structure with:
- base/ - Source models (no dependencies)
- staging/ - Models that ref() base models
- intermediate/ - Models that ref() staging models
- marts/ - Models that ref() intermediate models
"""
import argparse
import os
from pathlib import Path


def generate_project(
    output_dir: str,
    num_chains: int = 125,
    chain_depth: int = 4,
    project_name: str = "chain_models"
):
    """
    Generate a dbt project with dependency chains.
    
    Args:
        output_dir: Output directory for the project
        num_chains: Number of parallel model chains
        chain_depth: Depth of each chain (number of layers)
        project_name: Name for the dbt project
    """
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    
    # Layer names based on chain depth
    layer_names = ["base", "staging", "intermediate", "marts", "reports"][:chain_depth]
    
    total_models = num_chains * chain_depth
    print(f"Generating {project_name} with {total_models} models...")
    print(f"  {num_chains} chains × {chain_depth} layers = {total_models} models")
    
    # Create dbt_project.yml
    # Note: dbt project names cannot start with a digit, so prefix with 'project_'
    safe_project_name = project_name if not project_name[0].isdigit() else f"project_{project_name}"
    dbt_project_content = f"""name: {safe_project_name}
version: "1.0.0"
config-version: 2

profile: default

model-paths: ["models"]
analysis-paths: ["analyses"]
test-paths: ["tests"]
seed-paths: ["seeds"]
macro-paths: ["macros"]
snapshot-paths: ["snapshots"]

clean-targets:
  - "target"
  - "dbt_packages"
  - "dbt_modules"
"""
    (output_path / "dbt_project.yml").write_text(dbt_project_content)
    
    # Create profiles.yml (for local testing only)
    # Note: Performance tests use --profiles-dir ../../project_config/
    profiles_content = f"""config:
  send_anonymous_usage_stats: false

default:
  target: dev
  outputs:
    dev:
      type: postgres
      host: localhost
      port: 5432
      user: postgres
      pass: postgres
      dbname: postgres
      schema: {safe_project_name}
      threads: 4
"""
    (output_path / "profiles.yml").write_text(profiles_content)
    
    # Create models directory
    models_path = output_path / "models"
    models_path.mkdir(exist_ok=True)
    
    # Generate models for each layer
    for layer_idx, layer_name in enumerate(layer_names):
        layer_path = models_path / layer_name
        layer_path.mkdir(exist_ok=True)
        
        for chain_idx in range(num_chains):
            model_name = f"{layer_name}_{chain_idx}"
            
            # Generate SQL file
            if layer_idx == 0:
                # Base layer - no dependencies
                sql_content = f"select {chain_idx} as id, 'chain_{chain_idx}' as chain_name"
            else:
                # Higher layers - ref() previous layer
                prev_layer = layer_names[layer_idx - 1]
                prev_model = f"{prev_layer}_{chain_idx}"
                sql_content = f"select * from {{{{ ref('{prev_model}') }}}}"
            
            sql_file = layer_path / f"{model_name}.sql"
            sql_file.write_text(sql_content)
            
            # Generate YAML file with tests
            yaml_content = f"""version: 2

models:
  - name: {model_name}
    columns:
      - name: id
        tests:
          - not_null
"""
            yaml_file = layer_path / f"{model_name}.yml"
            yaml_file.write_text(yaml_content)
    
    print(f"[OK] Generated project at {output_dir}")
    print(f"  Layers: {', '.join(layer_names)}")
    print(f"  Models per layer: {num_chains}")
    return total_models


def main():
    parser = argparse.ArgumentParser(
        description="Generate dbt test project with dependency chains"
    )
    parser.add_argument(
        "--output",
        "-o",
        required=True,
        help="Output directory for the project"
    )
    parser.add_argument(
        "--num-chains",
        "-n",
        type=int,
        default=125,
        help="Number of parallel model chains (default: 125)"
    )
    parser.add_argument(
        "--chain-depth",
        "-d",
        type=int,
        default=4,
        help="Depth of each chain in layers (default: 4)"
    )
    parser.add_argument(
        "--project-name",
        "-p",
        default="chain_models",
        help="Name for the dbt project (default: chain_models)"
    )
    
    args = parser.parse_args()
    
    total = generate_project(
        output_dir=args.output,
        num_chains=args.num_chains,
        chain_depth=args.chain_depth,
        project_name=args.project_name
    )
    
    print(f"\n[OK] Success! Generated {total} models")
    print(f"\nTo test:")
    print(f"  cd {args.output}")
    print(f"  dbt parse --profiles-dir .")


if __name__ == "__main__":
    main()
