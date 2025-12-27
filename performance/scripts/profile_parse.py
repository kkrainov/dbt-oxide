#!/usr/bin/env python
"""Profile dbt parse to identify performance bottlenecks."""
import cProfile
import pstats
import io
import sys
import os

# Change to project directory
os.chdir("/Users/kirillkrainov/apps/pet_projects/dbt-oxide/performance/projects/01_2000_simple_models")

# Profile the parse command
from dbt.cli.main import cli

profiler = cProfile.Profile()
profiler.enable()

try:
    cli(["parse", "--no-version-check", "--profiles-dir", "../../project_config/"])
except SystemExit:
    pass

profiler.disable()

# Generate stats
stats = pstats.Stats(profiler, stream=sys.stdout)
stats.strip_dirs()
stats.sort_stats('cumulative')

print("\n" + "="*80)
print("TOP 50 FUNCTIONS BY CUMULATIVE TIME")
print("="*80 + "\n")
stats.print_stats(50)

print("\n" + "="*80)
print("JINJA/MINIJINJA RELATED FUNCTIONS")
print("="*80 + "\n")

# Save full profile for later analysis
profiler.dump_stats("/tmp/dbt_parse_profile.prof")

# Filter for jinja-related functions
jinja_stats = pstats.Stats(profiler, stream=sys.stdout)
jinja_stats.strip_dirs()
jinja_stats.sort_stats('cumulative')
jinja_stats.print_stats('jinja|minijinja|render|template', 30)

print("\n" + "="*80)
print("STATIC PARSER / DBT-EXTRACTOR FUNCTIONS")
print("="*80 + "\n")
extractor_stats = pstats.Stats(profiler, stream=sys.stdout)
extractor_stats.strip_dirs()
extractor_stats.sort_stats('cumulative')
extractor_stats.print_stats('extract|static|populate|parse_ref|parse_source', 20)

print("\n" + "="*80)
print("OXIDE/RUST-RELATED FUNCTIONS") 
print("="*80 + "\n")
oxide_stats = pstats.Stats(profiler, stream=sys.stdout)
oxide_stats.strip_dirs()
oxide_stats.sort_stats('cumulative')
oxide_stats.print_stats('oxide|dbt_rs|rust', 20)
