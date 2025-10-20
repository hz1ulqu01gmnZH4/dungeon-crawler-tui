use crate::ecs::{
    Position, Renderable, Player, CombatStats, TriMeter, Viewshed,
    Name, Monster, BlocksMovement, Uid,
};
use crate::ecs::resources::Resources;
use crate::world::{Overmap, Settlement, WorldTime};
use crate::map::Map;
use crate::domain_types::Depth;
use hecs::World;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

/// Version for save file compatibility
const SAVE_VERSION: u32 = 2;

/// Serializable entity data
#[derive(Serialize, Deserialize)]
struct EntityData {
    // Unique identifier
    uid: Uid,

    // Required components
    position: Option<Position>,
    renderable: Option<Renderable>,
    name: Option<String>,

    // Character components
    player: Option<Player>,
    monster: Option<Monster>,
    combat_stats: Option<CombatStats>,
    tri_meter: Option<TriMeter>,
    viewshed: Option<Viewshed>,
    blocks_movement: Option<BlocksMovement>,
}

/// Complete save game state
#[derive(Serialize, Deserialize)]
pub struct SaveGame {
    version: u32,

    // World state
    overmap: Overmap,
    settlements: Vec<Settlement>,
    world_time: WorldTime,

    // Player state
    player_overmap_pos: (i32, i32),
    current_location: Option<usize>,

    // Dungeon state
    dungeon_levels: HashMap<i32, Map>,
    current_depth: i32,

    // ECS state
    entities: Vec<EntityData>,
    player_entity_index: Option<usize>,

    // Game state
    seed: u64,
    message_log: Vec<String>,
}

impl SaveGame {
    /// Create a SaveGame from current game state
    pub fn from_game(world: &World, resources: &Resources, seed: u64) -> Self {
        let mut entities = Vec::new();
        let mut player_entity_index = None;

        // Serialize all entities
        for (idx, entity_ref) in world.iter().enumerate() {
            let entity = entity_ref.entity();

            // Get or create Uid for this entity
            let uid = world.get::<&Uid>(entity).ok().map(|u| *u).unwrap_or_else(|| Uid::new());

            let entity_data = EntityData {
                uid,
                position: world.get::<&Position>(entity).ok().map(|c| *c),
                renderable: world.get::<&Renderable>(entity).ok().map(|c| *c),
                name: world.get::<&Name>(entity).ok().map(|n| n.0.clone()),
                player: world.get::<&Player>(entity).ok().map(|c| *c),
                monster: world.get::<&Monster>(entity).ok().map(|c| *c),
                combat_stats: world.get::<&CombatStats>(entity).ok().map(|c| *c),
                tri_meter: world.get::<&TriMeter>(entity).ok().map(|c| *c),
                viewshed: world.get::<&Viewshed>(entity).ok().map(|v| (*v).clone()),
                blocks_movement: world.get::<&BlocksMovement>(entity).ok().map(|c| *c),
            };

            // Track player entity index
            if entity_data.player.is_some() {
                player_entity_index = Some(idx);
            }

            entities.push(entity_data);
        }

        // Collect dungeon levels from both sources, preferring map_cache
        let mut dungeon_levels_merged: HashMap<Depth, Map> = resources.world.dungeon_levels.clone();

        // Overlay map_cache dungeons (these take precedence)
        for (depth, map) in resources.world.map_cache.dungeons() {
            dungeon_levels_merged.insert(*depth, map.clone());
        }

        // Convert HashMap<Depth, Map> to HashMap<i32, Map> for serialization
        let dungeon_levels: HashMap<i32, Map> = dungeon_levels_merged
            .iter()
            .map(|(depth, map)| (depth.value(), map.clone()))
            .collect();

        SaveGame {
            version: SAVE_VERSION,
            overmap: resources.world.overmap.clone(),
            settlements: resources.world.settlements.clone(),
            world_time: resources.sim.world_time.clone(),
            player_overmap_pos: resources.world.player_overmap_pos,
            current_location: resources.world.current_location,
            dungeon_levels,
            current_depth: resources.world.current_depth.value(),
            entities,
            player_entity_index,
            seed,
            message_log: resources.ui.log.messages.clone(),
        }
    }

    /// Restore game state from SaveGame
    pub fn restore_game(
        &self,
        world: &mut World,
        resources: &mut Resources,
    ) -> anyhow::Result<()> {
        // Clear existing world
        world.clear();

        // Restore world state
        resources.world.overmap = self.overmap.clone();
        resources.world.settlements = self.settlements.clone();
        resources.sim.world_time = self.world_time.clone();
        resources.world.player_overmap_pos = self.player_overmap_pos;
        resources.world.current_location = self.current_location;

        // Restore dungeon state - convert HashMap<i32, Map> to HashMap<Depth, Map>
        resources.world.dungeon_levels = self.dungeon_levels
            .iter()
            .map(|(depth_val, map)| (Depth(*depth_val), map.clone()))
            .collect();
        resources.world.current_depth = Depth(self.current_depth);

        // Sync dungeon levels to map_cache
        for (depth, map) in &resources.world.dungeon_levels {
            resources.world.map_cache.insert(crate::map::MapId::Dungeon(*depth), map.clone());
        }

        // Restore message log
        resources.ui.log.messages = self.message_log.clone();

        // Recreate entities
        let mut player_entity = None;
        for (idx, entity_data) in self.entities.iter().enumerate() {
            let mut builder = hecs::EntityBuilder::new();

            // Restore Uid and ensure counter is updated
            builder.add(Uid::from_raw(entity_data.uid.0));

            if let Some(pos) = entity_data.position {
                builder.add(pos);
            }
            if let Some(rend) = entity_data.renderable {
                builder.add(rend);
            }
            if let Some(name) = &entity_data.name {
                builder.add(Name(name.clone()));
            }
            if let Some(player) = entity_data.player {
                builder.add(player);
            }
            if let Some(monster) = entity_data.monster {
                builder.add(monster);
            }
            if let Some(stats) = entity_data.combat_stats {
                builder.add(stats);
            }
            if let Some(tri) = entity_data.tri_meter {
                builder.add(tri);
            }
            if let Some(vs) = &entity_data.viewshed {
                builder.add(vs.clone());
            }
            if let Some(blocks) = entity_data.blocks_movement {
                builder.add(blocks);
            }

            let entity = world.spawn(builder.build());

            // Track player entity
            if Some(idx) == self.player_entity_index {
                player_entity = Some(entity);
            }
        }

        resources.player.player_entity = player_entity;

        Ok(())
    }

    /// Save game to file (uncompressed JSON)
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Save game to file with gzip compression
    pub fn save_to_file_compressed<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let json = serde_json::to_string(self)?;
        let file = File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::best());
        encoder.write_all(json.as_bytes())?;
        encoder.finish()?;
        Ok(())
    }

    /// Load game from file (uncompressed JSON)
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let json = fs::read_to_string(path)?;
        let save_game: SaveGame = serde_json::from_str(&json)?;

        // Version check
        if save_game.version != SAVE_VERSION {
            return Err(anyhow::anyhow!(
                "Save file version mismatch: expected {}, found {}",
                SAVE_VERSION,
                save_game.version
            ));
        }

        Ok(save_game)
    }

    /// Load game from compressed file
    pub fn load_from_file_compressed<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let mut decoder = GzDecoder::new(file);
        let mut json = String::new();
        decoder.read_to_string(&mut json)?;
        let save_game: SaveGame = serde_json::from_str(&json)?;

        // Version check
        if save_game.version != SAVE_VERSION {
            return Err(anyhow::anyhow!(
                "Save file version mismatch: expected {}, found {}",
                SAVE_VERSION,
                save_game.version
            ));
        }

        Ok(save_game)
    }

    /// Get the size of the save data in bytes (uncompressed)
    pub fn uncompressed_size(&self) -> anyhow::Result<usize> {
        let json = serde_json::to_string(self)?;
        Ok(json.len())
    }

    /// Get the size of the save data in bytes (compressed)
    pub fn compressed_size(&self) -> anyhow::Result<usize> {
        let json = serde_json::to_string(self)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(json.as_bytes())?;
        let compressed = encoder.finish()?;
        Ok(compressed.len())
    }
}

/// Quick save to default location
pub fn quick_save(world: &World, resources: &Resources, seed: u64) -> anyhow::Result<()> {
    let save_game = SaveGame::from_game(world, resources, seed);
    save_game.save_to_file("savegame.json")?;
    Ok(())
}

/// Quick load from default location
pub fn quick_load(world: &mut World, resources: &mut Resources) -> anyhow::Result<u64> {
    let save_game = SaveGame::load_from_file("savegame.json")?;
    save_game.restore_game(world, resources)?;
    Ok(save_game.seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::resources::RunMode;
    use crate::domain_types::{Defense, MaxHitPoints, Power};

    #[test]
    fn test_save_version() {
        assert_eq!(SAVE_VERSION, 2);
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        // Create a simple world and resources
        let mut world = World::new();
        let resources = Resources::new(80, 50, 12345);

        // Create the save
        let save_game = SaveGame::from_game(&world, &resources, 12345);

        // Verify basic fields
        assert_eq!(save_game.version, SAVE_VERSION);
        assert_eq!(save_game.seed, 12345);
        assert_eq!(save_game.player_overmap_pos, (25, 25)); // Center of 50x50 map

        // Restore to new world/resources
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);

        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify restoration
        assert_eq!(new_resources.world.player_overmap_pos, (25, 25));
    }

    #[test]
    fn test_save_load_entities() {
        use crate::ecs::{Player, Position, Renderable, Name, CombatStats, RealityLayer};

        let mut world = World::new();
        let resources = Resources::new(80, 50, 12345);

        // Spawn player entity with multiple components
        let player = world.spawn((
            Player,
            Position::new(10, 15, RealityLayer::Normal),
            Renderable {
                glyph: '@',
                fg: ratatui::style::Color::Yellow,
                bg: ratatui::style::Color::Reset,
                z: 10,
            },
            Name("Player".to_string()),
            CombatStats::new(30, 5, 2),
        ));

        // Spawn monster
        let monster = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Renderable {
                glyph: 'g',
                fg: ratatui::style::Color::Red,
                bg: ratatui::style::Color::Reset,
                z: 5,
            },
            Name("Goblin".to_string()),
            CombatStats::new(10, 3, 1),
        ));

        // Save
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        assert_eq!(save_game.entities.len(), 2, "Should save 2 entities");

        // Load into new world
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify entity count
        let entity_count = new_world.iter().count();
        assert_eq!(entity_count, 2, "Should restore 2 entities");

        // Verify player exists and has correct components
        let mut found_player = false;
        for (_, (player_tag, pos, name, stats)) in new_world.query::<(&Player, &Position, &Name, &CombatStats)>().iter() {
            found_player = true;
            assert_eq!(pos.x, 10);
            assert_eq!(pos.y, 15);
            assert_eq!(pos.layer, RealityLayer::Normal);
            assert_eq!(name.0, "Player");
            assert_eq!(stats.max_hp, MaxHitPoints::new(30));
            assert_eq!(stats.power, Power::new(5));
            assert_eq!(stats.defense, Defense::new(2));
        }
        assert!(found_player, "Should find restored player");
    }

    #[test]
    fn test_save_load_dungeon_levels() {
        use crate::map::{Map, Tile};

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create and add some dungeon levels
        let mut dungeon_level_1 = Map::new(50, 50);
        let idx = dungeon_level_1.xy_idx(10, 10);
        dungeon_level_1.tiles[idx] = Tile::StairsDown;
        resources.world.dungeon_levels.insert(Depth(1), dungeon_level_1);

        let mut dungeon_level_2 = Map::new(50, 50);
        let idx = dungeon_level_2.xy_idx(20, 20);
        dungeon_level_2.tiles[idx] = Tile::StairsUp;
        resources.world.dungeon_levels.insert(Depth(2), dungeon_level_2);

        resources.world.current_depth = Depth(2);

        // Save
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        assert_eq!(save_game.dungeon_levels.len(), 2, "Should save 2 dungeon levels");
        assert_eq!(save_game.current_depth, 2);

        // Load
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify dungeon levels restored
        assert_eq!(new_resources.world.dungeon_levels.len(), 2, "Should restore 2 dungeon levels");
        assert_eq!(new_resources.world.current_depth, Depth(2));

        // Verify tiles in dungeon levels
        let level_1 = new_resources.world.dungeon_levels.get(&Depth(1)).unwrap();
        let idx = level_1.xy_idx(10, 10);
        assert_eq!(level_1.tiles[idx], Tile::StairsDown);

        let level_2 = new_resources.world.dungeon_levels.get(&Depth(2)).unwrap();
        let idx = level_2.xy_idx(20, 20);
        assert_eq!(level_2.tiles[idx], Tile::StairsUp);
    }

    #[test]
    fn test_save_load_world_time() {
        use crate::world::WorldTime;

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Advance time
        resources.sim.world_time.advance_minutes(1500); // More than a day

        let initial_time = resources.sim.world_time.total_minutes();

        // Save and load
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify time preserved
        assert_eq!(new_resources.sim.world_time.total_minutes(), initial_time);
    }

    #[test]
    fn test_save_load_player_position() {
        use crate::ecs::{Player, Position, Name, RealityLayer};

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Spawn player at specific position
        let player = world.spawn((
            Player,
            Position::new(42, 73, RealityLayer::Cosmic),
            Name("Player".to_string()),
        ));

        resources.player.player_entity = Some(player);

        // Save and load
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Find player and verify position
        let mut found = false;
        for (_, (_, pos)) in new_world.query::<(&Player, &Position)>().iter() {
            assert_eq!(pos.x, 42);
            assert_eq!(pos.y, 73);
            assert_eq!(pos.layer, RealityLayer::Cosmic);
            found = true;
        }
        assert!(found, "Player should be restored");
    }

    #[test]
    fn test_save_load_empty_dungeon_levels() {
        // Test with no dungeon levels (surface only)
        let mut world = World::new();
        let resources = Resources::new(80, 50, 12345);

        assert_eq!(resources.world.dungeon_levels.len(), 0);
        assert_eq!(resources.world.current_depth, Depth::SURFACE);

        let save_game = SaveGame::from_game(&world, &resources, 12345);
        assert_eq!(save_game.dungeon_levels.len(), 0);

        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        assert_eq!(new_resources.world.dungeon_levels.len(), 0);
        assert_eq!(new_resources.world.current_depth, Depth::SURFACE);
    }

    #[test]
    fn test_save_load_file_io() {
        use std::fs;

        let mut world = World::new();
        let resources = Resources::new(80, 50, 12345);

        // Save to file
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        save_game.save_to_file("test_save.json").unwrap();

        // Verify file exists
        assert!(std::path::Path::new("test_save.json").exists());

        // Load from file
        let loaded = SaveGame::load_from_file("test_save.json").unwrap();
        assert_eq!(loaded.version, SAVE_VERSION);
        assert_eq!(loaded.seed, 12345);

        // Cleanup
        let _ = fs::remove_file("test_save.json");
    }

    #[test]
    fn test_save_load_message_log() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        resources.ui.log.add("Test message 1");
        resources.ui.log.add("Test message 2");
        resources.ui.log.add("Test message 3");

        let save_game = SaveGame::from_game(&world, &resources, 12345);
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        assert_eq!(new_resources.ui.log.messages.len(), 3);
        assert_eq!(new_resources.ui.log.messages[0], "Test message 1");
        assert_eq!(new_resources.ui.log.messages[2], "Test message 3");
    }

    #[test]
    fn test_compressed_save_load() {
        use std::fs;

        let mut world = World::new();
        let resources = Resources::new(80, 50, 12345);

        // Save to compressed file
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        save_game.save_to_file_compressed("test_save.gz").unwrap();

        // Verify file exists
        assert!(std::path::Path::new("test_save.gz").exists());

        // Load from compressed file
        let loaded = SaveGame::load_from_file_compressed("test_save.gz").unwrap();
        assert_eq!(loaded.version, SAVE_VERSION);
        assert_eq!(loaded.seed, 12345);

        // Cleanup
        let _ = fs::remove_file("test_save.gz");
    }

    #[test]
    fn test_compression_ratio() {
        use crate::ecs::{Player, Position, Renderable, Name, CombatStats, Monster, RealityLayer};

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create a more complex game state to test compression
        for i in 0..50 {
            world.spawn((
                Monster,
                Position::new(i * 2, i * 3, RealityLayer::Normal),
                Renderable {
                    glyph: 'g',
                    fg: ratatui::style::Color::Green,
                    bg: ratatui::style::Color::Reset,
                    z: 5,
                },
                Name(format!("Goblin {}", i)),
                CombatStats::new(10, 3, 1),
            ));
        }

        // Add some log messages
        for i in 0..20 {
            resources.ui.log.add(format!("Message number {}", i));
        }

        let save_game = SaveGame::from_game(&world, &resources, 12345);

        let uncompressed = save_game.uncompressed_size().unwrap();
        let compressed = save_game.compressed_size().unwrap();

        // Compression should provide meaningful reduction
        println!("Uncompressed: {} bytes", uncompressed);
        println!("Compressed: {} bytes", compressed);
        println!("Compression ratio: {:.2}%", (compressed as f64 / uncompressed as f64) * 100.0);

        // Verify compression achieves at least some reduction
        assert!(compressed < uncompressed);
        // With JSON and repetitive game data, we should get at least 30% compression
        assert!(compressed < uncompressed * 70 / 100);
    }

    #[test]
    fn test_stress_save_load_with_many_entities() {
        use crate::ecs::{Position, Monster, Renderable, Name, CombatStats, Item, ItemData, RealityLayer};

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create 200 entities (mix of monsters and items)
        for i in 0..100 {
            world.spawn((
                Monster,
                Position::new(i % 80, i / 80, RealityLayer::Normal),
                Renderable {
                    glyph: 'g',
                    fg: ratatui::style::Color::Green,
                    bg: ratatui::style::Color::Reset,
                    z: 5,
                },
                Name(format!("Monster {}", i)),
                CombatStats::new(10, 3, 1),
                Viewshed::new(8),
            ));
        }

        for i in 0..100 {
            world.spawn((
                Item,
                Position::new((i * 2) % 80, (i * 3) % 50, RealityLayer::Normal),
                Renderable {
                    glyph: '!',
                    fg: ratatui::style::Color::Yellow,
                    bg: ratatui::style::Color::Reset,
                    z: 1,
                },
                Name(format!("Item {}", i)),
                ItemData::new(format!("Item {}", i), "A test item", 1, 10),
            ));
        }

        // Add many log messages
        for i in 0..100 {
            resources.ui.log.add(format!("Log message number {}", i));
        }

        // Save to file
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        save_game.save_to_file_compressed("stress_test_save.gz").unwrap();

        // Load and verify
        let loaded = SaveGame::load_from_file_compressed("stress_test_save.gz").unwrap();
        assert_eq!(loaded.entities.len(), 200);
        assert_eq!(loaded.seed, 12345);

        // Restore to new world
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        loaded.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify entity count
        let entity_count = new_world.iter().count();
        assert_eq!(entity_count, 200);

        // Verify log messages
        assert_eq!(new_resources.ui.log.messages.len(), 100);

        // Cleanup
        let _ = std::fs::remove_file("stress_test_save.gz");
    }

    #[test]
    fn test_stress_save_load_with_multiple_dungeons() {
        use crate::map::Map;
        use crate::domain_types::Depth;

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create multiple dungeon levels
        for i in 1..=10 {
            let map = Map::new(80, 50);
            resources.world.dungeon_levels.insert(Depth(i), map.clone());
            resources.world.map_cache.insert(crate::map::MapId::Dungeon(Depth(i)), map);
        }

        resources.world.current_depth = Depth(5);

        // Save and load
        let save_game = SaveGame::from_game(&world, &resources, 12345);
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save_game.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify all dungeons were saved
        assert_eq!(new_resources.world.dungeon_levels.len(), 10);
        assert_eq!(new_resources.world.current_depth, Depth(5));

        // Verify dungeons are in cache
        for i in 1..=10 {
            assert!(new_resources.world.map_cache.contains(crate::map::MapId::Dungeon(Depth(i))));
        }
    }

    #[test]
    fn test_stress_large_world_compression() {
        use crate::ecs::{Position, Monster, Renderable, Name, CombatStats, RealityLayer};

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create a large game state with many entities
        for i in 0..100 {
            world.spawn((
                Monster,
                Position::new(i % 80, i / 80, RealityLayer::Normal),
                Renderable {
                    glyph: 'g',
                    fg: ratatui::style::Color::Green,
                    bg: ratatui::style::Color::Reset,
                    z: 5,
                },
                Name(format!("Monster {}", i)),
                CombatStats::new(10 + (i % 5) as i32, 3, 1),
                Viewshed::new(8),
            ));
        }

        // Add large amount of log data
        for i in 0..200 {
            resources.ui.log.add(format!("Turn {} - Various game events occurred", i));
        }

        let save_game = SaveGame::from_game(&world, &resources, 12345);
        let uncompressed = save_game.uncompressed_size().unwrap();
        let compressed = save_game.compressed_size().unwrap();

        // Verify significant compression
        println!("Large world - Uncompressed: {}, Compressed: {}", uncompressed, compressed);
        assert!(compressed < uncompressed / 5, "Should achieve at least 5:1 compression ratio");
    }
}
