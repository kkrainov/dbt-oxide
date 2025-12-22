from typing import Iterable, Iterator, List, NewType, Optional, Set

from dbt_common.exceptions import DbtInternalError
import dbt_rs

UniqueId = NewType("UniqueId", str)


class EdgeView:
    def __init__(self, edges):
        self._edges = edges

    def __iter__(self):
        return iter(self._edges)

    def __len__(self):
        return len(self._edges)

    def __eq__(self, other):
        if isinstance(other, list):
            return list(self._edges) == other
        return set(self._edges) == other

    def __repr__(self):
        return repr(self._edges)


class Graph:
    """A wrapper around the rust DbtGraph that understands SelectionCriteria
    and how they interact with the graph.
    """

    def __init__(self, graph: dbt_rs.DbtGraph) -> None:
        if not isinstance(graph, dbt_rs.DbtGraph):
            raise DbtInternalError(
                f"Graph must be initialized with dbt_rs.DbtGraph, got {type(graph)}"
            )
        self.graph = graph

    @classmethod
    def empty(cls) -> "Graph":
        """Create an empty graph."""
        return cls(dbt_rs.DbtGraph())

    @classmethod
    def from_json(cls, json_str: str) -> "Graph":
        """Build graph from JSON manifest string."""
        rust_graph = dbt_rs.build_graph_from_manifest_json(json_str)
        return cls(rust_graph)

    def find_cycle(self):
        """Detect cycle in graph. Returns cycle path or None."""
        return self.graph.find_cycle()

    def nodes(self) -> Set[UniqueId]:
        return set(self.graph.nodes())

    def edges(self):
        return EdgeView(self.graph.edges())

    def __iter__(self) -> Iterator[UniqueId]:
        return iter(self.graph.nodes())

    def ancestors(self, node: UniqueId, max_depth: Optional[int] = None) -> Set[UniqueId]:
        """Returns all nodes having a path to `node` in `graph`"""
        return self.graph.ancestors(node, max_depth)

    def descendants(self, node: UniqueId, max_depth: Optional[int] = None) -> Set[UniqueId]:
        """Returns all nodes reachable from `node` in `graph`"""
        return self.graph.descendants(node, max_depth)

    def exclude_edge_type(self, edge_type_to_exclude):
        # DbtGraph internally handles ignoring 'parent_test' for traversals
        return self

    def select_childrens_parents(self, selected: Set[UniqueId]) -> Set[UniqueId]:
        ancestors_for = self.select_children(selected) | selected
        return self.select_parents(ancestors_for) | ancestors_for

    def select_children(
        self, selected: Set[UniqueId], max_depth: Optional[int] = None
    ) -> Set[UniqueId]:
        """Returns all nodes which are descendants of the 'selected' set.
        Nodes in the 'selected' set are counted as children only if
        they are descendants of other nodes in the 'selected' set."""
        return self.graph.select_children(selected, max_depth)

    def select_parents(
        self, selected: Set[UniqueId], max_depth: Optional[int] = None
    ) -> Set[UniqueId]:
        """Returns all nodes which are ancestors of the 'selected' set.
        Nodes in the 'selected' set are counted as parents only if
        they are ancestors of other nodes in the 'selected' set."""
        return self.graph.select_parents(selected, max_depth)

    def select_successors(self, selected: Set[UniqueId]) -> Set[UniqueId]:
        successors: Set[UniqueId] = set()
        for node in selected:
            successors.update(self.graph.successors(node))
        return successors

    def get_subset_graph(self, selected: Iterable[UniqueId]) -> "Graph":
        """Create and return a new graph that is a shallow copy of the graph,
        but with only the nodes in include_nodes. Transitive edges across
        removed nodes are preserved as explicit new edges.
        """
        return Graph(self.graph.get_subset_graph(set(selected)))

    def subgraph(self, nodes: Iterable[UniqueId]) -> "Graph":
        # Return a subgraph containing only the selected unique_id nodes.
        return Graph(self.graph.subgraph(set(nodes)))

    def get_dependent_nodes(self, node: UniqueId):
        return self.graph.descendants(node, None)

    def topological_sort_grouped(self) -> List[List[UniqueId]]:
        return self.graph.topological_sort_grouped()

    def add_node(self, node: UniqueId):
        self.graph.add_node(node)

    def add_edge(self, source: UniqueId, target: UniqueId, edge_type: Optional[str] = None):
        self.graph.add_edge(source, target, edge_type)

    def remove_node(self, node: UniqueId):
        self.graph.remove_node(node)

    def in_degree(self, node: UniqueId) -> int:
        return self.graph.in_degree(node)

    def successors(self, node: UniqueId) -> Set[UniqueId]:
        return self.graph.successors(node)

    def __len__(self):
        return self.graph.number_of_nodes()
