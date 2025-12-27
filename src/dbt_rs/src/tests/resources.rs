use super::helpers::{
    ColumnBuilder, ExposureBuilder, ExternalPartitionBuilder, ExternalTableBuilder,
    FreshnessBuilder, MacroBuilder, MetricBuilder, SourceBuilder, SourceConfigBuilder,
};
use crate::manifest::*;
use serde_json;

#[test]
fn test_serialize_source_definition_full() {
    let mut source = SourceBuilder::new("src", "tbl", "pkg")
        .with_database("raw")
        .with_loader("snowflake")
        .build();
    source.source_description = "A source table".to_string();
    source.description = "My source".to_string();
    source.schema = "public".to_string();
    source.tags = vec!["source".to_string()];
    source.relation_name = Some("raw.public.tbl".to_string());
    source.loaded_at_field = Some("_loaded_at".to_string());
    source.freshness = Some(
        FreshnessBuilder::new()
            .with_warn_after(1, "hour")
            .with_error_after(24, "hour")
            .build(),
    );
    source.external = Some(
        ExternalTableBuilder::new()
            .with_location("s3://bucket/path")
            .with_file_format("parquet")
            .build(),
    );

    let json = serde_json::to_string(&source).unwrap();
    assert!(json.contains("\"loader\":\"snowflake\""));
}

#[test]
fn test_serialize_source_definition_minimal() {
    let source = SourceBuilder::new("src", "tbl", "pkg").build();
    let json = serde_json::to_string(&source).unwrap();
    assert!(json.contains("\"source_name\":\"src\""));
}

#[test]
fn test_serialize_external_table() {
    let external = ExternalTableBuilder::new()
        .with_location("s3://bucket")
        .with_file_format("csv")
        .build();
    let json = serde_json::to_string(&external).unwrap();
    assert!(json.contains("\"location\":\"s3://bucket\""));
}

#[test]
fn test_serialize_external_partition() {
    let partition = ExternalPartitionBuilder::new("dt")
        .with_data_type("date")
        .build();
    let json = serde_json::to_string(&partition).unwrap();
    assert!(json.contains("\"name\":\"dt\""));
}

#[test]
fn test_serialize_source_config_full() {
    let config = SourceConfigBuilder::new().build();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"enabled\":true"));
}

#[test]
fn test_serialize_source_with_freshness() {
    let mut source = SourceBuilder::new("src", "tbl", "pkg").build();
    source.loaded_at_field = Some("_loaded_at".to_string());
    source.freshness = Some(FreshnessBuilder::new().with_warn_after(12, "hour").build());
    let json = serde_json::to_string(&source).unwrap();
    assert!(json.contains("\"warn_after\""));
}

#[test]
fn test_serialize_source_with_external() {
    let mut source = SourceBuilder::new("src", "tbl", "pkg").build();
    source.external = Some(
        ExternalTableBuilder::new()
            .with_location("s3://bucket")
            .build(),
    );
    let json = serde_json::to_string(&source).unwrap();
    assert!(json.contains("\"location\":\"s3://bucket\""));
}

#[test]
fn test_serialize_source_with_columns() {
    let mut columns = std::collections::HashMap::new();
    let col = ColumnBuilder::new("id")
        .with_description("PK")
        .with_data_type("integer")
        .build();
    columns.insert("id".to_string(), col);

    let mut source = SourceBuilder::new("src", "tbl", "pkg").build();
    source.columns = columns;

    let json = serde_json::to_string(&source).unwrap();
    assert!(json.contains("\"id\":{"));
    assert!(json.contains("\"description\":\"PK\""));
}

#[test]
fn test_serialize_macro() {
    let mut macro_node = MacroBuilder::new("my_macro", "pkg").build();
    macro_node.macro_sql = "{% macro my_macro() %}...{% endmacro %}".to_string();

    let json = serde_json::to_string(&macro_node).unwrap();
    assert!(json.contains("\"macro_sql\""));
}

#[test]
fn test_serialize_macro_with_arguments() {
    let mut macro_node = MacroBuilder::new("my_macro", "pkg").build();
    macro_node.arguments = vec![MacroArgument {
        name: "arg1".to_string(),
        arg_type: Some("string".to_string()),
        description: String::new(),
    }];

    let json = serde_json::to_string(&macro_node).unwrap();
    assert!(json.contains("\"arguments\":["));
    assert!(json.contains("\"name\":\"arg1\""));
}

#[test]
fn test_serialize_macro_argument() {
    let arg = MacroArgument {
        name: "arg1".to_string(),
        arg_type: Some("string".to_string()),
        description: "An argument".to_string(),
    };
    let json = serde_json::to_string(&arg).unwrap();
    assert!(json.contains("\"name\":\"arg1\""));
}

#[test]
fn test_serialize_macro_with_supported_languages() {
    let mut macro_node = MacroBuilder::new("my_macro", "pkg").build();
    macro_node.supported_languages = Some(vec!["python".to_string(), "sql".to_string()]);

    let json = serde_json::to_string(&macro_node).unwrap();
    assert!(json.contains("\"supported_languages\":[\"python\",\"sql\"]"));
}

#[test]
fn test_serialize_exposure() {
    let mut exposure = ExposureBuilder::new("my_exposure", "pkg")
        .with_owner("user@example.com", Some("User Name"))
        .build();
    exposure.description = "My exposure".to_string();
    exposure.maturity = Some("high".to_string());
    exposure.url = Some("http://example.com".to_string());
    exposure.depends_on = OxideDependsOn {
        nodes: vec!["model.pkg.my_model".to_string()],
        macros: vec![],
    };

    let json = serde_json::to_string(&exposure).unwrap();
    assert!(json.contains("\"type\":\"dashboard\""));
    assert!(json.contains("\"owner\":{"));
}

#[test]
fn test_serialize_exposure_with_owner() {
    let mut exposure = ExposureBuilder::new("my_exposure", "pkg")
        .with_owner("user@example.com", Some("User Name"))
        .build();
    exposure.description = "My exposure".to_string();

    let json = serde_json::to_string(&exposure).unwrap();
    assert!(json.contains("\"email\":\"user@example.com\""));
}

#[test]
fn test_serialize_exposure_config() {
    let config = ExposureConfig {
        enabled: true,
        tags: vec![],
        meta: std::collections::HashMap::new(),
    };
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"enabled\":true"));
}

#[test]
fn test_serialize_owner() {
    let owner = Owner {
        email: "user@example.com".to_string(),
        name: Some("User".to_string()),
    };
    let json = serde_json::to_string(&owner).unwrap();
    assert!(json.contains("\"email\":\"user@example.com\""));
}

#[test]
fn test_serialize_metric() {
    let mut metric = MetricBuilder::new("my_metric", "pkg").build();
    metric.description = "A metric".to_string();
    metric.label = "My Metric".to_string();
    metric.type_params = serde_json::json!({"measure": "m"});

    let json = serde_json::to_string(&metric).unwrap();
    assert!(json.contains("\"type\":\"simple\""));
}
