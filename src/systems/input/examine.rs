use crate::ecs::resources::{Resources, UiMode};
use crossterm::event::KeyCode;
use hecs::World;

/// Handle examine mode input
pub(super) fn handle_examine_input(key_code: KeyCode, _world: &mut World, resources: &mut Resources) -> bool {
    // Extract current cursor position
    let (mut cx, mut cy) = if let UiMode::Examine { cursor } = resources.ui.ui_mode {
        cursor
    } else {
        return true; // Shouldn't happen, but handle gracefully
    };

    match key_code {
        // Close examine mode
        KeyCode::Char('x') | KeyCode::Esc => {
            resources.ui.ui_mode = UiMode::InGame;
        }
        // Move examine cursor
        KeyCode::Char('h') | KeyCode::Left => {
            cx -= 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('l') | KeyCode::Right => {
            cx += 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('k') | KeyCode::Up => {
            cy -= 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('j') | KeyCode::Down => {
            cy += 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('y') => {
            cx -= 1;
            cy -= 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('u') => {
            cx += 1;
            cy -= 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('b') => {
            cx -= 1;
            cy += 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        KeyCode::Char('n') => {
            cx += 1;
            cy += 1;
            resources.ui.ui_mode = UiMode::Examine { cursor: (cx, cy) };
        }
        _ => {}
    }
    true
}
