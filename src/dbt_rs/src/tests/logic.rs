// Tests for OxideManifest logic methods

use super::helpers::{default_internal_packages, ManifestBuilder, ModelBuilder};
use crate::manifest::*;

#[test]
fn test_build_parent_map() {
    let mut manifest = ManifestBuilder::new().build();

    // Create parent node
    let parent = ModelBuilder::new("parent", "pkg").build();
    manifest.add_node(OxideNode::Model(parent));

    // Create child node depending on parent
    let child = ModelBuilder::new("child", "pkg")
        .depends_on(vec!["model.pkg.parent"])
        .build();
    manifest.add_node(OxideNode::Model(child));

    // Build parent map and verify
    let parent_map = manifest.build_parent_map();

    assert!(parent_map.contains_key("model.pkg.child"));
    let child_parents = parent_map.get("model.pkg.child").unwrap();
    assert!(child_parents.contains(&"model.pkg.parent".to_string()));
}

#[test]
fn test_build_child_map() {
    let mut manifest = ManifestBuilder::new().build();

    // Create parent node
    let parent = ModelBuilder::new("parent", "pkg").build();
    manifest.add_node(OxideNode::Model(parent));

    // Create child node depending on parent
    let child = ModelBuilder::new("child", "pkg")
        .depends_on(vec!["model.pkg.parent"])
        .build();
    manifest.add_node(OxideNode::Model(child));

    // Build child map and verify
    let child_map = manifest.build_child_map();

    assert!(child_map.contains_key("model.pkg.parent"));
    let parent_children = child_map.get("model.pkg.parent").unwrap();
    assert!(parent_children.contains(&"model.pkg.child".to_string()));
}

#[test]
fn test_build_group_map() {
    let mut manifest = ManifestBuilder::new().build();

    // Create node in group A
    let node1 = ModelBuilder::new("node1", "pkg")
        .with_group("group_a")
        .build();
    manifest.add_node(OxideNode::Model(node1));

    // Create another node in group A
    let node2 = ModelBuilder::new("node2", "pkg")
        .with_group("group_a")
        .build();
    manifest.add_node(OxideNode::Model(node2));

    // Create node in group B
    let node3 = ModelBuilder::new("node3", "pkg")
        .with_group("group_b")
        .build();
    manifest.add_node(OxideNode::Model(node3));

    // Build group map and verify
    let group_map = manifest.build_group_map();

    assert!(group_map.contains_key("group_a"));
    let group_a_nodes = group_map.get("group_a").unwrap();
    assert_eq!(group_a_nodes.len(), 2);
    assert!(group_a_nodes.contains(&"model.pkg.node1".to_string()));
    assert!(group_a_nodes.contains(&"model.pkg.node2".to_string()));

    assert!(group_map.contains_key("group_b"));
    let group_b_nodes = group_map.get("group_b").unwrap();
    assert_eq!(group_b_nodes.len(), 1);
    assert!(group_b_nodes.contains(&"model.pkg.node3".to_string()));
}


#[test]
fn test_find_macro_by_name() {
    let mut manifest = ManifestBuilder::new().build();
    use super::helpers::MacroBuilder;

    // Create a macro
    let macro_obj = MacroBuilder::new("generate_alias_name", "pkg").build();
    manifest.add_macro(macro_obj);

    // Find macro by name
    let internal_packages = default_internal_packages();
    let result =
        manifest.find_macro_by_name("generate_alias_name", "root", &internal_packages, None);
    assert_eq!(result, Some("macro.pkg.generate_alias_name".to_string()));
}

#[test]
fn test_find_macro_by_name_with_package() {
    let mut manifest = ManifestBuilder::new().build();
    use super::helpers::MacroBuilder;

    // Create two macros with same name in different packages
    let macro1 = MacroBuilder::new("my_macro", "pkg1").build();
    manifest.add_macro(macro1);

    let macro2 = MacroBuilder::new("my_macro", "pkg2").build();
    manifest.add_macro(macro2);

    // Find macro with specific package
    let internal_packages = default_internal_packages();
    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, Some("pkg2"));
    assert_eq!(result, Some("macro.pkg2.my_macro".to_string()));

    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, Some("pkg1"));
    assert_eq!(result, Some("macro.pkg1.my_macro".to_string()));
}

#[test]
fn test_find_macro_not_found() {
    let manifest = ManifestBuilder::new().build();
    let internal_packages = default_internal_packages();

    let result = manifest.find_macro_by_name("nonexistent", "root", &internal_packages, None);
    assert_eq!(result, None);
}

#[test]
fn test_find_materialization_by_name() {
    let mut manifest = ManifestBuilder::new().build();
    use super::helpers::MacroBuilder;

    // Create a materialization macro
    let macro_obj = MacroBuilder::new("materialization_table_default", "dbt").build();
    manifest.add_macro(macro_obj);

    // Find materialization
    let internal_packages = default_internal_packages();
    let adapter_types = vec!["default".to_string()];
    let result = manifest.find_materialization_macro_by_name(
        "root",
        "table",
        &adapter_types,
        &internal_packages,
        true,
    );
    assert_eq!(
        result,
        Some("macro.dbt.materialization_table_default".to_string())
    );
}

#[test]
fn test_find_materialization_with_adapter() {
    let mut manifest = ManifestBuilder::new().build();
    use super::helpers::MacroBuilder;

    // Create default and postgres-specific materializations
    let default_macro = MacroBuilder::new("materialization_view_default", "dbt").build();
    manifest.add_macro(default_macro);

    let postgres_macro = MacroBuilder::new("materialization_view_postgres", "dbt").build();
    manifest.add_macro(postgres_macro);

    // Should find postgres-specific version
    let internal_packages = default_internal_packages();
    let adapter_types_postgres = vec!["postgres".to_string(), "default".to_string()];
    let result = manifest.find_materialization_macro_by_name(
        "root",
        "view",
        &adapter_types_postgres,
        &internal_packages,
        true,
    );
    assert_eq!(
        result,
        Some("macro.dbt.materialization_view_postgres".to_string())
    );

    // Should find default version when adapter doesn't exist
    let adapter_types_snowflake = vec!["snowflake".to_string(), "default".to_string()];
    let result = manifest.find_materialization_macro_by_name(
        "root",
        "view",
        &adapter_types_snowflake,
        &internal_packages,
        true,
    );
    assert_eq!(
        result,
        Some("macro.dbt.materialization_view_default".to_string())
    );

    // Should find default when explicitly requested
    let adapter_types_default = vec!["default".to_string()];
    let result = manifest.find_materialization_macro_by_name(
        "root",
        "view",
        &adapter_types_default,
        &internal_packages,
        true,
    );
    assert_eq!(
        result,
        Some("macro.dbt.materialization_view_default".to_string())
    );
}
