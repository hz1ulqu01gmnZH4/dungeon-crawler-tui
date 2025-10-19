use crate::ecs::{Position, WantsToMove, WantsToMelee, BlocksMovement, Viewshed};
use crate::ecs::resources::Resources;
use hecs::World;

pub fn movement_system(world: &mut World, resources: &mut Resources) {
    let mut moves = Vec::new();
    let mut attacks = Vec::new();

    // Collect movement intents
    for (entity, (pos, wants_move)) in world.query::<(&Position, &WantsToMove)>().iter() {
        let dest_x = wants_move.dest_x;
        let dest_y = wants_move.dest_y;

        // Check if destination is walkable
        let map = resources.maps.active_map();
        if !map.is_walkable(dest_x, dest_y) {
            continue;
        }

        // Check for blocking entities
        let mut blocked = false;
        let mut target = None;
        for (other_entity, (other_pos, _)) in world.query::<(&Position, &BlocksMovement)>().iter() {
            if other_pos.x == dest_x && other_pos.y == dest_y && other_pos.layer == pos.layer {
                blocked = true;
                target = Some(other_entity);
                break;
            }
        }

        if blocked {
            if let Some(target_entity) = target {
                attacks.push((entity, target_entity));
            }
        } else {
            moves.push((entity, dest_x, dest_y));
        }
    }

    // Apply movements
    for (entity, dest_x, dest_y) in moves {
        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            pos.x = dest_x;
            pos.y = dest_y;
        }
        if let Ok(mut viewshed) = world.get::<&mut Viewshed>(entity) {
            viewshed.dirty = true;
        }
    }

    // Convert blocked movements to melee attacks
    for (attacker, target) in attacks {
        let _ = world.insert_one(attacker, WantsToMelee::new(target));
    }

    // Clean up movement intents
    let entities_to_clean: Vec<_> = world
        .query::<&WantsToMove>()
        .iter()
        .map(|(e, _)| e)
        .collect();
    for entity in entities_to_clean {
        let _ = world.remove_one::<WantsToMove>(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::{Player, RealityLayer};
    use crate::map::{Map, Tile};

    fn create_test_map() -> Map {
        let mut map = Map::new(10, 10);
        // Clear some floor space
        for y in 1..9 {
            for x in 1..9 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
        map
    }

    #[test]
    fn test_movement_basic_success() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.maps.active_map_mut() = create_test_map();

        // Create entity at (5, 5)
        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Add movement intent to (6, 5)
        world.insert_one(entity, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // Should have moved
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 6);
        assert_eq!(pos.y, 5);

        // Movement intent should be removed
        assert!(world.get::<&WantsToMove>(entity).is_err());

        // Viewshed should be marked dirty
        let viewshed = world.get::<&Viewshed>(entity).unwrap();
        assert!(viewshed.dirty, "Viewshed should be marked dirty after movement");
    }

    #[test]
    fn test_movement_wall_collision() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        let mut map = create_test_map();

        // Place wall at (6, 5)
        let idx = map.xy_idx(6, 5);
        map.tiles[idx] = Tile::Wall;
        *resources.maps.active_map_mut() = map;

        // Create entity at (5, 5)
        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Try to move into wall at (6, 5)
        world.insert_one(entity, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // Should NOT have moved
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 5, "Should not move through walls");
        assert_eq!(pos.y, 5, "Should not move through walls");
    }

    #[test]
    fn test_movement_entity_collision() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.maps.active_map_mut() = create_test_map();

        // Create blocking entity at (6, 5)
        let blocker = world.spawn((
            Position::new(6, 5, RealityLayer::Normal),
            BlocksMovement,
        ));

        // Create moving entity at (5, 5)
        let mover = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Try to move into blocker
        world.insert_one(mover, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // Should NOT have moved
        let pos = world.get::<&Position>(mover).unwrap();
        assert_eq!(pos.x, 5, "Should not move through entities");
        assert_eq!(pos.y, 5, "Should not move through entities");

        // Should have created melee attack instead
        assert!(world.get::<&WantsToMelee>(mover).is_ok(), "Should create melee attack on collision");
        let melee = world.get::<&WantsToMelee>(mover).unwrap();
        assert_eq!(melee.target, blocker, "Should target the blocking entity");
    }

    #[test]
    fn test_movement_different_layers_no_collision() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.maps.active_map_mut() = create_test_map();

        // Create entity on Cosmic layer at (6, 5)
        let cosmic_entity = world.spawn((
            Position::new(6, 5, RealityLayer::Cosmic),
            BlocksMovement,
        ));

        // Create entity on Normal layer at (5, 5)
        let normal_entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Try to move to (6, 5) - same position but different layer
        world.insert_one(normal_entity, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // SHOULD have moved (different layers don't block)
        let pos = world.get::<&Position>(normal_entity).unwrap();
        assert_eq!(pos.x, 6, "Should move through entities on different layers");
        assert_eq!(pos.y, 5, "Should move through entities on different layers");

        // Should NOT create melee attack
        assert!(world.get::<&WantsToMelee>(normal_entity).is_err(), "Should not attack entities on different layers");
    }

    #[test]
    fn test_movement_multiple_entities() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.maps.active_map_mut() = create_test_map();

        // Create multiple entities
        let entity1 = world.spawn((Position::new(1, 1, RealityLayer::Normal), Viewshed::new(8)));
        let entity2 = world.spawn((Position::new(2, 2, RealityLayer::Normal), Viewshed::new(8)));
        let entity3 = world.spawn((Position::new(3, 3, RealityLayer::Normal), Viewshed::new(8)));

        // All want to move
        world.insert_one(entity1, WantsToMove { dest_x: 1, dest_y: 2 }).unwrap();
        world.insert_one(entity2, WantsToMove { dest_x: 3, dest_y: 2 }).unwrap();
        world.insert_one(entity3, WantsToMove { dest_x: 3, dest_y: 4 }).unwrap();

        movement_system(&mut world, &mut resources);

        // All should have moved
        let pos1 = world.get::<&Position>(entity1).unwrap();
        assert_eq!((pos1.x, pos1.y), (1, 2));

        let pos2 = world.get::<&Position>(entity2).unwrap();
        assert_eq!((pos2.x, pos2.y), (3, 2));

        let pos3 = world.get::<&Position>(entity3).unwrap();
        assert_eq!((pos3.x, pos3.y), (3, 4));
    }

    #[test]
    fn test_movement_out_of_bounds() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.maps.active_map_mut() = create_test_map();

        // Create entity at edge
        let entity = world.spawn((
            Position::new(9, 9, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Try to move out of bounds
        world.insert_one(entity, WantsToMove { dest_x: 10, dest_y: 9 }).unwrap();

        movement_system(&mut world, &mut resources);

        // Should NOT have moved
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 9, "Should not move out of bounds");
        assert_eq!(pos.y, 9, "Should not move out of bounds");
    }

    #[test]
    fn test_movement_clears_intents() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.maps.active_map_mut() = create_test_map();

        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        world.insert_one(entity, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // Movement intent should be removed whether move succeeded or not
        assert!(world.get::<&WantsToMove>(entity).is_err(), "WantsToMove should be removed after processing");
    }

    #[test]
    fn test_movement_door_blocking() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        let mut map = create_test_map();

        // Place closed door at (6, 5)
        let idx = map.xy_idx(6, 5);
        map.tiles[idx] = Tile::ClosedDoor;
        *resources.maps.active_map_mut() = map;

        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        world.insert_one(entity, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // Should NOT move through closed door
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 5, "Should not move through closed doors");
        assert_eq!(pos.y, 5, "Should not move through closed doors");
    }

    #[test]
    fn test_movement_open_door_passable() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        let mut map = create_test_map();

        // Place open door at (6, 5)
        let idx = map.xy_idx(6, 5);
        map.tiles[idx] = Tile::OpenDoor;
        *resources.maps.active_map_mut() = map;

        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        world.insert_one(entity, WantsToMove { dest_x: 6, dest_y: 5 }).unwrap();

        movement_system(&mut world, &mut resources);

        // SHOULD move through open door
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 6, "Should move through open doors");
        assert_eq!(pos.y, 5, "Should move through open doors");
    }
}
