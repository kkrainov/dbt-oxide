// manifest/nodes.rs - Node types (Model, Seed, Snapshot, Test, etc.)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::config::{OxideNodeConfig, OxideTestConfig};
use super::types::{
    default_sql, ColumnInfo, Contract, Docs, FileHash, InjectedCTE, OxideDependsOn, RefArgs,
};

// Phase C.1.3: Node Types (Full Schema based on validated Python dataclasses)

/// ParsedResource: Base for Seed nodes (30 fields total)
/// Extends: GraphResource + HasRelationMetadata + ParsedResourceMandatory
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParsedResource {
    // From BaseResource (6 fields)
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,

    // From GraphResource (1 field)
    pub fqn: Vec<String>,

    // From HasRelationMetadata (2 fields)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,

    // From ParsedResourceMandatory (3 fields)
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,

    // ParsedResource specific (17 fields)
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default = "default_created_at")]
    pub created_at: f64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
}

fn default_created_at() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// CompiledResource: Base for Model, Snapshot, Test, Analysis (40 fields total)
/// Extends ParsedResource + adds compilation fields
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompiledResource {
    // All ParsedResource fields (30 fields)
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default = "default_created_at")]
    pub created_at: f64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,

    // CompiledResource specific (10 fields + 1 private = 11)
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<RefArgs>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<Vec<String>>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
}

fn default_language() -> String {
    "sql".to_string()
}

/// OxideModel: Full Model node (47 fields total, 44 serialized)
/// Extends CompiledResource + model-specific fields
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideModel {
    // All CompiledResource fields (40 fields - includes ParsedResource 30)
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default = "default_created_at")]
    pub created_at: f64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<RefArgs>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<Vec<String>>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,

    // Model-specific fields (7 fields, defer_relation excluded in artifact context)
    #[serde(default = "default_access")]
    pub access: String,
    #[serde(default)]
    pub constraints: Vec<serde_json::Value>, // ModelLevelConstraint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<serde_json::Value>, // NodeVersion = Union[str, float]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<String>, // datetime serialized as string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_relation: Option<serde_json::Value>, // DeferRelation
    #[serde(default)]
    pub primary_key: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spine: Option<serde_json::Value>, // TimeSpine
}

fn default_access() -> String {
    "protected".to_string()
}

/// OxideSeed: Seed node (30 fields total)
/// Extends ParsedResource (NOT CompiledResource!)
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideSeed {
    // All ParsedResource fields (30 fields)
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default = "default_created_at")]
    pub created_at: f64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,

    // Seed-specific: Seed extends ParsedResource so it has MacroDependsOn not full DependsOn
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_relation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path: Option<String>,
}

/// OxideSnapshot: Snapshot node (extends CompiledResource)
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideSnapshot {
    // From CompiledResource (40 fields)
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_sql")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_relation: Option<serde_json::Value>,
}

/// OxideGenericTest: Generic test node
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideGenericTest {
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideTestConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_sql")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
    // GenericTest specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_metadata: Option<serde_json::Value>,
}

/// OxideSingularTest: Singular test node
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideSingularTest {
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideTestConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_sql")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
}

/// OxideAnalysis: Analysis node
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideAnalysis {
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_sql")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
}

/// OxideHookNode: Hook node (run operations)
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideHookNode {
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_sql")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
    // Hook specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

/// OxideSqlOperation: SQL operation node
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideSqlOperation {
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub alias: String,
    pub checksum: FileHash,
    #[serde(default)]
    pub config: OxideNodeConfig,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub unrendered_config_call_dict: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default = "default_sql")]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: bool,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    #[serde(default)]
    pub contract: Contract,
}
