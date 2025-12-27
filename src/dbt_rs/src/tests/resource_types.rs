// Tests for all resource types (seeds, snapshots, tests, exposures, metrics, semantic models)

use super::helpers::{
    AnalysisBuilder, ExposureBuilder, ManifestBuilder, MetricBuilder, ModelBuilder, SeedBuilder,
    SnapshotBuilder, SourceBuilder, TestBuilder,
};
use crate::manifest::*;

#[test]
fn test_manifest_with_seeds() {
    let mut manifest = ManifestBuilder::new().build();

    // Create and add a seed
    let seed = SeedBuilder::new("customers", "pkg").build();
    manifest.add_node(OxideNode::Seed(seed));

    // Verify seed was added
    assert_eq!(manifest.node_count(), 1);

    // Verify we can find it in nodes
    let seed_node = manifest.get_node("seed.pkg.customers");
    assert!(seed_node.is_some());
    assert_eq!(seed_node.unwrap().resource_type(), "seed");
}

#[test]
fn test_manifest_with_snapshots() {
    let mut manifest = ManifestBuilder::new().build();

    // Create and add a snapshot
    let snapshot = SnapshotBuilder::new("orders_snapshot", "pkg").build();
    manifest.add_node(OxideNode::Snapshot(snapshot));

    // Verify snapshot was added
    assert_eq!(manifest.node_count(), 1);

    // Verify we can find it
    let snapshot_node = manifest.get_node("snapshot.pkg.orders_snapshot");
    assert!(snapshot_node.is_some());
    assert_eq!(snapshot_node.unwrap().resource_type(), "snapshot");
}

#[test]
fn test_manifest_with_generic_tests() {
    let mut manifest = ManifestBuilder::new().build();

    // Create and add a generic test
    let test = TestBuilder::new_generic("not_null_customers_id", "pkg").build();
    manifest.add_node(test);

    // Verify test was added
    assert_eq!(manifest.node_count(), 1);

    // Verify we can find it
    let test_node = manifest.get_node("test.pkg.not_null_customers_id");
    assert!(test_node.is_some());
    assert_eq!(test_node.unwrap().resource_type(), "test");
}

#[test]
fn test_manifest_with_singular_tests() {
    let mut manifest = ManifestBuilder::new().build();

    // Create and add a singular test
    let test = TestBuilder::new_singular("assert_positive_revenue", "pkg").build();
    manifest.add_node(test);

    // Verify test was added
    assert_eq!(manifest.node_count(), 1);

    // Verify we can find it
    let test_node = manifest.get_node("test.pkg.assert_positive_revenue");
    assert!(test_node.is_some());
    assert_eq!(test_node.unwrap().resource_type(), "test");
}

#[test]
fn test_manifest_with_exposures() {
    let mut manifest = ManifestBuilder::new().build();

    // Create and add an exposure
    let exposure = ExposureBuilder::new("weekly_dashboard", "pkg")
        .with_owner("data@company.com", Some("Data Team"))
        .build();
    manifest
        .exposures
        .insert(exposure.unique_id.clone(), exposure);

    // Verify exposure was added
    assert_eq!(manifest.exposures.len(), 1);

    // Verify we can find it
    let exp = manifest.exposures.get("exposure.pkg.weekly_dashboard");
    assert!(exp.is_some());
}

#[test]
fn test_manifest_with_metrics() {
    let mut manifest = ManifestBuilder::new().build();

    // Create and add a metric
    let metric = MetricBuilder::new("total_revenue", "pkg").build();
    manifest.add_metric(metric);

    // Verify metric was added
    assert_eq!(manifest.metrics.len(), 1);

    // Verify we can find it
    let m = manifest.metrics.get("metric.pkg.total_revenue");
    assert!(m.is_some());
}

#[test]
fn test_manifest_with_semantic_models() {
    // Semantic models are already part of OxideManifest
    let manifest = ManifestBuilder::new().build();

    // Semantic models exist in the manifest structure
    assert_eq!(manifest.semantic_models.len(), 0);

    // This test validates the field exists and is accessible
    // Full semantic model tests would require a builder (future work)
}

#[test]
fn test_build_parent_map_mixed_types() {
    let mut manifest = ManifestBuilder::new().build();

    // Create a seed (no dependencies)
    let seed = SeedBuilder::new("raw_customers", "pkg").build();
    manifest.add_node(OxideNode::Seed(seed));

    // Create model depending on seed
    let model = ModelBuilder::new("stg_customers", "pkg")
        .depends_on(vec!["seed.pkg.raw_customers"])
        .build();
    manifest.add_node(OxideNode::Model(model));

    // Create exposure depending on model
    let mut exposure = ExposureBuilder::new("customer_dashboard", "pkg")
        .with_owner("analytics@company.com", None)
        .build();
    exposure.depends_on.nodes = vec!["model.pkg.stg_customers".to_string()];
    manifest
        .exposures
        .insert(exposure.unique_id.clone(), exposure);

    // Build parent map
    let parent_map = manifest.build_parent_map();

    // Verify relationships
    // Seed has no parents
    let seed_parents = parent_map.get("seed.pkg.raw_customers").unwrap();
    assert_eq!(seed_parents.len(), 0);

    // Model depends on seed
    let model_parents = parent_map.get("model.pkg.stg_customers").unwrap();
    assert_eq!(model_parents.len(), 1);
    assert!(model_parents.contains(&"seed.pkg.raw_customers".to_string()));

    // Exposure depends on model
    let exposure_parents = parent_map.get("exposure.pkg.customer_dashboard").unwrap();
    assert_eq!(exposure_parents.len(), 1);
    assert!(exposure_parents.contains(&"model.pkg.stg_customers".to_string()));
}

#[test]
fn test_build_child_map_mixed_types() {
    let mut manifest = ManifestBuilder::new().build();

    // Create a source
    let source = SourceBuilder::new("raw", "orders", "pkg").build();
    manifest.add_source(source);

    // Create seed (no dependencies)
    let seed = SeedBuilder::new("raw_customers", "pkg").build();
    manifest.add_node(OxideNode::Seed(seed));

    // Create model depending on both source and seed
    let model = ModelBuilder::new("stg_customers", "pkg")
        .depends_on(vec!["source.pkg.raw.orders", "seed.pkg.raw_customers"])
        .build();
    manifest.add_node(OxideNode::Model(model));

    // Create exposure depending on model
    let mut exposure = ExposureBuilder::new("customer_dashboard", "pkg")
        .with_owner("analytics@company.com", None)
        .build();
    exposure.depends_on.nodes = vec!["model.pkg.stg_customers".to_string()];
    manifest
        .exposures
        .insert(exposure.unique_id.clone(), exposure);

    // Build child map
    let child_map = manifest.build_child_map();

    // Verify relationships
    // Source has model as child
    let source_children = child_map.get("source.pkg.raw.orders").unwrap();
    assert_eq!(source_children.len(), 1);
    assert!(source_children.contains(&"model.pkg.stg_customers".to_string()));

    // Seed has model as child
    let seed_children = child_map.get("seed.pkg.raw_customers").unwrap();
    assert_eq!(seed_children.len(), 1);
    assert!(seed_children.contains(&"model.pkg.stg_customers".to_string()));

    // Model has exposure as child
    let model_children = child_map.get("model.pkg.stg_customers").unwrap();
    assert_eq!(model_children.len(), 1);
    assert!(model_children.contains(&"exposure.pkg.customer_dashboard".to_string()));

    // Exposure has no children
    let exposure_children = child_map.get("exposure.pkg.customer_dashboard").unwrap();
    assert_eq!(exposure_children.len(), 0);
}
