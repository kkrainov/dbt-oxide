use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideDependsOn {
    #[serde(default)]
    pub nodes: Vec<String>,
    #[serde(default)]
    pub macros: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideNodeConfig {
    #[serde(default)]
    pub materialized: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideNode {
    pub unique_id: String,
    pub name: String,
    pub resource_type: String,
    pub package_name: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(default)]
    pub raw_code: Option<String>,
    #[serde(default)]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub config: OxideNodeConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideSource {
    pub unique_id: String,
    pub source_name: String,
    pub name: String,
    #[serde(default)]
    pub database: Option<String>,
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub package_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideMacro {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    #[serde(default)]
    pub macro_sql: Option<String>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideExposure {
    pub unique_id: String,
    pub name: String,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideMetric {
    pub unique_id: String,
    pub name: String,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideGroup {
    pub unique_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideSemanticModel {
    pub unique_id: String,
    pub name: String,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideSavedQuery {
    pub unique_id: String,
    pub name: String,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideUnitTest {
    pub unique_id: String,
    pub name: String,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideManifestMetadata {
    #[serde(default)]
    pub dbt_version: String,
    #[serde(default)]
    pub adapter_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideManifest {
    #[serde(default)]
    pub nodes: HashMap<String, OxideNode>,
    #[serde(default)]
    pub sources: HashMap<String, OxideSource>,
    #[serde(default)]
    pub macros: HashMap<String, OxideMacro>,
    #[serde(default)]
    pub exposures: HashMap<String, OxideExposure>,
    #[serde(default)]
    pub metrics: HashMap<String, OxideMetric>,
    #[serde(default)]
    pub groups: HashMap<String, OxideGroup>,
    #[serde(default)]
    pub semantic_models: HashMap<String, OxideSemanticModel>,
    #[serde(default)]
    pub saved_queries: HashMap<String, OxideSavedQuery>,
    #[serde(default)]
    pub unit_tests: HashMap<String, OxideUnitTest>,
    #[serde(default)]
    pub metadata: Option<OxideManifestMetadata>,
}

impl OxideManifest {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    pub fn get_node(&self, unique_id: &str) -> Option<&OxideNode> {
        self.nodes.get(unique_id)
    }
    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_empty_manifest() {
        let json = r#"{"nodes": {}, "sources": {}, "macros": {}}"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        assert_eq!(manifest.node_count(), 0);
    }
    
    #[test]
    fn test_parse_single_node() {
        let json = r#"{
            "nodes": {
                "model.my_project.my_model": {
                    "unique_id": "model.my_project.my_model",
                    "name": "my_model",
                    "resource_type": "model",
                    "package_name": "my_project",
                    "depends_on": {"nodes": ["model.my_project.upstream"], "macros": []}
                }
            },
            "sources": {},
            "macros": {},
            "exposures": {}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        assert_eq!(manifest.node_count(), 1);
        let node = manifest.get_node("model.my_project.my_model").unwrap();
        assert_eq!(node.depends_on.nodes, vec!["model.my_project.upstream"]);
    }
    
    #[test]
    fn test_parse_with_missing_optional_fields() {
        let json = r#"{
            "nodes": {
                "model.test.a": {
                    "unique_id": "model.test.a",
                    "name": "a",
                    "resource_type": "model",
                    "package_name": "test"
                }
            },
            "sources": {},
            "macros": {},
            "exposures": {}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let node = manifest.get_node("model.test.a").unwrap();
        assert!(node.raw_code.is_none());
        assert!(node.depends_on.nodes.is_empty());
    }
    
    #[test]
    fn test_parse_all_node_types() {
        let json = r#"{
            "nodes": {
                "model.test.m": {
                    "unique_id": "model.test.m",
                    "name": "m",
                    "resource_type": "model",
                    "package_name": "test"
                }
            },
            "sources": {
                "source.test.src.tbl": {
                    "unique_id": "source.test.src.tbl",
                    "source_name": "src",
                    "name": "tbl",
                    "package_name": "test"
                }
            },
            "macros": {
                "macro.test.my_macro": {
                    "unique_id": "macro.test.my_macro",
                    "name": "my_macro",
                    "package_name": "test"
                }
            },
            "exposures": {
                "exposure.test.exp": {
                    "unique_id": "exposure.test.exp",
                    "name": "exp"
                }
            },
            "metrics": {
                "metric.test.met": {
                    "unique_id": "metric.test.met",
                    "name": "met"
                }
            }
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        
        assert_eq!(manifest.nodes.len(), 1);
        assert_eq!(manifest.sources.len(), 1);
        assert_eq!(manifest.macros.len(), 1);
        assert_eq!(manifest.exposures.len(), 1);
        assert_eq!(manifest.metrics.len(), 1);
    }
    
    #[test]
    fn test_invalid_json_returns_error() {
        let invalid_json = r#"{"nodes": invalid}"#;
        let result = OxideManifest::from_json_str(invalid_json);
        assert!(result.is_err());
    }
}
