# Phase 1: The Graph Engine - Implementation Plan

**Objective:** Replace the `networkx` graph implementation with a high-performance Rust engine using `petgraph`.
**Constraint:** Strict Test-Driven Development (TDD) for all Rust components.
**Performance Goal:** 100x improvement in topological sorting and cycle detection.

## Architecture: The Strangler Pattern
We initially implemented a "Shadow Graph" strategy where the Rust `DbtGraph` existed alongside the legacy Python `networkx` graph.
- **Python:** Handled API compatibility and legacy behavior.
- **Rust:** Acceleration layer for hot paths (traversal, sorting).

**Phase 1.5 Update:** We are now transitioning to **Exclusive Mode**. The Rust engine is the *only* implementation for graph algorithms. The Python implementation is removed, though the NetworkX object remains for external API compatibility.

## Sub-Phase 1: Rust Implementation (`src/dbt_rs`)

### Step 1.1: Foundation & Skeleton
*   **Goal:** Establish the `DbtGraph` struct and compilation target.
*   **Actions:**
    1.  Add `petgraph = "0.6"` to `src/dbt_rs/Cargo.toml`.
    2.  Create `src/dbt_rs/src/graph.rs`.
    3.  Define `struct DbtGraph` with:
        - `graph: StableDiGraph<String, String>`
        - `node_map: HashMap<String, NodeIndex>`
    4.  Expose `DbtGraph` class in `lib.rs` via PyO3.

### Step 1.2: Core Operations (TDD)
*   **Goal:** Implement safe node and edge management that mimics `networkx` behavior.
*   **Tests (Rust):**
    - `test_add_node_idempotency`: Adding the same node ID twice returns the same `NodeIndex` and doesn't grow the graph.
    - `test_add_edge_no_parallel`: Adding `A->B` twice must result in a single edge (update weight or no-op).
    - `test_add_edge_missing_node`: Adding an edge for non-existent nodes must implicitly create them (NetworkX parity).
*   **Implementation:**
    - `add_node(id: String)`
    - `add_edge(source: String, target: String, type: String)`: implicitly creates nodes if missing.

### Step 1.3: Traversal Algorithms (TDD)
*   **Goal:** High-performance BFS traversals with edge filtering.
*   **Tests (Rust):**
    - `test_ancestors_linear`: `A->B->C`. `ancestors(C)` should be `{A, B}`.
    - `test_descendants_branching`: `A->B, A->C`. `descendants(A)` should be `{B, C}`.
    - `test_filtering_parent_test`: `A -[parent_test]-> B`. `descendants(A)` should be empty. `ancestors(B)` should be empty.
*   **Implementation:**
    - `ancestors(node)`: Reverse BFS.
    - `descendants(node)`: Forward BFS.
    - **Constraint:** Both must ignore edges where weight == "parent_test".

### Step 1.4: Selection Algorithms (TDD)
*   **Goal:** Optimized immediate neighbor selection.
*   **Tests (Rust):**
    - `test_select_children`: `A->B`. `select_children([A])` -> `{B}`.
    - `test_select_children_filtered`: `A -[parent_test]-> B`. `select_children([A])` -> `{}`.
    - `test_select_parents`: `A->B`. `select_parents([B])` -> `{A}`.
*   **Implementation:**
    - `select_children(nodes)`
    - `select_parents(nodes)`

### Step 1.5: Topological Sort (TDD)
*   **Goal:** Grouped topological sort that handles disconnected subgraphs (islands).
*   **Tests (Rust):**
    - `test_topo_sort_simple`: `A->B`. Result: `[[A], [B]]`.
    - `test_topo_sort_islands`: `A->B`, `C->D`. Result: Level 0 `{A, C}`, Level 1 `{B, D}`.
    - `test_topo_sort_cycle`: `A->B->A`. Must return specific Error.
*   **Implementation:**
    - `topological_sort_grouped()`: Implement Kahn's Algorithm manually to support "grouping" by level.

## Sub-Phase 2: Python Integration (`core/dbt`)

### Step 2.1: Wrapper & Loading
*   **Goal:** Initialize the Rust graph inside the Python Graph object.
*   **Actions:**
    - Modify `core/dbt/graph/graph.py`.
    - In `__init__`:
        - Initialize `self._rust_graph = dbt_rs.DbtGraph()`.
        - **Bulk Load:** Iterate `nx_graph` and populate Rust graph.

### Step 2.2: Method Routing
*   **Goal:** Delegate hot methods to Rust.
*   **Actions:**
    - Update `ancestors`, `descendants`, `select_children`, `select_parents`.
    - Add logic: `if self._rust_graph: return self._rust_graph.method(...)`.

### Step 2.3: Queue Acceleration
*   **Goal:** The primary performance unlock (100x sort).
*   **Actions:**
    - Modify `core/dbt/graph/queue.py`.
    - In `_get_scores`: Call `self.graph._rust_graph.topological_sort_grouped()`.
    - Transform Rust `Vec<Vec<String>>` into the required score dictionary.

### Step 2.4: Validation & Verification
*   **Goal:** Ensure correctness and measure performance.
*   **Actions:**
    - **Unit Tests:** Run `uv run pytest tests/unit/test_graph.py` (and related) to ensure no regressions in standard logic.
    - **Performance Benchmark:**
        - **Baseline:** Run `performance/runner` with `DBT_OXIDE_DISABLED=1`.
        - **Oxide:** Run `performance/runner` with Rust graph enabled.
        - **Compare:** Verify speedup in compilation/loading phases.

## Sub-Phase 3: Rust Exclusivity (Phase 1.5)

### Step 3.1: Enforce Rust Availability
*   **Goal:** Make `dbt-oxide` strictly dependent on the Rust extension for graph logic.
*   **Actions:**
    - Modify `core/dbt/graph/graph.py`:
        - Raise `DbtRuntimeError` if `dbt_rs` is missing (unless explicitly disabled).
        - Remove Python fallback logic from `ancestors`, `descendants`, etc.

### Step 3.2: Clean Queue Logic
*   **Goal:** Remove legacy Python topological sort.
*   **Actions:**
    - Modify `core/dbt/graph/queue.py`:
        - Delete `_grouped_topological_sort`.
        - Remove fallback logic in `_get_scores`.

### Step 3.3: Final Verification
*   **Goal:** Ensure `dbt-core` still functions correctly without the Python safety net.
*   **Actions:**
    - Run unit tests: `uv run pytest tests/unit/graph`.
    - Run integration: `dbt parse` on a sample project.

## Sub-Phase 4: Complete NetworkX Removal

### Step 4.1: Structural Parity (Rust TDD)
*   **Goal:** Implement graph mutation and inspection methods required by `Linker` and `Selector` to fully replace `networkx`.
*   **Tests (Rust):**
    - `test_graph_mutation`: `add_node`, `remove_node` verifies node count and connectivity.
    - `test_degrees`: `in_degree`, `out_degree` matches expected counts.
    - `test_edges_iteration`: `edges()` returns all edges.
    - `test_subgraph`: Creating a subgraph retains correct nodes/edges.
    - `test_cycle_detection`: `find_cycles` identifies loops correctly.
    - `test_transitive_reduction`: `get_subset_graph` correctly preserves transitive edges when removing nodes.
*   **Implementation:**
    - `remove_node(id: String)`: Remove node and associated edges.
    - `edges()`, `nodes()`: Return iterators/vectors.
    - `in_degree(id)`, `out_degree(id)`.
    - `successors(id)`, `predecessors(id)`.
    - `find_cycles()`: detect loops.
    - `subgraph(nodes)`: Create a new graph with only selected nodes.
    - `get_subset_graph(nodes)`: Implement efficient $O(N)$ transitive closure preserving subgraph generation (replaces slow Python logic).

### Step 4.2: Linker & Compiler Refactor
*   **Goal:** Construct the Rust graph directly during compilation.
*   **Actions:**
    - Modify `core/dbt/compilation.py`:
        - `Linker.__init__` initializes `dbt_rs.DbtGraph`.
        - Update `dependency`, `add_node`, `link_node` to call Rust methods.
        - Update `find_cycles` to use Rust.
        - Update `add_test_edges` to use Rust traversal/methods.

### Step 4.3: Graph Wrapper & Queue Refactor
*   **Goal:** Update consumers to use `DbtGraph` exclusively.
*   **Actions:**
    - [x] Modify `core/dbt/graph/graph.py`:
        - Remove `import networkx` and `try-except` for `dbt_rs` (make it mandatory).
        - Type hint `self.graph` as `DbtGraph` exclusively.
        - Remove `_rust_graph` attribute logic (the graph IS the rust graph).
        - Delegate `get_subset_graph`, `subgraph`, `get_dependent_nodes` to Rust.
        - Refactor `exclude_edge_type` to assume Rust graph behavior.
        - **Refactor: Encapsulate `dbt_rs.DbtGraph` entirely within `dbt.graph.Graph`.**
    - [x] Modify `core/dbt/graph/queue.py`:
        - Remove `import networkx`.
        - Update `__init__` to accept `dbt.graph.Graph` (Wrapper).
        - Refactor `preserve_edges=False` logic to create new `Graph` copy.
        - Remove `_get_scores` fallback logic.
    - [x] Inspect and fix `core/dbt/graph/selector.py` if it relies on NetworkX behavior of the `Graph` object.

### Step 4.4: Test Suite Updates
*   **Goal:** Fix unit tests that rely on `nx.DiGraph`.
*   **Actions:**
    - [x] Refactor `tests/unit/graph/test_graph.py` and other tests.
    - [x] Create `dbt_rs` test helpers for graph construction.

### Step 4.5: Deprecation
*   **Goal:** Cleanup.
*   **Actions:**
    - [x] Remove `import networkx` from graph modules.
    - [x] Verify `dbt` runs without `networkx` in hot paths.
