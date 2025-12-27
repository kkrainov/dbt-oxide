"""OxideManifest - Python wrapper around Rust OxideManifest.

This is a separate class that delegates all operations to Rust.
It follows the same pattern as Graph (wrapping dbt_rs.DbtGraph).

Usage:
    # Create from JSON
    manifest = OxideManifest.from_json(json_str)
    
    # Or from existing Python Manifest
    manifest = OxideManifest.from_manifest(python_manifest)
    
    # Use Rust operations
    parent_map = manifest.build_parent_map()
    child_map = manifest.build_child_map()
"""

from typing import Any, Dict, Iterator, List, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from dbt.contracts.graph.manifest import Manifest
    from dbt.contracts.graph.nodes import Macro

from dbt.adapters.factory import get_adapter_package_names, get_adapter_type_names
from dbt.contracts.graph.nodes import Macro
from dbt.flags import get_flags

import dbt_rs


class OxideManifest:
    """Python wrapper around Rust OxideManifest - Rust owns all data."""
    
    def __init__(
        self,
        nodes: Optional[Dict[str, Any]] = None,
        sources: Optional[Dict[str, Any]] = None,
        macros: Optional[Dict[str, Any]] = None,
        docs: Optional[Dict[str, Any]] = None,
        exposures: Optional[Dict[str, Any]] = None,
        metrics: Optional[Dict[str, Any]] = None,
        groups: Optional[Dict[str, Any]] = None,
        selectors: Optional[Dict[str, Any]] = None,
        disabled: Optional[Dict[str, Any]] = None,
        files: Optional[Dict[str, Any]] = None,
        semantic_models: Optional[Dict[str, Any]] = None,
        unit_tests: Optional[Dict[str, Any]] = None,
        saved_queries: Optional[Dict[str, Any]] = None,
        fixtures: Optional[Dict[str, Any]] = None,
        metadata: Optional[Dict[str, Any]] = None,
        _rust_manifest: Any = None,  # For internal use (from_json)
    ):
        if _rust_manifest is not None:
            self._inner = _rust_manifest
        else:
            # Forward to Rust constructor - same API as Python Manifest!
            self._inner = dbt_rs.OxideManifest(
                nodes=nodes,
                sources=sources,
                macros=macros,
                docs=docs,
                exposures=exposures,
                metrics=metrics,
                groups=groups,
                selectors=selectors,
                disabled=disabled,
                files=files,
                semantic_models=semantic_models,
                unit_tests=unit_tests,
                saved_queries=saved_queries,
                fixtures=fixtures,
                metadata=metadata,
            )
    
    @classmethod
    def from_json(cls, json_str: str) -> "OxideManifest":
        """Load manifest from JSON string."""
        rust_manifest = dbt_rs.OxideManifest.from_json(json_str)
        return cls(_rust_manifest=rust_manifest)
    
    def build_parent_map(self) -> Dict[str, List[str]]:
        """Build parent map: maps each node to its direct dependencies."""
        return self._inner.build_parent_map()
    
    def build_child_map(self) -> Dict[str, List[str]]:
        """Build child map: maps each node to what depends on it."""
        return self._inner.build_child_map()
    
    def build_group_map(self) -> Dict[str, List[str]]:
        """Build group map: maps group names to their member nodes."""
        return self._inner.build_group_map()
    
    def resolve_ref(
        self,
        source_node,
        target_model_name: str,
        target_model_package: Optional[str],
        target_model_version: Optional[int],
        current_project: str,
        node_package: str,
    ) -> Optional[str]:
        """Resolve a ref to its unique_id."""
        source_node_id = source_node.unique_id if source_node else None
        return self._inner.resolve_ref(
            source_node_id,
            target_model_name,
            target_model_package,
            target_model_version,
            current_project,
            node_package,
        )
    
    def resolve_source(
        self,
        target_source_name: str,
        target_table_name: str,
        current_project: str,
        node_package: str,
    ) -> Optional[str]:
        """Resolve a source to its unique_id."""
        return self._inner.resolve_source(
            target_source_name,
            target_table_name,
            current_project,
            node_package,
        )
    
    def resolve_doc(
        self,
        name: str,
        package: Optional[str],
        current_project: str,
        node_package: str,
    ) -> Optional[str]:
        """Resolve a doc to its unique_id."""
        return self._inner.resolve_doc(name, package, current_project, node_package)
    
    def find_macro_by_name(
        self, name: str, root_project_name: str, package: Optional[str] = None
    ) -> Optional["Macro"]:
        """Find macro by name with priority and package filtering."""
        adapter_type = self._inner.get_adapter_type()
        internal_packages = set(get_adapter_package_names(adapter_type))
        
        unique_id = self._inner.find_macro_by_name(
            name=name,
            root_project_name=root_project_name,
            internal_packages=internal_packages,
            package=package,
        )
        
        if unique_id:
            macro_dict = self._inner.get_macro(unique_id)
            if macro_dict is not None:
                return Macro.from_dict(macro_dict)
        return None
    
    def find_materialization_macro_by_name(
        self, project_name: str, materialization_name: str, adapter_type: str
    ) -> Optional["Macro"]:
        """Find materialization macro by name with adapter inheritance."""
        internal_packages = set(get_adapter_package_names(adapter_type))
        adapter_types = get_adapter_type_names(adapter_type) + ["default"]
        allow_override = not get_flags().require_explicit_package_overrides_for_builtin_materializations
        
        unique_id = self._inner.find_materialization_macro_by_name(
            project_name=project_name,
            materialization_name=materialization_name,
            adapter_types=adapter_types,
            internal_packages=internal_packages,
            allow_package_override=allow_override,
        )
        
        if unique_id:
            macro_dict = self._inner.get_macro(unique_id)
            if macro_dict is not None:
                return Macro.from_dict(macro_dict)
        return None
    
    @property
    def node_count(self) -> int:
        """Number of nodes in manifest."""
        return self._inner.node_count()
    
    def write(self, path: str) -> None:
        """Write manifest to file."""
        self._inner.write(path)
