use super::helpers::{
    AnalysisBuilder, ManifestBuilder, MetadataBuilder, ModelBuilder, SeedBuilder, SnapshotBuilder,
    TestBuilder,
};
use crate::manifest::*;
use serde_json;

#[test]
fn test_serialize_empty_manifest() {
    let manifest = ManifestBuilder::new().build();
    let json = serde_json::to_string(&manifest).unwrap();
    assert!(json.contains("\"nodes\""));
}

#[test]
fn test_serialize_manifest_metadata_all_fields() {
    let metadata = MetadataBuilder::new().build();
    let json = serde_json::to_string(&metadata).unwrap();
    assert!(json.contains("\"dbt_schema_version\""));
}

#[test]
fn test_serialize_writable_manifest() {
    let manifest = ManifestBuilder::new().build();
    let json = serde_json::to_string_pretty(&manifest).unwrap();
    assert!(json.contains("\"metadata\""));
}

#[test]
fn test_serialize_manifest_with_all_resource_types() {
    let manifest = ManifestBuilder::new().build();
    let json = serde_json::to_string(&manifest).unwrap();
    assert!(json.contains("\"semantic_models\""));
}

#[test]
fn test_manifest_round_trip() {
    let manifest = ManifestBuilder::new().build();
    let json = serde_json::to_string(&manifest).unwrap();
    let deserialized: OxideManifest = serde_json::from_str(&json).unwrap();
    let json2 = serde_json::to_string(&deserialized).unwrap();
    assert_eq!(json, json2);
}

#[test]
fn test_serialize_manifest_with_all_node_types() {
    let mut manifest = ManifestBuilder::new().build();

    // Model
    let model = ModelBuilder::new("m", "pkg").build();
    manifest.add_node(OxideNode::Model(model));

    // Seed
    let seed = SeedBuilder::new("s", "pkg").build();
    manifest.add_node(OxideNode::Seed(seed));

    // Snapshot
    let snapshot = SnapshotBuilder::new("sn", "pkg").build();
    manifest.add_node(OxideNode::Snapshot(snapshot));

    // Analysis
    let analysis = AnalysisBuilder::new("a", "pkg").build();
    manifest.add_node(OxideNode::Analysis(analysis));

    // SingularTest
    let singular_test = TestBuilder::new_singular("t1", "pkg").build();
    manifest.add_node(singular_test);

    // GenericTest
    let generic_test = TestBuilder::new_generic("t2", "pkg").build();
    manifest.add_node(generic_test);

    // Operation (Hook)
    let mut hook = OxideHookNode::default();
    hook.unique_id = "operation.pkg.h".to_string();
    manifest.add_node(OxideNode::Operation(hook));

    // SqlOperation
    let mut sql_op = OxideSqlOperation::default();
    sql_op.unique_id = "sql_operation.pkg.s".to_string();
    manifest.add_node(OxideNode::SqlOperation(sql_op));

    let json_val: serde_json::Value = serde_json::to_value(&manifest).unwrap();
    let nodes = json_val.get("nodes").unwrap().as_object().unwrap();

    assert!(nodes
        .values()
        .any(|n| n.get("resource_type").and_then(|t| t.as_str()) == Some("model")));
    assert!(nodes
        .values()
        .any(|n| n.get("resource_type").and_then(|t| t.as_str()) == Some("seed")));
    assert!(nodes
        .values()
        .any(|n| n.get("resource_type").and_then(|t| t.as_str()) == Some("snapshot")));
    assert!(nodes
        .values()
        .any(|n| n.get("resource_type").and_then(|t| t.as_str()) == Some("analysis")));
    assert!(nodes
        .values()
        .any(|n| n.get("resource_type").and_then(|t| t.as_str()) == Some("test")));
    // Operation might be hook or sql_operation
    assert!(nodes
        .values()
        .any(
            |n| n.get("resource_type").and_then(|t| t.as_str()) == Some("operation")
                || n.get("resource_type").and_then(|t| t.as_str()) == Some("sql_operation")
        ));
}
