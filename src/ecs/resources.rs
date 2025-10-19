
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RunMode {
    AwaitingInput,
    PlayerTurn,
    MonstersTurn,
    GameOver,
}

/// UI mode determines which screen/interface the player is currently viewing
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiMode {
    /// Main menu (startup screen)
    MainMenu { selection: crate::ui::MenuOption },
    /// Normal gameplay (local map view)
    InGame,
    /// Overworld/overmap view
    Overmap,
    /// Inventory screen
    Inventory { selection: usize },
    /// Character stats screen (@)
    CharacterScreen,
    /// Examine/look mode (x)
    Examine { cursor: (i32, i32) },
}

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn center_on(&mut self, x: i32, y: i32) {
        self.x = x - self.width / 2;
        self.y = y - self.height / 2;
    }
}

pub struct GameLog {
    pub messages: Vec<String>,
    pub max_size: usize,
}

impl GameLog {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_size,
        }
    }

    pub fn add<S: Into<String>>(&mut self, message: S) {
        self.messages.push(message.into());
        if self.messages.len() > self.max_size {
            self.messages.remove(0);
        }
    }

    pub fn recent(&self, count: usize) -> &[String] {
        let start = self.messages.len().saturating_sub(count);
        &self.messages[start..]
    }
}

pub struct Resources {
    // Simulation resources (time, weather, RNG, game mode)
    pub sim: crate::ecs::typed_resources::SimResources,
    // World/map resources (overmap, dungeons, settlements)
    pub world: crate::ecs::typed_resources::WorldResources,
    // UI resources (camera, log, UI mode)
    pub ui: crate::ecs::typed_resources::UiResources,
    // Player resources
    pub player: crate::ecs::typed_resources::PlayerResources,
    // Event queue for game actions
    pub events: crate::ecs::events::Events<crate::ecs::events::GameEvent>,
}

impl Resources {
    pub fn new(map_width: i32, map_height: i32, seed: u64) -> Self {
        Self {
            sim: crate::ecs::typed_resources::SimResources::new(seed),
            world: crate::ecs::typed_resources::WorldResources::new(map_width, map_height, seed),
            ui: crate::ecs::typed_resources::UiResources::new(),
            player: crate::ecs::typed_resources::PlayerResources::new(),
            events: crate::ecs::events::Events::new(),
        }
    }
}
