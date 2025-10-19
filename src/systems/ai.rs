use crate::ecs::{Position, Monster, Player, Viewshed, GameEvent};
use crate::ecs::resources::Resources;
use hecs::World;

pub fn monster_ai_system(world: &mut World, resources: &mut Resources) {
    // Get player position
    let player_pos = {
        let mut player_pos = None;
        for (_, (pos, _)) in world.query::<(&Position, &Player)>().iter() {
            player_pos = Some((*pos).clone());
            break;
        }
        player_pos
    };

    let Some(player_pos) = player_pos else {
        return;
    };

    // For each monster, decide action
    let mut monster_moves = Vec::new();
    for (entity, (pos, viewshed, _monster)) in world.query::<(&Position, &Viewshed, &Monster)>().iter() {
        // Only act if same layer as player
        if pos.layer != player_pos.layer {
            continue;
        }

        // Check if player is visible
        let can_see_player = viewshed.visible.contains(&(player_pos.x, player_pos.y));

        if can_see_player {
            // Move towards player
            let dx = (player_pos.x - pos.x).signum();
            let dy = (player_pos.y - pos.y).signum();

            let dest_x = pos.x + dx;
            let dest_y = pos.y + dy;

            monster_moves.push((entity, dest_x, dest_y));
        }
    }

    // Send Move events for monster movement
    for (entity, dest_x, dest_y) in monster_moves {
        resources.events.send(GameEvent::Move { entity, dest_x, dest_y });
    }
}
