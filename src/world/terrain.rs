use serde::{Deserialize, Serialize};

/// Terrain types for overmap tiles
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainType {
    // Natural terrain
    Plains,
    Forest,
    DenseForest,
    Hills,
    Mountains,
    Lake,
    River,
    Swamp,

    // Civilized
    Road,
    KingRoad,    // Major road
    TradePath,   // Lesser road

    // Special
    Settlement,  // Town/city location
    Dungeon,     // Dungeon entrance
    SpecialLocation, // Hand-crafted location
}

impl TerrainType {
    /// Whether this terrain type can be walked on
    pub fn is_walkable(&self) -> bool {
        match self {
            TerrainType::Mountains => false,
            TerrainType::Lake => false,
            _ => true,
        }
    }

    /// Travel time modifier (1.0 = base speed)
    pub fn travel_speed(&self) -> f32 {
        match self {
            TerrainType::Road | TerrainType::KingRoad => 2.0,  // Faster
            TerrainType::TradePath => 1.5,
            TerrainType::Plains => 1.0,
            TerrainType::Forest => 0.7,
            TerrainType::DenseForest => 0.5,
            TerrainType::Hills => 0.6,
            TerrainType::Mountains => 0.3,
            TerrainType::Swamp => 0.5,
            TerrainType::River => 0.8,
            TerrainType::Lake => 0.0,  // Can't walk on water
            TerrainType::Settlement => 1.0,
            TerrainType::Dungeon => 1.0,
            TerrainType::SpecialLocation => 1.0,
        }
    }

    /// Display character for rendering
    pub fn glyph(&self) -> char {
        match self {
            TerrainType::Plains => '.',
            TerrainType::Forest => 't',
            TerrainType::DenseForest => 'T',
            TerrainType::Hills => 'n',
            TerrainType::Mountains => '^',
            TerrainType::Lake => '~',
            TerrainType::River => '≈',
            TerrainType::Swamp => '%',
            TerrainType::Road => '=',
            TerrainType::KingRoad => '≡',
            TerrainType::TradePath => '-',
            TerrainType::Settlement => '⌂',
            TerrainType::Dungeon => '▼',
            TerrainType::SpecialLocation => '◆',
        }
    }

    /// Display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            TerrainType::Plains => "Plains",
            TerrainType::Forest => "Forest",
            TerrainType::DenseForest => "Dense Forest",
            TerrainType::Hills => "Hills",
            TerrainType::Mountains => "Mountains",
            TerrainType::Lake => "Lake",
            TerrainType::River => "River",
            TerrainType::Swamp => "Swamp",
            TerrainType::Road => "Road",
            TerrainType::KingRoad => "King's Road",
            TerrainType::TradePath => "Trade Path",
            TerrainType::Settlement => "Settlement",
            TerrainType::Dungeon => "Dungeon",
            TerrainType::SpecialLocation => "Special Location",
        }
    }
}
