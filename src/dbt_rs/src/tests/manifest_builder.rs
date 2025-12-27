// Tests for PyManifestBuilder class
// Following TDD approach: RED → GREEN → REFACTOR

use super::helpers::{MetadataBuilder, ModelBuilder};
use crate::manifest::*;
use crate::py_manifest_builder::PyManifestBuilder;

#[test]
fn test_new_manifest_builder_no_metadata() {
    // GREEN PHASE: Test creating builder without metadata
    let builder = PyManifestBuilder::new(None).unwrap();
    assert_eq!(builder.node_count(), 0);
    assert_eq!(builder.source_count(), 0);
    assert_eq!(builder.macro_count(), 0);
}

#[test]
fn test_new_manifest_builder_with_metadata() {
    // GREEN PHASE: Cannot test with actual PyDict in unit tests
    // This would require Python runtime. Test with None for now.
    // Full integration testing will be done via Python tests.
    let builder = PyManifestBuilder::new(None).unwrap();
    assert_eq!(builder.node_count(), 0);
}

#[test]
fn test_builder_node_count_getter() {
    // Test that node_count getter works
    let builder = PyManifestBuilder::new(None).unwrap();
    assert_eq!(builder.node_count(), 0);
}

#[test]
fn test_builder_source_count_getter() {
    // Test that source_count getter works
    let builder = PyManifestBuilder::new(None).unwrap();
    assert_eq!(builder.source_count(), 0);
}

#[test]
fn test_builder_macro_count_getter() {
    // Test that macro_count getter works
    let builder = PyManifestBuilder::new(None).unwrap();
    assert_eq!(builder.macro_count(), 0);
}

// Note: Tests for add_node(), add_source(), add_macro() with PyDict
// require Python runtime (pyo3::Python::with_gil).
// These will be tested via Python integration tests in tests/unit/test_rust_manifest.py
//
// For now, we verify the methods compile and the builder is created successfully.
