use super::helpers::{
    adapter_internal_packages, default_internal_packages, MacroBuilder, ManifestBuilder,
};
use std::collections::HashSet;

#[test]
fn test_locality_core() {
    let packages = adapter_internal_packages();
    let locality = get_locality("dbt", "myproject", &packages);
    assert_eq!(locality, Locality::Core);
}

#[test]
fn test_locality_root() {
    let packages = default_internal_packages();
    let locality = get_locality("myproject", "myproject", &packages);
    assert_eq!(locality, Locality::Root);
}

#[test]
fn test_locality_imported() {
    let packages = default_internal_packages();
    let locality = get_locality("dep_pkg", "myproject", &packages);
    assert_eq!(locality, Locality::Imported);
}

#[test]
fn test_find_macro_empty_manifest() {
    let manifest = ManifestBuilder::new().build();
    let internal_packages = default_internal_packages();

    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, None);
    assert!(result.is_none());
}

#[test]
fn test_find_macro_single() {
    let mut manifest = ManifestBuilder::new().build();
    let macro1 = MacroBuilder::new("my_macro", "pkg").build();
    manifest.macros.insert(macro1.unique_id.clone(), macro1);

    let internal_packages = default_internal_packages();

    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, None);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.pkg.my_macro");
}

#[test]
fn test_find_macro_priority_root_over_imported() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_imported = MacroBuilder::new("my_macro", "imported_pkg").build();
    let macro_root = MacroBuilder::new("my_macro", "root").build();

    manifest
        .macros
        .insert(macro_imported.unique_id.clone(), macro_imported);
    manifest
        .macros
        .insert(macro_root.unique_id.clone(), macro_root);

    let internal_packages = default_internal_packages();

    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, None);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.root.my_macro");
}

#[test]
fn test_find_macro_priority_imported_over_core() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_core = MacroBuilder::new("my_macro", "dbt").build();
    let macro_imported = MacroBuilder::new("my_macro", "imported_pkg").build();

    manifest
        .macros
        .insert(macro_core.unique_id.clone(), macro_core);
    manifest
        .macros
        .insert(macro_imported.unique_id.clone(), macro_imported);

    let internal_packages = default_internal_packages();

    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, None);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.imported_pkg.my_macro");
}

#[test]
fn test_find_macro_with_package_filter() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_pkg1 = MacroBuilder::new("my_macro", "pkg1").build();
    let macro_pkg2 = MacroBuilder::new("my_macro", "pkg2").build();

    manifest
        .macros
        .insert(macro_pkg1.unique_id.clone(), macro_pkg1);
    manifest
        .macros
        .insert(macro_pkg2.unique_id.clone(), macro_pkg2);

    let internal_packages = default_internal_packages();

    let result = manifest.find_macro_by_name("my_macro", "root", &internal_packages, Some("pkg2"));
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.pkg2.my_macro");
}

#[test]
fn test_materialization_macro_name_default() {
    let name = get_materialization_macro_name("table", "default");
    assert_eq!(name, "materialization_table_default");
}

#[test]
fn test_materialization_macro_name_adapter() {
    let name = get_materialization_macro_name("table", "postgres");
    assert_eq!(name, "materialization_table_postgres");
}

#[test]
fn test_find_materialization_empty() {
    let manifest = ManifestBuilder::new().build();
    let internal_packages = default_internal_packages();
    let adapter_types = vec!["postgres".to_string(), "default".to_string()];

    let result = manifest.find_materialization_macro_by_name(
        "root",
        "table",
        &adapter_types,
        &internal_packages,
        true,
    );
    assert!(result.is_none());
}

#[test]
fn test_find_materialization_default_fallback() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_default = MacroBuilder::new("materialization_table_default", "dbt").build();
    manifest
        .macros
        .insert(macro_default.unique_id.clone(), macro_default);

    let internal_packages = default_internal_packages();
    let adapter_types = vec!["postgres".to_string(), "default".to_string()];

    let result = manifest.find_materialization_macro_by_name(
        "root",
        "table",
        &adapter_types,
        &internal_packages,
        true,
    );
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.dbt.materialization_table_default");
}

#[test]
fn test_find_materialization_adapter_specific() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_postgres = MacroBuilder::new("materialization_table_postgres", "dbt").build();
    let macro_default = MacroBuilder::new("materialization_table_default", "dbt").build();

    manifest
        .macros
        .insert(macro_postgres.unique_id.clone(), macro_postgres);
    manifest
        .macros
        .insert(macro_default.unique_id.clone(), macro_default);

    let internal_packages = default_internal_packages();
    let adapter_types = vec!["postgres".to_string(), "default".to_string()];

    let result = manifest.find_materialization_macro_by_name(
        "root",
        "table",
        &adapter_types,
        &internal_packages,
        true,
    );
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.dbt.materialization_table_postgres");
}

#[test]
fn test_find_materialization_core_protection() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_core = MacroBuilder::new("materialization_table_default", "dbt").build();
    let macro_imported = MacroBuilder::new("materialization_table_default", "imported_pkg").build();

    manifest
        .macros
        .insert(macro_core.unique_id.clone(), macro_core);
    manifest
        .macros
        .insert(macro_imported.unique_id.clone(), macro_imported);

    let internal_packages = default_internal_packages();
    let adapter_types = vec!["default".to_string()];

    let result = manifest.find_materialization_macro_by_name(
        "root",
        "table",
        &adapter_types,
        &internal_packages,
        false,
    );
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "macro.dbt.materialization_table_default");
}

#[test]
fn test_find_materialization_legacy_mode() {
    let mut manifest = ManifestBuilder::new().build();

    let macro_core = MacroBuilder::new("materialization_table_default", "dbt").build();
    let macro_imported = MacroBuilder::new("materialization_table_default", "imported_pkg").build();

    manifest
        .macros
        .insert(macro_core.unique_id.clone(), macro_core);
    manifest
        .macros
        .insert(macro_imported.unique_id.clone(), macro_imported);

    let internal_packages = default_internal_packages();
    let adapter_types = vec!["default".to_string()];

    let result = manifest.find_materialization_macro_by_name(
        "root",
        "table",
        &adapter_types,
        &internal_packages,
        true,
    );
    assert!(result.is_some());
    assert_eq!(
        result.unwrap(),
        "macro.imported_pkg.materialization_table_default"
    );
}

#[test]
fn test_get_macro_found() {
    let mut manifest = ManifestBuilder::new().build();
    let macro1 = MacroBuilder::new("test_macro", "pkg").build();
    let unique_id = macro1.unique_id.clone();
    manifest.macros.insert(unique_id.clone(), macro1);

    let result = manifest.get_macro(&unique_id);
    assert!(result.is_some());
    let macro_ref = result.unwrap();
    assert_eq!(macro_ref.name, "test_macro");
    assert_eq!(macro_ref.package_name, "pkg");
}

#[test]
fn test_get_macro_not_found() {
    let manifest = ManifestBuilder::new().build();

    let result = manifest.get_macro("macro.nonexistent.test");
    assert!(result.is_none());
}

use crate::manifest::macro_lookup::{get_locality, get_materialization_macro_name, Locality};
