use dungeon_clawler_tui::world::{Overmap, BiomeId, TerrainType, get_core_biomes};
use std::fs;

#[test]
fn test_overmap_tiles_have_biome_data() {
    let overmap = Overmap::new(10, 10, 12345);

    // All tiles should have default biome (Plains)
    for tile in overmap.tiles.iter() {
        assert_eq!(tile.biome, BiomeId::Plains);
        assert_eq!(tile.biome_strength, 1.0);
    }
}

#[test]
fn test_overmap_tile_with_biome_constructor() {
    let tile = dungeon_clawler_tui::world::OvermapTile::with_biome(
        5,
        5,
        TerrainType::Forest,
        BiomeId::Forest,
        0.8
    );

    assert_eq!(tile.x, 5);
    assert_eq!(tile.y, 5);
    assert_eq!(tile.terrain, TerrainType::Forest);
    assert_eq!(tile.biome, BiomeId::Forest);
    assert_eq!(tile.biome_strength, 0.8);
}

#[test]
fn test_biome_strength_clamping() {
    // Test that biome_strength is clamped to [0.0, 1.0]
    let tile1 = dungeon_clawler_tui::world::OvermapTile::with_biome(
        0, 0, TerrainType::Plains, BiomeId::Plains, 1.5
    );
    assert_eq!(tile1.biome_strength, 1.0);

    let tile2 = dungeon_clawler_tui::world::OvermapTile::with_biome(
        0, 0, TerrainType::Plains, BiomeId::Plains, -0.5
    );
    assert_eq!(tile2.biome_strength, 0.0);
}

#[test]
fn test_core_biomes_available() {
    let biomes = get_core_biomes();

    assert_eq!(biomes.len(), 6);

    // Verify all biome IDs are present
    let ids: Vec<BiomeId> = biomes.iter().map(|b| b.id).collect();
    assert!(ids.contains(&BiomeId::Plains));
    assert!(ids.contains(&BiomeId::Forest));
    assert!(ids.contains(&BiomeId::DeepWilderness));
    assert!(ids.contains(&BiomeId::Hills));
    assert!(ids.contains(&BiomeId::Mountains));
    assert!(ids.contains(&BiomeId::Wetlands));
}

#[test]
fn test_biome_data_serialization() {
    use dungeon_clawler_tui::{Resources, quick_save, quick_load};
    use hecs::World;

    // Create a game world with biome data
    let mut world = World::new();
    let mut resources = Resources::new(80, 50, 12345);

    // Modify some overmap tiles to have different biomes
    if let Some(tile) = resources.world.overmap.get_tile_mut(5, 5) {
        tile.biome = BiomeId::Forest;
        tile.biome_strength = 0.75;
    }

    if let Some(tile) = resources.world.overmap.get_tile_mut(10, 10) {
        tile.biome = BiomeId::Mountains;
        tile.biome_strength = 0.9;
    }

    let original_seed = 12345;

    // Save the game
    quick_save(&world, &resources, original_seed).expect("Failed to save game");

    // Load the game
    let mut loaded_world = World::new();
    let mut loaded_resources = Resources::new(80, 50, 99999); // Different seed initially
    let loaded_seed = quick_load(&mut loaded_world, &mut loaded_resources).expect("Failed to load game");

    // Verify seed was restored
    assert_eq!(loaded_seed, original_seed);

    // Verify biome data was preserved
    let tile1 = loaded_resources.world.overmap.get_tile(5, 5).unwrap();
    assert_eq!(tile1.biome, BiomeId::Forest);
    assert_eq!(tile1.biome_strength, 0.75);

    let tile2 = loaded_resources.world.overmap.get_tile(10, 10).unwrap();
    assert_eq!(tile2.biome, BiomeId::Mountains);
    assert_eq!(tile2.biome_strength, 0.9);

    // Cleanup
    fs::remove_file("quicksave.json.gz").ok();
}

#[test]
fn test_biomes_have_terrain_preferences() {
    let biomes = get_core_biomes();

    for biome in biomes.iter() {
        // Every biome should have at least one terrain type with weight > 0
        assert!(
            biome.total_terrain_weight() > 0,
            "Biome {:?} has no terrain weights",
            biome.id
        );

        // Biome should have settlement spacing configured
        let (min_spacing, max_spacing) = biome.city_spacing_range;
        assert!(min_spacing <= max_spacing, "Biome {:?} has invalid city spacing", biome.id);
    }
}

#[test]
fn test_biome_neighbor_influences() {
    let biomes = get_core_biomes();

    // Find the DeepWilderness biome
    let deep_wilderness = biomes.iter()
        .find(|b| b.id == BiomeId::DeepWilderness)
        .expect("DeepWilderness biome not found");

    // Deep Wilderness should have strong self-clustering
    assert!(deep_wilderness.neighbor_multiplier(BiomeId::DeepWilderness) > 1.0);

    // Deep Wilderness should never be adjacent to Plains
    assert_eq!(deep_wilderness.neighbor_multiplier(BiomeId::Plains), 0.0);
}

#[test]
fn test_mountain_biome_directional_preference() {
    let biomes = get_core_biomes();

    // Find the Mountains biome
    let mountains = biomes.iter()
        .find(|b| b.id == BiomeId::Mountains)
        .expect("Mountains biome not found");

    // Mountains should prefer northern directions
    let north_mult = mountains.directional_multiplier(dungeon_clawler_tui::world::Direction::North);
    let south_mult = mountains.directional_multiplier(dungeon_clawler_tui::world::Direction::South);

    assert!(north_mult > south_mult, "Mountains should prefer north over south");
}
