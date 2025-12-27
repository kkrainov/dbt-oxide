// manifest/testing.rs - Unit test and documentation types
use serde::{Deserialize, Serialize};

use super::config::UnitTestConfig;
use super::types::{FileHash, Owner, OxideDependsOn};

// Unit Test Types

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnitTestInputFixture {
    pub input: String,
    #[serde(default)]
    pub rows: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnitTestOutputFixture {
    #[serde(default)]
    pub rows: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnitTestOverrides {
    #[serde(default)]
    pub macros: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub vars: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub env_vars: std::collections::HashMap<String, String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnitTestNodeVersions {
    #[serde(default)]
    pub include: Vec<serde_json::Value>,
    #[serde(default)]
    pub exclude: Vec<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnitTestDefinition {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    pub model: String,
    #[serde(default)]
    pub given: Vec<UnitTestInputFixture>,
    #[serde(default)]
    pub expect: UnitTestOutputFixture,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<UnitTestOverrides>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<UnitTestNodeVersions>,
    #[serde(default)]
    pub config: UnitTestConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<FileHash>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(default)]
    pub schema: String,
}

// Documentation Type

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Documentation {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub block_contents: String,
}

// Group Type

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct OxideGroupFull {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub owner: Owner,
}

// Manifest Metadata

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ManifestMetadata {
    #[serde(default)]
    pub dbt_schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_anonymous_usage_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_type: Option<String>,
}
