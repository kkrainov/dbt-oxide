// manifest/resources.rs - Resource types (Source, Macro, Exposure, Metric, etc.)
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::config::{ExposureConfig, MetricConfig, SourceConfig};
use super::nodes::{
    OxideAnalysis, OxideGenericTest, OxideHookNode, OxideModel, OxideSeed, OxideSingularTest,
    OxideSnapshot, OxideSqlOperation,
};
use super::semantic::{SavedQuery as OxideSavedQuery, SemanticModel as OxideSemanticModel};
use super::testing::ManifestMetadata;
use super::testing::UnitTestDefinition as OxideUnitTest;
use super::types::{
    ColumnInfo, Docs, FreshnessThreshold, MacroArgument, MacroDependsOn, Owner, OxideDependsOn,
    Quoting,
};

fn default_macro_resource_type() -> String {
    "macro".to_string()
}

// Existing Component Types (already implemented)

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OxideTestNode {
    Generic(OxideGenericTest),
    Singular(OxideSingularTest),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "resource_type")]
pub enum OxideNode {
    #[serde(rename = "model")]
    Model(OxideModel),
    #[serde(rename = "seed")]
    Seed(OxideSeed),
    #[serde(rename = "snapshot")]
    Snapshot(OxideSnapshot),
    #[serde(rename = "test")]
    Test(OxideTestNode),
    #[serde(rename = "analysis")]
    Analysis(OxideAnalysis),
    #[serde(rename = "operation")]
    Operation(OxideHookNode),
    #[serde(rename = "sql_operation")]
    SqlOperation(OxideSqlOperation),
}

impl OxideNode {
    pub fn resource_type(&self) -> &str {
        match self {
            OxideNode::Model(_) => "model",
            OxideNode::Seed(_) => "seed",
            OxideNode::Snapshot(_) => "snapshot",
            OxideNode::Test(_) => "test",
            OxideNode::Analysis(_) => "analysis",
            OxideNode::Operation(_) => "operation",
            OxideNode::SqlOperation(_) => "sql_operation",
        }
    }

    pub fn is_external_node(&self) -> bool {
        match self {
            OxideNode::Model(n) => n.original_file_path.is_empty() && n.path.is_empty(),
            // Default to false, mirroring Python behavior. Only Model overrides this.
            _ => false,
        }
    }

    pub fn depends_on(&self) -> &OxideDependsOn {
        match self {
            OxideNode::Model(n) => &n.depends_on,
            OxideNode::Seed(n) => &n.depends_on,
            OxideNode::Snapshot(n) => &n.depends_on,
            OxideNode::Test(node) => match node {
                OxideTestNode::Generic(n) => &n.depends_on,
                OxideTestNode::Singular(n) => &n.depends_on,
            },
            OxideNode::Analysis(n) => &n.depends_on,
            OxideNode::Operation(n) => &n.depends_on,
            OxideNode::SqlOperation(n) => &n.depends_on,
        }
    }

    pub fn package_name(&self) -> &str {
        match self {
            OxideNode::Model(n) => &n.package_name,
            OxideNode::Seed(n) => &n.package_name,
            OxideNode::Snapshot(n) => &n.package_name,
            OxideNode::Test(node) => match node {
                OxideTestNode::Generic(n) => &n.package_name,
                OxideTestNode::Singular(n) => &n.package_name,
            },
            OxideNode::Analysis(n) => &n.package_name,
            OxideNode::Operation(n) => &n.package_name,
            OxideNode::SqlOperation(n) => &n.package_name,
        }
    }

    pub fn group_name(&self) -> Option<&str> {
        match self {
            OxideNode::Model(n) => n.config.group.as_deref(),
            OxideNode::Seed(n) => n.config.group.as_deref(),
            OxideNode::Snapshot(n) => n.config.group.as_deref(),
            OxideNode::Test(node) => match node {
                OxideTestNode::Generic(n) => n.config.group.as_deref(),
                OxideTestNode::Singular(n) => n.config.group.as_deref(),
            },
            OxideNode::Analysis(n) => n.config.group.as_deref(),
            OxideNode::Operation(_) => None, // Hooks do not support groups
            OxideNode::SqlOperation(_) => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            OxideNode::Model(n) => &n.name,
            OxideNode::Seed(n) => &n.name,
            OxideNode::Snapshot(n) => &n.name,
            OxideNode::Test(node) => match node {
                OxideTestNode::Generic(n) => &n.name,
                OxideTestNode::Singular(n) => &n.name,
            },
            OxideNode::Analysis(n) => &n.name,
            OxideNode::Operation(n) => &n.name,
            OxideNode::SqlOperation(n) => &n.name,
        }
    }

    pub fn version(&self) -> Option<String> {
        match self {
            OxideNode::Model(n) => n.version.as_ref().and_then(|v| {
                match v {
                    serde_json::Value::String(s) => Some(s.clone()),
                    serde_json::Value::Number(num) => Some(num.to_string()),
                    _ => None,
                }
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
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

// Full Source Types for proper tests

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ExternalTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbl_properties: Option<String>,
    #[serde(default)]
    pub partitions: Vec<ExternalPartition>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ExternalPartition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct SourceDefinition {
    pub unique_id: String,
    pub name: String,
    pub source_name: String,
    #[serde(default)]
    pub source_description: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub schema: String,
    pub identifier: String,
    #[serde(default)]
    pub loader: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: HashMap<String, ColumnInfo>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: SourceConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(default)]
    pub quoting: Quoting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_at_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness: Option<FreshnessThreshold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
}

// Full Macro struct
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct OxideMacroFull {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub macro_sql: String,
    #[serde(default)]
    pub depends_on: MacroDependsOn,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(default)]
    pub arguments: Vec<MacroArgument>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<String>>,
}

#[cfg_attr(feature = "extension-module", pyo3::pyclass)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideMacro {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    #[serde(default = "default_macro_resource_type")]
    pub resource_type: String,
    pub path: String,
    pub original_file_path: String,
    pub macro_sql: String,
    #[serde(default)]
    pub depends_on: MacroDependsOn,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub docs: Docs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_path: Option<String>,
    #[serde(default)]
    pub arguments: Vec<MacroArgument>,
    #[serde(default)]
    pub created_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideExposure {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    #[serde(rename = "type")]
    pub exposure_type: String,
    pub owner: Owner,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity: Option<String>,
    #[serde(default)]
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: ExposureConfig,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    pub depends_on: OxideDependsOn,
    #[serde(default)]
    pub refs: Vec<serde_json::Value>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<serde_json::Value>,
    #[serde(default)]
    pub created_at: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideMetric {
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
    pub meta: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: MetricConfig,
    #[serde(default)]
    pub unrendered_config: HashMap<String, serde_json::Value>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideGroup {
    pub unique_id: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub owner: Owner,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OxideManifestMetadata {
    #[serde(default)]
    pub dbt_version: String,
    #[serde(default)]
    pub adapter_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OxideManifest {
    #[serde(default)]
    pub metadata: ManifestMetadata,
    #[serde(default)]
    pub nodes: HashMap<String, OxideNode>,
    #[serde(default)]
    pub sources: HashMap<String, OxideSource>,
    #[serde(default)]
    pub macros: HashMap<String, OxideMacro>,
    #[serde(default)]
    pub docs: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub exposures: HashMap<String, OxideExposure>,
    #[serde(default)]
    pub metrics: HashMap<String, OxideMetric>,
    #[serde(default)]
    pub groups: HashMap<String, OxideGroup>,
    #[serde(default)]
    pub selectors: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub disabled: HashMap<String, Vec<serde_json::Value>>,
    #[serde(default)]
    pub semantic_models: HashMap<String, OxideSemanticModel>,
    #[serde(default)]
    pub unit_tests: HashMap<String, OxideUnitTest>,
    #[serde(default)]
    pub saved_queries: HashMap<String, OxideSavedQuery>,
}

#[allow(dead_code)]
impl OxideManifest {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let json = self.to_json_str()?;
        std::fs::write(path, json)
    }

    pub fn get_node(&self, unique_id: &str) -> Option<&OxideNode> {
        self.nodes.get(unique_id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Create a new manifest with the given metadata
    pub fn with_metadata(metadata: ManifestMetadata) -> Self {
        Self {
            metadata,
            ..Default::default()
        }
    }

    /// Add a node to the manifest
    pub fn add_node(&mut self, node: OxideNode) {
        let unique_id = match &node {
            OxideNode::Model(n) => n.unique_id.clone(),
            OxideNode::Seed(n) => n.unique_id.clone(),
            OxideNode::Snapshot(n) => n.unique_id.clone(),
            OxideNode::Test(n) => match n {
                OxideTestNode::Generic(t) => t.unique_id.clone(),
                OxideTestNode::Singular(t) => t.unique_id.clone(),
            },
            OxideNode::Analysis(n) => n.unique_id.clone(),
            OxideNode::Operation(n) => n.unique_id.clone(),
            OxideNode::SqlOperation(n) => n.unique_id.clone(),
        };
        self.nodes.insert(unique_id, node);
    }

    /// Add a source to the manifest
    pub fn add_source(&mut self, source: SourceDefinition) {
        let oxide_source = OxideSource {
            unique_id: source.unique_id.clone(),
            source_name: source.source_name.clone(),
            name: source.name.clone(),
            database: source.database.clone(),
            schema: Some(source.schema.clone()),
            package_name: source.package_name.clone(),
        };
        self.sources.insert(source.unique_id.clone(), oxide_source);
    }

    /// Add a macro to the manifest
    pub fn add_macro(&mut self, macro_node: OxideMacro) {
        self.macros.insert(macro_node.unique_id.clone(), macro_node);
    }

    /// Get a macro by unique_id
    pub fn get_macro(&self, unique_id: &str) -> Option<&OxideMacro> {
        self.macros.get(unique_id)
    }

    /// Get the adapter type from metadata
    pub fn get_adapter_type(&self) -> Option<&str> {
        self.metadata.adapter_type.as_deref()
    }

    /// Build parent map: maps each node to its direct dependencies.
    pub fn build_parent_map(&self) -> HashMap<String, Vec<String>> {
        let mut parent_map = HashMap::new();

        for (unique_id, node) in &self.nodes {
            parent_map.insert(unique_id.clone(), node.depends_on().nodes.clone());
        }

        for unique_id in self.sources.keys() {
            parent_map.insert(unique_id.clone(), Vec::new());
        }

        for (unique_id, exposure) in &self.exposures {
            parent_map.insert(unique_id.clone(), exposure.depends_on.nodes.clone());
        }

        for (unique_id, metric) in &self.metrics {
            parent_map.insert(unique_id.clone(), metric.depends_on.nodes.clone());
        }

        for (unique_id, sm) in &self.semantic_models {
            parent_map.insert(unique_id.clone(), sm.depends_on.nodes.clone());
        }

        for (unique_id, sq) in &self.saved_queries {
            parent_map.insert(unique_id.clone(), sq.depends_on.nodes.clone());
        }

        for (unique_id, ut) in &self.unit_tests {
            parent_map.insert(unique_id.clone(), ut.depends_on.nodes.clone());
        }

        parent_map
    }

    /// Build child map: maps each node to what depends on it.
    pub fn build_child_map(&self) -> HashMap<String, Vec<String>> {
        let mut child_map: HashMap<String, Vec<String>> = HashMap::new();

        for unique_id in self.nodes.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }
        for unique_id in self.sources.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }
        for unique_id in self.exposures.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }
        for unique_id in self.metrics.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }
        for unique_id in self.semantic_models.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }
        for unique_id in self.saved_queries.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }
        for unique_id in self.unit_tests.keys() {
            child_map.insert(unique_id.clone(), Vec::new());
        }

        let parent_map = self.build_parent_map();
        for (child_id, parents) in &parent_map {
            for parent_id in parents {
                if let Some(children) = child_map.get_mut(parent_id) {
                    children.push(child_id.clone());
                }
            }
        }

        child_map
    }

    /// Build group map (maps group names to nodes in that group)
    /// Returns HashMap<group_name, Vec<node_unique_id>>
    pub fn build_group_map(&self) -> HashMap<String, Vec<String>> {
        let mut group_map = HashMap::new();
        for (unique_id, node) in &self.nodes {
            if let Some(group_name) = node.group_name() {
                if !group_name.is_empty() {
                    group_map
                        .entry(group_name.to_string())
                        .or_insert_with(Vec::new)
                        .push(unique_id.clone());
                }
            }
        }
        group_map
    }

    #[cfg(test)]
    pub fn add_doc(&mut self, doc: crate::manifest::types::Doc) {
        let json_doc = serde_json::to_value(&doc).unwrap();
        self.docs.insert(doc.unique_id, json_doc);
    }

    #[cfg(test)]
    pub fn add_metric(&mut self, metric: OxideMetric) {
        self.metrics.insert(metric.unique_id.clone(), metric);
    }

    fn packages_to_search(
        current_project: &str,
        node_package: &str,
        target_package: Option<&str>,
    ) -> Vec<Option<String>> {
        if let Some(pkg) = target_package {
            vec![Some(pkg.to_string())]
        } else if current_project == node_package {
            vec![Some(current_project.to_string()), None]
        } else {
            vec![
                Some(current_project.to_string()),
                Some(node_package.to_string()),
                None,
            ]
        }
    }

    pub fn resolve_ref(
        &self,
        _source_node_id: Option<&str>,
        target_model_name: &str,
        target_model_package: Option<&str>,
        target_model_version: Option<i64>,
        current_project: &str,
        node_package: &str,
    ) -> Option<&OxideNode> {
        let packages = Self::packages_to_search(current_project, node_package, target_model_package);

        for package_opt in packages {
            for (unique_id, node) in &self.nodes {
                if node.name() != target_model_name {
                    continue;
                }

                // Check package match
                if let Some(ref pkg) = package_opt {
                    if node.package_name() != pkg {
                        continue;
                    }
                } // None means search all packages

                // Check version match
                if let Some(ver) = target_model_version {
                    if let Some(node_ver) = node.version() {
                        if node_ver != ver.to_string() {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                return Some(node);
            }
        }
        None
    }

    pub fn resolve_source(
        &self,
        target_source_name: &str,
        target_table_name: &str,
        current_project: &str,
        node_package: &str,
    ) -> Option<&OxideSource> {
        let packages = Self::packages_to_search(current_project, node_package, None);

        for package_opt in packages {
            for (_unique_id, source) in &self.sources {
                if source.source_name != target_source_name || source.name != target_table_name {
                    continue;
                }

                if let Some(ref pkg) = package_opt {
                    if &source.package_name != pkg {
                        continue;
                    }
                }

                return Some(source);
            }
        }
        None
    }

    pub fn resolve_metric(&self, name: &str, package: Option<&str>) -> Option<String> {
        for (unique_id, metric) in &self.metrics {
            if metric.name == name {
                if let Some(pkg) = package {
                    if metric.package_name != pkg {
                        continue;
                    }
                }
                return Some(unique_id.clone());
            }
        }
        None
    }

    pub fn resolve_doc(
        &self,
        name: &str,
        package: Option<&str>,
        current_project: &str,
        node_package: &str,
    ) -> Option<&serde_json::Value> {
        let packages = Self::packages_to_search(current_project, node_package, package);

        for package_opt in packages {
            for (_unique_id, doc) in &self.docs {
                // Extract name from doc Value
                let doc_name = doc.get("name").and_then(|v| v.as_str());
                if doc_name != Some(name) {
                    continue;
                }

                // Extract package_name from doc Value
                if let Some(ref pkg) = package_opt {
                    let doc_package = doc.get("package_name").and_then(|v| v.as_str());
                    if doc_package != Some(pkg.as_str()) {
                        continue;
                    }
                }

                return Some(doc);
            }
        }
        None
    }

    pub fn resolve_saved_query(&self, name: &str, package: Option<&str>) -> Option<String> {
        for (unique_id, sq) in &self.saved_queries {
            if sq.name == name {
                if let Some(pkg) = package {
                    if sq.package_name != pkg {
                        continue;
                    }
                }
                return Some(unique_id.clone());
            }
        }
        None
    }

    pub fn find_macro_by_name(
        &self,
        name: &str,
        root_project_name: &str,
        internal_packages: &HashSet<String>,
        package: Option<&str>,
    ) -> Option<String> {
        use crate::manifest::macro_lookup;
        macro_lookup::find_macro_by_name(
            &self.macros,
            name,
            root_project_name,
            internal_packages,
            package,
        )
    }

    pub fn find_materialization_macro_by_name(
        &self,
        project_name: &str,
        materialization_name: &str,
        adapter_types: &[String],
        internal_packages: &HashSet<String>,
        allow_package_override: bool,
    ) -> Option<String> {
        use crate::manifest::macro_lookup;
        macro_lookup::find_materialization_macro_by_name(
            &self.macros,
            project_name,
            materialization_name,
            adapter_types,
            internal_packages,
            allow_package_override,
        )
    }

    pub fn find_generate_macro_by_name(
        &self,
        _name: &str,
        _package: Option<&str>,
    ) -> Option<String> {
        unimplemented!("find_generate_macro_by_name not implemented")
    }

    pub fn get_macros_by_name(&self, _name: &str) -> Vec<String> {
        unimplemented!("get_macros_by_name not implemented")
    }

    pub fn disabled_lookup(&self, _unique_id: &str) -> Option<&Vec<serde_json::Value>> {
        unimplemented!("disabled_lookup not implemented")
    }

    pub fn get_nodes_iterator(&self) -> std::collections::hash_map::Iter<String, OxideNode> {
        self.nodes.iter()
    }

    pub fn get_sources_iterator(&self) -> std::collections::hash_map::Iter<String, OxideSource> {
        self.sources.iter()
    }

    pub fn get_macros_iterator(&self) -> std::collections::hash_map::Iter<String, OxideMacro> {
        self.macros.iter()
    }

    pub fn filter_nodes_by_resource_type(&self, resource_type: &str) -> Vec<&OxideNode> {
        self.nodes
            .values()
            .filter(|node| node.resource_type() == resource_type)
            .collect()
    }

    pub fn filter_nodes_by_package(&self, package_name: &str) -> Vec<&OxideNode> {
        self.nodes
            .values()
            .filter(|node| node.package_name() == package_name)
            .collect()
    }

    pub fn external_node_unique_ids(&self) -> Vec<String> {
        self.nodes
            .iter()
            .filter(|(_, node)| node.is_external_node())
            .map(|(unique_id, _)| unique_id.clone())
            .collect()
    }
}
