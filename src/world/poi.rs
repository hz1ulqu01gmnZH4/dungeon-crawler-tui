use crate::world::{Overmap, TerrainType};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Point of Interest types
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum POIType {
    Dungeon,        // Standard dungeon entrance
    Cave,           // Natural cave system
    Ruin,           // Ancient ruins
    Shrine,         // Mystical shrine
    TombCrypt,      // Burial site
    Tower,          // Abandoned tower
}

impl POIType {
    pub fn name(&self) -> &'static str {
        match self {
            POIType::Dungeon => "Dungeon",
            POIType::Cave => "Cave",
            POIType::Ruin => "Ruin",
            POIType::Shrine => "Shrine",
            POIType::TombCrypt => "Tomb",
            POIType::Tower => "Tower",
        }
    }

    /// Minimum distance from settlements
    pub fn min_settlement_distance(&self) -> i32 {
        match self {
            POIType::Dungeon => 5,     // Dungeons away from civilization
            POIType::Cave => 3,         // Caves can be closer
            POIType::Ruin => 4,
            POIType::Shrine => 2,       // Shrines can be near settlements
            POIType::TombCrypt => 4,
            POIType::Tower => 6,        // Towers are isolated
        }
    }

    /// Minimum distance from other POIs
    pub fn min_poi_distance(&self) -> i32 {
        4 // General spacing
    }

    /// Preferred terrain types
    pub fn suitable_terrain(&self) -> Vec<TerrainType> {
        match self {
            POIType::Dungeon => vec![
                TerrainType::Hills,
                TerrainType::Mountains,
                TerrainType::Forest,
            ],
            POIType::Cave => vec![
                TerrainType::Mountains,
                TerrainType::Hills,
            ],
            POIType::Ruin => vec![
                TerrainType::Plains,
                TerrainType::Forest,
                TerrainType::Hills,
            ],
            POIType::Shrine => vec![
                TerrainType::Forest,
                TerrainType::Hills,
                TerrainType::Mountains,
            ],
            POIType::TombCrypt => vec![
                TerrainType::Plains,
                TerrainType::Hills,
            ],
            POIType::Tower => vec![
                TerrainType::Hills,
                TerrainType::Plains,
            ],
        }
    }
}

/// A point of interest in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct POI {
    pub id: usize,
    pub poi_type: POIType,
    pub name: String,
    pub position: (i32, i32),
    pub discovered: bool,  // Has player found it yet
    pub cleared: bool,     // Has player completed it
}

impl POI {
    pub fn new(id: usize, poi_type: POIType, name: String, position: (i32, i32)) -> Self {
        Self {
            id,
            poi_type,
            name,
            position,
            discovered: false,
            cleared: false,
        }
    }
}

/// Name generator for POIs
pub struct POINameGenerator {
    seed: u64,
    counter: usize,
}

impl POINameGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed, counter: 0 }
    }

    pub fn generate_name(&mut self, poi_type: POIType) -> String {
        let (prefixes, suffixes) = match poi_type {
            POIType::Dungeon => (
                vec!["Dark", "Deep", "Forsaken", "Shadow", "Lost", "Ancient", "Cursed"],
                vec!["Depths", "Halls", "Catacombs", "Dungeon", "Vault", "Labyrinth"],
            ),
            POIType::Cave => (
                vec!["Whispering", "Echo", "Crystal", "Iron", "Dark", "Flooded"],
                vec!["Cavern", "Grotto", "Cave", "Hollow", "Den"],
            ),
            POIType::Ruin => (
                vec!["Fallen", "Crumbling", "Ancient", "Overgrown", "Shattered"],
                vec!["Ruins", "Temple", "Keep", "Fortress", "Citadel"],
            ),
            POIType::Shrine => (
                vec!["Sacred", "Forgotten", "Hidden", "Silent", "Blessed"],
                vec!["Shrine", "Altar", "Sanctum", "Grove", "Monument"],
            ),
            POIType::TombCrypt => (
                vec!["Silent", "Eternal", "Royal", "Sealed", "Cursed"],
                vec!["Tomb", "Crypt", "Mausoleum", "Barrow", "Cairn"],
            ),
            POIType::Tower => (
                vec!["Lonely", "Broken", "Crumbling", "Wizard's", "Watchers"],
                vec!["Tower", "Spire", "Keep", "Pinnacle", "Beacon"],
            ),
        };

        let prefix_idx = ((self.seed.wrapping_add(self.counter as u64)) % prefixes.len() as u64) as usize;
        let suffix_idx = ((self.seed.wrapping_add(self.counter as u64 * 23)) % suffixes.len() as u64) as usize;

        self.counter += 1;

        format!("{} {}", prefixes[prefix_idx], suffixes[suffix_idx])
    }
}

/// POI placement algorithm
pub struct POIPlacer {
    seed: u64,
    name_gen: POINameGenerator,
}

impl POIPlacer {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            name_gen: POINameGenerator::new(seed.wrapping_add(1000)),
        }
    }

    /// Place POIs on the overmap
    pub fn place_pois(
        &mut self,
        overmap: &mut Overmap,
        settlement_positions: &[(i32, i32)],
    ) -> Vec<POI> {
        let mut pois = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.seed);

        let map_area = overmap.width * overmap.height;

        // Target counts based on map size
        let target_dungeons = (map_area / 400).max(2);  // ~1 dungeon per 400 tiles
        let target_caves = (map_area / 600).max(2);
        let target_ruins = (map_area / 500).max(2);
        let target_shrines = (map_area / 700).max(1);
        let target_tombs = (map_area / 800).max(1);
        let target_towers = (map_area / 900).max(1);

        // Place each POI type
        self.place_poi_type(&mut pois, &mut rng, overmap, settlement_positions, POIType::Dungeon, target_dungeons as usize);
        self.place_poi_type(&mut pois, &mut rng, overmap, settlement_positions, POIType::Cave, target_caves as usize);
        self.place_poi_type(&mut pois, &mut rng, overmap, settlement_positions, POIType::Ruin, target_ruins as usize);
        self.place_poi_type(&mut pois, &mut rng, overmap, settlement_positions, POIType::Shrine, target_shrines as usize);
        self.place_poi_type(&mut pois, &mut rng, overmap, settlement_positions, POIType::TombCrypt, target_tombs as usize);
        self.place_poi_type(&mut pois, &mut rng, overmap, settlement_positions, POIType::Tower, target_towers as usize);

        pois
    }

    fn place_poi_type(
        &mut self,
        pois: &mut Vec<POI>,
        rng: &mut StdRng,
        overmap: &mut Overmap,
        settlement_positions: &[(i32, i32)],
        poi_type: POIType,
        count: usize,
    ) {
        for _ in 0..count {
            if let Some(poi) = self.try_place_poi(rng, overmap, settlement_positions, pois, poi_type) {
                // Mark on overmap
                let (x, y) = poi.position;
                if let Some(tile) = overmap.get_tile_mut(x, y) {
                    tile.terrain = if poi_type == POIType::Dungeon {
                        TerrainType::Dungeon
                    } else {
                        TerrainType::SpecialLocation
                    };
                }
                pois.push(poi);
            }
        }
    }

    fn try_place_poi(
        &mut self,
        rng: &mut StdRng,
        overmap: &Overmap,
        settlement_positions: &[(i32, i32)],
        existing_pois: &[POI],
        poi_type: POIType,
    ) -> Option<POI> {
        let max_attempts = 100;
        let suitable_terrain = poi_type.suitable_terrain();

        for _ in 0..max_attempts {
            let x = rng.gen_range(0..overmap.width);
            let y = rng.gen_range(0..overmap.height);

            // Check terrain suitability
            if let Some(tile) = overmap.get_tile(x, y) {
                if !suitable_terrain.contains(&tile.terrain) {
                    continue;
                }
            } else {
                continue;
            }

            // Check distance from settlements
            let min_settlement_dist = poi_type.min_settlement_distance();
            let too_close_to_settlement = settlement_positions.iter().any(|(sx, sy)| {
                let dist = (x - sx).abs() + (y - sy).abs();
                dist < min_settlement_dist
            });

            if too_close_to_settlement {
                continue;
            }

            // Check distance from other POIs
            let min_poi_dist = poi_type.min_poi_distance();
            let too_close_to_poi = existing_pois.iter().any(|p| {
                let (px, py) = p.position;
                let dist = (x - px).abs() + (y - py).abs();
                dist < min_poi_dist
            });

            if too_close_to_poi {
                continue;
            }

            // Generate POI
            let id = existing_pois.len();
            let name = self.name_gen.generate_name(poi_type);

            return Some(POI::new(id, poi_type, name, (x, y)));
        }

        None // Failed to find suitable location
    }
}

/// Helper function to place POIs on an overmap
pub fn place_pois(overmap: &mut Overmap, settlement_positions: &[(i32, i32)]) -> Vec<POI> {
    let mut placer = POIPlacer::new(overmap.seed);
    placer.place_pois(overmap, settlement_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poi_types() {
        let dungeon = POIType::Dungeon;
        assert_eq!(dungeon.name(), "Dungeon");
        assert_eq!(dungeon.min_settlement_distance(), 5);
        assert_eq!(dungeon.min_poi_distance(), 4);
    }

    #[test]
    fn test_poi_creation() {
        let poi = POI::new(
            0,
            POIType::Cave,
            "Test Cave".to_string(),
            (10, 10),
        );

        assert_eq!(poi.id, 0);
        assert_eq!(poi.poi_type, POIType::Cave);
        assert_eq!(poi.position, (10, 10));
        assert!(!poi.discovered);
        assert!(!poi.cleared);
    }

    #[test]
    fn test_name_generator() {
        let mut gen = POINameGenerator::new(12345);

        let name1 = gen.generate_name(POIType::Dungeon);
        let name2 = gen.generate_name(POIType::Cave);

        assert!(!name1.is_empty());
        assert!(!name2.is_empty());
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_deterministic_names() {
        let mut gen1 = POINameGenerator::new(99999);
        let mut gen2 = POINameGenerator::new(99999);

        let name1 = gen1.generate_name(POIType::Dungeon);
        let name2 = gen2.generate_name(POIType::Dungeon);

        assert_eq!(name1, name2);
    }

    #[test]
    fn test_poi_placement() {
        let mut overmap = Overmap::new(50, 50, 12345);

        // Generate terrain first
        use crate::world::generate_terrain;
        generate_terrain(&mut overmap);

        let settlement_positions = vec![(25, 25)];
        let pois = place_pois(&mut overmap, &settlement_positions);

        // Should have placed some POIs
        assert!(!pois.is_empty());

        // Check POIs are marked on overmap
        for poi in pois.iter() {
            let (x, y) = poi.position;
            if let Some(tile) = overmap.get_tile(x, y) {
                assert!(
                    tile.terrain == TerrainType::Dungeon
                    || tile.terrain == TerrainType::SpecialLocation
                );
            }
        }
    }
}
