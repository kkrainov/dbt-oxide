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
        for dep_id in &node.depends_on().nodes {
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
                "model.test.a": {
                    "unique_id":"model.test.a","name":"a","resource_type":"model","package_name":"test",
                    "path":"a.sql","original_file_path":"models/a.sql","schema":"test","alias":"a",
                    "fqn":["test", "a"],
                    "checksum":{"name":"sha256","checksum":"abc"}
                },
                "model.test.b": {
                    "unique_id":"model.test.b","name":"b","resource_type":"model","package_name":"test",
                    "path":"b.sql","original_file_path":"models/b.sql","schema":"test","alias":"b",
                    "fqn":["test", "b"],
                    "checksum":{"name":"sha256","checksum":"abc"},
                    "depends_on":{"nodes":["model.test.a"]}
                }
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
            "nodes": {"model.test.m": {
                "unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test",
                "path":"m.sql","original_file_path":"models/m.sql","schema":"test","alias":"m",
                "fqn":["test", "m"],
                "checksum":{"name":"sha256","checksum":"abc"}
            }},
            "sources": {"source.test.raw.tbl": {"unique_id":"source.test.raw.tbl","source_name":"raw","name":"tbl","package_name":"test","original_file_path":"sources.yml","resource_type":"source","schema":"test","identifier":"tbl","path":"sources.yml","fqn":["test", "raw", "tbl"]}},
            "exposures": {"exposure.test.e": {"unique_id":"exposure.test.e","name":"e","resource_type":"exposure","type":"dashboard","owner":{"email":"e","name":"n"},"package_name":"test","path":"e.yml","original_file_path":"models/e.yml","description":"desc","fqn":["test", "e"]}},
            "metrics": {"metric.test.met": {"unique_id":"metric.test.met","name":"met","resource_type":"metric","type":"simple","label":"Metric","package_name":"test","path":"m.yml","original_file_path":"models/m.yml","description":"desc","fqn":["test", "met"]}},
            "semantic_models": {"semantic_model.test.sm": {"unique_id":"semantic_model.test.sm","name":"sm","resource_type":"semantic_model","package_name":"test","path":"sm.yml","original_file_path":"models/sm.yml","model":"ref('m')","description":"desc"}},
            "saved_queries": {"saved_query.test.sq": {"unique_id":"saved_query.test.sq","name":"sq","resource_type":"saved_query","package_name":"test","path":"sq.yml","original_file_path":"models/sq.yml","label":"SQ","description":"desc"}},
            "unit_tests": {"unit_test.test.ut": {"unique_id":"unit_test.test.ut","name":"ut","resource_type":"unit_test","package_name":"test","path":"ut.yml","original_file_path":"tests/ut.yml","model":"ref('m')","schema":"test"}}
        }"#;
        let manifest = OxideManifest::from_json_str(json).unwrap();
        let graph = build_graph_from_manifest(&manifest);
        assert_eq!(graph.node_count(), 7);
    }

    #[test]
    fn test_build_graph_with_source_dependency() {
        let json = r#"{
            "nodes": {
                "model.test.m": {
                    "unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test",
                    "path":"m.sql","original_file_path":"models/m.sql","schema":"test","alias":"m",
                    "fqn":["test", "m"],
                    "checksum":{"name":"sha256","checksum":"abc"},
                    "depends_on":{"nodes":["source.test.raw.tbl"]}
                }
            },
            "sources": {"source.test.raw.tbl": {"unique_id":"source.test.raw.tbl","source_name":"raw","name":"tbl","package_name":"test","original_file_path":"sources.yml","resource_type":"source","schema":"test","identifier":"tbl","path":"sources.yml","fqn":["test", "raw", "tbl"]}}
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
            "nodes": {
                "model.test.m": {
                    "unique_id":"model.test.m","name":"m","resource_type":"model","package_name":"test",
                    "path":"m.sql","original_file_path":"models/m.sql","schema":"test","alias":"m",
                    "fqn":["test", "m"],
                    "checksum":{"name":"sha256","checksum":"abc"}
                }
            },
            "sources": {},
            "exposures": {"exposure.test.e": {"unique_id":"exposure.test.e","name":"e","resource_type":"exposure","type":"dashboard","owner":{"email":"e","name":"n"},"package_name":"test","path":"e.yml","original_file_path":"models/e.yml","description":"desc","fqn":["test", "e"],"depends_on":{"nodes":["model.test.m"]}}}
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
