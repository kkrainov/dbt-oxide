// Tests for parent/child map building (C.4.3)
// Option A: Simple iteration matching Python's implementation

use super::helpers::ModelBuilder;
use crate::manifest::*;

#[test]
fn test_build_parent_map_simple() {
    // Test simple parent map with one dependency
    // Given: model_a depends on model_b (model_a refs model_b)
    // When: build_parent_map()
    // Then: parent_map["model.pkg.model_a"] = ["model.pkg.model_b"]

    let mut manifest = OxideManifest::default();

    // Create model_b (no dependencies)
    let model_b = ModelBuilder::new("model_b", "pkg").build();
    manifest.add_node(OxideNode::Model(model_b));

    // Create model_a that depends on model_b
    let model_a = ModelBuilder::new("model_a", "pkg")
        .depends_on(vec!["model.pkg.model_b"])
        .build();
    manifest.add_node(OxideNode::Model(model_a));

    let parent_map = manifest.build_parent_map();

    assert!(parent_map.contains_key("model.pkg.model_a"));
    assert_eq!(
        parent_map.get("model.pkg.model_a").unwrap(),
        &vec!["model.pkg.model_b".to_string()]
    );

    // model_b has no dependencies
    assert!(parent_map.contains_key("model.pkg.model_b"));
    assert!(parent_map.get("model.pkg.model_b").unwrap().is_empty());
}

#[test]
fn test_build_child_map_simple() {
    // Test simple child map with one dependency
    // Given: model_a depends on model_b
    // When: build_child_map()
    // Then: child_map["model.pkg.model_b"] = ["model.pkg.model_a"]

    let mut manifest = OxideManifest::default();

    let model_b = ModelBuilder::new("model_b", "pkg").build();
    manifest.add_node(OxideNode::Model(model_b));

    let model_a = ModelBuilder::new("model_a", "pkg")
        .depends_on(vec!["model.pkg.model_b"])
        .build();
    manifest.add_node(OxideNode::Model(model_a));

    let child_map = manifest.build_child_map();

    // model_b is depended on by model_a
    assert!(child_map.contains_key("model.pkg.model_b"));
    assert_eq!(
        child_map.get("model.pkg.model_b").unwrap(),
        &vec!["model.pkg.model_a".to_string()]
    );

    // model_a has no children
    assert!(child_map.contains_key("model.pkg.model_a"));
    assert!(child_map.get("model.pkg.model_a").unwrap().is_empty());
}

#[test]
fn test_build_maps_multiple_deps() {
    // Test with multiple dependencies
    // Given: model_c depends on both model_a and model_b
    // When: build maps
    // Then: parent_map[model_c] = [model_a, model_b]
    //       child_map[model_a] = [model_c]
    //       child_map[model_b] = [model_c]

    let mut manifest = OxideManifest::default();

    let model_a = ModelBuilder::new("model_a", "pkg").build();
    let model_b = ModelBuilder::new("model_b", "pkg").build();
    manifest.add_node(OxideNode::Model(model_a));
    manifest.add_node(OxideNode::Model(model_b));

    let model_c = ModelBuilder::new("model_c", "pkg")
        .depends_on(vec!["model.pkg.model_a", "model.pkg.model_b"])
        .build();
    manifest.add_node(OxideNode::Model(model_c));

    let parent_map = manifest.build_parent_map();
    let child_map = manifest.build_child_map();

    // model_c has 2 parents
    let model_c_parents = parent_map.get("model.pkg.model_c").unwrap();
    assert_eq!(model_c_parents.len(), 2);
    assert!(model_c_parents.contains(&"model.pkg.model_a".to_string()));
    assert!(model_c_parents.contains(&"model.pkg.model_b".to_string()));

    // model_a and model_b each have model_c as child
    assert_eq!(
        child_map.get("model.pkg.model_a").unwrap(),
        &vec!["model.pkg.model_c".to_string()]
    );
    assert_eq!(
        child_map.get("model.pkg.model_b").unwrap(),
        &vec!["model.pkg.model_c".to_string()]
    );
}

#[test]
fn test_build_maps_transitive() {
    // Test transitive dependencies (A -> B -> C chain)
    // Parent map should only show DIRECT dependencies, not transitive

    let mut manifest = OxideManifest::default();

    let model_c = ModelBuilder::new("model_c", "pkg").build();
    manifest.add_node(OxideNode::Model(model_c));

    let model_b = ModelBuilder::new("model_b", "pkg")
        .depends_on(vec!["model.pkg.model_c"])
        .build();
    manifest.add_node(OxideNode::Model(model_b));

    let model_a = ModelBuilder::new("model_a", "pkg")
        .depends_on(vec!["model.pkg.model_b"])
        .build();
    manifest.add_node(OxideNode::Model(model_a));

    let parent_map = manifest.build_parent_map();

    // Each should only have direct dependencies
    assert_eq!(
        parent_map.get("model.pkg.model_a").unwrap(),
        &vec!["model.pkg.model_b".to_string()]
    );
    assert_eq!(
        parent_map.get("model.pkg.model_b").unwrap(),
        &vec!["model.pkg.model_c".to_string()]
    );
    assert!(parent_map.get("model.pkg.model_c").unwrap().is_empty());
}
