use super::helpers::{CompiledResourceBuilder, ModelBuilder, ParsedResourceBuilder, SeedBuilder};
use crate::manifest::*;
use serde_json;

#[test]
fn test_serialize_model() {
    let mut model = ModelBuilder::new("my_model", "pkg")
        .with_schema("analytics")
        .build();
    model.raw_code = "SELECT 1".to_string();
    model.checksum = FileHash {
        name: "sha256".to_string(),
        checksum: "abc123".to_string(),
    };

    let json = serde_json::to_string(&model).unwrap();
    assert!(json.contains("\"name\":\"my_model\""));
}

#[test]
fn test_serialize_seed() {
    let mut seed = SeedBuilder::new("data", "pkg").build();
    seed.schema = "analytics".to_string();
    seed.checksum = FileHash {
        name: "sha256".to_string(),
        checksum: "def456".to_string(),
    };
    seed.root_path = Some("/project".to_string());

    let node = OxideNode::Seed(seed);
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("\"resource_type\":\"seed\""));
}

#[test]
fn test_parsed_resource_serialization() {
    let parsed = ParsedResourceBuilder::new("test", "pkg").build();
    let json = serde_json::to_string(&parsed).unwrap();
    assert!(json.contains("\"name\":\"test\""));
}

#[test]
fn test_compiled_resource_serialization() {
    let compiled = CompiledResourceBuilder::new("test", "pkg").build();
    let json = serde_json::to_string(&compiled).unwrap();
    assert!(json.contains("\"compiled\":false"));
}

#[test]
fn test_serialize_model_minimal() {
    let mut model = ModelBuilder::new("m", "pkg").build();
    // Override fields to match original test specificities if needed, but builder default is quite minimal/standard
    model.path = "m.sql".to_string();
    model.original_file_path = "m.sql".to_string();
    model.fqn = vec![]; // Builder sets FQN, test wants empty?
    model.checksum = FileHash {
        name: "sha256".to_string(),
        checksum: "x".to_string(),
    };

    let json = serde_json::to_string(&model).unwrap();
    assert!(json.contains("\"name\":\"m\""));
}
