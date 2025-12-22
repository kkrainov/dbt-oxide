use crate::graph::OxideGraph;
use crate::manifest::OxideManifest;

#[allow(dead_code)]
pub fn build_graph_from_manifest(manifest: &OxideManifest) -> OxideGraph {
    let mut graph = OxideGraph::new();

    // Add all nodes from manifest collections
    for unique_id in manifest.sources.keys() {
        graph.add_node(unique_id.clone());
    }
    for unique_id in manifest.nodes.keys() {
        graph.add_node(unique_id.clone());
    }
    for unique_id in manifest.exposures.keys() {
        graph.add_node(unique_id.clone());
    }
    for unique_id in manifest.metrics.keys() {
        graph.add_node(unique_id.clone());
    }
    for unique_id in manifest.semantic_models.keys() {
        graph.add_node(unique_id.clone());
    }
    for unique_id in manifest.saved_queries.keys() {
        graph.add_node(unique_id.clone());
    }
    for unique_id in manifest.unit_tests.keys() {
        graph.add_node(unique_id.clone());
    }

    // Add edges from depends_on relationships
    for (unique_id, node) in &manifest.nodes {
        for dep_id in &node.depends_on.nodes {
            let _ = graph.add_edge(dep_id, unique_id, None);
        }
    }
    for (unique_id, exposure) in &manifest.exposures {
        for dep_id in &exposure.depends_on.nodes {
            let _ = graph.add_edge(dep_id, unique_id, None);
        }
    }
    for (unique_id, metric) in &manifest.metrics {
        for dep_id in &metric.depends_on.nodes {
            let _ = graph.add_edge(dep_id, unique_id, None);
        }
    }
    for (unique_id, sm) in &manifest.semantic_models {
        for dep_id in &sm.depends_on.nodes {
            let _ = graph.add_edge(dep_id, unique_id, None);
        }
    }
    for (unique_id, sq) in &manifest.saved_queries {
        for dep_id in &sq.depends_on.nodes {
            let _ = graph.add_edge(dep_id, unique_id, None);
        }
    }
    for (unique_id, ut) in &manifest.unit_tests {
        for dep_id in &ut.depends_on.nodes {
            let _ = graph.add_edge(dep_id, unique_id, None);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_graph_empty() {
        let manifest = OxideManifest::from_json_str(r#"{"nodes":{},"sources":{}}"#).unwrap();
        let graph = build_graph_from_manifest(&manifest);
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_build_graph_with_single_dependency() {
        let json = r#"{
            "nodes": {
                "model.test.a": {"unique_id":"model.test.a","name":"a","resource_type":"model","package_name":"test"},
                "model.test.b": {"unique_id":"model.test.b","name":"b","resource_type":"model","package_name":"test",
                                 "depends_on":{"nodes":["model.test.a"]}}
            },
            "sources": {}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph
            .ancestors("model.test.b", None)
            .contains("model.test.a"));
    }

    #[test]
    fn test_build_graph_includes_all_manifest_types() {
        let json = r#"{
            "nodes": {"model.test.m": {"unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test"}},
            "sources": {"source.test.raw.tbl": {"unique_id":"source.test.raw.tbl","source_name":"raw","name":"tbl","package_name":"test"}},
            "exposures": {"exposure.test.e": {"unique_id":"exposure.test.e","name":"e"}},
            "metrics": {"metric.test.met": {"unique_id":"metric.test.met","name":"met"}},
            "semantic_models": {"semantic_model.test.sm": {"unique_id":"semantic_model.test.sm","name":"sm"}},
            "saved_queries": {"saved_query.test.sq": {"unique_id":"saved_query.test.sq","name":"sq"}},
            "unit_tests": {"unit_test.test.ut": {"unique_id":"unit_test.test.ut","name":"ut"}}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);
        assert_eq!(graph.node_count(), 7);
    }

    #[test]
    fn test_build_graph_with_source_dependency() {
        let json = r#"{
            "nodes": {
                "model.test.m": {"unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test",
                                 "depends_on":{"nodes":["source.test.raw.tbl"]}}
            },
            "sources": {"source.test.raw.tbl": {"unique_id":"source.test.raw.tbl","source_name":"raw","name":"tbl","package_name":"test"}}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);

        assert_eq!(graph.node_count(), 2);
        let ancestors = graph.ancestors("model.test.m", None);
        assert!(ancestors.contains("source.test.raw.tbl"));
    }

    #[test]
    fn test_build_graph_with_exposure_dependencies() {
        let json = r#"{
            "nodes": {"model.test.m": {"unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test"}},
            "sources": {},
            "exposures": {"exposure.test.e": {"unique_id":"exposure.test.e","name":"e","depends_on":{"nodes":["model.test.m"]}}}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph
            .ancestors("exposure.test.e", None)
            .contains("model.test.m"));
    }
}
