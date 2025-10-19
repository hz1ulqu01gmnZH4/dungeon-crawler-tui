use crate::ecs::resources::{Resources, UiMode};
use crossterm::event::KeyCode;

/// Handle character screen input
pub(super) fn handle_character_screen_input(key_code: KeyCode, resources: &mut Resources) -> bool {
    match key_code {
        // Close character screen
        KeyCode::Char('@') | KeyCode::Esc => {
            resources.ui.ui_mode = UiMode::InGame;
        }
        // Future: navigation between tabs, etc.
        _ => {}
    }
    true
}
