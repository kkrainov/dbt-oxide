"""dbt graph contracts package."""

import os
from typing import Type


def get_manifest_class() -> Type:
    """Return OxideManifest if Rust enabled, else Python Manifest.
    
    Set DBT_USE_RUST_MANIFEST=1 to use OxideManifest.
    """
    if os.getenv("DBT_USE_RUST_MANIFEST") == "1":
        try:
            from .oxide_manifest import OxideManifest
            return OxideManifest
        except ImportError:
            pass
    from .manifest import Manifest
    return Manifest
