//! Max-Flow/Min-Cut algorithm for graph-cut optimization
//!
//! Implements Edmonds-Karp algorithm (Ford-Fulkerson with BFS)
//! for computing maximum flow and minimum cut in a flow network.
//!
//! Used by MRF biome generator for alpha-expansion optimization.

use std::collections::VecDeque;

/// Flow capacity type
type Capacity = f64;

/// Flow network for max-flow/min-cut computation
pub struct FlowNetwork {
    num_nodes: usize,
    /// Adjacency list: edges[i] = list of (neighbor, capacity, reverse_edge_index)
    edges: Vec<Vec<FlowEdge>>,
    /// Source node index
    source: usize,
    /// Sink node index
    sink: usize,
}

#[derive(Clone, Debug)]
struct FlowEdge {
    to: usize,
    capacity: Capacity,
    flow: Capacity,
    reverse: usize, // Index of reverse edge in edges[to]
}

impl FlowNetwork {
    /// Create new flow network
    ///
    /// # Arguments
    /// * `num_nodes` - Total number of nodes (including source and sink)
    /// * `source` - Index of source node
    /// * `sink` - Index of sink node
    pub fn new(num_nodes: usize, source: usize, sink: usize) -> Self {
        Self {
            num_nodes,
            edges: vec![Vec::new(); num_nodes],
            source,
            sink,
        }
    }

    /// Add directed edge from u to v with given capacity
    ///
    /// Also creates reverse edge with 0 capacity for residual graph
    pub fn add_edge(&mut self, from: usize, to: usize, capacity: Capacity) {
        let forward_idx = self.edges[to].len();
        let reverse_idx = self.edges[from].len();

        // Forward edge
        self.edges[from].push(FlowEdge {
            to,
            capacity,
            flow: 0.0,
            reverse: forward_idx,
        });

        // Reverse edge (for residual graph)
        self.edges[to].push(FlowEdge {
            to: from,
            capacity: 0.0,
            flow: 0.0,
            reverse: reverse_idx,
        });
    }

    /// Compute maximum flow from source to sink using Edmonds-Karp algorithm
    ///
    /// Returns the maximum flow value
    pub fn max_flow(&mut self) -> Capacity {
        let mut total_flow = 0.0;

        // Repeatedly find augmenting paths using BFS
        loop {
            // Find augmenting path
            let path = self.find_augmenting_path();

            if path.is_empty() {
                break; // No more augmenting paths
            }

            // Find minimum capacity along path (bottleneck)
            let mut bottleneck = f64::INFINITY;
            for &(from, edge_idx) in &path {
                let edge = &self.edges[from][edge_idx];
                let residual = edge.capacity - edge.flow;
                bottleneck = bottleneck.min(residual);
            }

            // Augment flow along path
            for &(from, edge_idx) in &path {
                // Forward edge: increase flow
                self.edges[from][edge_idx].flow += bottleneck;

                // Reverse edge: decrease flow
                let to = self.edges[from][edge_idx].to;
                let reverse_idx = self.edges[from][edge_idx].reverse;
                self.edges[to][reverse_idx].flow -= bottleneck;
            }

            total_flow += bottleneck;
        }

        total_flow
    }

    /// Find augmenting path from source to sink using BFS
    ///
    /// Returns path as list of (node, edge_index) pairs
    fn find_augmenting_path(&self) -> Vec<(usize, usize)> {
        let mut parent = vec![None; self.num_nodes];
        let mut visited = vec![false; self.num_nodes];
        let mut queue = VecDeque::new();

        queue.push_back(self.source);
        visited[self.source] = true;

        // BFS to find path
        while let Some(node) = queue.pop_front() {
            if node == self.sink {
                break; // Found path to sink
            }

            for (edge_idx, edge) in self.edges[node].iter().enumerate() {
                let residual = edge.capacity - edge.flow;

                if !visited[edge.to] && residual > 0.0 {
                    visited[edge.to] = true;
                    parent[edge.to] = Some((node, edge_idx));
                    queue.push_back(edge.to);
                }
            }
        }

        // Reconstruct path from sink to source
        if !visited[self.sink] {
            return Vec::new(); // No path found
        }

        let mut path = Vec::new();
        let mut current = self.sink;

        while let Some((prev, edge_idx)) = parent[current] {
            path.push((prev, edge_idx));
            current = prev;
        }

        path.reverse();
        path
    }

    /// Compute minimum cut (returns nodes reachable from source)
    ///
    /// Must be called after max_flow()
    pub fn min_cut(&self) -> Vec<bool> {
        let mut reachable = vec![false; self.num_nodes];
        let mut queue = VecDeque::new();

        queue.push_back(self.source);
        reachable[self.source] = true;

        // BFS in residual graph
        while let Some(node) = queue.pop_front() {
            for edge in &self.edges[node] {
                let residual = edge.capacity - edge.flow;

                if !reachable[edge.to] && residual > 0.0 {
                    reachable[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }

        reachable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_flow_network() {
        // Simple network: S -> A -> T
        //                  |    |
        //                  v    v
        //                  B -> T
        // Capacities: S->A=10, S->B=10, A->T=10, B->T=10, A->B=2

        let mut network = FlowNetwork::new(4, 0, 3);
        let s = 0;
        let a = 1;
        let b = 2;
        let t = 3;

        network.add_edge(s, a, 10.0);
        network.add_edge(s, b, 10.0);
        network.add_edge(a, b, 2.0);
        network.add_edge(a, t, 10.0);
        network.add_edge(b, t, 10.0);

        let max_flow = network.max_flow();
        assert_eq!(max_flow, 20.0); // Total flow = min(10+10, 10+10) = 20
    }

    #[test]
    fn test_min_cut() {
        // Network with bottleneck
        let mut network = FlowNetwork::new(4, 0, 3);
        let s = 0;
        let a = 1;
        let b = 2;
        let t = 3;

        // S -> A (capacity 100)
        // A -> B (capacity 1) <-- bottleneck
        // B -> T (capacity 100)
        network.add_edge(s, a, 100.0);
        network.add_edge(a, b, 1.0);
        network.add_edge(b, t, 100.0);

        let max_flow = network.max_flow();
        assert_eq!(max_flow, 1.0);

        let cut = network.min_cut();
        assert!(cut[s]); // Source reachable
        assert!(cut[a]); // A reachable
        assert!(!cut[b]); // B not reachable (cut between A and B)
        assert!(!cut[t]); // Sink not reachable
    }

    #[test]
    fn test_no_flow() {
        // Disconnected network
        let mut network = FlowNetwork::new(3, 0, 2);
        network.add_edge(0, 1, 10.0);
        // No edge from 1 to 2

        let max_flow = network.max_flow();
        assert_eq!(max_flow, 0.0);
    }

    #[test]
    fn test_multiple_paths() {
        // Two paths from S to T
        let mut network = FlowNetwork::new(5, 0, 4);
        let s = 0;
        let a = 1;
        let b = 2;
        let c = 3;
        let t = 4;

        // Path 1: S -> A -> T (capacity 5)
        network.add_edge(s, a, 5.0);
        network.add_edge(a, t, 5.0);

        // Path 2: S -> B -> C -> T (capacity 7)
        network.add_edge(s, b, 10.0);
        network.add_edge(b, c, 7.0);
        network.add_edge(c, t, 10.0);

        let max_flow = network.max_flow();
        assert_eq!(max_flow, 12.0); // 5 + 7 = 12
    }
}
