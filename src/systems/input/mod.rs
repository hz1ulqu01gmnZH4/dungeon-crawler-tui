// Input handling modules organized by UI mode
mod main_menu;
mod character_screen;
mod examine;
mod inventory;
mod equipment;
mod gameplay;

#[cfg(test)]
mod tests;

use crate::ecs::{Position, resources::{Resources, RunMode, UiMode}};
use crate::save::{quick_save, quick_load};
use crossterm::event::{self, Event, KeyCode};
use hecs::World;
use std::time::Duration;

// Re-export the main entry point
pub use gameplay::{handle_movement, try_move_player, try_move_overmap};

/// Main input handling entry point - polls for keyboard events
pub fn handle_input(world: &mut World, resources: &mut Resources) -> anyhow::Result<bool> {
    if resources.sim.mode != RunMode::AwaitingInput {
        return Ok(true);
    }

    if !event::poll(Duration::from_millis(100))? {
        return Ok(true);
    }

    if let Event::Key(key) = event::read()? {
        return Ok(handle_key(key.code, world, resources));
    }

    Ok(true)
}

/// Handle a specific key press (useful for testing and direct key injection)
pub fn handle_key(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    // Route to appropriate handler based on UI mode
    match &resources.ui.ui_mode {
        UiMode::MainMenu { .. } => {
            return main_menu::handle_main_menu_input(key_code, world, resources);
        }
        UiMode::CharacterScreen => {
            return character_screen::handle_character_screen_input(key_code, resources);
        }
        UiMode::Examine { .. } => {
            return examine::handle_examine_input(key_code, world, resources);
        }
        UiMode::Inventory { .. } => {
            return inventory::handle_inventory_input(key_code, world, resources);
        }
        UiMode::InGame | UiMode::Overmap => {
            // Normal gameplay or overmap - handle below
        }
    }

    // In-game and overmap key handling
    match key_code {
        KeyCode::Char('q') => return false,
        KeyCode::Char('S') => {
            // Save game (capital S to avoid conflict with future 's' for smash)
            match quick_save(world, resources, resources.sim.seed) {
                Ok(_) => resources.ui.log.add("Game saved to savegame.json"),
                Err(e) => resources.ui.log.add(format!("Failed to save: {}", e)),
            }
        }
        KeyCode::Char('L') => {
            // Load game (capital L to avoid conflict with 'l' for movement)
            match quick_load(world, resources) {
                Ok(seed) => {
                    resources.sim.seed = seed;
                    resources.ui.log.add("Game loaded from savegame.json");
                    // Reset camera to player position
                    if let Some(player_entity) = resources.player.player_entity {
                        if let Ok(pos) = world.get::<&Position>(player_entity) {
                            resources.ui.camera.center_on(pos.x, pos.y);
                        }
                    }
                }
                Err(e) => resources.ui.log.add(format!("Failed to load: {}", e)),
            }
        }
        KeyCode::Tab => {
            gameplay::handle_tab_key(world, resources);
        }
        KeyCode::Char('h') | KeyCode::Left => gameplay::handle_movement(world, resources, -1, 0),
        KeyCode::Char('l') | KeyCode::Right => gameplay::handle_movement(world, resources, 1, 0),
        KeyCode::Char('k') | KeyCode::Up => gameplay::handle_movement(world, resources, 0, -1),
        KeyCode::Char('j') | KeyCode::Down => gameplay::handle_movement(world, resources, 0, 1),
        KeyCode::Char('y') => gameplay::handle_movement(world, resources, -1, -1),
        KeyCode::Char('u') => gameplay::handle_movement(world, resources, 1, -1),
        KeyCode::Char('b') => gameplay::handle_movement(world, resources, -1, 1),
        KeyCode::Char('n') => gameplay::handle_movement(world, resources, 1, 1),
        KeyCode::Char('.') => {
            // Wait/skip turn
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                resources.sim.mode = RunMode::PlayerTurn;
            }
        }
        KeyCode::Enter => {
            // Enter location (settlement, dungeon, etc.)
            if matches!(resources.ui.ui_mode, UiMode::Overmap) {
                gameplay::try_enter_location(world, resources);
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Rest/camp
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                gameplay::try_rest(world, resources);
            }
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            // Pickup items
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                gameplay::try_pickup_items(world, resources);
            }
        }
        KeyCode::Char('i') | KeyCode::Char('I') => {
            // Toggle inventory
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                resources.ui.ui_mode = UiMode::Inventory { selection: 0 };
            }
        }
        KeyCode::Char('@') => {
            // Toggle character screen
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                resources.ui.ui_mode = UiMode::CharacterScreen;
            }
        }
        KeyCode::Char('x') => {
            // Toggle examine mode
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                // Entering examine mode - set cursor to player position
                if let Some(player_entity) = resources.player.player_entity {
                    if let Ok(pos) = world.get::<&Position>(player_entity) {
                        resources.ui.ui_mode = UiMode::Examine { cursor: (pos.x, pos.y) };
                        resources.ui.log.add("Examine mode: Move cursor with hjkl/arrows. Press 'x' or ESC to exit.");
                    }
                }
            }
        }
        KeyCode::Char('w') => {
            // Wield weapon (quick equip to main hand)
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                equipment::quick_wield_weapon(world, resources);
            }
        }
        KeyCode::Char('W') => {
            // Wear armor (quick equip to body)
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                equipment::quick_wear_armor(world, resources);
            }
        }
        KeyCode::Char('T') => {
            // Take off (unequip)
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                equipment::quick_take_off(world, resources);
            }
        }
        KeyCode::Char('o') => {
            // Open door
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                gameplay::try_open_door(world, resources);
            }
        }
        KeyCode::Char('c') => {
            // Close door
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                gameplay::try_close_door(world, resources);
            }
        }
        KeyCode::Char('<') => {
            // Ascend stairs
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                gameplay::try_use_stairs(world, resources, true);
            }
        }
        KeyCode::Char('>') => {
            // Descend stairs
            if matches!(resources.ui.ui_mode, UiMode::InGame) {
                gameplay::try_use_stairs(world, resources, false);
            }
        }
        _ => {}
    }

    true
}
