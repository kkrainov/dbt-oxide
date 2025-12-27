use super::helpers::{BaseResourceBuilder, DocsBuilder, FileHashBuilder, GraphResourceBuilder};
use crate::manifest::*;
use serde_json;

#[test]
fn test_serialize_file_hash() {
    let file_hash = FileHashBuilder::new("abcd1234").build();
    let json = serde_json::to_string(&file_hash).unwrap();
    let deserialized: FileHash = serde_json::from_str(&json).unwrap();
    assert_eq!(file_hash, deserialized);
}

#[test]
fn test_serialize_docs() {
    let docs = DocsBuilder::new().with_color("#FF5733").build();
    let json = serde_json::to_string(&docs).unwrap();
    assert!(json.contains("\"show\":true"));
}

#[test]
fn test_serialize_base_resource() {
    let resource = BaseResourceBuilder::new("my_model", "my_pkg").build();
    let json = serde_json::to_string(&resource).unwrap();
    assert!(json.contains("\"name\":\"my_model\""));
}

#[test]
fn test_serialize_graph_resource() {
    let resource = GraphResourceBuilder::new("my_model", "my_pkg").build();
    let json = serde_json::to_string(&resource).unwrap();
    assert!(json.contains("\"fqn\""));
}
