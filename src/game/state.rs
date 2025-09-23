pub struct GameState {
    pub running: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self { running: true }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}