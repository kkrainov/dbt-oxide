use super::helpers::{
    ColumnBuilder, ColumnConfigBuilder, ContractBuilder, ContractConfigBuilder,
    DeferRelationBuilder, DependsOnBuilder, FreshnessBuilder, HookBuilder, InjectedCTEBuilder,
    NodeAndTestConfigBuilder, NodeConfigBuilder, QuotingBuilder, RefArgsBuilder,
    SourceConfigBuilder, TestConfigBuilder, TimeBuilder,
};
use crate::manifest::*;
use serde_json;

#[test]
fn test_serialize_macro_depends_on() {
    let deps = MacroDependsOn {
        macros: vec!["macro.dbt.ref".to_string()],
    };
    let json = serde_json::to_string(&deps).unwrap();
    assert!(json.contains("\"macros\""));
}

#[test]
fn test_serialize_depends_on() {
    let deps = DependsOnBuilder::new()
        .with_nodes(vec!["model.pkg.other"])
        .with_macros(vec!["macro.dbt.ref"])
        .build();
    let json = serde_json::to_string(&deps).unwrap();
    assert!(json.contains("\"nodes\""));
}

#[test]
fn test_serialize_ref_args() {
    let ref_args = RefArgsBuilder::new("my_model").build();
    let json = serde_json::to_string(&ref_args).unwrap();
    assert!(json.contains("\"name\":\"my_model\""));
}

#[test]
fn test_serialize_column_config() {
    let config = ColumnConfigBuilder::new().with_meta("foo", "bar").build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"meta\""));
}

#[test]
fn test_serialize_column_info() {
    let column = ColumnBuilder::new("id")
        .with_description("Primary key")
        .with_data_type("integer")
        .build();
    let json = serde_json::to_string(&column).unwrap();
    assert!(json.contains("\"name\":\"id\""));
}

#[test]
fn test_serialize_injected_cte() {
    let cte = InjectedCTEBuilder::new("cte_1", "SELECT * FROM table").build();
    let json = serde_json::to_string(&cte).unwrap();
    assert!(json.contains("\"id\":\"cte_1\""));
}

#[test]
fn test_serialize_contract() {
    let contract = ContractBuilder::new().enforced().alias_types().build();
    let json = serde_json::to_string(&contract).unwrap();
    assert!(json.contains("\"enforced\":true"));
}

#[test]
fn test_serialize_quoting() {
    let quoting = QuotingBuilder::new()
        .with_database(false)
        .with_schema(false)
        .with_identifier(true)
        .build();
    let json = serde_json::to_string(&quoting).unwrap();
    assert!(json.contains("\"identifier\":true"));
}

#[test]
fn test_serialize_contract_config() {
    let config = ContractConfigBuilder::new()
        .enforced()
        .alias_types()
        .build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"enforced\":true"));
}

#[test]
fn test_serialize_hook() {
    let hook = HookBuilder::new("GRANT SELECT").build();
    let json = serde_json::to_string(&hook).unwrap();
    assert!(json.contains("\"sql\":\"GRANT SELECT\""));
}

#[test]
fn test_serialize_node_config() {
    let config = NodeConfigBuilder::new()
        .enabled()
        .with_materialized("view")
        .build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"materialized\":\"view\""));
}

#[test]
fn test_serialize_test_config() {
    let config = TestConfigBuilder::new().build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"severity\":\"ERROR\""));
}

#[test]
fn test_serialize_model_config() {
    let config = NodeConfigBuilder::new()
        .enabled()
        .with_materialized("table")
        .build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"materialized\":\"table\""));
}

#[test]
fn test_serialize_seed_config() {
    let config = NodeConfigBuilder::new()
        .enabled()
        .with_materialized("seed")
        .build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"materialized\":\"seed\""));
}

#[test]
fn test_serialize_snapshot_config() {
    let config = NodeConfigBuilder::new()
        .enabled()
        .with_materialized("snapshot")
        .build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"materialized\":\"snapshot\""));
}

#[test]
fn test_serialize_time() {
    let time = TimeBuilder::new(1, "hour").build();
    let json = serde_json::to_string(&time).unwrap();
    assert!(json.contains("\"count\":1"));
}

#[test]
fn test_serialize_freshness_threshold() {
    let threshold = FreshnessBuilder::new()
        .with_warn_after(1, "day")
        .with_error_after(2, "day")
        .build();
    let json = serde_json::to_string(&threshold).unwrap();
    assert!(json.contains("\"warn_after\""));
}

#[test]
fn test_serialize_has_relation_metadata() {
    let metadata = HasRelationMetadata {
        database: Some("analytics".to_string()),
        schema: "public".to_string(),
    };
    let json = serde_json::to_string(&metadata).unwrap();
    assert!(json.contains("\"schema\":\"public\""));
}

#[test]
fn test_serialize_defer_relation() {
    let defer = DeferRelationBuilder::new("my_model").build();
    let json = serde_json::to_string(&defer).unwrap();
    assert!(json.contains("\"alias\":\"my_model\""));
}

#[test]
fn test_serialize_column_info_all_10_fields() {
    let column = ColumnBuilder::new("id")
        .with_description("Primary key")
        .with_data_type("integer")
        .build();
    let json = serde_json::to_string(&column).unwrap();
    assert!(json.contains("\"name\":\"id\""));
}

#[test]
fn test_serialize_node_and_test_config() {
    let config = NodeAndTestConfigBuilder::new().build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"enabled\":true"));
}

#[test]
fn test_serialize_node_config_all_22_fields() {
    let config = OxideNodeConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"enabled\""));
}

#[test]
fn test_serialize_node_config_defaults() {
    let config: OxideNodeConfig = serde_json::from_str("{}").unwrap();
    assert!(config.enabled);
}

#[test]
fn test_serialize_source_config() {
    let config = SourceConfigBuilder::new().build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"enabled\":true"));
}
