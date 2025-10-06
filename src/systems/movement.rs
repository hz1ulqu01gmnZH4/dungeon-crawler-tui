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
