use crate::ecs::{Position, BlocksMovement, Viewshed, GameEvent};
use crate::ecs::resources::Resources;
use hecs::World;

pub fn movement_system(world: &mut World, resources: &mut Resources) {
    let mut moves = Vec::new();
    let mut attacks = Vec::new();

    // Process Move events from the queue
    for event in resources.events.iter() {
        if let GameEvent::Move { entity, dest_x, dest_y } = event {
            // Get current position
            let pos = match world.get::<&Position>(*entity) {
                Ok(p) => *p,
                Err(_) => continue, // Entity doesn't exist or has no position
            };

            // Check if destination is walkable
            let map = resources.world.maps.active_map();
            if !map.is_walkable(*dest_x, *dest_y) {
                continue;
            }

            // Check for blocking entities
            let mut blocked = false;
            let mut target = None;
            for (other_entity, (other_pos, _)) in world.query::<(&Position, &BlocksMovement)>().iter() {
                if other_pos.x == *dest_x && other_pos.y == *dest_y && other_pos.layer == pos.layer {
                    blocked = true;
                    target = Some(other_entity);
                    break;
                }
            }

            if blocked {
                if let Some(target_entity) = target {
                    attacks.push((*entity, target_entity));
                }
            } else {
                moves.push((*entity, *dest_x, *dest_y));
            }
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
        // Send Melee event instead of adding component
        resources.events.send(GameEvent::Melee { attacker, target });
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
        *resources.world.maps.active_map_mut() = create_test_map();

        // Create entity at (5, 5)
        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Send movement event to (6, 5)
        resources.events.send(GameEvent::Move { entity, dest_x: 6, dest_y: 5 });

        movement_system(&mut world, &mut resources);

        // Should have moved
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 6);
        assert_eq!(pos.y, 5);

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
        *resources.world.maps.active_map_mut() = map;

        // Create entity at (5, 5)
        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Try to move into wall at (6, 5)
        resources.events.send(GameEvent::Move { entity, dest_x: 6, dest_y: 5 });

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
        *resources.world.maps.active_map_mut() = create_test_map();

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
        resources.events.send(GameEvent::Move { entity: mover, dest_x: 6, dest_y: 5 });

        movement_system(&mut world, &mut resources);

        // Should NOT have moved
        let pos = world.get::<&Position>(mover).unwrap();
        assert_eq!(pos.x, 5, "Should not move through entities");
        assert_eq!(pos.y, 5, "Should not move through entities");

        // Should have sent melee event instead
        let melee_events: Vec<_> = resources.events.iter()
            .filter_map(|e| match e {
                GameEvent::Melee { attacker, target } => Some((*attacker, *target)),
                _ => None,
            })
            .collect();
        assert_eq!(melee_events.len(), 1, "Should send exactly one melee event on collision");
        assert_eq!(melee_events[0].0, mover, "Mover should be the attacker");
        assert_eq!(melee_events[0].1, blocker, "Should target the blocking entity");
    }

    #[test]
    fn test_movement_different_layers_no_collision() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.world.maps.active_map_mut() = create_test_map();

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
        resources.events.send(GameEvent::Move { entity: normal_entity, dest_x: 6, dest_y: 5 });

        movement_system(&mut world, &mut resources);

        // SHOULD have moved (different layers don't block)
        let pos = world.get::<&Position>(normal_entity).unwrap();
        assert_eq!(pos.x, 6, "Should move through entities on different layers");
        assert_eq!(pos.y, 5, "Should move through entities on different layers");

        // Should NOT send melee event
        let melee_events: Vec<_> = resources.events.iter()
            .filter_map(|e| match e {
                GameEvent::Melee { .. } => Some(()),
                _ => None,
            })
            .collect();
        assert_eq!(melee_events.len(), 0, "Should not attack entities on different layers");
    }

    #[test]
    fn test_movement_multiple_entities() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.world.maps.active_map_mut() = create_test_map();

        // Create multiple entities
        let entity1 = world.spawn((Position::new(1, 1, RealityLayer::Normal), Viewshed::new(8)));
        let entity2 = world.spawn((Position::new(2, 2, RealityLayer::Normal), Viewshed::new(8)));
        let entity3 = world.spawn((Position::new(3, 3, RealityLayer::Normal), Viewshed::new(8)));

        // All want to move
        resources.events.send(GameEvent::Move { entity: entity1, dest_x: 1, dest_y: 2 });
        resources.events.send(GameEvent::Move { entity: entity2, dest_x: 3, dest_y: 2 });
        resources.events.send(GameEvent::Move { entity: entity3, dest_x: 3, dest_y: 4 });

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
        *resources.world.maps.active_map_mut() = create_test_map();

        // Create entity at edge
        let entity = world.spawn((
            Position::new(9, 9, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Try to move out of bounds
        resources.events.send(GameEvent::Move { entity, dest_x: 10, dest_y: 9 });

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
        *resources.world.maps.active_map_mut() = create_test_map();

        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        resources.events.send(GameEvent::Move { entity, dest_x: 6, dest_y: 5 });

        movement_system(&mut world, &mut resources);

        // Movement should have succeeded
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 6);
        assert_eq!(pos.y, 5);
    }

    #[test]
    fn test_movement_door_blocking() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        let mut map = create_test_map();

        // Place closed door at (6, 5)
        let idx = map.xy_idx(6, 5);
        map.tiles[idx] = Tile::ClosedDoor;
        *resources.world.maps.active_map_mut() = map;

        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        resources.events.send(GameEvent::Move { entity, dest_x: 6, dest_y: 5 });

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
        *resources.world.maps.active_map_mut() = map;

        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        resources.events.send(GameEvent::Move { entity, dest_x: 6, dest_y: 5 });

        movement_system(&mut world, &mut resources);

        // SHOULD move through open door
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 6, "Should move through open doors");
        assert_eq!(pos.y, 5, "Should move through open doors");
    }

    // NEW TEST: Event-based movement
    #[test]
    fn test_movement_via_events() {
        let mut world = World::new();
        let mut resources = Resources::new(10, 10, 12345);
        *resources.world.maps.active_map_mut() = create_test_map();

        // Create entity at (5, 5)
        let entity = world.spawn((
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed::new(8),
        ));

        // Send Move event instead of adding component
        resources.events.send(GameEvent::Move {
            entity,
            dest_x: 6,
            dest_y: 5,
        });

        movement_system(&mut world, &mut resources);

        // Should have moved
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 6);
        assert_eq!(pos.y, 5);

        // Viewshed should be marked dirty
        let viewshed = world.get::<&Viewshed>(entity).unwrap();
        assert!(viewshed.dirty, "Viewshed should be marked dirty after movement");
    }
}
