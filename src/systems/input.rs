use crate::ecs::{Position, Player, WantsToMove};
use crate::ecs::resources::{Resources, RunMode};
use crossterm::event::{self, Event, KeyCode};
use hecs::World;
use std::time::Duration;

pub fn handle_input(world: &mut World, resources: &mut Resources) -> anyhow::Result<bool> {
    if resources.mode != RunMode::AwaitingInput {
        return Ok(true);
    }

    if !event::poll(Duration::from_millis(100))? {
        return Ok(true);
    }

    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => return Ok(false),
            KeyCode::Char('h') | KeyCode::Left => try_move_player(world, resources, -1, 0),
            KeyCode::Char('l') | KeyCode::Right => try_move_player(world, resources, 1, 0),
            KeyCode::Char('k') | KeyCode::Up => try_move_player(world, resources, 0, -1),
            KeyCode::Char('j') | KeyCode::Down => try_move_player(world, resources, 0, 1),
            KeyCode::Char('y') => try_move_player(world, resources, -1, -1),
            KeyCode::Char('u') => try_move_player(world, resources, 1, -1),
            KeyCode::Char('b') => try_move_player(world, resources, -1, 1),
            KeyCode::Char('n') => try_move_player(world, resources, 1, 1),
            KeyCode::Char('.') => {
                // Wait/skip turn
                resources.mode = RunMode::PlayerTurn;
            }
            _ => {}
        }
    }

    Ok(true)
}

fn try_move_player(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    let player_move = {
        let mut result = None;
        for (entity, (pos, _)) in world.query::<(&Position, &Player)>().iter() {
            result = Some((entity, pos.x + dx, pos.y + dy));
            break;
        }
        result
    };

    if let Some((entity, dest_x, dest_y)) = player_move {
        let _ = world.insert_one(entity, WantsToMove::new(dest_x, dest_y));
        resources.mode = RunMode::PlayerTurn;
    }
}
