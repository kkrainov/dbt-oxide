use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashMap, HashSet, VecDeque};

#[cfg_attr(not(feature = "extension-module"), allow(dead_code))]
const PARENT_TEST_EDGE: &str = "parent_test";

#[cfg_attr(not(feature = "extension-module"), allow(dead_code))]
#[derive(Clone)]
pub struct OxideGraph {
    graph: StableDiGraph<String, String>,
    node_map: HashMap<String, NodeIndex>,
}

#[cfg_attr(not(feature = "extension-module"), allow(dead_code))]
impl OxideGraph {
    pub fn new() -> Self {
        OxideGraph {
            graph: StableDiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    // Returns true if the edge represents a data dependency (should be traversed)
    fn is_data_edge(&self, edge_weight: &str) -> bool {
        edge_weight != PARENT_TEST_EDGE
    }

    pub fn add_node(&mut self, id: String) -> String {
        if !self.node_map.contains_key(&id) {
            let idx = self.graph.add_node(id.clone());
            self.node_map.insert(id.clone(), idx);
        }
        id
    }

    pub fn add_edge(
        &mut self,
        source: &str,
        target: &str,
        edge_type: Option<String>,
    ) -> Result<(), String> {
        let source_idx = if let Some(idx) = self.node_map.get(source) {
            *idx
        } else {
            let idx = self.graph.add_node(source.to_string());
            self.node_map.insert(source.to_string(), idx);
            idx
        };

        let target_idx = if let Some(idx) = self.node_map.get(target) {
            *idx
        } else {
            let idx = self.graph.add_node(target.to_string());
            self.node_map.insert(target.to_string(), idx);
            idx
        };

        if let Some(edge) = self.graph.find_edge(source_idx, target_idx) {
            if let Some(w) = self.graph.edge_weight_mut(edge) {
                *w = edge_type.unwrap_or_default();
            }
        } else {
            self.graph
                .add_edge(source_idx, target_idx, edge_type.unwrap_or_default());
        }
        Ok(())
    }

    pub fn remove_node(&mut self, node: &str) {
        if let Some(idx) = self.node_map.remove(node) {
            self.graph.remove_node(idx);
        }
    }

    pub fn nodes(&self) -> HashSet<String> {
        self.node_map.keys().cloned().collect()
    }

    pub fn edges(&self) -> Vec<(String, String)> {
        self.graph
            .edge_indices()
            .filter_map(|edge_idx| {
                let (source_idx, target_idx) = self.graph.edge_endpoints(edge_idx)?;
                let source = self.graph.node_weight(source_idx)?.clone();
                let target = self.graph.node_weight(target_idx)?.clone();
                Some((source, target))
            })
            .collect()
    }

    pub fn in_degree(&self, node: &str) -> Option<usize> {
        self.node_map
            .get(node)
            .map(|idx| self.graph.edges_directed(*idx, Direction::Incoming).count())
    }

    pub fn out_degree(&self, node: &str) -> Option<usize> {
        self.node_map
            .get(node)
            .map(|idx| self.graph.edges_directed(*idx, Direction::Outgoing).count())
    }

    pub fn successors(&self, node: &str) -> HashSet<String> {
        self.get_neighbors(node, Direction::Outgoing)
    }

    pub fn predecessors(&self, node: &str) -> HashSet<String> {
        self.get_neighbors(node, Direction::Incoming)
    }

    pub fn find_cycle(&self) -> Option<Vec<(String, String)>> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut path = Vec::new();

        for node_idx in self.graph.node_indices() {
            if self.detect_cycle_dfs(node_idx, &mut visited, &mut recursion_stack, &mut path) {
                // Construct the cycle from path
                // The path contains the DFS traversal. The cycle is the suffix starting from the repeated node.
                let last = path.last().unwrap();
                let cycle_start_pos = path.iter().position(|x| x == last).unwrap();

                let mut cycle_edges = Vec::new();
                for i in cycle_start_pos..path.len() - 1 {
                    let u = self.graph.node_weight(path[i]).unwrap().clone();
                    let v = self.graph.node_weight(path[i + 1]).unwrap().clone();
                    cycle_edges.push((u, v));
                }

                return Some(cycle_edges);
            }
        }
        None
    }

    fn detect_cycle_dfs(
        &self,
        node_idx: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        recursion_stack: &mut HashSet<NodeIndex>,
        path: &mut Vec<NodeIndex>,
    ) -> bool {
        visited.insert(node_idx);
        recursion_stack.insert(node_idx);
        path.push(node_idx);

        for neighbor in self.graph.neighbors_directed(node_idx, Direction::Outgoing) {
            if recursion_stack.contains(&neighbor) {
                path.push(neighbor);
                return true;
            }
            if !visited.contains(&neighbor)
                && self.detect_cycle_dfs(neighbor, visited, recursion_stack, path)
            {
                return true;
            }
        }

        path.pop();
        recursion_stack.remove(&node_idx);
        false
    }

    pub fn subgraph(&self, nodes: &HashSet<String>) -> OxideGraph {
        let mut new_graph = OxideGraph::new();

        for node in nodes {
            if self.node_map.contains_key(node) {
                new_graph.add_node(node.clone());
            }
        }

        for edge_idx in self.graph.edge_indices() {
            if let Some((source_idx, target_idx)) = self.graph.edge_endpoints(edge_idx) {
                if let (Some(source), Some(target)) = (
                    self.graph.node_weight(source_idx),
                    self.graph.node_weight(target_idx),
                ) {
                    if nodes.contains(source) && nodes.contains(target) {
                        if let Some(weight) = self.graph.edge_weight(edge_idx) {
                            let _ = new_graph.add_edge(source, target, Some(weight.clone()));
                        }
                    }
                }
            }
        }
        new_graph
    }

    pub fn get_subset_graph(&self, nodes: &HashSet<String>) -> OxideGraph {
        let mut new_graph = self.clone();

        let all_nodes: HashSet<String> = self.nodes();
        let to_remove: Vec<String> = all_nodes.difference(nodes).cloned().collect();

        for node in to_remove {
            let preds: Vec<String> = new_graph.predecessors(&node).into_iter().collect();
            let succs: Vec<String> = new_graph.successors(&node).into_iter().collect();

            for p in &preds {
                for s in &succs {
                    if p == s {
                        continue;
                    }
                    let _ = new_graph.add_edge(p, s, None);
                }
            }
            new_graph.remove_node(&node);
        }

        new_graph
    }

    fn get_neighbors(&self, node: &str, direction: Direction) -> HashSet<String> {
        let mut result = HashSet::new();
        if let Some(idx) = self.node_map.get(node) {
            for edge in self.graph.edges_directed(*idx, direction) {
                let neighbor_idx = match direction {
                    Direction::Outgoing => edge.target(),
                    Direction::Incoming => edge.source(),
                };
                if let Some(neighbor_id) = self.graph.node_weight(neighbor_idx) {
                    result.insert(neighbor_id.clone());
                }
            }
        }
        result
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn get_edge_weight(&self, source: &str, target: &str) -> Option<&String> {
        let s = self.node_map.get(source)?;
        let t = self.node_map.get(target)?;
        let e = self.graph.find_edge(*s, *t)?;
        self.graph.edge_weight(e)
    }

    pub fn descendants(&self, node: &str, limit: Option<usize>) -> HashSet<String> {
        self.bfs_traversal(node, Direction::Outgoing, limit)
    }

    pub fn ancestors(&self, node: &str, limit: Option<usize>) -> HashSet<String> {
        self.bfs_traversal(node, Direction::Incoming, limit)
    }

    pub fn select_children(
        &self,
        selected: &HashSet<String>,
        limit: Option<usize>,
    ) -> HashSet<String> {
        self.select_neighbors(selected, Direction::Outgoing, limit)
    }

    pub fn select_parents(
        &self,
        selected: &HashSet<String>,
        limit: Option<usize>,
    ) -> HashSet<String> {
        self.select_neighbors(selected, Direction::Incoming, limit)
    }

    fn select_neighbors(
        &self,
        selected: &HashSet<String>,
        direction: Direction,
        limit: Option<usize>,
    ) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Initialize queue with all selected nodes at depth 0
        for node_id in selected {
            if let Some(idx) = self.node_map.get(node_id) {
                visited.insert(*idx);
                queue.push_back((*idx, 0));
            }
        }

        while let Some((current_idx, depth)) = queue.pop_front() {
            if let Some(l) = limit {
                if depth >= l {
                    continue;
                }
            }

            let edges = match direction {
                Direction::Outgoing => self.graph.edges_directed(current_idx, Direction::Outgoing),
                Direction::Incoming => self.graph.edges_directed(current_idx, Direction::Incoming),
            };

            for edge in edges {
                if !self.is_data_edge(edge.weight()) {
                    continue;
                }

                let neighbor_idx = match direction {
                    Direction::Outgoing => edge.target(),
                    Direction::Incoming => edge.source(),
                };

                if visited.insert(neighbor_idx) {
                    if let Some(neighbor_id) = self.graph.node_weight(neighbor_idx) {
                        result.insert(neighbor_id.clone());
                    }
                    queue.push_back((neighbor_idx, depth + 1));
                }
            }
        }
        result
    }

    pub fn topological_sort_grouped(&self) -> Result<Vec<Vec<String>>, String> {
        let mut in_degree: HashMap<NodeIndex, usize> = HashMap::new();
        let mut queue: Vec<NodeIndex> = Vec::new();
        let mut processed_count = 0;

        // 1. Calculate in-degrees
        for node_idx in self.graph.node_indices() {
            let degree = self
                .graph
                .edges_directed(node_idx, Direction::Incoming)
                .count();
            in_degree.insert(node_idx, degree);
            if degree == 0 {
                queue.push(node_idx);
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();

        while !queue.is_empty() {
            // Sort queue to ensure deterministic output for nodes at the same level
            queue.sort_by(|a, b| {
                let node_a = self.graph.node_weight(*a);
                let node_b = self.graph.node_weight(*b);
                match (node_a, node_b) {
                    (Some(na), Some(nb)) => na.cmp(nb),
                    _ => std::cmp::Ordering::Equal,
                }
            });

            let mut current_level_nodes: Vec<String> = Vec::new();
            let mut next_queue: Vec<NodeIndex> = Vec::new();

            for node_idx in &queue {
                processed_count += 1;

                if let Some(id) = self.graph.node_weight(*node_idx) {
                    current_level_nodes.push(id.clone());
                }

                for edge in self.graph.edges_directed(*node_idx, Direction::Outgoing) {
                    let neighbor_idx = edge.target();
                    if let Some(degree) = in_degree.get_mut(&neighbor_idx) {
                        *degree -= 1;
                        if *degree == 0 {
                            next_queue.push(neighbor_idx);
                        }
                    }
                }
            }

            result.push(current_level_nodes);
            queue = next_queue;
        }

        if processed_count != self.graph.node_count() {
            return Err("Cycle detected in graph".to_string());
        }

        Ok(result)
    }

    fn bfs_traversal(
        &self,
        start_node: &str,
        direction: Direction,
        limit: Option<usize>,
    ) -> HashSet<String> {
        let mut result = HashSet::new();
        let start_index = match self.node_map.get(start_node) {
            Some(idx) => *idx,
            None => return result,
        };

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start_index);
        queue.push_back((start_index, 0));

        while let Some((current_idx, depth)) = queue.pop_front() {
            if let Some(l) = limit {
                if depth >= l {
                    continue;
                }
            }

            let edges = match direction {
                Direction::Outgoing => self.graph.edges_directed(current_idx, Direction::Outgoing),
                Direction::Incoming => self.graph.edges_directed(current_idx, Direction::Incoming),
            };

            for edge in edges {
                if !self.is_data_edge(edge.weight()) {
                    continue;
                }

                let neighbor_idx = match direction {
                    Direction::Outgoing => edge.target(),
                    Direction::Incoming => edge.source(),
                };

                if visited.insert(neighbor_idx) {
                    if let Some(node_id) = self.graph.node_weight(neighbor_idx) {
                        result.insert(node_id.clone());
                    }
                    queue.push_back((neighbor_idx, depth + 1));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_node_idempotency() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("A".to_string());

        assert_eq!(g.node_count(), 1);
    }

    #[test]
    fn test_add_edge_no_parallel() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());

        g.add_edge("A", "B", Some("type1".to_string())).unwrap();
        g.add_edge("A", "B", Some("type2".to_string())).unwrap();

        assert_eq!(g.edge_count(), 1);
        assert_eq!(g.get_edge_weight("A", "B").unwrap(), "type2");
    }

    #[test]
    fn test_add_edge_missing_node() {
        let mut g = OxideGraph::new();
        // Don't add nodes A or B explicitly

        let res = g.add_edge("A", "B", None);
        assert!(res.is_ok());

        assert!(g.node_map.contains_key("A"));
        assert!(g.node_map.contains_key("B"));
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn test_descendants_filtering() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());

        // A -> B (data dependency)
        // B -> C (test dependency - should be ignored)
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "C", Some("parent_test".to_string()))
            .unwrap();

        let descendants = g.descendants("A", None);
        assert!(descendants.contains("B"));
        assert!(!descendants.contains("C")); // Should stop at B
    }

    #[test]
    fn test_ancestors_filtering() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());

        // A -> B (test dependency - ignored)
        // B -> C (data dependency)
        g.add_edge("A", "B", Some("parent_test".to_string()))
            .unwrap();
        g.add_edge("B", "C", None).unwrap();

        let ancestors = g.ancestors("C", None);
        assert!(ancestors.contains("B"));
        assert!(!ancestors.contains("A")); // Should stop at B
    }

    #[test]
    fn test_select_children() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());

        g.add_edge("A", "B", None).unwrap();
        g.add_edge("A", "C", Some("parent_test".to_string()))
            .unwrap();

        let mut selected = HashSet::new();
        selected.insert("A".to_string());

        let children = g.select_children(&selected, None);
        assert!(children.contains("B"));
        assert!(!children.contains("C")); // Filtered out
    }

    #[test]
    fn test_select_parents() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());

        g.add_edge("A", "B", None).unwrap();
        g.add_edge("C", "B", Some("parent_test".to_string()))
            .unwrap();

        let mut selected = HashSet::new();
        selected.insert("B".to_string());

        let parents = g.select_parents(&selected, None);
        assert!(parents.contains("A"));
        assert!(!parents.contains("C")); // Filtered out
    }

    #[test]
    fn test_descendants_depth_limit() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());

        // A -> B -> C
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "C", None).unwrap();

        // Limit 1: Should find B, but NOT C
        let descendants = g.descendants("A", Some(1));
        assert!(descendants.contains("B"));
        assert!(!descendants.contains("C"));
    }

    #[test]
    fn test_select_children_infinite() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());

        // A -> B -> C
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "C", None).unwrap();

        let mut selected = HashSet::new();
        selected.insert("A".to_string());

        // Limit None: Should find B AND C
        let children = g.select_children(&selected, None);
        assert!(children.contains("B"));
        assert!(children.contains("C"));
    }

    #[test]
    fn test_topo_sort_simple() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_edge("A", "B", None).unwrap();

        let result = g.topological_sort_grouped().unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["A".to_string()]);
        assert_eq!(result[1], vec!["B".to_string()]);
    }

    #[test]
    fn test_topo_sort_islands() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());
        g.add_node("D".to_string());

        // A -> B
        // C -> D
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("C", "D", None).unwrap();

        let result = g.topological_sort_grouped().unwrap();
        assert_eq!(result.len(), 2);

        let level0: HashSet<_> = result[0].iter().cloned().collect();
        assert!(level0.contains("A"));
        assert!(level0.contains("C"));

        let level1: HashSet<_> = result[1].iter().cloned().collect();
        assert!(level1.contains("B"));
        assert!(level1.contains("D"));
    }

    #[test]
    fn test_topo_sort_cycle() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "A", None).unwrap();

        let result = g.topological_sort_grouped();
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Cycle detected in graph".to_string()));
    }

    #[test]
    fn test_remove_node() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_edge("A", "B", None).unwrap();

        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);

        g.remove_node("A");

        assert_eq!(g.node_count(), 1);
        assert_eq!(g.edge_count(), 0);
        assert!(g.node_map.get("A").is_none());
    }

    #[test]
    fn test_basic_inspection() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "C", None).unwrap();

        let nodes = g.nodes();
        assert_eq!(nodes.len(), 3);
        assert!(nodes.contains(&"A".to_string()));

        let edges = g.edges();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&("A".to_string(), "B".to_string())));

        assert_eq!(g.in_degree("B"), Some(1));
        assert_eq!(g.out_degree("B"), Some(1));
        assert_eq!(g.in_degree("A"), Some(0));
        assert_eq!(g.out_degree("A"), Some(1));
    }

    #[test]
    fn test_neighbors() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("A", "C", None).unwrap();

        let successors = g.successors("A");
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&"B".to_string()));
        assert!(successors.contains(&"C".to_string()));

        let predecessors = g.predecessors("B");
        assert_eq!(predecessors.len(), 1);
        assert!(predecessors.contains(&"A".to_string()));
    }

    #[test]
    fn test_subgraph() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "C", None).unwrap();

        let mut nodes = std::collections::HashSet::new();
        nodes.insert("A".to_string());
        nodes.insert("C".to_string());

        let sub = g.subgraph(&nodes);
        assert_eq!(sub.node_count(), 2);
        assert!(sub.get_edge_weight("A", "C").is_none());
    }

    #[test]
    fn test_get_subset_graph_transitive() {
        let mut g = OxideGraph::new();
        g.add_node("A".to_string());
        g.add_node("B".to_string());
        g.add_node("C".to_string());
        g.add_edge("A", "B", None).unwrap();
        g.add_edge("B", "C", None).unwrap();

        let mut nodes = std::collections::HashSet::new();
        nodes.insert("A".to_string());
        nodes.insert("C".to_string());

        let sub = g.get_subset_graph(&nodes);
        assert_eq!(sub.node_count(), 2);
        // Should preserve A -> C via B
        assert!(sub.get_edge_weight("A", "C").is_some());
    }
}
