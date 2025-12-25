"""minijinja-py wrapper for dbt template rendering.

This module provides a Rust-powered Jinja2-compatible rendering engine
using minijinja, achieving 3x faster template compilation.

Key differences from Jinja2:
- Uses minijinja.Environment instead of jinja2.Environment
- Hybrid approach: minijinja for capture_macros=False, Jinja2 for capture_macros=True
- Native type conversion handled via special marker prefixes and ast.literal_eval

The native rendering mode works by:
1. Filters wrap values in special marker strings (e.g., "__AS_NATIVE:1991")
2. After rendering, markers are detected and values converted via literal_eval
3. This matches dbt_common's behavior while using Rust-powered rendering
"""

from ast import literal_eval
from typing import Any, Dict

from minijinja import Environment

from dbt_common.exceptions import JinjaRenderingError

# Special prefixes to mark values for type conversion
_AS_TEXT_PREFIX = "__AS_TEXT:"
_AS_BOOL_PREFIX = "__AS_BOOL:"
_AS_NATIVE_PREFIX = "__AS_NATIVE:"
_AS_NUMBER_PREFIX = "__AS_NUMBER:"


def _is_number(value: Any) -> bool:
    """Check if value is a number (int or float, but not bool)."""
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _convert_native(value: str) -> Any:
    """Convert marker-prefixed values to native Python types.

    Special cases handled:
    1. Multiple markers (from multiple Jinja nodes) -> concatenated string
    2. Markers embedded in string (e.g., "'__AS_BOOL:True'") -> strip markers, return string
    3. Marker at start of string -> convert to native type

    Args:
        value: The rendered string, potentially with marker prefix

    Returns:
        - Multiple markers: strip all prefixes and return concatenated string
        - Embedded marker: strip prefix and return as string (no conversion)
        - __AS_TEXT: returns string without prefix
        - __AS_BOOL: converts to bool via literal_eval, raises if not boolean
        - __AS_NUMBER: converts to number via literal_eval, raises if not numeric
        - __AS_NATIVE: converts via literal_eval, returns string on failure
        - No marker: returns string unchanged

    Raises:
        JinjaRenderingError: If marker conversion fails (e.g., 'bar' | as_bool)
    """
    if not isinstance(value, str):
        return value

    all_markers = [_AS_TEXT_PREFIX, _AS_BOOL_PREFIX, _AS_NATIVE_PREFIX, _AS_NUMBER_PREFIX]

    # Check if value contains any markers
    marker_count = sum(value.count(marker) for marker in all_markers)

    if marker_count == 0:
        # No markers - return as-is
        return value

    if marker_count > 1:
        # Multiple markers - strip all and return concatenated string
        result = value
        for prefix in all_markers:
            result = result.replace(prefix, "")
        return result

    # Single marker present - check if it's at the start or embedded
    for prefix in all_markers:
        if prefix in value and not value.startswith(prefix):
            # Marker is embedded (e.g., "'__AS_BOOL:True'") - strip and return as string
            return value.replace(prefix, "")

    # Single marker at start of string - process for type conversion
    if value.startswith(_AS_TEXT_PREFIX):
        return value[len(_AS_TEXT_PREFIX) :]

    if value.startswith(_AS_NATIVE_PREFIX):
        raw = value[len(_AS_NATIVE_PREFIX) :]
        try:
            return literal_eval(raw)
        except (ValueError, SyntaxError, MemoryError):
            return raw

    if value.startswith(_AS_BOOL_PREFIX):
        raw = value[len(_AS_BOOL_PREFIX) :]
        try:
            result = literal_eval(raw)
        except (ValueError, SyntaxError, MemoryError):
            raise JinjaRenderingError(f"Could not convert value '{raw!s}' into type 'bool'")

        if not isinstance(result, bool):
            raise JinjaRenderingError(f"Could not convert value '{raw!s}' into type 'bool'")
        return result

    if value.startswith(_AS_NUMBER_PREFIX):
        raw = value[len(_AS_NUMBER_PREFIX) :]
        try:
            result = literal_eval(raw)
        except (ValueError, SyntaxError, MemoryError):
            raise JinjaRenderingError(f"Could not convert value '{raw!s}' into type 'number'")

        if not _is_number(result):
            raise JinjaRenderingError(f"Could not convert value '{raw!s}' into type 'number'")
        return result

    # Shouldn't reach here but just in case
    return value


def _value_finalizer(value: Any) -> Any:
    """Normalize Python values to match Jinja2 string representation.

    minijinja renders booleans as 'true'/'false' and None as 'none',
    but Jinja2 uses Python's 'True'/'False' and 'None'.
    """
    if value is None:
        return "None"
    if isinstance(value, bool):
        return "True" if value else "False"
    return value


def create_environment(ctx: Dict[str, Any], native: bool = False) -> Environment:
    """Create minijinja Environment with dbt-specific configuration.

    Args:
        ctx: Context dictionary with variables and functions to make available
        native: If True, use native type conversion filters

    Returns:
        Configured minijinja Environment
    """
    env = Environment(
        pycompat=True,  # Enable Python compatibility mode
        undefined_behavior="strict",  # Fail on undefined variables
        finalizer=_value_finalizer,  # Normalize booleans and None to match Jinja2
    )

    # Register all context variables and functions as globals
    for name, value in ctx.items():
        env.add_global(name, value)

    # Register dbt-specific filters
    if native:
        # Native mode: filters add marker prefixes for post-processing
        env.add_filter("as_text", lambda x: f"{_AS_TEXT_PREFIX}{x}")
        env.add_filter("as_bool", lambda x: f"{_AS_BOOL_PREFIX}{x}")
        env.add_filter("as_native", lambda x: f"{_AS_NATIVE_PREFIX}{x}")
        env.add_filter("as_number", lambda x: f"{_AS_NUMBER_PREFIX}{x}")
    else:
        # Text mode: filters convert to string
        env.add_filter("as_text", lambda x: str(x))
        env.add_filter("as_bool", lambda x: str(x))
        env.add_filter("as_native", lambda x: str(x))
        env.add_filter("as_number", lambda x: str(x))

    env.add_filter("is_list", lambda x: isinstance(x, list))

    return env


def render(template: str, ctx: Dict[str, Any], native: bool = False) -> Any:
    """Render a Jinja template using minijinja.

    Args:
        template: Template string to render
        ctx: Context dictionary with variables/functions
        native: If True, convert result to native Python types

    Returns:
        Rendered template as string (text mode) or native type (native mode)

    Raises:
        JinjaRenderingError: If template rendering or type conversion fails
    """
    #  Handle non-string templates (comes from YAML parsing)
    # Convert to string for rendering
    if template is None:
        template = "None"
    elif not isinstance(template, str):
        template = str(template)

    env = create_environment(ctx, native)

    try:
        result = env.render_str(template, **ctx)
    except Exception as e:
        raise JinjaRenderingError(str(e)) from e

    # In native mode, process marker prefixes and convert to Python types
    if native:
        result = _convert_native(result)

    return result
