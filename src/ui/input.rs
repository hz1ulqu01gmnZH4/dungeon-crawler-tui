use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
    None,
}

pub fn poll_input() -> Action {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            return match code {
                KeyCode::Up | KeyCode::Char('w') => Action::MoveUp,
                KeyCode::Down | KeyCode::Char('s') => Action::MoveDown,
                KeyCode::Left | KeyCode::Char('a') => Action::MoveLeft,
                KeyCode::Right | KeyCode::Char('d') => Action::MoveRight,
                KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                _ => Action::None,
            };
        }
    }
    Action::None
}