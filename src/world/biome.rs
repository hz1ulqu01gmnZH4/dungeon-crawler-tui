use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::terrain::TerrainType;
use super::poi::POIType;

/// Unique identifier for biome types
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum BiomeId {
    Plains,
    Forest,
    DeepWilderness,
    Hills,
    Mountains,
    Wetlands,
}

impl BiomeId {
    pub fn name(&self) -> &'static str {
        match self {
            BiomeId::Plains => "Plains",
            BiomeId::Forest => "Forest",
            BiomeId::DeepWilderness => "Deep Wilderness",
            BiomeId::Hills => "Hills",
            BiomeId::Mountains => "Mountains",
            BiomeId::Wetlands => "Wetlands",
        }
    }

    /// Get all biome types
    pub fn all() -> [BiomeId; 6] {
        [
            BiomeId::Plains,
            BiomeId::Forest,
            BiomeId::DeepWilderness,
            BiomeId::Hills,
            BiomeId::Mountains,
            BiomeId::Wetlands,
        ]
    }
}

/// Cardinal and ordinal directions for directional biome preferences
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }

    /// Get the offset (dx, dy) for this direction
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }
}

// POIType is imported from super::poi and reused here for biome configuration

/// Biome definition with generation parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Biome {
    /// Unique identifier for this biome
    pub id: BiomeId,

    /// Base weight for biome selection (higher = more common)
    pub weight: u32,

    /// Terrain type weights for this biome (terrain -> weight)
    pub terrain_weights: HashMap<TerrainType, u32>,

    /// Settlement size range (min, max buildings)
    pub city_size_range: (i32, i32),

    /// Settlement spacing range (min, max tiles between settlements)
    pub city_spacing_range: (i32, i32),

    /// POI types allowed in this biome
    pub allowed_pois: Vec<POIType>,

    /// Maximum count for specific POI types (POI -> max count)
    pub special_limits: HashMap<POIType, u32>,

    /// Neighboring biome influence (biome -> multiplier)
    /// Multiplier > 1.0 = encourages this biome near target
    /// Multiplier < 1.0 = discourages this biome near target
    /// Multiplier = 0.0 = prevents this biome near target
    pub near_biomes: HashMap<BiomeId, f32>,

    /// Directional preferences (direction -> multiplier)
    /// Used to create geographic features (e.g., mountains in the north)
    pub directional_weights: HashMap<Direction, f32>,
}

impl Biome {
    /// Create a new biome with default values
    pub fn new(id: BiomeId, weight: u32) -> Self {
        Self {
            id,
            weight,
            terrain_weights: HashMap::new(),
            city_size_range: (0, 0),
            city_spacing_range: (10, 20),
            allowed_pois: Vec::new(),
            special_limits: HashMap::new(),
            near_biomes: HashMap::new(),
            directional_weights: HashMap::new(),
        }
    }

    /// Builder: Set terrain weights
    pub fn with_terrain_weights(mut self, weights: Vec<(TerrainType, u32)>) -> Self {
        self.terrain_weights = weights.into_iter().collect();
        self
    }

    /// Builder: Set city size range
    pub fn with_city_size(mut self, min: i32, max: i32) -> Self {
        self.city_size_range = (min, max);
        self
    }

    /// Builder: Set city spacing range
    pub fn with_city_spacing(mut self, min: i32, max: i32) -> Self {
        self.city_spacing_range = (min, max);
        self
    }

    /// Builder: Set allowed POIs
    pub fn with_allowed_pois(mut self, pois: Vec<POIType>) -> Self {
        self.allowed_pois = pois;
        self
    }

    /// Builder: Set special limits
    pub fn with_special_limits(mut self, limits: Vec<(POIType, u32)>) -> Self {
        self.special_limits = limits.into_iter().collect();
        self
    }

    /// Builder: Set neighboring biome influence
    pub fn with_near_biomes(mut self, near: Vec<(BiomeId, f32)>) -> Self {
        self.near_biomes = near.into_iter().collect();
        self
    }

    /// Builder: Set directional weights
    pub fn with_directional_weights(mut self, weights: Vec<(Direction, f32)>) -> Self {
        self.directional_weights = weights.into_iter().collect();
        self
    }

    /// Get terrain weight for a specific terrain type (returns 0 if not set)
    pub fn terrain_weight(&self, terrain: TerrainType) -> u32 {
        self.terrain_weights.get(&terrain).copied().unwrap_or(0)
    }

    /// Get total terrain weight (sum of all terrain weights)
    pub fn total_terrain_weight(&self) -> u32 {
        self.terrain_weights.values().sum()
    }

    /// Check if a POI type is allowed in this biome
    pub fn allows_poi(&self, poi: POIType) -> bool {
        self.allowed_pois.contains(&poi)
    }

    /// Get special limit for a POI type (returns None if no limit)
    pub fn poi_limit(&self, poi: POIType) -> Option<u32> {
        self.special_limits.get(&poi).copied()
    }

    /// Get neighboring biome multiplier (returns 1.0 if not set)
    pub fn neighbor_multiplier(&self, biome: BiomeId) -> f32 {
        self.near_biomes.get(&biome).copied().unwrap_or(1.0)
    }

    /// Get directional weight multiplier (returns 1.0 if not set)
    pub fn directional_multiplier(&self, direction: Direction) -> f32 {
        self.directional_weights.get(&direction).copied().unwrap_or(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biome_id_all() {
        let biomes = BiomeId::all();
        assert_eq!(biomes.len(), 6);
        assert!(biomes.contains(&BiomeId::Plains));
        assert!(biomes.contains(&BiomeId::Mountains));
    }

    #[test]
    fn test_biome_id_name() {
        assert_eq!(BiomeId::Plains.name(), "Plains");
        assert_eq!(BiomeId::DeepWilderness.name(), "Deep Wilderness");
    }

    #[test]
    fn test_direction_offset() {
        assert_eq!(Direction::North.offset(), (0, -1));
        assert_eq!(Direction::East.offset(), (1, 0));
        assert_eq!(Direction::SouthWest.offset(), (-1, 1));
    }

    #[test]
    fn test_biome_creation() {
        let biome = Biome::new(BiomeId::Plains, 100);
        assert_eq!(biome.id, BiomeId::Plains);
        assert_eq!(biome.weight, 100);
        assert_eq!(biome.terrain_weights.len(), 0);
    }

    #[test]
    fn test_biome_builder() {
        let biome = Biome::new(BiomeId::Forest, 150)
            .with_terrain_weights(vec![
                (TerrainType::Forest, 60),
                (TerrainType::DenseForest, 30),
            ])
            .with_city_size(0, 3)
            .with_city_spacing(8, 15)
            .with_allowed_pois(vec![POIType::Cave, POIType::Shrine])
            .with_near_biomes(vec![
                (BiomeId::Forest, 1.5),
                (BiomeId::Plains, 0.5),
            ]);

        assert_eq!(biome.weight, 150);
        assert_eq!(biome.terrain_weight(TerrainType::Forest), 60);
        assert_eq!(biome.terrain_weight(TerrainType::DenseForest), 30);
        assert_eq!(biome.terrain_weight(TerrainType::Plains), 0); // Not set
        assert_eq!(biome.total_terrain_weight(), 90);
        assert_eq!(biome.city_size_range, (0, 3));
        assert!(biome.allows_poi(POIType::Cave));
        assert!(!biome.allows_poi(POIType::Dungeon));
        assert_eq!(biome.neighbor_multiplier(BiomeId::Forest), 1.5);
        assert_eq!(biome.neighbor_multiplier(BiomeId::Mountains), 1.0); // Default
    }

    #[test]
    fn test_poi_limit() {
        let biome = Biome::new(BiomeId::Mountains, 100)
            .with_special_limits(vec![
                (POIType::Cave, 10),
                (POIType::Dungeon, 5),
            ]);

        assert_eq!(biome.poi_limit(POIType::Cave), Some(10));
        assert_eq!(biome.poi_limit(POIType::Dungeon), Some(5));
        assert_eq!(biome.poi_limit(POIType::Shrine), None);
    }

    #[test]
    fn test_directional_multiplier() {
        let biome = Biome::new(BiomeId::Mountains, 100)
            .with_directional_weights(vec![
                (Direction::North, 2.0),
                (Direction::South, 0.5),
            ]);

        assert_eq!(biome.directional_multiplier(Direction::North), 2.0);
        assert_eq!(biome.directional_multiplier(Direction::South), 0.5);
        assert_eq!(biome.directional_multiplier(Direction::East), 1.0); // Default
    }
}
