//! MRF (Markov Random Field) based biome generation using graph-cut optimization
//!
//! This module implements biome generation using energy minimization with:
//! - Data term: How well each biome fits local elevation/moisture
//! - Pairwise term: Neighbor compatibility between biomes
//! - Directional term: Geographic preferences (mountains north, wetlands south)
//!
//! Uses alpha-expansion algorithm for optimization, which guarantees convergence
//! and produces results within a factor of 2 of optimal for metric energies.

use super::{Biome, BiomeId, BiomeMap, get_core_biomes, Direction, FlowNetwork};
use noise::NoiseFn;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

/// Energy cost for assigning a biome to a cell (lower = better)
type EnergyCost = f64;

/// MRF-based biome generator using graph-cut optimization
pub struct MRFBiomeGenerator<N: NoiseFn<f64, 2>> {
    width: i32,
    height: i32,
    seed: u64,
    biomes: Vec<Biome>,
    biome_lookup: HashMap<BiomeId, Biome>,
    elevation_noise: N,
    moisture_noise: N,
}

impl<N: NoiseFn<f64, 2>> MRFBiomeGenerator<N> {
    pub fn new(width: i32, height: i32, seed: u64, elevation_noise: N, moisture_noise: N) -> Self {
        let biomes = get_core_biomes();
        let biome_lookup: HashMap<BiomeId, Biome> = biomes
            .iter()
            .map(|b| (b.id, b.clone()))
            .collect();

        Self {
            width,
            height,
            seed,
            biomes,
            biome_lookup,
            elevation_noise,
            moisture_noise,
        }
    }

    /// Generate biome map using alpha-expansion MRF optimization
    pub fn generate(&self) -> BiomeMap {
        println!("  [MRF] Initializing with random biomes...");
        let mut biome_map = self.initialize_random();

        println!("  [MRF] Running alpha-expansion optimization...");
        let mut iteration = 0;
        let max_iterations = 10;

        loop {
            iteration += 1;
            let mut changed = false;

            // Try expanding each biome label
            for biome in &self.biomes {
                if self.alpha_expansion(&mut biome_map, biome.id) {
                    changed = true;
                }
            }

            println!("  [MRF]   Iteration {}: {}", iteration,
                if changed { "Energy reduced" } else { "Converged" });

            if !changed || iteration >= max_iterations {
                break;
            }
        }

        println!("  [MRF] Optimization complete after {} iterations", iteration);
        biome_map
    }

    /// Initialize biome map with random weighted assignment
    fn initialize_random(&self) -> BiomeMap {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let size = (self.width * self.height) as usize;
        let mut biomes = Vec::with_capacity(size);
        let mut strengths = Vec::with_capacity(size);

        for _ in 0..(self.width * self.height) {
            let biome_id = self.select_random_biome(&mut rng);
            biomes.push(biome_id);
            strengths.push(1.0); // Full strength initially
        }

        BiomeMap {
            width: self.width,
            height: self.height,
            biomes,
            strengths,
        }
    }

    /// Select random biome using weights
    fn select_random_biome(&self, rng: &mut StdRng) -> BiomeId {
        let total_weight: u32 = self.biomes.iter().map(|b| b.weight).sum();
        let mut roll = rng.gen_range(0..total_weight);

        for biome in &self.biomes {
            if roll < biome.weight {
                return biome.id;
            }
            roll -= biome.weight;
        }

        self.biomes[0].id // Fallback
    }

    /// Alpha-expansion step: try expanding alpha to minimize energy
    ///
    /// Returns true if any labels changed (energy was reduced)
    fn alpha_expansion(&self, biome_map: &mut BiomeMap, alpha: BiomeId) -> bool {
        // Build graph for alpha-expansion
        // Nodes: each cell can be either current label or alpha
        // Edges: pairwise costs between neighbors

        let graph = self.build_alpha_expansion_graph(biome_map, alpha);
        let cut = self.compute_min_cut(&graph, alpha);

        // Apply cut: cells in source set get alpha, cells in sink set keep current label
        let mut changed = false;
        for (idx, &in_source) in cut.iter().enumerate() {
            if in_source && biome_map.biomes[idx] != alpha {
                biome_map.biomes[idx] = alpha;
                changed = true;
            }
        }

        changed
    }

    /// Build graph for alpha-expansion
    ///
    /// Graph structure:
    /// - Source node S connects to cells where alpha has low cost
    /// - Sink node T connects to cells where current label has low cost
    /// - Neighbor edges encode pairwise compatibility costs
    fn build_alpha_expansion_graph(&self, biome_map: &BiomeMap, alpha: BiomeId) -> AlphaExpansionGraph {
        let num_cells = (self.width * self.height) as usize;
        let mut graph = AlphaExpansionGraph::new(num_cells);

        // For each cell, add terminal edges (source/sink costs)
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);
                let current_label = biome_map.biomes[idx];

                // Cost of assigning alpha to this cell
                let alpha_cost = self.data_cost(x, y, alpha) +
                                 self.directional_cost(x, y, alpha);

                // Cost of keeping current label
                let current_cost = if current_label == alpha {
                    0.0 // Already alpha
                } else {
                    self.data_cost(x, y, current_label) +
                    self.directional_cost(x, y, current_label)
                };

                // Terminal edges
                graph.add_terminal_edge(idx, current_cost, alpha_cost);
            }
        }

        // For each pair of neighbors, add pairwise edge
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);
                let current_label = biome_map.biomes[idx];

                // Check right and down neighbors to avoid duplicates
                for (nx, ny) in [(x + 1, y), (x, y + 1)] {
                    if nx >= self.width || ny >= self.height {
                        continue;
                    }

                    let n_idx = self.xy_idx(nx, ny);
                    let neighbor_label = biome_map.biomes[n_idx];

                    // Add pairwise edge
                    let cost = self.compute_pairwise_edge_cost(
                        current_label,
                        neighbor_label,
                        alpha,
                    );

                    if cost > 0.0 {
                        graph.add_neighbor_edge(idx, n_idx, cost);
                    }
                }
            }
        }

        graph
    }

    /// Compute pairwise edge cost for alpha-expansion
    ///
    /// This encodes the cost of having different labels at neighboring cells
    fn compute_pairwise_edge_cost(
        &self,
        label_i: BiomeId,
        label_j: BiomeId,
        alpha: BiomeId,
    ) -> EnergyCost {
        // Case 1: Both cells keep current labels (both not alpha)
        // Case 2: One cell becomes alpha, other keeps current
        // Case 3: Both cells become alpha

        if label_i == alpha && label_j == alpha {
            return 0.0; // Both already alpha
        }

        if label_i == alpha || label_j == alpha {
            // One is already alpha, compute cost with other
            let other = if label_i == alpha { label_j } else { label_i };
            return self.pairwise_cost(alpha, other);
        }

        // Neither is alpha - cost of keeping both current labels
        // This is added to the graph as a neighbor edge weight
        self.pairwise_cost(label_i, label_j)
    }

    /// Data cost: how well does this biome fit the local environment?
    ///
    /// Lower cost = better fit
    fn data_cost(&self, x: i32, y: i32, biome: BiomeId) -> EnergyCost {
        let scale = 0.05;
        let elevation = ((self.elevation_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0) as f32;
        let moisture = ((self.moisture_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0) as f32;

        // Each biome has preferred elevation and moisture ranges
        let (elev_pref, moist_pref) = self.biome_environmental_preferences(biome);

        // Cost is squared distance from preference
        let elev_cost = (elevation - elev_pref).powi(2) as f64;
        let moist_cost = (moisture - moist_pref).powi(2) as f64;

        (elev_cost + moist_cost) * 100.0 // Scale factor
    }

    /// Get biome preferences for elevation and moisture
    fn biome_environmental_preferences(&self, biome: BiomeId) -> (f32, f32) {
        // Returns (preferred_elevation, preferred_moisture) in range [0.0, 1.0]
        match biome {
            BiomeId::Mountains => (0.8, 0.4),      // High elevation, moderate moisture
            BiomeId::Hills => (0.6, 0.5),          // Mid-high elevation, moderate moisture
            BiomeId::DeepWilderness => (0.5, 0.7), // Mid elevation, high moisture
            BiomeId::Forest => (0.4, 0.6),         // Mid-low elevation, good moisture
            BiomeId::Plains => (0.3, 0.4),         // Low elevation, moderate moisture
            BiomeId::Wetlands => (0.2, 0.8),       // Very low elevation, very high moisture
        }
    }

    /// Pairwise cost: compatibility between neighboring biomes
    ///
    /// Based on the neighbor multipliers from biome configuration
    /// Lower cost = more compatible (want to be neighbors)
    fn pairwise_cost(&self, biome_i: BiomeId, biome_j: BiomeId) -> EnergyCost {
        if biome_i == biome_j {
            return 0.0; // Same biome = no cost
        }

        // Get neighbor multiplier (high = want to be neighbors, low = avoid)
        let biome_i_data = self.biome_lookup.get(&biome_i).unwrap();
        let multiplier = biome_i_data.neighbor_multiplier(biome_j);

        // Convert multiplier to cost:
        // - High multiplier (2.0) = low cost (want to be neighbors)
        // - Zero multiplier (0.0) = very high cost (never neighbors)
        // - Mid multiplier (1.0) = neutral cost

        if multiplier == 0.0 {
            return 10000.0; // Very high cost = avoid
        }

        // Cost = 1 / multiplier, scaled
        (50.0 / multiplier as f64).max(0.1)
    }

    /// Directional cost: geographic preferences
    ///
    /// Mountains prefer north, wetlands prefer south, etc.
    fn directional_cost(&self, x: i32, y: i32, biome: BiomeId) -> EnergyCost {
        let norm_x = x as f64 / self.width as f64;
        let norm_y = y as f64 / self.height as f64;

        let biome_data = self.biome_lookup.get(&biome).unwrap();

        // Compute directional multiplier
        let mut multiplier = 1.0;

        // North preference
        multiplier *= self.interpolate_direction_weight(
            biome_data.directional_multiplier(Direction::North) as f64,
            1.0 - norm_y,
        );

        // South preference
        multiplier *= self.interpolate_direction_weight(
            biome_data.directional_multiplier(Direction::South) as f64,
            norm_y,
        );

        // East preference
        multiplier *= self.interpolate_direction_weight(
            biome_data.directional_multiplier(Direction::East) as f64,
            norm_x,
        );

        // West preference
        multiplier *= self.interpolate_direction_weight(
            biome_data.directional_multiplier(Direction::West) as f64,
            1.0 - norm_x,
        );

        // Convert multiplier to cost (higher multiplier = lower cost)
        if multiplier > 0.0 {
            10.0 / multiplier
        } else {
            100.0 // High cost if no directional preference
        }
    }

    /// Interpolate directional weight based on position
    fn interpolate_direction_weight(&self, direction_mult: f64, position: f64) -> f64 {
        if direction_mult <= 1.0 {
            1.0
        } else {
            1.0 + (direction_mult - 1.0) * position
        }
    }

    /// Compute minimum cut using max-flow algorithm (Edmonds-Karp)
    fn compute_min_cut(&self, graph: &AlphaExpansionGraph, alpha: BiomeId) -> Vec<bool> {
        let num_cells = graph.num_nodes;

        // Create flow network with source and sink nodes
        // Nodes: 0=source, 1..num_cells=cells, num_cells+1=sink
        let source = 0;
        let sink = num_cells + 1;
        let total_nodes = num_cells + 2;

        let mut network = FlowNetwork::new(total_nodes, source, sink);

        // Add terminal edges (source -> cell, cell -> sink)
        for i in 0..num_cells {
            let cell_node = i + 1; // Offset by 1 (node 0 is source)
            let (source_cost, sink_cost) = graph.terminal_costs[i];

            // Source -> cell edge (capacity = cost of NOT assigning alpha)
            if sink_cost > 0.0 {
                network.add_edge(source, cell_node, sink_cost);
            }

            // Cell -> sink edge (capacity = cost of assigning alpha)
            if source_cost > 0.0 {
                network.add_edge(cell_node, sink, source_cost);
            }
        }

        // Add neighbor edges
        for &(node_i, node_j, cost) in &graph.neighbor_edges {
            let cell_i = node_i + 1;
            let cell_j = node_j + 1;

            // Bidirectional edge with given cost
            if cost > 0.0 {
                network.add_edge(cell_i, cell_j, cost);
                network.add_edge(cell_j, cell_i, cost);
            }
        }

        // Compute max-flow
        let _max_flow = network.max_flow();

        // Get min-cut (cells reachable from source)
        let cut = network.min_cut();

        // Convert to cell indices (skip source node)
        let mut in_source = vec![false; num_cells];
        for i in 0..num_cells {
            in_source[i] = cut[i + 1];
        }

        in_source
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}

/// Graph structure for alpha-expansion
struct AlphaExpansionGraph {
    num_nodes: usize,
    /// Terminal costs: (source_cost, sink_cost) for each node
    terminal_costs: Vec<(EnergyCost, EnergyCost)>,
    /// Neighbor edges: (node_i, node_j, cost)
    neighbor_edges: Vec<(usize, usize, EnergyCost)>,
}

impl AlphaExpansionGraph {
    fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            terminal_costs: vec![(0.0, 0.0); num_nodes],
            neighbor_edges: Vec::new(),
        }
    }

    fn add_terminal_edge(&mut self, node: usize, source_cost: EnergyCost, sink_cost: EnergyCost) {
        self.terminal_costs[node] = (source_cost, sink_cost);
    }

    fn add_neighbor_edge(&mut self, node_i: usize, node_j: usize, cost: EnergyCost) {
        self.neighbor_edges.push((node_i, node_j, cost));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use noise::Perlin;

    #[test]
    fn test_mrf_biome_generator_initialization() {
        let elevation = Perlin::new(12345);
        let moisture = Perlin::new(54321);
        let generator = MRFBiomeGenerator::new(20, 20, 99999, elevation, moisture);

        assert_eq!(generator.biomes.len(), 6);
        assert_eq!(generator.width, 20);
        assert_eq!(generator.height, 20);
    }

    #[test]
    fn test_biome_environmental_preferences() {
        let elevation = Perlin::new(12345);
        let moisture = Perlin::new(54321);
        let generator = MRFBiomeGenerator::new(10, 10, 99999, elevation, moisture);

        let (mtn_elev, _) = generator.biome_environmental_preferences(BiomeId::Mountains);
        let (wet_elev, _) = generator.biome_environmental_preferences(BiomeId::Wetlands);

        // Mountains prefer high elevation, wetlands prefer low
        assert!(mtn_elev > wet_elev);
    }

    #[test]
    fn test_pairwise_cost_same_biome() {
        let elevation = Perlin::new(12345);
        let moisture = Perlin::new(54321);
        let generator = MRFBiomeGenerator::new(10, 10, 99999, elevation, moisture);

        let cost = generator.pairwise_cost(BiomeId::Forest, BiomeId::Forest);
        assert_eq!(cost, 0.0); // Same biome = no cost
    }

    #[test]
    fn test_pairwise_cost_incompatible_biomes() {
        let elevation = Perlin::new(12345);
        let moisture = Perlin::new(54321);
        let generator = MRFBiomeGenerator::new(10, 10, 99999, elevation, moisture);

        // DeepWilderness and Plains have 0.0 multiplier (never neighbors)
        let cost = generator.pairwise_cost(BiomeId::DeepWilderness, BiomeId::Plains);
        assert!(cost > 1000.0); // Very high cost
    }
}
