use crate::ecs::{Position, resources::{Resources, UiMode}};
use crate::ui::MenuOption;
use crate::save::quick_load;
use crossterm::event::KeyCode;
use hecs::World;

/// Handle main menu input
pub(super) fn handle_main_menu_input(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    // Extract current selection
    let mut selection = if let UiMode::MainMenu { selection } = resources.ui.ui_mode {
        selection
    } else {
        return true; // Shouldn't happen, but handle gracefully
    };

    match key_code {
        KeyCode::Char('q') | KeyCode::Esc => {
            // Quit from main menu
            return false;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Move selection up
            selection = selection.prev();
            resources.ui.ui_mode = UiMode::MainMenu { selection };
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Move selection down
            selection = selection.next();
            resources.ui.ui_mode = UiMode::MainMenu { selection };
        }
        KeyCode::Enter => {
            let save_exists = std::path::Path::new("savegame.json").exists();

            match selection {
                MenuOption::Continue => {
                    if save_exists {
                        // Load game
                        match quick_load(world, resources) {
                            Ok(seed) => {
                                resources.sim.seed = seed;
                                resources.ui.ui_mode = UiMode::InGame;
                                resources.ui.log.add("Game loaded successfully!");

                                // Reset camera to player position
                                if let Some(player_entity) = resources.player.player_entity {
                                    if let Ok(pos) = world.get::<&Position>(player_entity) {
                                        resources.ui.camera.center_on(pos.x, pos.y);
                                    }
                                }
                            }
                            Err(e) => {
                                // If load fails, stay in menu and show error
                                resources.ui.log.add(format!("Failed to load: {}", e));
                            }
                        }
                    } else {
                        // No save file, can't continue
                        resources.ui.log.add("No save file found.");
                    }
                }
                MenuOption::NewGame => {
                    // Exit main menu and start new game
                    resources.ui.ui_mode = UiMode::InGame;
                    resources.ui.log.add("Starting new game...");
                }
                MenuOption::Quit => {
                    return false;
                }
            }
        }
        _ => {}
    }

    true
}
