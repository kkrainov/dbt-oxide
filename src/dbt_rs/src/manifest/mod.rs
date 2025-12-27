// manifest/mod.rs - Manifest module organization
//
// This module provides the complete manifest schema for dbt-oxide.
// Types are organized into logical submodules:
// - types: Base types (FileHash, Docs, Contract, ColumnInfo, etc.)
// - config: Configuration types (OxideNodeConfig, OxideTestConfig, etc.)
// - nodes: Node types (OxideModel, OxideSeed, OxideSnapshot, etc.)
// - resources: Resource types (OxideSource, OxideMacro, OxideExposure, etc.)
// - semantic: Semantic layer types (Metric, SemanticModel, SavedQuery)
// - testing: Unit test types (UnitTestDefinition, Documentation, etc.)
// - macro_lookup: Macro and materialization lookup logic

pub mod config;
pub mod macro_lookup;
pub mod nodes;
pub mod resources;
pub mod semantic;
pub mod testing;
pub mod types;

// Re-export all public types for backward compatibility
pub use config::*;
pub use macro_lookup::*;
pub use nodes::*;
pub use resources::*;
pub use semantic::*;
pub use testing::*;
pub use types::*;
