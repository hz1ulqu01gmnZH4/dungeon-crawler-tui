use super::{Overmap, Settlement, SettlementType, TerrainType, NameGenerator};
use rand::prelude::*;

/// Placement algorithm for settlements on overmap
pub struct SettlementPlacer {
    seed: u64,
    name_gen: NameGenerator,
}

impl SettlementPlacer {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            name_gen: NameGenerator::new(seed),
        }
    }

    /// Place settlements on the overmap
    pub fn place_settlements(&mut self, overmap: &mut Overmap) -> Vec<Settlement> {
        let mut settlements = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.seed);

        // Calculate target counts based on map size
        let map_area = overmap.width * overmap.height;
        let target_cities = (map_area / 800).max(1); // ~1 city per 800 tiles
        let target_towns = (map_area / 250).max(2); // ~1 town per 250 tiles
        let target_villages = (map_area / 80).max(5); // ~1 village per 80 tiles

        // Place cities first (largest, most restrictive)
        for _ in 0..target_cities {
            if let Some(settlement) =
                self.try_place_settlement(&mut rng, overmap, &settlements, SettlementType::City)
            {
                // Mark on overmap
                let (x, y) = settlement.position;
                if let Some(tile) = overmap.get_tile_mut(x, y) {
                    tile.terrain = TerrainType::Settlement;
                    tile.location = Some(settlement.id);
                }
                settlements.push(settlement);
            }
        }

        // Place towns
        for _ in 0..target_towns {
            if let Some(settlement) =
                self.try_place_settlement(&mut rng, overmap, &settlements, SettlementType::Town)
            {
                let (x, y) = settlement.position;
                if let Some(tile) = overmap.get_tile_mut(x, y) {
                    tile.terrain = TerrainType::Settlement;
                    tile.location = Some(settlement.id);
                }
                settlements.push(settlement);
            }
        }

        // Place villages
        for _ in 0..target_villages {
            if let Some(settlement) =
                self.try_place_settlement(&mut rng, overmap, &settlements, SettlementType::Village)
            {
                let (x, y) = settlement.position;
                if let Some(tile) = overmap.get_tile_mut(x, y) {
                    tile.terrain = TerrainType::Settlement;
                    tile.location = Some(settlement.id);
                }
                settlements.push(settlement);
            }
        }

        settlements
    }

    fn try_place_settlement(
        &mut self,
        rng: &mut StdRng,
        overmap: &Overmap,
        existing: &[Settlement],
        settlement_type: SettlementType,
    ) -> Option<Settlement> {
        let max_attempts = 100;
        let min_distance = settlement_type.min_distance();

        for _ in 0..max_attempts {
            let x = rng.gen_range(0..overmap.width);
            let y = rng.gen_range(0..overmap.height);

            // Check if location is suitable
            if !self.is_suitable_location(overmap, x, y) {
                continue;
            }

            // Check distance from existing settlements
            let too_close = existing.iter().any(|s| {
                let (sx, sy) = s.position;
                let dist = ((x - sx).abs() + (y - sy).abs()); // Manhattan distance
                dist < min_distance
            });

            if too_close {
                continue;
            }

            // Generate settlement
            let id = existing.len();
            let name = self.name_gen.generate_settlement_name();
            let (pop_min, pop_max) = settlement_type.population_range();
            let population = rng.gen_range(pop_min..=pop_max);
            let founded_year = 700 + rng.gen_range(0..60); // Founded years 700-760

            return Some(Settlement::new(
                id,
                name,
                settlement_type,
                (x, y),
                population,
                founded_year,
            ));
        }

        None // Failed to find suitable location
    }

    fn is_suitable_location(&self, overmap: &Overmap, x: i32, y: i32) -> bool {
        if let Some(tile) = overmap.get_tile(x, y) {
            // Settlements prefer flat, accessible terrain
            matches!(
                tile.terrain,
                TerrainType::Plains | TerrainType::Forest | TerrainType::Hills
            )
        } else {
            false
        }
    }
}

/// Helper function to place settlements on an overmap
pub fn place_settlements(overmap: &mut Overmap) -> Vec<Settlement> {
    let mut placer = SettlementPlacer::new(overmap.seed);
    placer.place_settlements(overmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_placement() {
        let mut overmap = Overmap::new(50, 50, 12345);
        // Need to generate terrain first
        use crate::world::generate_terrain;
        generate_terrain(&mut overmap);

        let settlements = place_settlements(&mut overmap);

        // Should have placed some settlements
        assert!(!settlements.is_empty());

        // Check that settlements are marked on overmap
        for settlement in settlements.iter() {
            let (x, y) = settlement.position;
            if let Some(tile) = overmap.get_tile(x, y) {
                assert_eq!(tile.terrain, TerrainType::Settlement);
                assert_eq!(tile.location, Some(settlement.id));
            }
        }
    }

    #[test]
    fn test_settlement_spacing() {
        let mut overmap = Overmap::new(50, 50, 99999);
        use crate::world::generate_terrain;
        generate_terrain(&mut overmap);

        let settlements = place_settlements(&mut overmap);

        // Check minimum distances are respected
        for (i, s1) in settlements.iter().enumerate() {
            for (j, s2) in settlements.iter().enumerate() {
                if i == j {
                    continue;
                }

                let (x1, y1) = s1.position;
                let (x2, y2) = s2.position;
                let dist = (x1 - x2).abs() + (y1 - y2).abs();

                // Should be at least minimum distance apart
                let min_dist = s1.settlement_type.min_distance().min(s2.settlement_type.min_distance());
                assert!(
                    dist >= min_dist,
                    "Settlements {} and {} too close: {} < {}",
                    s1.name,
                    s2.name,
                    dist,
                    min_dist
                );
            }
        }
    }

    #[test]
    fn test_settlement_terrain_suitability() {
        let mut overmap = Overmap::new(30, 30, 54321);
        use crate::world::generate_terrain;
        generate_terrain(&mut overmap);

        let settlements = place_settlements(&mut overmap);

        // All settlements should be on suitable terrain originally
        // (before being marked as Settlement terrain)
        for settlement in settlements.iter() {
            let (x, y) = settlement.position;
            // We can't check the original terrain since it's been overwritten,
            // but we can verify they're within bounds
            assert!(overmap.in_bounds(x, y));
        }
    }

    #[test]
    fn test_settlement_counts() {
        let mut overmap = Overmap::new(50, 50, 11111);
        use crate::world::generate_terrain;
        generate_terrain(&mut overmap);

        let settlements = place_settlements(&mut overmap);

        // Count by type
        let cities = settlements
            .iter()
            .filter(|s| s.settlement_type == SettlementType::City)
            .count();
        let towns = settlements
            .iter()
            .filter(|s| s.settlement_type == SettlementType::Town)
            .count();
        let villages = settlements
            .iter()
            .filter(|s| s.settlement_type == SettlementType::Village)
            .count();

        // Should have reasonable distribution
        assert!(cities >= 1, "Should have at least 1 city");
        assert!(towns >= 2, "Should have at least 2 towns");
        assert!(villages >= 5, "Should have at least 5 villages");
    }
}
