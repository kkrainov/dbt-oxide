// Tests for resolve_ref, resolve_source, and resolve_doc
// Following TDD: tests now expect node/source/doc references

use crate::manifest::{OxideManifest, OxideNode, OxideModel, OxideSource, ManifestMetadata};
use std::collections::HashMap;

#[cfg(test)]
mod test_resolve_ref {
    use super::*;

    #[test]
    fn test_resolve_ref_not_found() {
        let manifest = OxideManifest::default();
        
        let result = manifest.resolve_ref(
            None, "my_model", None, None, "root", "root",
        );
        
        assert!(result.is_none(), "Expected None for non-existent model");
    }

    #[test]
    fn test_resolve_ref_by_name() {
        let mut nodes = HashMap::new();
        nodes.insert(
            "model.root.my_model".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "root".to_string(),
                unique_id: "model.root.my_model".to_string(),
                ..Default::default()
            }),
        );

        let manifest = OxideManifest {
            nodes,
            ..Default::default()
        };

        let result = manifest.resolve_ref(None, "my_model", None, None, "root", "root");

        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(node.name(), "my_model");
        assert_eq!(node.package_name(), "root");
    }

    #[test]
    fn test_resolve_ref_by_name_and_package() {
        let mut nodes = HashMap::new();
        
        nodes.insert(
            "model.root.my_model".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "root".to_string(),
                unique_id: "model.root.my_model".to_string(),
                ..Default::default()
            }),
        );
        
        nodes.insert(
            "model.dep.my_model".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "dep".to_string(),
                unique_id: "model.dep.my_model".to_string(),
                ..Default::default()
            }),
        );

        let manifest = OxideManifest { nodes, ..Default::default() };

        let result = manifest.resolve_ref(None, "my_model", Some("dep"), None, "root", "root");

        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(node.package_name(), "dep");
       assert_eq!(node.name(), "my_model");
    }

    #[test]
    fn test_resolve_ref_package_priority() {
        let mut nodes = HashMap::new();
        
        nodes.insert(
            "model.root.my_model".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "root".to_string(),
                unique_id: "model.root.my_model".to_string(),
                ..Default::default()
            }),
        );
        
        nodes.insert(
            "model.dep.my_model".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "dep".to_string(),
                unique_id: "model.dep.my_model".to_string(),
                ..Default::default()
            }),
        );

        let manifest = OxideManifest { nodes, ..Default::default() };

        let result = manifest.resolve_ref(None, "my_model", None, None, "root", "root");

        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(node.package_name(), "root", "Should prefer current project");
    }

    #[test]
    fn test_resolve_ref_with_version() {
        let mut nodes = HashMap::new();
        
        nodes.insert(
            "model.root.my_model.v1".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "root".to_string(),
                unique_id: "model.root.my_model.v1".to_string(),
                version: Some(serde_json::Value::String("1".to_string())),
                ..Default::default()
            }),
        );
        
        nodes.insert(
            "model.root.my_model.v2".to_string(),
            OxideNode::Model(OxideModel {
                name: "my_model".to_string(),
                package_name: "root".to_string(),
                unique_id: "model.root.my_model.v2".to_string(),
                version: Some(serde_json::Value::String("2".to_string())),
                ..Default::default()
            }),
        );

        let manifest = OxideManifest { nodes, ..Default::default() };

        let result = manifest.resolve_ref(None, "my_model", None, Some(2), "root", "root");

        assert!(result.is_some());
        let node = result.unwrap();
        assert_eq!(node.version(), Some("2".to_string()));
    }
}

#[cfg(test)]
mod test_resolve_source {
    use super::*;

    #[test]
    fn test_resolve_source_not_found() {
        let manifest = OxideManifest::default();
        
        let result = manifest.resolve_source("my_source", "my_table", "root", "root");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_resolve_source_simple() {
        let mut sources = HashMap::new();
        sources.insert(
            "source.root.my_source.my_table".to_string(),
            OxideSource {
                source_name: "my_source".to_string(),
                name: "my_table".to_string(),
                package_name: "root".to_string(),
                unique_id: "source.root.my_source.my_table".to_string(),
                ..Default::default()
            },
        );

        let manifest = OxideManifest { sources, ..Default::default() };

        let result = manifest.resolve_source("my_source", "my_table", "root", "root");

        assert!(result.is_some());
        let source = result.unwrap();
        assert_eq!(source.source_name, "my_source");
        assert_eq!(source.name, "my_table");
    }
}

#[cfg(test)]
mod test_resolve_doc {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_resolve_doc_not_found() {
        let manifest = OxideManifest::default();
        
        let result = manifest.resolve_doc("my_doc", None, "root", "root");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_resolve_doc_by_name() {
        let mut docs = HashMap::new();
        let mut doc_value = serde_json::Map::new();
        doc_value.insert("name".to_string(), Value::String("my_doc".to_string()));
        doc_value.insert("package_name".to_string(), Value::String("root".to_string()));
        doc_value.insert("unique_id".to_string(), Value::String("doc.root.my_doc".to_string()));
        
        docs.insert("doc.root.my_doc".to_string(), Value::Object(doc_value));

        let manifest = OxideManifest { docs, ..Default::default() };

        let result = manifest.resolve_doc("my_doc", None, "root", "root");

        assert!(result.is_some());
        let doc = result.unwrap();
        assert_eq!(doc.get("name").and_then(|v| v.as_str()), Some("my_doc"));
    }
}
