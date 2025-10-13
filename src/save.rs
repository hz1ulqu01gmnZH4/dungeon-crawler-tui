use crate::ecs::{
    Position, Renderable, Player, CombatStats, TriMeter, Viewshed,
    WantsToMove, Name, Monster, BlocksMovement,
};
use crate::ecs::resources::Resources;
use crate::world::{Overmap, Settlement, WorldTime};
use hecs::World;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Version for save file compatibility
const SAVE_VERSION: u32 = 1;

/// Serializable entity data
#[derive(Serialize, Deserialize)]
struct EntityData {
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

    // Intent components (usually not saved)
    wants_to_move: Option<WantsToMove>,
    // Note: WantsToMelee not saved (contains Entity)
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
            let entity_data = EntityData {
                position: world.get::<&Position>(entity).ok().map(|c| *c),
                renderable: world.get::<&Renderable>(entity).ok().map(|c| *c),
                name: world.get::<&Name>(entity).ok().map(|n| n.0.clone()),
                player: world.get::<&Player>(entity).ok().map(|c| *c),
                monster: world.get::<&Monster>(entity).ok().map(|c| *c),
                combat_stats: world.get::<&CombatStats>(entity).ok().map(|c| *c),
                tri_meter: world.get::<&TriMeter>(entity).ok().map(|c| *c),
                viewshed: world.get::<&Viewshed>(entity).ok().map(|v| (*v).clone()),
                blocks_movement: world.get::<&BlocksMovement>(entity).ok().map(|c| *c),
                wants_to_move: world.get::<&WantsToMove>(entity).ok().map(|c| *c),
            };

            // Track player entity index
            if entity_data.player.is_some() {
                player_entity_index = Some(idx);
            }

            entities.push(entity_data);
        }

        SaveGame {
            version: SAVE_VERSION,
            overmap: resources.overmap.clone(),
            settlements: resources.settlements.clone(),
            world_time: resources.world_time.clone(),
            player_overmap_pos: resources.player_overmap_pos,
            current_location: resources.current_location,
            entities,
            player_entity_index,
            seed,
            message_log: resources.log.messages.clone(),
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
        resources.overmap = self.overmap.clone();
        resources.settlements = self.settlements.clone();
        resources.world_time = self.world_time.clone();
        resources.player_overmap_pos = self.player_overmap_pos;
        resources.current_location = self.current_location;

        // Restore message log
        resources.log.messages = self.message_log.clone();

        // Recreate entities
        let mut player_entity = None;
        for (idx, entity_data) in self.entities.iter().enumerate() {
            let mut builder = hecs::EntityBuilder::new();

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
            if let Some(wtm) = entity_data.wants_to_move {
                builder.add(wtm);
            }
            // Note: WantsToMelee not loaded (contains Entity, not serializable)

            let entity = world.spawn(builder.build());

            // Track player entity
            if Some(idx) == self.player_entity_index {
                player_entity = Some(entity);
            }
        }

        resources.player_entity = player_entity;

        Ok(())
    }

    /// Save game to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load game from file
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

    #[test]
    fn test_save_version() {
        assert_eq!(SAVE_VERSION, 1);
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
        assert_eq!(new_resources.player_overmap_pos, (25, 25));
    }
}
