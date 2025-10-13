use crate::map::MapSet;
use crate::world::{Overmap, WorldTime, Settlement};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RunMode {
    AwaitingInput,
    PlayerTurn,
    MonstersTurn,
    GameOver,
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
    pub maps: MapSet,
    pub overmap: Overmap,
    pub settlements: Vec<Settlement>,     // All settlements in the world
    pub world_time: WorldTime,
    pub camera: Camera,
    pub rng: StdRng,
    pub mode: RunMode,
    pub log: GameLog,
    pub player_entity: Option<hecs::Entity>,
    pub player_overmap_pos: (i32, i32),  // Player position on overmap
    pub in_overmap_mode: bool,            // True when viewing/navigating overmap
    pub current_location: Option<usize>,  // Current settlement/location ID (None = wilderness)
    pub seed: u64,                        // World seed for save/load
}

impl Resources {
    pub fn new(map_width: i32, map_height: i32, seed: u64) -> Self {
        // Start player at center of a 50x50 overmap (MVP size)
        let overmap_size = 50;
        let start_pos = (overmap_size / 2, overmap_size / 2);

        Self {
            maps: MapSet::new(map_width, map_height),
            overmap: Overmap::new(overmap_size, overmap_size, seed),
            settlements: Vec::new(),  // Will be populated when terrain is generated
            world_time: WorldTime::new(),
            camera: Camera::new(80, 24),
            rng: StdRng::seed_from_u64(seed),
            mode: RunMode::AwaitingInput,
            log: GameLog::new(100),
            player_entity: None,
            player_overmap_pos: start_pos,
            in_overmap_mode: false,
            current_location: None,  // Start in wilderness
            seed,
        }
    }
}
