use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn default_true() -> bool {
    true
}

pub fn default_sql() -> String {
    "sql".to_string()
}

// Base Types (Phase C.1.1)

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct FileHash {
    pub name: String,
    pub checksum: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Doc {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub block_contents: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Docs {
    #[serde(default = "default_true")]
    pub show: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_color: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct BaseResource {
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct GraphResource {
    // BaseResource fields
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    // Additional GraphResource field
    pub fqn: Vec<String>,
}

// Component Types (Phase C.1.1)

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct RefArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ColumnConfig {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default)]
    pub constraints: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<bool>,
    #[serde(default)]
    pub config: ColumnConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct InjectedCTE {
    pub id: String,
    pub sql: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Contract {
    #[serde(default)]
    pub enforced: bool,
    #[serde(default = "default_true")]
    pub alias_types: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Quoting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<bool>,
}

// Additional Component Types (GREEN phase additions)

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Time {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct FreshnessThreshold {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_after: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_after: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MacroDependsOn {
    #[serde(default)]
    pub macros: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct HasRelationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct DeferRelation {
    pub alias: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Owner {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MacroArgument {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub arg_type: Option<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct OxideDependsOn {
    #[serde(default)]
    pub nodes: Vec<String>,
    #[serde(default)]
    pub macros: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Hook {
    pub sql: String,
    #[serde(default = "default_true")]
    pub transaction: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}
