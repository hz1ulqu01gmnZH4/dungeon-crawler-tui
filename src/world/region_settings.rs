use super::biome::{Biome, BiomeId, Direction};
use super::poi::POIType;
use super::terrain::TerrainType;

/// Get all core biome definitions with their configuration
pub fn get_core_biomes() -> Vec<Biome> {
    vec![
        create_plains_biome(),
        create_forest_biome(),
        create_deep_wilderness_biome(),
        create_hills_biome(),
        create_mountains_biome(),
        create_wetlands_biome(),
    ]
}

/// Plains: Open grasslands, moderate settlement density
fn create_plains_biome() -> Biome {
    Biome::new(BiomeId::Plains, 100)
        .with_terrain_weights(vec![
            (TerrainType::Plains, 80),
            (TerrainType::Forest, 20),
        ])
        .with_city_size(3, 8)
        .with_city_spacing(5, 10)
        .with_allowed_pois(vec![
            POIType::Ruin,
            POIType::Shrine,
            POIType::Tower,
        ])
        .with_near_biomes(vec![
            (BiomeId::Plains, 1.3),        // Plains cluster together
            (BiomeId::Forest, 1.2),        // Often near forests
            (BiomeId::DeepWilderness, 0.3), // Rarely near deep wilderness
            (BiomeId::Mountains, 0.5),      // Sometimes near mountain foothills
        ])
        .with_directional_weights(vec![
            (Direction::South, 1.2),        // More common in south (temperate)
        ])
}

/// Forest: Wooded areas, low settlement density
fn create_forest_biome() -> Biome {
    Biome::new(BiomeId::Forest, 120)
        .with_terrain_weights(vec![
            (TerrainType::Forest, 60),
            (TerrainType::DenseForest, 20),
            (TerrainType::Plains, 20),
        ])
        .with_city_size(0, 3)
        .with_city_spacing(8, 15)
        .with_allowed_pois(vec![
            POIType::Cave,
            POIType::Shrine,
            POIType::Ruin,
            POIType::Tower,
        ])
        .with_special_limits(vec![
            (POIType::Cave, 8),
        ])
        .with_near_biomes(vec![
            (BiomeId::Forest, 1.5),         // Forests cluster strongly
            (BiomeId::DeepWilderness, 1.3), // Often transition to deep wilderness
            (BiomeId::Plains, 1.1),         // Common border with plains
            (BiomeId::Wetlands, 0.8),       // Sometimes near wetlands
        ])
}

/// Deep Wilderness: Dense untamed forests, no settlements
fn create_deep_wilderness_biome() -> Biome {
    Biome::new(BiomeId::DeepWilderness, 200)
        .with_terrain_weights(vec![
            (TerrainType::DenseForest, 80),
            (TerrainType::Forest, 20),
        ])
        .with_city_size(0, 1)
        .with_city_spacing(15, 25)
        .with_allowed_pois(vec![
            POIType::Dungeon,
            POIType::Cave,
            POIType::Ruin,
            POIType::Shrine,
        ])
        .with_special_limits(vec![
            (POIType::Dungeon, 3),
            (POIType::Cave, 12),
        ])
        .with_near_biomes(vec![
            (BiomeId::DeepWilderness, 1.8), // Deep wilderness clusters very strongly
            (BiomeId::Forest, 1.2),         // Often surrounded by forest
            (BiomeId::Plains, 0.0),         // Never directly adjacent to plains
            (BiomeId::Mountains, 0.7),      // Sometimes in mountain valleys
        ])
        .with_directional_weights(vec![
            (Direction::North, 1.3),        // More common in the north (remote)
            (Direction::NorthWest, 1.2),
            (Direction::NorthEast, 1.2),
        ])
}

/// Hills: Rolling terrain, sparse settlements
fn create_hills_biome() -> Biome {
    Biome::new(BiomeId::Hills, 90)
        .with_terrain_weights(vec![
            (TerrainType::Hills, 60),
            (TerrainType::Plains, 20),
            (TerrainType::Forest, 15),
            (TerrainType::Mountains, 5),
        ])
        .with_city_size(1, 4)
        .with_city_spacing(7, 12)
        .with_allowed_pois(vec![
            POIType::Cave,
            POIType::Tower,
            POIType::Ruin,
            POIType::TombCrypt,
        ])
        .with_special_limits(vec![
            (POIType::Cave, 6),
        ])
        .with_near_biomes(vec![
            (BiomeId::Hills, 1.4),          // Hills form ranges
            (BiomeId::Mountains, 1.5),      // Often near mountains (foothills)
            (BiomeId::Plains, 1.2),         // Common transition from plains
            (BiomeId::Forest, 1.0),         // Neutral with forests
        ])
}

/// Mountains: High peaks, minimal settlements, rich in resources
fn create_mountains_biome() -> Biome {
    Biome::new(BiomeId::Mountains, 80)
        .with_terrain_weights(vec![
            (TerrainType::Mountains, 80),
            (TerrainType::Hills, 20),
        ])
        .with_city_size(0, 2)
        .with_city_spacing(12, 20)
        .with_allowed_pois(vec![
            POIType::Cave,
            POIType::Dungeon,
            POIType::Tower,
            POIType::Shrine,
        ])
        .with_special_limits(vec![
            (POIType::Cave, 15),
            (POIType::Dungeon, 2),
        ])
        .with_near_biomes(vec![
            (BiomeId::Mountains, 1.6),      // Mountains form ranges
            (BiomeId::Hills, 1.5),          // Always have foothills
            (BiomeId::Plains, 0.5),         // Rarely directly adjacent
            (BiomeId::DeepWilderness, 0.8), // Sometimes border deep wilderness
        ])
        .with_directional_weights(vec![
            (Direction::North, 1.5),        // Mountains in the north
            (Direction::NorthEast, 1.3),
            (Direction::NorthWest, 1.3),
            (Direction::South, 0.6),        // Less common in south
        ])
}

/// Wetlands: Swamps and marshes, no settlements, dangerous
fn create_wetlands_biome() -> Biome {
    Biome::new(BiomeId::Wetlands, 70)
        .with_terrain_weights(vec![
            (TerrainType::Swamp, 70),
            (TerrainType::Lake, 20),
            (TerrainType::Plains, 10),
        ])
        .with_city_size(0, 1)
        .with_city_spacing(15, 25)
        .with_allowed_pois(vec![
            POIType::Ruin,
            POIType::Shrine,
            POIType::Cave,
            POIType::TombCrypt,
        ])
        .with_special_limits(vec![
            (POIType::Ruin, 4),
            (POIType::TombCrypt, 3),
        ])
        .with_near_biomes(vec![
            (BiomeId::Wetlands, 1.4),       // Wetlands cluster
            (BiomeId::Forest, 1.1),         // Often near forests
            (BiomeId::Plains, 0.8),         // Sometimes near plains (river deltas)
            (BiomeId::Mountains, 0.3),      // Rarely near mountains
            (BiomeId::DeepWilderness, 0.9),
        ])
        .with_directional_weights(vec![
            (Direction::South, 1.2),        // More common in lowlands (south)
            (Direction::SouthEast, 1.1),
            (Direction::SouthWest, 1.1),
        ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_core_biomes_count() {
        let biomes = get_core_biomes();
        assert_eq!(biomes.len(), 6);
    }

    #[test]
    fn test_all_biomes_have_unique_ids() {
        let biomes = get_core_biomes();
        let ids: Vec<BiomeId> = biomes.iter().map(|b| b.id).collect();

        // Check all biomes are present
        assert!(ids.contains(&BiomeId::Plains));
        assert!(ids.contains(&BiomeId::Forest));
        assert!(ids.contains(&BiomeId::DeepWilderness));
        assert!(ids.contains(&BiomeId::Hills));
        assert!(ids.contains(&BiomeId::Mountains));
        assert!(ids.contains(&BiomeId::Wetlands));
    }

    #[test]
    fn test_plains_biome_config() {
        let biome = create_plains_biome();
        assert_eq!(biome.id, BiomeId::Plains);
        assert_eq!(biome.weight, 100);
        assert!(biome.terrain_weight(TerrainType::Plains) > 0);
        assert!(biome.allows_poi(POIType::Ruin));
        assert_eq!(biome.city_size_range, (3, 8));
    }

    #[test]
    fn test_deep_wilderness_has_no_cities() {
        let biome = create_deep_wilderness_biome();
        assert_eq!(biome.city_size_range.1, 1); // Max 1 building (essentially no cities)
        assert!(biome.allows_poi(POIType::Dungeon));
    }

    #[test]
    fn test_mountains_rich_in_caves() {
        let biome = create_mountains_biome();
        assert!(biome.allows_poi(POIType::Cave));
        assert_eq!(biome.poi_limit(POIType::Cave), Some(15));
    }

    #[test]
    fn test_all_biomes_have_terrain_weights() {
        let biomes = get_core_biomes();
        for biome in biomes {
            assert!(
                biome.total_terrain_weight() > 0,
                "Biome {:?} has no terrain weights",
                biome.id
            );
        }
    }

    #[test]
    fn test_neighbor_multipliers_make_sense() {
        let plains = create_plains_biome();
        // Plains should encourage itself and forests, discourage deep wilderness
        assert!(plains.neighbor_multiplier(BiomeId::Plains) > 1.0);
        assert!(plains.neighbor_multiplier(BiomeId::DeepWilderness) < 1.0);
    }

    #[test]
    fn test_directional_weights_applied() {
        let mountains = create_mountains_biome();
        // Mountains should be more common in the north
        assert!(mountains.directional_multiplier(Direction::North) > 1.0);
        assert!(mountains.directional_multiplier(Direction::South) < 1.0);
    }
}
