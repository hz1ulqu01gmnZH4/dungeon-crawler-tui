use super::biome::{Biome, BiomeId, Direction};
use super::overmap::Overmap;
use super::region_settings::get_core_biomes;
use rand::prelude::*;
use std::collections::HashMap;

/// Biome map - stores biome assignments and strengths for overmap generation
#[derive(Clone, Debug)]
pub struct BiomeMap {
    pub width: i32,
    pub height: i32,
    /// Biome ID for each tile (y * width + x)
    pub biomes: Vec<BiomeId>,
    /// Biome strength for each tile (0.0 = weak, 1.0 = strong)
    pub strengths: Vec<f32>,
}

impl BiomeMap {
    /// Create a new biome map with given dimensions
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            biomes: vec![BiomeId::Plains; size],
            strengths: vec![1.0; size],
        }
    }

    /// Get index from coordinates
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Check if coordinates are in bounds
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Get biome at coordinates
    pub fn get_biome(&self, x: i32, y: i32) -> Option<BiomeId> {
        if !self.in_bounds(x, y) {
            return None;
        }
        Some(self.biomes[self.xy_idx(x, y)])
    }

    /// Get biome strength at coordinates
    pub fn get_strength(&self, x: i32, y: i32) -> Option<f32> {
        if !self.in_bounds(x, y) {
            return None;
        }
        Some(self.strengths[self.xy_idx(x, y)])
    }

    /// Set biome at coordinates
    pub fn set_biome(&mut self, x: i32, y: i32, biome: BiomeId, strength: f32) {
        if !self.in_bounds(x, y) {
            return;
        }
        let idx = self.xy_idx(x, y);
        self.biomes[idx] = biome;
        self.strengths[idx] = strength.clamp(0.0, 1.0);
    }

    /// Get 8-directional neighbors with their directions
    fn get_neighbors_with_directions(&self, x: i32, y: i32) -> Vec<((i32, i32), Direction)> {
        let mut neighbors = Vec::new();

        let offsets = [
            ((0, -1), Direction::North),
            ((1, -1), Direction::NorthEast),
            ((1, 0), Direction::East),
            ((1, 1), Direction::SouthEast),
            ((0, 1), Direction::South),
            ((-1, 1), Direction::SouthWest),
            ((-1, 0), Direction::West),
            ((-1, -1), Direction::NorthWest),
        ];

        for ((dx, dy), dir) in offsets.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if self.in_bounds(nx, ny) {
                neighbors.push(((nx, ny), *dir));
            }
        }

        neighbors
    }
}

/// Biome generator - creates biome maps using weighted selection with neighbor influence
pub struct BiomeGenerator {
    seed: u64,
    biomes: Vec<Biome>,
    biome_lookup: HashMap<BiomeId, usize>, // BiomeId -> index in biomes vec
}

impl BiomeGenerator {
    /// Create a new biome generator with default biomes
    pub fn new(seed: u64) -> Self {
        let biomes = get_core_biomes();
        let biome_lookup: HashMap<BiomeId, usize> = biomes
            .iter()
            .enumerate()
            .map(|(idx, biome)| (biome.id, idx))
            .collect();

        Self {
            seed,
            biomes,
            biome_lookup,
        }
    }

    /// Generate a biome map for the given dimensions
    pub fn generate(&self, width: i32, height: i32) -> BiomeMap {
        let mut biome_map = BiomeMap::new(width, height);
        let mut rng = StdRng::seed_from_u64(self.seed);

        // Pass 1: Initial random weighted selection
        self.initial_placement(&mut biome_map, &mut rng);

        // Pass 2-3: Apply neighbor influence (2 passes for better clustering)
        for pass in 0..2 {
            self.apply_neighbor_influence(&mut biome_map, &mut rng, pass);
        }

        // Pass 4: Apply directional preferences
        self.apply_directional_weights(&mut biome_map, &mut rng);

        biome_map
    }

    /// Pass 1: Place biomes randomly based on base weights
    fn initial_placement(&self, biome_map: &mut BiomeMap, rng: &mut StdRng) {
        for y in 0..biome_map.height {
            for x in 0..biome_map.width {
                let biome = self.select_biome_weighted(&self.biomes, rng);
                biome_map.set_biome(x, y, biome, 1.0);
            }
        }
    }

    /// Pass 2-3: Apply neighbor influence to create biome clusters
    fn apply_neighbor_influence(&self, biome_map: &mut BiomeMap, rng: &mut StdRng, pass: usize) {
        // Create a copy to read from while we modify
        let old_biomes = biome_map.biomes.clone();

        for y in 0..biome_map.height {
            for x in 0..biome_map.width {
                // Calculate weights for each biome based on neighbors
                let mut biome_weights: HashMap<BiomeId, f32> = HashMap::new();

                // Start with base biome weight
                let current_idx = biome_map.xy_idx(x, y);
                let current_biome_id = old_biomes[current_idx];

                if let Some(&biome_idx) = self.biome_lookup.get(&current_biome_id) {
                    let base_weight = self.biomes[biome_idx].weight as f32;
                    biome_weights.insert(current_biome_id, base_weight);
                }

                // Add influence from neighbors
                let neighbors = biome_map.get_neighbors_with_directions(x, y);
                for ((nx, ny), _) in neighbors.iter() {
                    let neighbor_idx = biome_map.xy_idx(*nx, *ny);
                    let neighbor_biome = old_biomes[neighbor_idx];

                    if let Some(&neighbor_biome_idx) = self.biome_lookup.get(&neighbor_biome) {
                        let neighbor_biome_data = &self.biomes[neighbor_biome_idx];

                        // For each possible biome, add weighted influence
                        for biome in self.biomes.iter() {
                            let influence = neighbor_biome_data.neighbor_multiplier(biome.id);
                            let weight = biome.weight as f32 * influence;

                            *biome_weights.entry(biome.id).or_insert(0.0) += weight;
                        }
                    }
                }

                // Select new biome based on accumulated weights
                let new_biome = self.select_biome_from_weights(&biome_weights, rng);

                // Increase strength on second pass
                let strength = if pass == 0 { 0.7 } else { 0.9 };
                biome_map.set_biome(x, y, new_biome, strength);
            }
        }
    }

    /// Pass 4: Apply directional weighting to create geographic features
    fn apply_directional_weights(&self, biome_map: &mut BiomeMap, rng: &mut StdRng) {
        let old_biomes = biome_map.biomes.clone();

        for y in 0..biome_map.height {
            for x in 0..biome_map.width {
                let current_idx = biome_map.xy_idx(x, y);
                let current_biome_id = old_biomes[current_idx];

                // Calculate normalized position (0.0 = north/west, 1.0 = south/east)
                let norm_x = x as f32 / biome_map.width as f32;
                let norm_y = y as f32 / biome_map.height as f32;

                // Calculate directional influence
                let mut total_weight = 0.0;
                let mut weighted_votes: HashMap<BiomeId, f32> = HashMap::new();

                for biome in self.biomes.iter() {
                    let mut directional_multiplier = 1.0;

                    // North preference (y closer to 0)
                    directional_multiplier *= Self::interpolate_direction_weight(
                        biome.directional_multiplier(Direction::North),
                        1.0 - norm_y,
                    );

                    // South preference (y closer to max)
                    directional_multiplier *= Self::interpolate_direction_weight(
                        biome.directional_multiplier(Direction::South),
                        norm_y,
                    );

                    // East preference (x closer to max)
                    directional_multiplier *= Self::interpolate_direction_weight(
                        biome.directional_multiplier(Direction::East),
                        norm_x,
                    );

                    // West preference (x closer to 0)
                    directional_multiplier *= Self::interpolate_direction_weight(
                        biome.directional_multiplier(Direction::West),
                        1.0 - norm_x,
                    );

                    let weight = biome.weight as f32 * directional_multiplier;
                    weighted_votes.insert(biome.id, weight);
                    total_weight += weight;
                }

                // Prefer current biome if directional influence is weak
                if let Some(&current_weight) = weighted_votes.get(&current_biome_id) {
                    let current_ratio = current_weight / total_weight;

                    // Keep current biome if it has at least 30% of the weight
                    if current_ratio >= 0.3 {
                        biome_map.set_biome(x, y, current_biome_id, 1.0);
                        continue;
                    }
                }

                // Otherwise select based on directional weights
                let new_biome = self.select_biome_from_weights(&weighted_votes, rng);
                biome_map.set_biome(x, y, new_biome, 1.0);
            }
        }
    }

    /// Helper: Interpolate direction weight based on position
    fn interpolate_direction_weight(multiplier: f32, position: f32) -> f32 {
        // position: 0.0 = far from direction, 1.0 = close to direction
        // Returns value between 1.0 (no effect) and multiplier (full effect)
        1.0 + (multiplier - 1.0) * position
    }

    /// Select a biome using weighted random selection
    fn select_biome_weighted(&self, biomes: &[Biome], rng: &mut StdRng) -> BiomeId {
        let total_weight: u32 = biomes.iter().map(|b| b.weight).sum();
        let mut roll = rng.gen_range(0..total_weight);

        for biome in biomes.iter() {
            if roll < biome.weight {
                return biome.id;
            }
            roll -= biome.weight;
        }

        // Fallback (shouldn't reach here)
        biomes[0].id
    }

    /// Select a biome from a weight map (deterministic by using fixed biome order)
    fn select_biome_from_weights(&self, weights: &HashMap<BiomeId, f32>, rng: &mut StdRng) -> BiomeId {
        let total_weight: f32 = weights.values().sum();
        if total_weight <= 0.0 {
            return BiomeId::Plains; // Fallback
        }

        let mut roll = rng.gen::<f32>() * total_weight;

        // Use fixed biome order from self.biomes to ensure determinism
        for biome in self.biomes.iter() {
            if let Some(&weight) = weights.get(&biome.id) {
                if roll < weight {
                    return biome.id;
                }
                roll -= weight;
            }
        }

        // Fallback
        self.biomes[0].id
    }

    /// Apply the biome map to an overmap
    pub fn apply_to_overmap(&self, biome_map: &BiomeMap, overmap: &mut Overmap) {
        for y in 0..overmap.height.min(biome_map.height) {
            for x in 0..overmap.width.min(biome_map.width) {
                if let Some(biome) = biome_map.get_biome(x, y) {
                    if let Some(strength) = biome_map.get_strength(x, y) {
                        if let Some(tile) = overmap.get_tile_mut(x, y) {
                            tile.biome = biome;
                            tile.biome_strength = strength;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biome_map_creation() {
        let biome_map = BiomeMap::new(10, 10);
        assert_eq!(biome_map.width, 10);
        assert_eq!(biome_map.height, 10);
        assert_eq!(biome_map.biomes.len(), 100);
        assert_eq!(biome_map.strengths.len(), 100);
    }

    #[test]
    fn test_biome_map_get_set() {
        let mut biome_map = BiomeMap::new(10, 10);

        biome_map.set_biome(5, 5, BiomeId::Forest, 0.8);
        assert_eq!(biome_map.get_biome(5, 5), Some(BiomeId::Forest));
        assert_eq!(biome_map.get_strength(5, 5), Some(0.8));
    }

    #[test]
    fn test_biome_map_bounds_checking() {
        let biome_map = BiomeMap::new(10, 10);

        assert_eq!(biome_map.get_biome(5, 5), Some(BiomeId::Plains));
        assert_eq!(biome_map.get_biome(-1, 5), None);
        assert_eq!(biome_map.get_biome(5, -1), None);
        assert_eq!(biome_map.get_biome(10, 5), None);
        assert_eq!(biome_map.get_biome(5, 10), None);
    }

    #[test]
    fn test_biome_generator_creation() {
        let generator = BiomeGenerator::new(12345);
        assert_eq!(generator.biomes.len(), 6);
        assert_eq!(generator.biome_lookup.len(), 6);
    }

    #[test]
    fn test_biome_generation() {
        let generator = BiomeGenerator::new(12345);
        let biome_map = generator.generate(50, 50);

        assert_eq!(biome_map.width, 50);
        assert_eq!(biome_map.height, 50);

        // Check that we have variety (not all same biome)
        let mut biome_counts: HashMap<BiomeId, usize> = HashMap::new();
        for biome_id in biome_map.biomes.iter() {
            *biome_counts.entry(*biome_id).or_insert(0) += 1;
        }

        // Should have at least 3 different biomes in a 50x50 map
        assert!(biome_counts.len() >= 3, "Expected at least 3 biomes, got {}", biome_counts.len());
    }

    #[test]
    fn test_biome_generation_deterministic() {
        let generator1 = BiomeGenerator::new(99999);
        let biome_map1 = generator1.generate(20, 20);

        let generator2 = BiomeGenerator::new(99999);
        let biome_map2 = generator2.generate(20, 20);

        // Same seed should produce same biome map
        assert_eq!(biome_map1.biomes, biome_map2.biomes);
    }

    #[test]
    fn test_neighbor_influence_creates_clusters() {
        let generator = BiomeGenerator::new(54321);
        let biome_map = generator.generate(30, 30);

        // Check that biomes form clusters (neighbors tend to be the same)
        let mut same_neighbor_count = 0;
        let mut total_neighbor_checks = 0;

        for y in 1..biome_map.height - 1 {
            for x in 1..biome_map.width - 1 {
                let current = biome_map.get_biome(x, y).unwrap();

                // Check 4-directional neighbors
                let neighbors = [
                    biome_map.get_biome(x - 1, y),
                    biome_map.get_biome(x + 1, y),
                    biome_map.get_biome(x, y - 1),
                    biome_map.get_biome(x, y + 1),
                ];

                for neighbor in neighbors.iter().flatten() {
                    total_neighbor_checks += 1;
                    if *neighbor == current {
                        same_neighbor_count += 1;
                    }
                }
            }
        }

        // At least 30% of neighbors should be the same biome (clustering)
        // Note: With 6 different biomes, random would be ~16.7%, so 30%+ shows significant clustering
        let clustering_ratio = same_neighbor_count as f32 / total_neighbor_checks as f32;
        assert!(
            clustering_ratio >= 0.30,
            "Expected clustering ratio >= 0.30, got {}",
            clustering_ratio
        );
    }

    #[test]
    fn test_apply_to_overmap() {
        let generator = BiomeGenerator::new(11111);
        let biome_map = generator.generate(10, 10);
        let mut overmap = Overmap::new(10, 10, 11111);

        generator.apply_to_overmap(&biome_map, &mut overmap);

        // Verify biomes were applied
        for y in 0..10 {
            for x in 0..10 {
                let expected_biome = biome_map.get_biome(x, y).unwrap();
                let actual_biome = overmap.get_tile(x, y).unwrap().biome;
                assert_eq!(actual_biome, expected_biome);
            }
        }
    }

    #[test]
    fn test_strength_clamping() {
        let mut biome_map = BiomeMap::new(10, 10);

        biome_map.set_biome(0, 0, BiomeId::Plains, 1.5);
        assert_eq!(biome_map.get_strength(0, 0), Some(1.0));

        biome_map.set_biome(1, 1, BiomeId::Forest, -0.5);
        assert_eq!(biome_map.get_strength(1, 1), Some(0.0));
    }
}
