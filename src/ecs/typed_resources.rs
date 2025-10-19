use crate::map::{MapSet, Map};
use crate::world::{Overmap, WorldTime, Settlement, POI, WeatherSystem, Road};
use crate::domain_types::Depth;
use crate::ecs::resources::{Camera, GameLog, RunMode, UiMode};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;
use std::any::{Any, TypeId};
use hecs::Entity;

/// Simulation-related resources (time, weather, RNG, game mode)
pub struct SimResources {
    pub world_time: WorldTime,
    pub weather: WeatherSystem,
    pub rng: StdRng,
    pub mode: RunMode,
    pub seed: u64,
}

impl SimResources {
    pub fn new(seed: u64) -> Self {
        Self {
            world_time: WorldTime::new(),
            weather: WeatherSystem::new(seed.wrapping_add(1)),
            rng: StdRng::seed_from_u64(seed),
            mode: RunMode::AwaitingInput,
            seed,
        }
    }
}

/// World/map-related resources (overmap, dungeons, settlements)
pub struct WorldResources {
    pub maps: MapSet,
    pub overmap: Overmap,
    pub settlements: Vec<Settlement>,
    pub pois: Vec<POI>,
    pub roads: Vec<Road>,
    pub settlement_maps: HashMap<usize, Map>,
    pub dungeon_levels: HashMap<Depth, Map>,
    pub current_depth: Depth,
    pub current_location: Option<usize>,
    pub player_overmap_pos: (i32, i32),
}

impl WorldResources {
    pub fn new(map_width: i32, map_height: i32, seed: u64) -> Self {
        let overmap_size = 50;
        let start_pos = (overmap_size / 2, overmap_size / 2);

        Self {
            maps: MapSet::new(map_width, map_height),
            overmap: Overmap::new(overmap_size, overmap_size, seed),
            settlements: Vec::new(),
            pois: Vec::new(),
            roads: Vec::new(),
            settlement_maps: HashMap::new(),
            dungeon_levels: HashMap::new(),
            current_depth: Depth::SURFACE,
            current_location: None,
            player_overmap_pos: start_pos,
        }
    }
}

/// UI-related resources (camera, log, UI mode)
pub struct UiResources {
    pub ui_mode: UiMode,
    pub camera: Camera,
    pub log: GameLog,
}

impl UiResources {
    pub fn new() -> Self {
        Self {
            ui_mode: UiMode::MainMenu { selection: crate::ui::MenuOption::Continue },
            camera: Camera::new(80, 24),
            log: GameLog::new(100),
        }
    }
}

/// Player-specific resources
pub struct PlayerResources {
    pub player_entity: Option<Entity>,
    pub last_combat_target: Option<Entity>,
}

impl PlayerResources {
    pub fn new() -> Self {
        Self {
            player_entity: None,
            last_combat_target: None,
        }
    }
}

/// Generic typed storage for resources using TypeId
/// Allows systems to request specific resource types
pub struct ResourceMap {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceMap {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Insert a resource of type T
    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    /// Get an immutable reference to a resource of type T
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|r| r.downcast_ref::<T>())
    }

    /// Get a mutable reference to a resource of type T
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|r| r.downcast_mut::<T>())
    }

    /// Check if a resource of type T exists
    pub fn contains<T: 'static>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    /// Remove a resource of type T
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.resources
            .remove(&TypeId::of::<T>())
            .and_then(|r| r.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }
}
