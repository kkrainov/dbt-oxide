// manifest/config.rs - Configuration types
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{default_true, Docs, FreshnessThreshold, Hook};

// Config Types (Phase C.1.2)

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ContractConfig {
    #[serde(default)]
    pub enforced: bool,
    #[serde(default = "default_true")]
    pub alias_types: bool,
}

// Expanded OxideNodeConfig with 22+ fields from Python NodeConfig
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideNodeConfig {
    // NodeAndTestConfig base fields
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    // NodeConfig specific fields
    #[serde(default = "default_materialized")]
    pub materialized: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<serde_json::Value>,
    #[serde(default = "default_lookback")]
    pub lookback: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub persist_docs: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub post_hook: Vec<Hook>,
    #[serde(default)]
    pub pre_hook: Vec<Hook>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub quoting: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub column_types: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refresh: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_key: Option<serde_json::Value>,
    #[serde(default = "default_on_schema_change")]
    pub on_schema_change: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub grants: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub packages: Vec<String>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(default)]
    pub contract: ContractConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_batches: Option<serde_json::Value>,
    // Additional fields used by tests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_configuration_change: Option<String>,
}

fn default_materialized() -> String {
    "view".to_string()
}

fn default_lookback() -> serde_json::Value {
    serde_json::json!(1)
}

fn default_on_schema_change() -> String {
    "ignore".to_string()
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideTestConfig {
    // NodeAndTestConfig base fields
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default = "default_test_schema")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    // TestConfig specific
    #[serde(default = "default_materialized_test")]
    pub materialized: String,
    #[serde(default = "default_severity")]
    pub severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_failures: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_failures_as: Option<String>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub where_clause: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub where_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default = "default_fail_calc")]
    pub fail_calc: String,
    #[serde(default = "default_warn_if")]
    pub warn_if: String,
    #[serde(default = "default_error_if")]
    pub error_if: String,
}

fn default_test_schema() -> Option<String> {
    Some("dbt_test__audit".to_string())
}

fn default_materialized_test() -> String {
    "test".to_string()
}

fn default_severity() -> String {
    "ERROR".to_string()
}

fn default_fail_calc() -> String {
    "count(*)".to_string()
}

fn default_warn_if() -> String {
    "!= 0".to_string()
}

fn default_error_if() -> String {
    "!= 0".to_string()
}

// Additional Config Types (GREEN phase)

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct NodeAndTestConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SourceConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness: Option<FreshnessThreshold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_at_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_at_query: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ExposureConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MetricConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SemanticModelConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SavedQueryConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_as: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<SavedQueryCache>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SavedQueryCache {
    #[serde(default)]
    pub enabled: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct UnitTestConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SemanticLayerElementConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ExportConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_as: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "schema")]
    pub schema_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}
