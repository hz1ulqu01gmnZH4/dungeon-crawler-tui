use super::{Overmap, TerrainType, BiomeGenerator, get_core_biomes, Biome};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::collections::HashMap;

/// Generates terrain for the overmap using noise-based generation
pub struct TerrainGenerator {
    seed: u64,
    elevation_noise: Perlin,
    moisture_noise: Perlin,
    temperature_noise: Perlin,
    biome_lookup: HashMap<super::BiomeId, Biome>,
}

impl TerrainGenerator {
    pub fn new(seed: u64) -> Self {
        let elevation_noise = Perlin::new(seed as u32);
        let moisture_noise = Perlin::new((seed + 1) as u32);
        let temperature_noise = Perlin::new((seed + 2) as u32);

        // Build biome lookup
        let biomes = get_core_biomes();
        let biome_lookup: HashMap<super::BiomeId, Biome> = biomes
            .into_iter()
            .map(|b| (b.id, b))
            .collect();

        Self {
            seed,
            elevation_noise,
            moisture_noise,
            temperature_noise,
            biome_lookup,
        }
    }

    /// Generate terrain for the entire overmap
    pub fn generate(&self, overmap: &mut Overmap) {
        let mut rng = StdRng::seed_from_u64(self.seed + 999); // Different seed offset for terrain

        // First pass: generate base terrain using biome weights + noise
        for y in 0..overmap.height {
            for x in 0..overmap.width {
                if let Some(tile) = overmap.get_tile(x, y) {
                    let biome_id = tile.biome;
                    let terrain = self.get_terrain_with_biome(x, y, biome_id, &mut rng);
                    if let Some(tile_mut) = overmap.get_tile_mut(x, y) {
                        tile_mut.terrain = terrain;
                    }
                }
            }
        }

        // Second pass: carve rivers
        self.generate_rivers(overmap);

        // Third pass: smooth transitions
        self.smooth_terrain(overmap);
    }

    /// Get terrain type using biome weights blended with noise for variation
    fn get_terrain_with_biome(&self, x: i32, y: i32, biome_id: super::BiomeId, rng: &mut StdRng) -> TerrainType {
        // Get biome terrain weights
        let biome = match self.biome_lookup.get(&biome_id) {
            Some(b) => b,
            None => return TerrainType::Plains, // Fallback
        };

        if biome.total_terrain_weight() == 0 {
            return TerrainType::Plains; // Fallback
        }

        // Get noise values for variation
        let scale = 0.05;
        let elevation = ((self.elevation_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0) as f32;
        let moisture = ((self.moisture_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0) as f32;

        // Build weighted terrain list based on biome preferences
        // Collect all terrain types in a fixed order for determinism
        let mut terrain_list: Vec<(TerrainType, u32)> = biome.terrain_weights.iter()
            .map(|(t, w)| (*t, *w))
            .collect();
        terrain_list.sort_by_key(|(t, _)| format!("{:?}", t)); // Sort by debug string for determinism

        let mut terrain_weights: Vec<(TerrainType, u32)> = Vec::new();

        for (terrain_type, base_weight) in terrain_list.iter() {
            let mut weight = *base_weight as f32;

            // Adjust weights based on noise to create variation within biomes
            match terrain_type {
                TerrainType::Mountains => {
                    // Mountains more likely at high elevation
                    weight *= 1.0 + (elevation - 0.5).max(0.0) * 2.0;
                }
                TerrainType::Hills => {
                    // Hills at mid-high elevation
                    if elevation > 0.4 && elevation < 0.7 {
                        weight *= 1.5;
                    }
                }
                TerrainType::Lake => {
                    // Lakes at low elevation + high moisture
                    if elevation < 0.4 && moisture > 0.6 {
                        weight *= 2.0;
                    } else {
                        weight *= 0.5;
                    }
                }
                TerrainType::Swamp => {
                    // Swamps at low elevation + moderate moisture
                    if elevation < 0.4 && moisture > 0.4 {
                        weight *= 1.5;
                    } else {
                        weight *= 0.7;
                    }
                }
                TerrainType::DenseForest => {
                    // Dense forest with high moisture
                    if moisture > 0.6 {
                        weight *= 1.5;
                    }
                }
                TerrainType::Forest => {
                    // Forest with moderate moisture
                    if moisture > 0.4 {
                        weight *= 1.2;
                    }
                }
                TerrainType::Plains => {
                    // Plains are default, slight boost at mid elevation
                    if elevation > 0.3 && elevation < 0.6 {
                        weight *= 1.1;
                    }
                }
                _ => {} // Other terrains use base weight
            }

            terrain_weights.push((*terrain_type, weight.max(1.0) as u32));
        }

        // Select terrain using weighted random
        let total_weight: u32 = terrain_weights.iter().map(|(_, w)| w).sum();
        if total_weight == 0 {
            return TerrainType::Plains;
        }

        let mut roll = rng.gen_range(0..total_weight);
        for (terrain, weight) in terrain_weights.iter() {
            if roll < *weight {
                return *terrain;
            }
            roll -= weight;
        }

        // Fallback
        terrain_weights[0].0
    }

    /// Get terrain type at specific coordinates based on noise (legacy, kept for reference)
    #[allow(dead_code)]
    fn get_terrain_at(&self, x: i32, y: i32) -> TerrainType {
        // Scale for noise (smaller = more zoomed in features)
        let scale = 0.05;

        // Get noise values (-1.0 to 1.0, normalized to 0.0 to 1.0)
        let elevation = (self.elevation_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0;
        let moisture = (self.moisture_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0;
        let temperature =
            (self.temperature_noise.get([x as f64 * scale, y as f64 * scale]) + 1.0) / 2.0;

        // Determine terrain based on elevation, moisture, and temperature
        match elevation {
            e if e > 0.75 => TerrainType::Mountains,
            e if e > 0.60 => TerrainType::Hills,
            e if e < 0.30 => {
                // Low elevation: check moisture for water
                if moisture > 0.60 {
                    TerrainType::Lake
                } else if moisture > 0.45 {
                    TerrainType::Swamp
                } else {
                    TerrainType::Plains
                }
            }
            _ => {
                // Mid elevation: forests, plains based on moisture/temperature
                if moisture > 0.65 && temperature > 0.40 {
                    TerrainType::DenseForest
                } else if moisture > 0.45 {
                    TerrainType::Forest
                } else {
                    TerrainType::Plains
                }
            }
        }
    }

    /// Generate rivers flowing from high to low elevation
    fn generate_rivers(&self, overmap: &mut Overmap) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        // Try to place a few rivers
        let num_rivers = (overmap.width * overmap.height) / 400; // ~1 river per 400 tiles

        for _ in 0..num_rivers {
            // Start from a high elevation area
            let start_x = rng.gen_range(0..overmap.width);
            let start_y = rng.gen_range(0..overmap.height);

            // Only start rivers from hills/mountains
            if let Some(start_tile) = overmap.get_tile(start_x, start_y) {
                if !matches!(
                    start_tile.terrain,
                    TerrainType::Hills | TerrainType::Mountains
                ) {
                    continue;
                }
            }

            // Trace river downhill
            self.trace_river(overmap, start_x, start_y, &mut rng);
        }
    }

    /// Trace a single river from starting point downhill
    fn trace_river(&self, overmap: &mut Overmap, start_x: i32, start_y: i32, rng: &mut StdRng) {
        let mut x = start_x;
        let mut y = start_y;
        let max_length = 50;

        for _ in 0..max_length {
            // Mark current tile as river (unless it's a lake)
            if let Some(tile) = overmap.get_tile_mut(x, y) {
                if tile.terrain == TerrainType::Lake {
                    break; // River flows into lake
                }
                if tile.terrain != TerrainType::River {
                    tile.terrain = TerrainType::River;
                }
            }

            // Find lowest neighbor
            let neighbors = overmap.get_neighbors(x, y);
            if neighbors.is_empty() {
                break;
            }

            let mut lowest: Option<(i32, i32, f64)> = None;
            for (nx, ny) in neighbors.iter().copied() {
                let scale = 0.05;
                let elevation = (self
                    .elevation_noise
                    .get([nx as f64 * scale, ny as f64 * scale])
                    + 1.0)
                    / 2.0;

                if let Some((_, _, lowest_elev)) = lowest {
                    if elevation < lowest_elev {
                        lowest = Some((nx, ny, elevation));
                    }
                } else {
                    lowest = Some((nx, ny, elevation));
                }
            }

            // Move to lowest neighbor (with occasional randomness)
            if let Some((nx, ny, _)) = lowest {
                if rng.gen::<f32>() < 0.9 {
                    // 90% follow gradient
                    x = nx;
                    y = ny;
                } else {
                    // 10% random direction for variety
                    let idx = rng.gen_range(0..neighbors.len());
                    let (nx, ny) = neighbors[idx];
                    x = nx;
                    y = ny;
                }
            } else {
                break;
            }
        }
    }

    /// Smooth terrain to remove single-tile inconsistencies
    fn smooth_terrain(&self, overmap: &mut Overmap) {
        let mut changes = Vec::new();

        for y in 1..overmap.height - 1 {
            for x in 1..overmap.width - 1 {
                if let Some(tile) = overmap.get_tile(x, y) {
                    let terrain = tile.terrain;

                    // Don't smooth special terrain
                    if matches!(
                        terrain,
                        TerrainType::River
                            | TerrainType::Road
                            | TerrainType::Settlement
                            | TerrainType::Dungeon
                    ) {
                        continue;
                    }

                    // Count similar neighbors
                    let neighbors = overmap.get_neighbors(x, y);
                    let similar_count = neighbors
                        .iter()
                        .filter(|(nx, ny)| {
                            if let Some(neighbor) = overmap.get_tile(*nx, *ny) {
                                neighbor.terrain == terrain
                            } else {
                                false
                            }
                        })
                        .count();

                    // If isolated (0-1 similar neighbors), change to most common neighbor
                    if similar_count <= 1 {
                        let mut terrain_counts: Vec<(TerrainType, usize)> = Vec::new();

                        for (nx, ny) in neighbors.iter() {
                            if let Some(neighbor) = overmap.get_tile(*nx, *ny) {
                                if let Some(entry) = terrain_counts.iter_mut().find(|(t, _)| *t == neighbor.terrain) {
                                    entry.1 += 1;
                                } else {
                                    terrain_counts.push((neighbor.terrain, 1));
                                }
                            }
                        }

                        // Find most common terrain (deterministic - first max wins)
                        if let Some((common_terrain, _)) =
                            terrain_counts.iter().max_by_key(|(_, count)| count)
                        {
                            changes.push((x, y, *common_terrain));
                        }
                    }
                }
            }
        }

        // Apply changes
        for (x, y, new_terrain) in changes {
            if let Some(tile) = overmap.get_tile_mut(x, y) {
                tile.terrain = new_terrain;
            }
        }
    }
}

/// Helper function to generate terrain for an overmap
pub fn generate_terrain(overmap: &mut Overmap) {
    // First, generate biomes
    let biome_generator = BiomeGenerator::new(overmap.seed);
    let biome_map = biome_generator.generate(overmap.width, overmap.height);
    biome_generator.apply_to_overmap(&biome_map, overmap);

    // Then, generate terrain (terrain will respect biomes in Phase C)
    let terrain_generator = TerrainGenerator::new(overmap.seed);
    terrain_generator.generate(overmap);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_generation() {
        let mut overmap = Overmap::new(50, 50, 12345);
        generate_terrain(&mut overmap);

        // Check that terrain was generated (not all plains)
        let mut terrain_types = std::collections::HashSet::new();
        for tile in overmap.tiles.iter() {
            terrain_types.insert(tile.terrain);
        }

        // Should have multiple terrain types
        assert!(terrain_types.len() > 1);
    }

    #[test]
    fn test_deterministic_generation() {
        let mut overmap1 = Overmap::new(20, 20, 99999);
        let mut overmap2 = Overmap::new(20, 20, 99999);

        generate_terrain(&mut overmap1);
        generate_terrain(&mut overmap2);

        // Same seed should produce same terrain
        for i in 0..overmap1.tiles.len() {
            assert_eq!(overmap1.tiles[i].terrain, overmap2.tiles[i].terrain);
        }
    }

    #[test]
    fn test_mountains_at_high_elevation() {
        let generator = TerrainGenerator::new(12345);

        // Test various coordinates - some should be mountains
        let mut found_mountains = false;
        for y in 0..50 {
            for x in 0..50 {
                let terrain = generator.get_terrain_at(x, y);
                if terrain == TerrainType::Mountains {
                    found_mountains = true;
                    break;
                }
            }
        }

        assert!(found_mountains);
    }

    #[test]
    fn test_terrain_respects_biome_weights() {
        let mut overmap = Overmap::new(50, 50, 54321);
        generate_terrain(&mut overmap);

        // Count terrain types per biome
        use std::collections::HashMap;
        use crate::world::BiomeId;

        let mut biome_terrain_counts: HashMap<BiomeId, HashMap<TerrainType, usize>> = HashMap::new();

        for tile in overmap.tiles.iter() {
            // Skip special terrains added by post-processing
            if matches!(tile.terrain, TerrainType::Road | TerrainType::KingRoad |
                        TerrainType::TradePath | TerrainType::Settlement |
                        TerrainType::Dungeon | TerrainType::SpecialLocation | TerrainType::River) {
                continue;
            }

            let biome_entry = biome_terrain_counts.entry(tile.biome).or_insert_with(HashMap::new);
            *biome_entry.entry(tile.terrain).or_insert(0) += 1;
        }

        // Verify that biomes have diverse terrain (not all the same)
        let biomes = get_core_biomes();
        let mut biomes_checked = 0;

        for biome in biomes.iter() {
            if let Some(terrain_counts) = biome_terrain_counts.get(&biome.id) {
                let total_tiles: usize = terrain_counts.values().sum();
                if total_tiles > 10 { // Only check biomes with decent sample size
                    biomes_checked += 1;

                    // Biome should have at least one terrain type from its weight list
                    let has_allowed_terrain = terrain_counts
                        .keys()
                        .any(|terrain| biome.terrain_weight(*terrain) > 0);

                    assert!(
                        has_allowed_terrain,
                        "Biome {:?} has no allowed terrain types present",
                        biome.id
                    );
                }
            }
        }

        // At least some biomes should be present
        assert!(biomes_checked >= 2, "Expected at least 2 biomes to be present in the map");
    }

    #[test]
    fn test_terrain_variety_within_biomes() {
        let mut overmap = Overmap::new(50, 50, 11111);
        generate_terrain(&mut overmap);

        use std::collections::HashMap;
        use crate::world::BiomeId;

        // Find a biome that has multiple terrain types defined
        let mut biome_terrain_types: HashMap<BiomeId, std::collections::HashSet<TerrainType>> = HashMap::new();

        for tile in overmap.tiles.iter() {
            // Skip special terrains
            if matches!(tile.terrain, TerrainType::Road | TerrainType::KingRoad |
                        TerrainType::TradePath | TerrainType::Settlement |
                        TerrainType::Dungeon | TerrainType::SpecialLocation | TerrainType::River) {
                continue;
            }

            biome_terrain_types.entry(tile.biome)
                .or_insert_with(std::collections::HashSet::new)
                .insert(tile.terrain);
        }

        // At least one biome should have multiple terrain types (showing variation)
        let has_variety = biome_terrain_types.values().any(|types| types.len() >= 2);
        assert!(has_variety, "Expected at least one biome to have multiple terrain types");
    }
}
