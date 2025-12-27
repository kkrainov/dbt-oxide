"""
Test OxideManifest Python wrapper (Option B)
Tests the separate OxideManifest class that delegates to Rust.
"""
import json
from unittest import mock

import pytest

from dbt.adapters.base.plugin import AdapterPlugin
from dbt.contracts.graph.oxide_manifest import OxideManifest
from tests.unit.utils import inject_plugin, clear_plugin

class TestOxideManifestImport:
    """Test OxideManifest can be imported."""

    def test_oxide_manifest_can_be_imported(self):
        """Test that OxideManifest can be imported from dbt_rs."""
        from dbt_rs import OxideManifest

        assert OxideManifest is not None

    def test_manifest_builder_can_be_imported(self):
        """Test that ManifestBuilder is still available."""
        from dbt_rs import ManifestBuilder

        assert ManifestBuilder is not None


class TestOxideManifestCreation:
    """Test OxideManifest creation."""

    def test_create_empty_manifest(self):
        """Test creating an empty OxideManifest."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest()
        assert manifest is not None
        assert manifest.node_count == 0

    def test_from_json_minimal(self):
        """Test loading manifest from minimal JSON."""
        from dbt_rs import OxideManifest

        minimal_json = json.dumps(
            {
                "metadata": {
                    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
                },
                "nodes": {},
                "sources": {},
                "macros": {},
                "docs": {},
                "exposures": {},
                "metrics": {},
                "groups": {},
                "selectors": {},
                "disabled": {},
                "parent_map": {},
                "child_map": {},
                "group_map": {},
                "semantic_models": {},
                "unit_tests": {},
                "saved_queries": {},
            }
        )

        manifest = OxideManifest.from_json(minimal_json)
        assert manifest is not None
        assert manifest.node_count == 0


class TestOxideManifestMaps:
    """Test OxideManifest map building."""

    def test_build_parent_map_empty(self):
        """Test build_parent_map on empty manifest."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest()
        parent_map = manifest.build_parent_map()
        assert isinstance(parent_map, dict)
        assert len(parent_map) == 0

    def test_build_child_map_empty(self):
        """Test build_child_map on empty manifest."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest()
        child_map = manifest.build_child_map()
        assert isinstance(child_map, dict)
        assert len(child_map) == 0

    def test_build_group_map_empty(self):
        """Test build_group_map on empty manifest."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest()
        group_map = manifest.build_group_map()
        assert isinstance(group_map, dict)
        assert len(group_map) == 0


class TestGetManifestClass:
    """Test factory function get_manifest_class."""

    def test_returns_manifest_when_disabled(self, monkeypatch):
        """Test get_manifest_class returns Manifest when Rust disabled."""
        monkeypatch.setenv("DBT_USE_RUST_MANIFEST", "0")

        from dbt.contracts.graph import get_manifest_class
        from dbt.contracts.graph.manifest import Manifest

        klass = get_manifest_class()
        assert klass is Manifest

    def test_returns_oxide_manifest_when_enabled(self, monkeypatch):
        """Test get_manifest_class returns OxideManifest when Rust enabled."""
        monkeypatch.setenv("DBT_USE_RUST_MANIFEST", "1")

        from dbt.contracts.graph import get_manifest_class

        klass = get_manifest_class()
        assert klass is OxideManifest


class TestParentChildMaps:
    """Test parent/child map building with model dependencies."""

    def _create_manifest_with_deps(self):
        """Create a manifest JSON with model dependencies for testing."""
        return json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {
                "model.pkg.model_a": {
                    "unique_id": "model.pkg.model_a",
                    "name": "model_a",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "model_a.sql",
                    "original_file_path": "models/model_a.sql",
                    "fqn": ["pkg", "model_a"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "model_a",
                    "checksum": {"name": "sha256", "checksum": "abc123"},
                },
                "model.pkg.model_b": {
                    "unique_id": "model.pkg.model_b",
                    "name": "model_b",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "model_b.sql",
                    "original_file_path": "models/model_b.sql",
                    "fqn": ["pkg", "model_b"],
                    "depends_on": {"nodes": ["model.pkg.model_a"], "macros": []},
                    "schema": "public",
                    "alias": "model_b",
                    "checksum": {"name": "sha256", "checksum": "def456"},
                },
                "model.pkg.model_c": {
                    "unique_id": "model.pkg.model_c",
                    "name": "model_c",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "model_c.sql",
                    "original_file_path": "models/model_c.sql",
                    "fqn": ["pkg", "model_c"],
                    "depends_on": {"nodes": ["model.pkg.model_a", "model.pkg.model_b"], "macros": []},
                    "schema": "public",
                    "alias": "model_c",
                    "checksum": {"name": "sha256", "checksum": "ghi789"},
                },
            },
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "parent_map": {},
            "child_map": {},
            "group_map": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

    def test_build_parent_map_with_dependencies(self):
        """Test parent map correctly captures dependencies."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest.from_json(self._create_manifest_with_deps())
        parent_map = manifest.build_parent_map()

        # model_a has no parents
        assert parent_map.get("model.pkg.model_a") == []
        # model_b depends on model_a
        assert parent_map.get("model.pkg.model_b") == ["model.pkg.model_a"]
        # model_c depends on both model_a and model_b
        deps_c = parent_map.get("model.pkg.model_c", [])
        assert "model.pkg.model_a" in deps_c
        assert "model.pkg.model_b" in deps_c

    def test_build_child_map_with_dependencies(self):
        """Test child map correctly reverses parent relationships."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest.from_json(self._create_manifest_with_deps())
        child_map = manifest.build_child_map()

        # model_a is parent of model_b and model_c
        children_a = child_map.get("model.pkg.model_a", [])
        assert "model.pkg.model_b" in children_a
        assert "model.pkg.model_c" in children_a
        # model_b is parent of model_c
        children_b = child_map.get("model.pkg.model_b", [])
        assert "model.pkg.model_c" in children_b
        # model_c has no children
        assert child_map.get("model.pkg.model_c") == []


class TestGroupMap:
    """Test group map building with models in groups."""

    def _create_manifest_with_groups(self):
        """Create a manifest JSON with models in groups."""
        return json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {
                "model.pkg.model_analytics_1": {
                    "unique_id": "model.pkg.model_analytics_1",
                    "name": "model_analytics_1",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "model_analytics_1.sql",
                    "original_file_path": "models/model_analytics_1.sql",
                    "fqn": ["pkg", "model_analytics_1"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "model_analytics_1",
                    "checksum": {"name": "sha256", "checksum": "aaa"},
                    "config": {"group": "analytics"},
                },
                "model.pkg.model_analytics_2": {
                    "unique_id": "model.pkg.model_analytics_2",
                    "name": "model_analytics_2",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "model_analytics_2.sql",
                    "original_file_path": "models/model_analytics_2.sql",
                    "fqn": ["pkg", "model_analytics_2"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "model_analytics_2",
                    "checksum": {"name": "sha256", "checksum": "bbb"},
                    "config": {"group": "analytics"},
                },
                "model.pkg.model_finance": {
                    "unique_id": "model.pkg.model_finance",
                    "name": "model_finance",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "model_finance.sql",
                    "original_file_path": "models/model_finance.sql",
                    "fqn": ["pkg", "model_finance"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "model_finance",
                    "checksum": {"name": "sha256", "checksum": "ccc"},
                    "config": {"group": "finance"},
                },
            },
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "parent_map": {},
            "child_map": {},
            "group_map": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

    def test_build_group_map_with_groups(self):
        """Test group map correctly groups models."""
        from dbt_rs import OxideManifest

        manifest = OxideManifest.from_json(self._create_manifest_with_groups())
        group_map = manifest.build_group_map()

        # analytics group has 2 models
        analytics = group_map.get("analytics", [])
        assert len(analytics) == 2
        assert "model.pkg.model_analytics_1" in analytics
        assert "model.pkg.model_analytics_2" in analytics
        # finance group has 1 model
        finance = group_map.get("finance", [])
        assert len(finance) == 1
        assert "model.pkg.model_finance" in finance


class TestResolveRefAndSource:
    """Test resolve_ref and resolve_source methods."""

    def test_resolve_ref_by_name(self):
        """Test resolving a ref by name only."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {
                "model.pkg.my_model": {
                    "unique_id": "model.pkg.my_model",
                    "name": "my_model",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "my_model.sql",
                    "original_file_path": "models/my_model.sql",
                    "fqn": ["pkg", "my_model"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "my_model",
                    "checksum": {"name": "sha256", "checksum": "abc123"},
                },
            },
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        result = manifest.resolve_ref("my_model")
        assert result == "model.pkg.my_model"

    def test_resolve_ref_by_name_and_package(self):
        """Test resolving a ref with package specification."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {
                "model.pkg1.shared_name": {
                    "unique_id": "model.pkg1.shared_name",
                    "name": "shared_name",
                    "resource_type": "model",
                    "package_name": "pkg1",
                    "path": "shared_name.sql",
                    "original_file_path": "models/shared_name.sql",
                    "fqn": ["pkg1", "shared_name"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "shared_name",
                    "checksum": {"name": "sha256", "checksum": "abc123"},
                },
                "model.pkg2.shared_name": {
                    "unique_id": "model.pkg2.shared_name",
                    "name": "shared_name",
                    "resource_type": "model",
                    "package_name": "pkg2",
                    "path": "shared_name.sql",
                    "original_file_path": "models/shared_name.sql",
                    "fqn": ["pkg2", "shared_name"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "shared_name",
                    "checksum": {"name": "sha256", "checksum": "def456"},
                },
            },
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        
        # With package filter, should return correct package
        result = manifest.resolve_ref("shared_name", package="pkg2")
        assert result == "model.pkg2.shared_name"
        
        result = manifest.resolve_ref("shared_name", package="pkg1")
        assert result == "model.pkg1.shared_name"

    def test_resolve_ref_not_found(self):
        """Test resolve_ref returns None for non-existent model."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {},
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        result = manifest.resolve_ref("nonexistent")
        assert result is None

    def test_resolve_source_simple(self):
        """Test resolving a source by source_name and table_name."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {},
            "sources": {
                "source.pkg.raw.users": {
                    "unique_id": "source.pkg.raw.users",
                    "source_name": "raw",
                    "name": "users",
                    "package_name": "pkg",
                },
            },
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        result = manifest.resolve_source("raw", "users")
        assert result == "source.pkg.raw.users"

    def test_resolve_source_not_found(self):
        """Test resolve_source returns None for non-existent source."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {},
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        result = manifest.resolve_source("nonexistent", "table")
        assert result is None


class TestResourceTypes:
    """Test various resource types beyond models."""

    def test_manifest_with_seeds(self):
        """Test that seeds are correctly loaded and counted."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {
                "seed.pkg.raw_customers": {
                    "unique_id": "seed.pkg.raw_customers",
                    "name": "raw_customers",
                    "resource_type": "seed",
                    "package_name": "pkg",
                    "path": "seeds/raw_customers.csv",
                    "original_file_path": "seeds/raw_customers.csv",
                    "fqn": ["pkg", "raw_customers"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "raw_customers",
                    "checksum": {"name": "sha256", "checksum": "seed123"},
                },
                "seed.pkg.raw_orders": {
                    "unique_id": "seed.pkg.raw_orders",
                    "name": "raw_orders",
                    "resource_type": "seed",
                    "package_name": "pkg",
                    "path": "seeds/raw_orders.csv",
                    "original_file_path": "seeds/raw_orders.csv",
                    "fqn": ["pkg", "raw_orders"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "raw_orders",
                    "checksum": {"name": "sha256", "checksum": "seed456"},
                },
            },
            "sources": {},
            "macros": {},
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        
        # Verify both seeds are counted
        assert manifest.node_count == 2

    def test_build_maps_mixed_resource_types(self):
        """Test parent/child maps with mixed resource types."""
        from dbt_rs import OxideManifest

        manifest_json = json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
            },
            "nodes": {
                "seed.pkg.raw_customers": {
                    "unique_id": "seed.pkg.raw_customers",
                    "name": "raw_customers",
                    "resource_type": "seed",
                    "package_name": "pkg",
                    "path": "seeds/raw_customers.csv",
                    "original_file_path": "seeds/raw_customers.csv",
                    "fqn": ["pkg", "raw_customers"],
                    "depends_on": {"nodes": [], "macros": []},
                    "schema": "public",
                    "alias": "raw_customers",
                    "checksum": {"name": "sha256", "checksum": "seed123"},
                },
                "model.pkg.stg_customers": {
                    "unique_id": "model.pkg.stg_customers",
                    "name": "stg_customers",
                    "resource_type": "model",
                    "package_name": "pkg",
                    "path": "staging/stg_customers.sql",
                    "original_file_path": "models/staging/stg_customers.sql",
                    "fqn": ["pkg", "staging", "stg_customers"],
                    "depends_on": {
                        "nodes": ["seed.pkg.raw_customers", "source.pkg.raw.orders"],
                        "macros": []
                    },
                    "schema": "public",
                    "alias": "stg_customers",
                    "checksum": {"name": "sha256", "checksum": "model123"},
                },
            },
            "sources": {
                "source.pkg.raw.orders": {
                    "unique_id": "source.pkg.raw.orders",
                    "source_name": "raw",
                    "name": "orders",
                    "package_name": "pkg",
                },
            },
            "macros": {},
            "docs": {},
            "exposures": {
                "exposure.pkg.customer_dashboard": {
                    "unique_id": "exposure.pkg.customer_dashboard",
                    "name": "customer_dashboard",
                    "package_name": "pkg",
                    "path": "models/schema.yml",
                    "original_file_path": "models/schema.yml",
                    "fqn": ["pkg", "customer_dashboard"],
                    "type": "dashboard",
                    "owner": {"email": "analytics@company.com"},
                    "depends_on": {
                        "nodes": ["model.pkg.stg_customers"],
                        "macros": []
                    },
                    "description": "",
                    "meta": {},
                    "tags": [],
                    "config": {},
                    "unrendered_config": {},
                    "refs": [],
                    "sources": [],
                    "metrics": [],
                    "created_at": 0.0,
                },
            },
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
            "semantic_models": {},
            "unit_tests": {},
            "saved_queries": {},
        })

        manifest = OxideManifest.from_json(manifest_json)
        
        # Build maps
        parent_map = manifest.build_parent_map()
        child_map = manifest.build_child_map()
        
        # Verify parent relationships
        # Seed has no parents
        assert parent_map["seed.pkg.raw_customers"] == []
        
        # Source has no parents
        assert parent_map["source.pkg.raw.orders"] == []
        
        # Model depends on seed and source
        model_parents = parent_map["model.pkg.stg_customers"]
        assert "seed.pkg.raw_customers" in model_parents
        assert "source.pkg.raw.orders" in model_parents
        
        # Exposure depends on model
        assert parent_map["exposure.pkg.customer_dashboard"] == ["model.pkg.stg_customers"]
        
        # Verify child relationships
        # Seed has model as child
        assert child_map["seed.pkg.raw_customers"] == ["model.pkg.stg_customers"]
        
        # Source has model as child
        assert child_map["source.pkg.raw.orders"] == ["model.pkg.stg_customers"]
        
        # Model has exposure as child
        assert child_map["model.pkg.stg_customers"] == ["exposure.pkg.customer_dashboard"]
        
        # Exposure has no children
        assert child_map["exposure.pkg.customer_dashboard"] == []


class TestOxideManifestMacroLookup:
    """Test OxideManifest macro lookup - mirrors test_manifest.py structure."""

    @pytest.fixture(autouse=True)
    def setup_adapter_plugin(self):
        """Inject adapter plugin for tests, same as Python Manifest tests do."""
        # Create and inject a mock postgres adapter plugin
        self.mock_plugin = AdapterPlugin(
            adapter=mock.MagicMock(),
            credentials=mock.MagicMock(),
            include_path="/path/to/root/plugin",
            project_name="postgres",
        )
        self.mock_plugin.adapter.type.return_value = "postgres"
        inject_plugin(self.mock_plugin)
        yield
        clear_plugin(self.mock_plugin)

    def _create_minimal_macro(self, package_name, name="my_macro"):
        """Create a minimal macro dict for JSON serialization."""
        return {
            "unique_id": f"macro.{package_name}.{name}",
            "name": name,
            "package_name": package_name,
            "resource_type": "macro",
            "path": f"macros/{name}.sql",
            "original_file_path": f"macros/{name}.sql",
            "macro_sql": "{% macro my_macro() %}{% endmacro %}",
            "depends_on": {"macros": []},
            "description": "",
            "meta": {},
            "docs": {"show": True},
        }

    def _create_manifest_json(self, macros):
        """Create manifest JSON from list of macro dicts."""
        macros_dict = {m["unique_id"]: m for m in macros}
        return json.dumps({
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
                "adapter_type": "postgres",
            },
            "nodes": {},
            "sources": {},
            "macros": macros_dict,
            "docs": {},
            "exposures": {},
            "metrics": {},
            "groups": {},
            "selectors": {},
            "disabled": {},
        })

    def test_find_macro_empty_manifest(self):
        """Test find_macro_by_name on empty manifest returns None."""

        manifest_json = self._create_manifest_json([])
        manifest = OxideManifest.from_json(manifest_json)

        result = manifest.find_macro_by_name(
            name="my_macro",
            root_project_name="root",
            package=None
        )

        assert result is None

    def test_find_macro_root_only(self):
        """Test finding macro when only root package has it."""

        macros = [self._create_minimal_macro("root")]
        manifest_json = self._create_manifest_json(macros)
        manifest = OxideManifest.from_json(manifest_json)

        # No package filter - should find root
        result = manifest.find_macro_by_name("my_macro", "root", None)
        assert result is not None
        assert result.package_name == "root"

        # Explicit root package
        result = manifest.find_macro_by_name("my_macro", "root", "root")
        assert result is not None
        assert result.package_name == "root"

        # Different package - should not find
        result = manifest.find_macro_by_name("my_macro", "root", "dep")
        assert result is None

    def test_find_macro_dep_only(self):
        """Test finding macro when only dep package has it."""

        macros = [self._create_minimal_macro("dep")]
        manifest_json = self._create_manifest_json(macros)
        manifest = OxideManifest.from_json(manifest_json)

        # No package filter - should find dep
        result = manifest.find_macro_by_name("my_macro", "root", None)
        assert result is not None
        assert result.package_name == "dep"

        # Explicit dep package
        result = manifest.find_macro_by_name("my_macro", "root", "dep")
        assert result is not None
        assert result.package_name == "dep"

    def test_find_macro_root_overrides_dep(self):
        """Test that root package macro is prioritized over dep."""

        macros = [
            self._create_minimal_macro("root"),
            self._create_minimal_macro("dep"),
        ]
        manifest_json = self._create_manifest_json(macros)
        manifest = OxideManifest.from_json(manifest_json)

        # No filter - root wins
        result = manifest.find_macro_by_name("my_macro", "root", None)
        assert result is not None
        assert result.package_name == "root"

        # Can still access dep explicitly
        result = manifest.find_macro_by_name("my_macro", "root", "dep")
        assert result is not None
        assert result.package_name == "dep"

    def test_find_macro_dep_overrides_dbt(self):
        """Test that dep package macro is prioritized over dbt (core)."""

        macros = [
            self._create_minimal_macro("dep"),
            self._create_minimal_macro("dbt"),
        ]
        manifest_json = self._create_manifest_json(macros)
        manifest = OxideManifest.from_json(manifest_json)

        # No filter - dep wins over dbt
        result = manifest.find_macro_by_name("my_macro", "root", None)
        assert result is not None
        assert result.package_name == "dep"

        # Can still access dbt explicitly
        result = manifest.find_macro_by_name("my_macro", "root", "dbt")
        assert result is not None
        assert result.package_name == "dbt"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
