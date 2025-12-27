// manifest/semantic.rs - Semantic layer types (Metric, SemanticModel, SavedQuery)
use serde::{Deserialize, Serialize};

use super::config::{
    ExportConfig, MetricConfig, SavedQueryConfig, SemanticLayerElementConfig, SemanticModelConfig,
};
use super::types::OxideDependsOn;

// Metric Types (GREEN phase)

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MetricTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerator: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denominator: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain_to_date: Option<String>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_type_params: Option<ConversionTypeParams>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MetricInputMeasure {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    pub join_to_timespine: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_nulls_with: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MetricTimeWindow {
    pub count: i32,
    pub granularity: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct MetricInput {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_window: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_to_grain: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ConversionTypeParams {
    pub base_measure: serde_json::Value,
    pub conversion_measure: serde_json::Value,
    pub entity: String,
    pub calculation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<serde_json::Value>,
    #[serde(default)]
    pub constant_properties: Vec<ConstantPropertyInput>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct CumulativeTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain_to_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_agg: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ConstantPropertyInput {
    pub base_property: String,
    pub conversion_property: String,
}

// Full Metric struct
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct OxideMetricFull {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    #[serde(default)]
    pub description: String,
    pub label: String,
    #[serde(rename = "type")]
    pub metric_type: String,
    #[serde(default)]
    pub type_params: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(default)]
    pub meta: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: MetricConfig,
    #[serde(default)]
    pub unrendered_config: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

// Semantic Model Types

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct NodeRelation {
    pub alias: String,
    pub schema_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub relation_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Dimension {
    pub name: String,
    #[serde(rename = "type")]
    pub dimension_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub is_partition: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_params: Option<DimensionTypeParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SemanticLayerElementConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct DimensionTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_params: Option<DimensionValidityParams>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct DimensionValidityParams {
    pub is_start: bool,
    pub is_end: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Entity {
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SemanticLayerElementConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Measure {
    pub name: String,
    pub agg: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub create_metric: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_additive_dimension: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_time_dimension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SemanticLayerElementConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_time_dimension: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SemanticModel {
    pub unique_id: String,
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_relation: Option<NodeRelation>,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(default)]
    pub entities: Vec<Entity>,
    #[serde(default)]
    pub measures: Vec<Measure>,
    #[serde(default)]
    pub dimensions: Vec<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_entity: Option<String>,
    #[serde(default)]
    pub meta: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: SemanticModelConfig,
    #[serde(default)]
    pub unrendered_config: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

// SavedQuery Types

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct QueryParams {
    #[serde(default)]
    pub metrics: Vec<String>,
    #[serde(default)]
    pub group_by: Vec<String>,
    #[serde(default, rename = "where")]
    pub where_: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Export {
    pub name: String,
    #[serde(default)]
    pub config: ExportConfig,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SavedQuery {
    pub unique_id: String,
    pub name: String,

    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    #[serde(default)]
    pub description: String,
    pub label: String,
    #[serde(default)]
    pub query_params: QueryParams,
    #[serde(default)]
    pub exports: Vec<Export>,
    #[serde(default)]
    pub meta: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: SavedQueryConfig,
    #[serde(default)]
    pub unrendered_config: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}
