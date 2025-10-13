use crate::ecs::{Position, Player, WantsToMove};
use crate::ecs::resources::{Resources, RunMode};
use crate::world::{TimeCosts, generate_terrain};
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
            KeyCode::Tab => {
                // Toggle between local and overmap mode
                resources.in_overmap_mode = !resources.in_overmap_mode;

                if resources.in_overmap_mode {
                    // Entering overmap mode - generate terrain if not done yet
                    if !resources.overmap.tiles.iter().any(|t| t.discovered) {
                        generate_terrain(&mut resources.overmap);
                        // Discover starting area
                        let (px, py) = resources.player_overmap_pos;
                        for dy in -3..=3 {
                            for dx in -3..=3 {
                                resources.overmap.discover_tile(px + dx, py + dy);
                            }
                        }
                    }
                    resources.log.add("You survey the surrounding area...");
                } else {
                    resources.log.add("You return your focus to the immediate area.");
                }
            }
            KeyCode::Char('h') | KeyCode::Left => handle_movement(world, resources, -1, 0),
            KeyCode::Char('l') | KeyCode::Right => handle_movement(world, resources, 1, 0),
            KeyCode::Char('k') | KeyCode::Up => handle_movement(world, resources, 0, -1),
            KeyCode::Char('j') | KeyCode::Down => handle_movement(world, resources, 0, 1),
            KeyCode::Char('y') => handle_movement(world, resources, -1, -1),
            KeyCode::Char('u') => handle_movement(world, resources, 1, -1),
            KeyCode::Char('b') => handle_movement(world, resources, -1, 1),
            KeyCode::Char('n') => handle_movement(world, resources, 1, 1),
            KeyCode::Char('.') => {
                // Wait/skip turn
                if !resources.in_overmap_mode {
                    resources.mode = RunMode::PlayerTurn;
                }
            }
            _ => {}
        }
    }

    Ok(true)
}

fn handle_movement(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    if resources.in_overmap_mode {
        try_move_overmap(resources, dx, dy);
    } else {
        try_move_player(world, resources, dx, dy);
    }
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

fn try_move_overmap(resources: &mut Resources, dx: i32, dy: i32) {
    let (current_x, current_y) = resources.player_overmap_pos;
    let new_x = current_x + dx;
    let new_y = current_y + dy;

    // Check if destination is walkable
    if !resources.overmap.is_walkable(new_x, new_y) {
        if let Some(tile) = resources.overmap.get_tile(new_x, new_y) {
            let terrain_name = tile.terrain.name();
            resources
                .log
                .add(format!("You cannot travel through {}.", terrain_name));
        }
        return;
    }

    // Move to new tile
    resources.player_overmap_pos = (new_x, new_y);

    // Mark tile as visited and discover nearby tiles
    resources.overmap.visit_tile(new_x, new_y);
    for dy in -2..=2 {
        for dx in -2..=2 {
            resources.overmap.discover_tile(new_x + dx, new_y + dy);
        }
    }

    // Calculate time cost based on terrain
    if let Some(tile) = resources.overmap.get_tile(new_x, new_y) {
        let base_time = TimeCosts::MOVE_OVERMAP; // 60 minutes
        let speed_mult = tile.terrain.travel_speed();
        let actual_time = (base_time as f32 / speed_mult) as i32;

        resources.world_time.advance_minutes(actual_time);

        // Log the movement
        let terrain_name = tile.terrain.name();
        let time_str = resources.world_time.time_string();
        resources
            .log
            .add(format!("You travel through {}. ({})", terrain_name, time_str));
    }
}
