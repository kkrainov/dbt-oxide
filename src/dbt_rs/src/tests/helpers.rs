use crate::manifest::*;
use std::collections::HashMap;

pub struct ModelBuilder {
    model: OxideModel,
}

impl ModelBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        Self {
            model: OxideModel {
                unique_id: format!("model.{}.{}", package, name),
                name: name.to_string(),
                package_name: package.to_string(),
                fqn: vec![package.to_string(), name.to_string()],
                original_file_path: format!("models/{}.sql", name),
                path: format!("models/{}.sql", name),
                schema: "public".to_string(),
                database: None,
                alias: name.to_string(),
                checksum: FileHash::default(),
                config: OxideNodeConfig::default(),
                tags: vec![],
                description: String::new(),
                columns: HashMap::new(),
                meta: HashMap::new(),
                docs: Docs::default(),
                patch_path: None,
                build_path: None,
                unrendered_config: HashMap::new(),
                created_at: 0.0,
                config_call_dict: HashMap::new(),
                unrendered_config_call_dict: HashMap::new(),
                relation_name: None,
                raw_code: String::new(),
                doc_blocks: vec![],
                language: "sql".to_string(),
                refs: vec![],
                sources: vec![],
                metrics: vec![],
                depends_on: OxideDependsOn::default(),
                compiled_path: None,
                compiled: false,
                compiled_code: None,
                extra_ctes_injected: false,
                extra_ctes: vec![],
                contract: Contract::default(),
                access: "protected".to_string(),
                constraints: vec![],
                version: None,
                latest_version: None,
                deprecation_date: None,
                defer_relation: None,
                primary_key: vec![],
                time_spine: None,
                group: None,
            },
        }
    }

    pub fn with_database(mut self, database: &str) -> Self {
        self.model.database = Some(database.to_string());
        self
    }

    pub fn with_schema(mut self, schema: &str) -> Self {
        self.model.schema = schema.to_string();
        self
    }

    pub fn with_group(mut self, group: &str) -> Self {
        self.model.config.group = Some(group.to_string());
        self.model.group = Some(group.to_string());
        self
    }

    pub fn depends_on(mut self, nodes: Vec<&str>) -> Self {
        self.model.depends_on.nodes = nodes.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_version(mut self, version: i64) -> Self {
        self.model.version = Some(serde_json::json!(version));
        self
    }

    pub fn build(self) -> OxideModel {
        self.model
    }
}

pub struct SourceBuilder {
    source: SourceDefinition,
}

impl SourceBuilder {
    pub fn new(source_name: &str, table_name: &str, package: &str) -> Self {
        Self {
            source: SourceDefinition {
                unique_id: format!("source.{}.{}.{}", package, source_name, table_name),
                source_name: source_name.to_string(),
                name: table_name.to_string(),
                package_name: package.to_string(),
                fqn: vec![
                    package.to_string(),
                    source_name.to_string(),
                    table_name.to_string(),
                ],
                path: "sources.yml".to_string(),
                original_file_path: "sources.yml".to_string(),
                source_description: String::new(),
                database: None,
                schema: "raw".to_string(),
                identifier: table_name.to_string(),
                loader: String::new(),
                description: String::new(),
                columns: HashMap::new(),
                meta: HashMap::new(),
                tags: vec![],
                config: SourceConfig::default(),
                patch_path: None,
                unrendered_config: HashMap::new(),
                relation_name: None,
                created_at: 0.0,
                quoting: Quoting::default(),
                loaded_at_field: None,
                freshness: None,
                external: None,
                event_time: None,
            },
        }
    }

    pub fn with_database(mut self, database: &str) -> Self {
        self.source.database = Some(database.to_string());
        self
    }

    pub fn with_loader(mut self, loader: &str) -> Self {
        self.source.loader = loader.to_string();
        self
    }

    pub fn build(self) -> SourceDefinition {
        self.source
    }
}

pub struct MacroBuilder {
    macro_node: OxideMacro,
}

impl MacroBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        Self {
            macro_node: OxideMacro {
                unique_id: format!("macro.{}.{}", package, name),
                name: name.to_string(),
                package_name: package.to_string(),
                resource_type: "macro".to_string(),
                path: format!("macros/{}.sql", name),
                original_file_path: format!("macros/{}.sql", name),
                macro_sql: String::new(),
                depends_on: MacroDependsOn { macros: vec![] },
                description: String::new(),
                meta: HashMap::new(),
                docs: Docs::default(),
                patch_path: None,
                arguments: vec![],
                created_at: 0.0,
                supported_languages: None,
            },
        }
    }

    pub fn build(self) -> OxideMacro {
        self.macro_node
    }
}

pub struct ExposureBuilder {
    exposure: OxideExposure,
}

impl ExposureBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        Self {
            exposure: OxideExposure {
                unique_id: format!("exposure.{}.{}", package, name),
                name: name.to_string(),
                package_name: package.to_string(),
                path: "models/schema.yml".to_string(),
                original_file_path: "models/schema.yml".to_string(),
                fqn: vec![package.to_string(), name.to_string()],
                exposure_type: "dashboard".to_string(),
                owner: Owner {
                    email: "".to_string(),
                    name: None,
                },
                description: String::new(),
                label: None,
                maturity: None,
                meta: HashMap::new(),
                tags: vec![],
                config: ExposureConfig::default(),
                unrendered_config: HashMap::new(),
                url: None,
                depends_on: OxideDependsOn::default(),
                refs: vec![],
                sources: vec![],
                metrics: vec![],
                created_at: 0.0,
            },
        }
    }

    pub fn with_owner(mut self, email: &str, name: Option<&str>) -> Self {
        self.exposure.owner = Owner {
            email: email.to_string(),
            name: name.map(|n| n.to_string()),
        };
        self
    }

    pub fn build(self) -> OxideExposure {
        self.exposure
    }
}

pub struct MetricBuilder {
    metric: OxideMetric,
}

impl MetricBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        Self {
            metric: OxideMetric {
                unique_id: format!("metric.{}.{}", package, name),
                name: name.to_string(),
                package_name: package.to_string(),
                path: "models/schema.yml".to_string(),
                original_file_path: "models/schema.yml".to_string(),
                fqn: vec![package.to_string(), name.to_string()],
                description: String::new(),
                label: String::new(),
                metric_type: "simple".to_string(),
                type_params: serde_json::json!({}),
                filter: None,
                meta: HashMap::new(),
                tags: vec![],
                config: MetricConfig::default(),
                unrendered_config: HashMap::new(),
                created_at: 0.0,
                depends_on: OxideDependsOn::default(),
                refs: vec![],
                metrics: vec![],
                group: None,
            },
        }
    }

    pub fn build(self) -> OxideMetric {
        self.metric
    }
}

pub struct SeedBuilder {
    seed: OxideSeed,
}

impl SeedBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        let unique_id = format!("seed.{}.{}", package, name);
        // Using OxideSeed explicitly instead of Default to control required fields if any
        let mut seed = OxideSeed::default();
        seed.unique_id = unique_id.clone();
        seed.name = name.to_string();
        seed.package_name = package.to_string();
        seed.original_file_path = format!("seeds/{}.csv", name);
        seed.path = format!("seeds/{}.csv", name);
        seed.fqn = vec![package.to_string(), name.to_string()];
        seed.database = None;
        seed.schema = "public".to_string();
        seed.alias = name.to_string();
        Self { seed }
    }

    pub fn build(self) -> OxideSeed {
        self.seed
    }
}

pub struct SnapshotBuilder {
    snapshot: OxideSnapshot,
}

impl SnapshotBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        let unique_id = format!("snapshot.{}.{}", package, name);
        let mut snapshot = OxideSnapshot::default();
        snapshot.unique_id = unique_id.clone();
        snapshot.name = name.to_string();
        snapshot.package_name = package.to_string();
        snapshot.original_file_path = format!("snapshots/{}.sql", name);
        snapshot.path = format!("snapshots/{}.sql", name);
        snapshot.fqn = vec![package.to_string(), name.to_string()];
        snapshot.database = None;
        snapshot.schema = "public".to_string();
        snapshot.alias = name.to_string();
        Self { snapshot }
    }

    pub fn build(self) -> OxideSnapshot {
        self.snapshot
    }
}

pub struct AnalysisBuilder {
    analysis: OxideAnalysis,
}

impl AnalysisBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        let unique_id = format!("analysis.{}.{}", package, name);
        let mut analysis = OxideAnalysis::default();
        analysis.unique_id = unique_id.clone();
        analysis.name = name.to_string();
        analysis.package_name = package.to_string();
        analysis.original_file_path = format!("analyses/{}.sql", name);
        analysis.path = format!("analyses/{}.sql", name);
        analysis.fqn = vec![package.to_string(), name.to_string()];
        analysis.database = None;
        analysis.schema = "public".to_string();
        analysis.alias = name.to_string();
        Self { analysis }
    }

    pub fn build(self) -> OxideAnalysis {
        self.analysis
    }
}

pub struct TestBuilder {
    // We can't hold a generic "Test" easily because OxideTestNode is an enum.
    // Let's support creating singular tests primarily for now as they are nodes.
    singular: Option<OxideSingularTest>,
    generic: Option<OxideGenericTest>,
}

impl TestBuilder {
    pub fn new_singular(name: &str, package: &str) -> Self {
        let unique_id = format!("test.{}.{}", package, name);
        let mut test = OxideSingularTest::default();
        test.unique_id = unique_id.clone();
        test.name = name.to_string();
        test.package_name = package.to_string();
        test.original_file_path = format!("tests/{}.sql", name);
        test.path = format!("tests/{}.sql", name);
        test.fqn = vec![package.to_string(), name.to_string()];
        test.database = None;
        test.schema = "public".to_string();
        test.alias = name.to_string();
        Self {
            singular: Some(test),
            generic: None,
        }
    }

    pub fn new_generic(name: &str, package: &str) -> Self {
        let unique_id = format!("test.{}.{}", package, name);
        let mut test = OxideGenericTest::default();
        test.unique_id = unique_id.clone();
        test.name = name.to_string();
        test.package_name = package.to_string();
        test.original_file_path = format!("tests/{}.sql", name);
        test.path = format!("tests/{}.sql", name);
        test.fqn = vec![package.to_string(), name.to_string()];
        test.database = None;
        test.schema = "public".to_string();
        test.alias = name.to_string();
        Self {
            singular: None,
            generic: Some(test),
        }
    }

    pub fn build(self) -> OxideNode {
        if let Some(s) = self.singular {
            OxideNode::Test(OxideTestNode::Singular(s))
        } else if let Some(g) = self.generic {
            OxideNode::Test(OxideTestNode::Generic(g))
        } else {
            panic!("TestBuilder invalid state");
        }
    }
}

pub struct ParsedResourceBuilder {
    resource: ParsedResource,
}

impl ParsedResourceBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        let unique_id = format!("model.{}.{}", package, name);
        let resource = ParsedResource {
            unique_id: unique_id.clone(),
            name: name.to_string(),
            package_name: package.to_string(),
            original_file_path: format!("models/{}.sql", name),
            path: format!("models/{}.sql", name),
            fqn: vec![package.to_string(), name.to_string()],
            database: None,
            schema: "public".to_string(),
            alias: name.to_string(),
            checksum: FileHash {
                name: "sha256".to_string(),
                checksum: name.to_string(),
            },
            config: OxideNodeConfig::default(),
            tags: vec![],
            description: String::new(),
            columns: HashMap::new(),
            meta: HashMap::new(),
            group: None,
            docs: Docs::default(),
            patch_path: None,
            build_path: None,
            unrendered_config: HashMap::new(),
            created_at: 0.0,
            config_call_dict: HashMap::new(),
            unrendered_config_call_dict: HashMap::new(),
            relation_name: None,
            raw_code: String::new(),
            doc_blocks: vec![],
        };
        Self { resource }
    }

    pub fn build(self) -> ParsedResource {
        self.resource
    }
}

pub struct CompiledResourceBuilder {
    resource: CompiledResource,
}

impl CompiledResourceBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        let unique_id = format!("model.{}.{}", package, name);
        let mut resource = CompiledResource {
            unique_id: unique_id.clone(),
            name: name.to_string(),
            package_name: package.to_string(),
            original_file_path: format!("models/{}.sql", name),
            path: format!("models/{}.sql", name),
            fqn: vec![package.to_string(), name.to_string()],
            database: None,
            schema: "public".to_string(),
            alias: name.to_string(),
            checksum: FileHash::default(),
            config: OxideNodeConfig::default(),
            tags: vec![],
            description: String::new(),
            columns: HashMap::new(),
            meta: HashMap::new(),
            group: None,
            docs: Docs::default(),
            patch_path: None,
            build_path: None,
            unrendered_config: HashMap::new(),
            created_at: 0.0,
            config_call_dict: HashMap::new(),
            unrendered_config_call_dict: HashMap::new(),
            relation_name: None,
            raw_code: String::new(),
            doc_blocks: vec![],
            language: "sql".to_string(),
            refs: vec![],
            sources: vec![],
            metrics: vec![],
            depends_on: OxideDependsOn::default(),
            compiled_path: None,
            compiled: false,
            compiled_code: None,
            extra_ctes_injected: false,
            extra_ctes: vec![],
            contract: Contract::default(),
        };
        resource.checksum = FileHash {
            name: "sha256".to_string(),
            checksum: name.to_string(),
        };
        Self { resource }
    }

    pub fn build(self) -> CompiledResource {
        self.resource
    }
}

pub struct ColumnBuilder {
    column: ColumnInfo,
}

impl ColumnBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            column: ColumnInfo {
                name: name.to_string(),
                description: String::new(),
                meta: HashMap::new(),
                data_type: None,
                constraints: vec![],
                quote: None,
                config: ColumnConfig::default(),
                tags: vec![],
                granularity: None,
                doc_blocks: vec![],
            },
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.column.description = desc.to_string();
        self
    }

    pub fn with_data_type(mut self, data_type: &str) -> Self {
        self.column.data_type = Some(data_type.to_string());
        self
    }

    pub fn build(self) -> ColumnInfo {
        self.column
    }
}

pub struct MetadataBuilder {
    metadata: ManifestMetadata,
}

impl MetadataBuilder {
    pub fn new() -> Self {
        Self {
            metadata: ManifestMetadata {
                dbt_schema_version: "https://schemas.getdbt.com/dbt/manifest/v12.json".to_string(),
                dbt_version: Some("1.8.0".to_string()),
                generated_at: Some("2024-01-01T00:00:00Z".to_string()),
                invocation_id: Some("uuid".to_string()),
                env: std::collections::HashMap::new(),
                project_name: Some("my_project".to_string()),
                project_id: None,
                user_id: None,
                send_anonymous_usage_stats: Some(false),
                adapter_type: Some("postgres".to_string()),
            },
        }
    }

    pub fn build(self) -> ManifestMetadata {
        self.metadata
    }
}

pub struct ManifestBuilder {
    manifest: OxideManifest,
}

impl ManifestBuilder {
    pub fn new() -> Self {
        Self {
            manifest: OxideManifest {
                metadata: ManifestMetadata::default(),
                nodes: std::collections::HashMap::new(),
                sources: std::collections::HashMap::new(),
                macros: std::collections::HashMap::new(),
                docs: std::collections::HashMap::new(),
                exposures: std::collections::HashMap::new(),
                metrics: std::collections::HashMap::new(),
                groups: std::collections::HashMap::new(),
                selectors: std::collections::HashMap::new(),
                disabled: std::collections::HashMap::new(),
                semantic_models: std::collections::HashMap::new(),
                unit_tests: std::collections::HashMap::new(),
                saved_queries: std::collections::HashMap::new(),
            },
        }
    }

    pub fn with_node(mut self, node: OxideNode) -> Self {
        self.manifest.add_node(node);
        self
    }

    pub fn with_source(mut self, source: SourceDefinition) -> Self {
        self.manifest.add_source(source);
        self
    }

    pub fn build(self) -> OxideManifest {
        self.manifest
    }
}

pub struct ExternalTableBuilder {
    external: ExternalTable,
}

impl ExternalTableBuilder {
    pub fn new() -> Self {
        Self {
            external: ExternalTable {
                location: None,
                file_format: None,
                row_format: None,
                tbl_properties: None,
                partitions: vec![],
            },
        }
    }

    pub fn with_location(mut self, location: &str) -> Self {
        self.external.location = Some(location.to_string());
        self
    }

    pub fn with_file_format(mut self, format: &str) -> Self {
        self.external.file_format = Some(format.to_string());
        self
    }

    pub fn build(self) -> ExternalTable {
        self.external
    }
}

pub struct ExternalPartitionBuilder {
    partition: ExternalPartition,
}

impl ExternalPartitionBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            partition: ExternalPartition {
                name: name.to_string(),
                description: String::new(),
                data_type: None,
                meta: std::collections::HashMap::new(),
            },
        }
    }

    pub fn with_data_type(mut self, dtype: &str) -> Self {
        self.partition.data_type = Some(dtype.to_string());
        self
    }

    pub fn build(self) -> ExternalPartition {
        self.partition
    }
}

pub struct SourceConfigBuilder {
    config: SourceConfig,
}

impl SourceConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: SourceConfig {
                enabled: true,
                event_time: None,
                freshness: None,
                loaded_at_field: None,
                loaded_at_query: None,
                meta: std::collections::HashMap::new(),
                tags: vec![],
            },
        }
    }

    pub fn build(self) -> SourceConfig {
        self.config
    }
}

pub struct TimeBuilder {
    time: Time,
}

impl TimeBuilder {
    pub fn new(count: i32, period: &str) -> Self {
        Self {
            time: Time {
                count: Some(count),
                period: Some(period.to_string()),
            },
        }
    }

    pub fn build(self) -> Time {
        self.time
    }
}

pub struct FreshnessBuilder {
    freshness: FreshnessThreshold,
}

impl FreshnessBuilder {
    pub fn new() -> Self {
        Self {
            freshness: FreshnessThreshold {
                warn_after: None,
                error_after: None,
                filter: None,
            },
        }
    }

    pub fn with_warn_after(mut self, count: i32, period: &str) -> Self {
        self.freshness.warn_after = Some(Time {
            count: Some(count),
            period: Some(period.to_string()),
        });
        self
    }

    pub fn with_error_after(mut self, count: i32, period: &str) -> Self {
        self.freshness.error_after = Some(Time {
            count: Some(count),
            period: Some(period.to_string()),
        });
        self
    }

    pub fn build(self) -> FreshnessThreshold {
        self.freshness
    }
}

pub struct DependsOnBuilder {
    deps: OxideDependsOn,
}

impl DependsOnBuilder {
    pub fn new() -> Self {
        Self {
            deps: OxideDependsOn {
                nodes: vec![],
                macros: vec![],
            },
        }
    }

    pub fn with_nodes(mut self, nodes: Vec<&str>) -> Self {
        self.deps.nodes = nodes.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_macros(mut self, macros: Vec<&str>) -> Self {
        self.deps.macros = macros.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> OxideDependsOn {
        self.deps
    }
}

pub struct RefArgsBuilder {
    args: RefArgs,
}

impl RefArgsBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            args: RefArgs {
                name: name.to_string(),
                package: None,
                version: None,
            },
        }
    }

    pub fn build(self) -> RefArgs {
        self.args
    }
}

pub struct ColumnConfigBuilder {
    config: ColumnConfig,
}

impl ColumnConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ColumnConfig {
                meta: HashMap::new(),
                tags: vec![],
            },
        }
    }

    pub fn with_meta(mut self, key: &str, value: &str) -> Self {
        self.config
            .meta
            .insert(key.to_string(), serde_json::json!(value));
        self
    }

    pub fn build(self) -> ColumnConfig {
        self.config
    }
}

pub struct InjectedCTEBuilder {
    cte: InjectedCTE,
}

impl InjectedCTEBuilder {
    pub fn new(id: &str, sql: &str) -> Self {
        Self {
            cte: InjectedCTE {
                id: id.to_string(),
                sql: sql.to_string(),
            },
        }
    }

    pub fn build(self) -> InjectedCTE {
        self.cte
    }
}

pub struct ContractBuilder {
    contract: Contract,
}

impl ContractBuilder {
    pub fn new() -> Self {
        Self {
            contract: Contract {
                enforced: false,
                alias_types: false,
                checksum: None,
            },
        }
    }

    pub fn enforced(mut self) -> Self {
        self.contract.enforced = true;
        self
    }

    pub fn alias_types(mut self) -> Self {
        self.contract.alias_types = true;
        self
    }

    pub fn build(self) -> Contract {
        self.contract
    }
}

pub struct QuotingBuilder {
    quoting: Quoting,
}

impl QuotingBuilder {
    pub fn new() -> Self {
        Self {
            quoting: Quoting {
                database: None,
                schema: None,
                identifier: None,
                column: None,
            },
        }
    }

    pub fn with_database(mut self, val: bool) -> Self {
        self.quoting.database = Some(val);
        self
    }

    pub fn with_schema(mut self, val: bool) -> Self {
        self.quoting.schema = Some(val);
        self
    }

    pub fn with_identifier(mut self, val: bool) -> Self {
        self.quoting.identifier = Some(val);
        self
    }

    pub fn build(self) -> Quoting {
        self.quoting
    }
}

pub struct ContractConfigBuilder {
    config: ContractConfig,
}

impl ContractConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ContractConfig {
                enforced: false,
                alias_types: false,
            },
        }
    }

    pub fn enforced(mut self) -> Self {
        self.config.enforced = true;
        self
    }

    pub fn alias_types(mut self) -> Self {
        self.config.alias_types = true;
        self
    }

    pub fn build(self) -> ContractConfig {
        self.config
    }
}

pub struct HookBuilder {
    hook: Hook,
}

impl HookBuilder {
    pub fn new(sql: &str) -> Self {
        Self {
            hook: Hook {
                sql: sql.to_string(),
                transaction: true,
                index: None,
            },
        }
    }

    pub fn build(self) -> Hook {
        self.hook
    }
}

pub struct NodeConfigBuilder {
    config: OxideNodeConfig,
}

impl NodeConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: OxideNodeConfig::default(),
        }
    }

    pub fn enabled(mut self) -> Self {
        self.config.enabled = true;
        self
    }

    pub fn with_materialized(mut self, mat: &str) -> Self {
        self.config.materialized = mat.to_string();
        self
    }

    pub fn build(self) -> OxideNodeConfig {
        self.config
    }
}

pub struct TestConfigBuilder {
    config: OxideTestConfig,
}

impl TestConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: OxideTestConfig {
                enabled: true,
                severity: "ERROR".to_string(),
                fail_calc: "count(*)".to_string(),
                warn_if: "!= 0".to_string(),
                error_if: "!= 0".to_string(),
                ..Default::default()
            },
        }
    }

    pub fn build(self) -> OxideTestConfig {
        self.config
    }
}

pub struct DeferRelationBuilder {
    defer: DeferRelation,
}

impl DeferRelationBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            defer: DeferRelation {
                alias: name.to_string(),
                relation_name: None,
                name: name.to_string(),
                description: String::new(),
                compiled_code: None,
                meta: HashMap::new(),
                tags: vec![],
            },
        }
    }

    pub fn build(self) -> DeferRelation {
        self.defer
    }
}

pub struct NodeAndTestConfigBuilder {
    config: NodeAndTestConfig,
}

impl NodeAndTestConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: NodeAndTestConfig {
                enabled: true,
                alias: None,
                schema: None,
                database: None,
                tags: vec![],
                meta: HashMap::new(),
                group: None,
            },
        }
    }

    pub fn build(self) -> NodeAndTestConfig {
        self.config
    }
}

pub struct FileHashBuilder {
    hash: FileHash,
}

impl FileHashBuilder {
    pub fn new(checksum: &str) -> Self {
        Self {
            hash: FileHash {
                name: "sha256".to_string(),
                checksum: checksum.to_string(),
            },
        }
    }

    pub fn build(self) -> FileHash {
        self.hash
    }
}

pub struct DocsBuilder {
    docs: Docs,
}

impl DocsBuilder {
    pub fn new() -> Self {
        Self {
            docs: Docs {
                show: true,
                node_color: None,
            },
        }
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.docs.node_color = Some(color.to_string());
        self
    }

    pub fn build(self) -> Docs {
        self.docs
    }
}

pub struct BaseResourceBuilder {
    resource: BaseResource,
}

impl BaseResourceBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        Self {
            resource: BaseResource {
                name: name.to_string(),
                package_name: package.to_string(),
                path: format!("models/{}.sql", name),
                original_file_path: format!("models/{}.sql", name),
                unique_id: format!("model.{}.{}", package, name),
            },
        }
    }

    pub fn build(self) -> BaseResource {
        self.resource
    }
}

pub struct GraphResourceBuilder {
    resource: GraphResource,
}

impl GraphResourceBuilder {
    pub fn new(name: &str, package: &str) -> Self {
        Self {
            resource: GraphResource {
                name: name.to_string(),
                package_name: package.to_string(),
                path: format!("models/{}.sql", name),
                original_file_path: format!("models/{}.sql", name),
                unique_id: format!("model.{}.{}", package, name),
                fqn: vec![package.to_string(), name.to_string()],
            },
        }
    }

    pub fn build(self) -> GraphResource {
        self.resource
    }
}

pub fn default_internal_packages() -> std::collections::HashSet<String> {
    ["dbt"].iter().map(|s| s.to_string()).collect()
}

pub fn adapter_internal_packages() -> std::collections::HashSet<String> {
    ["dbt", "dbt_postgres"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}
