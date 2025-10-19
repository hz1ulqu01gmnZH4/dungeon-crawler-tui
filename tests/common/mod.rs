/// Integration test harness for direct game interface testing
///
/// This module provides utilities for testing the game by simulating
/// player input and verifying game state transitions.

use dungeon_clawler_tui::*;
use hecs::World;
use crossterm::event::KeyCode;

/// Snapshot of game state for comparison in tests
#[derive(Debug, Clone, PartialEq)]
pub struct GameSnapshot {
    pub player_pos: (i32, i32),
    pub time: world::WorldTime,
    pub in_overmap_mode: bool,
    pub current_location: Option<usize>,
    pub player_overmap_pos: (i32, i32),
    pub active_layer: ecs::RealityLayer,
}

/// Test harness for integration tests
///
/// Provides methods to:
/// - Simulate keyboard input
/// - Run game ticks
/// - Query game state
/// - Verify invariants
pub struct GameTestHarness {
    pub world: World,
    pub resources: ecs::resources::Resources,
    pub frame_count: u32,
}

impl GameTestHarness {
    /// Create a new test harness with deterministic seed
    pub fn new(seed: u64) -> Self {
        let map_width = 80;
        let map_height = 40;

        let mut world = World::new();
        let mut resources = ecs::resources::Resources::new(map_width, map_height, seed);

        // Create a simple room around player spawn for testing
        let player_start_x = map_width / 2;
        let player_start_y = map_height / 2;
        let room_size = 15;

        for y in (player_start_y - room_size / 2)..(player_start_y + room_size / 2) {
            for x in (player_start_x - room_size / 2)..(player_start_x + room_size / 2) {
                if x > 0 && x < map_width && y > 0 && y < map_height {
                    let idx = resources.maps.active_map().xy_idx(x, y);
                    resources.maps.normal.tiles[idx] = map::Tile::Floor;
                }
            }
        }

        // Spawn player
        let player_entity = world.spawn((
            ecs::Position {
                x: player_start_x,
                y: player_start_y,
                layer: ecs::RealityLayer::Normal,
            },
            ecs::Renderable {
                glyph: '@',
                fg: ratatui::style::Color::Yellow,
                bg: ratatui::style::Color::Reset,
                z: 10,
            },
            ecs::Player,
            ecs::Viewshed {
                range: 8,
                visible: Vec::new(),
                dirty: true,
            },
            ecs::CombatStats {
                max_hp: 100,
                hp: 100,
                defense: 5,
                power: 10,
            },
            ecs::TriMeter {
                sanity: 100,
                insight: 50,
                notice: 50,
            },
        ));

        // Store player entity reference for FOV system
        resources.player_entity = Some(player_entity);

        let mut harness = Self {
            world,
            resources,
            frame_count: 0,
        };

        // Initialize FOV so player can see the room
        systems::fov::update_fov(&mut harness.world, &mut harness.resources);

        harness
    }

    /// Simulate a key press and process input
    pub fn press_key(&mut self, key: KeyCode) -> Result<(), String> {
        // Process the key through the input system
        let should_continue = systems::input::handle_key(key, &mut self.world, &mut self.resources);
        if !should_continue {
            return Err("Game quit".to_string());
        }
        Ok(())
    }

    /// Run game update cycles
    pub fn tick(&mut self, count: u32) {
        for _ in 0..count {
            // Update FOV
            systems::fov::update_fov(&mut self.world, &mut self.resources);

            // Update other systems as needed
            self.frame_count += 1;
        }
    }

    /// Get current player position
    pub fn player_position(&self) -> Option<(i32, i32)> {
        for (_, pos) in self.world.query::<&ecs::Position>().iter() {
            if pos.layer == self.resources.maps.active {
                return Some((pos.x, pos.y));
            }
        }
        None
    }

    /// Get player stats
    pub fn get_player_stats(&self) -> Option<ecs::CombatStats> {
        for (_, (_, stats)) in self.world.query::<(&ecs::Player, &ecs::CombatStats)>().iter() {
            return Some(*stats);
        }
        None
    }

    /// Get current time
    pub fn current_time(&self) -> &world::WorldTime {
        &self.resources.world_time
    }

    /// Get current weather
    pub fn current_weather(&self) -> world::Weather {
        self.resources.weather.current_weather
    }

    /// Check if in overmap mode
    pub fn is_in_overmap_mode(&self) -> bool {
        self.resources.in_overmap_mode
    }

    /// Get player overmap position
    pub fn player_overmap_position(&self) -> (i32, i32) {
        self.resources.player_overmap_pos
    }

    /// Get current location ID (if in a settlement/dungeon)
    pub fn current_location(&self) -> Option<usize> {
        self.resources.current_location
    }

    /// Get current reality layer
    pub fn current_layer(&self) -> ecs::RealityLayer {
        self.resources.maps.active
    }

    /// Count visible tiles in current map
    pub fn count_visible_tiles(&self) -> usize {
        let map = self.resources.maps.active_map();
        map.visible.iter().filter(|&&v| v).count()
    }

    /// Count revealed tiles in current map
    pub fn count_revealed_tiles(&self) -> usize {
        let map = self.resources.maps.active_map();
        map.revealed.iter().filter(|&&v| v).count()
    }

    /// Count discovered overmap tiles
    pub fn count_discovered_overmap_tiles(&self) -> usize {
        self.resources.overmap.tiles.iter().filter(|t| t.discovered).count()
    }

    /// Take a snapshot of current game state
    pub fn snapshot(&self) -> GameSnapshot {
        GameSnapshot {
            player_pos: self.player_position().unwrap_or((0, 0)),
            time: self.current_time().clone(),
            in_overmap_mode: self.is_in_overmap_mode(),
            current_location: self.current_location(),
            player_overmap_pos: self.player_overmap_position(),
            active_layer: self.current_layer(),
        }
    }

    /// Assert game state is valid (no crashes, invariants hold)
    pub fn assert_valid_state(&self) {
        // Player exists
        assert!(self.player_position().is_some(), "Player should exist");

        // Player is in valid location
        let pos = self.player_position().unwrap();
        let map = self.resources.maps.active_map();
        assert!(map.in_bounds(pos.0, pos.1), "Player should be in map bounds");

        // Player stats are reasonable
        if let Some(stats) = self.get_player_stats() {
            assert!(stats.hp >= 0, "HP should not be negative");
            assert!(stats.hp <= stats.max_hp, "HP should not exceed max");
            assert!(stats.max_hp > 0, "Max HP should be positive");
        }

        // Time is progressing forward
        assert!(self.current_time().total_minutes() >= 0, "Time should be non-negative");
    }

    /// Find the nearest settlement on the overmap
    pub fn find_nearest_settlement(&self) -> (i32, i32) {
        let player_pos = self.player_overmap_position();

        self.resources
            .settlements
            .iter()
            .min_by_key(|s| {
                let dx = s.position.0 - player_pos.0;
                let dy = s.position.1 - player_pos.1;
                dx * dx + dy * dy
            })
            .map(|s| s.position)
            .unwrap_or(player_pos)
    }

    /// Navigate to a specific overmap position
    pub fn navigate_overmap_to(&mut self, target: (i32, i32)) {
        let mut current = self.player_overmap_position();

        while current != target {
            // Move towards target
            let dx = target.0 - current.0;
            let dy = target.1 - current.1;

            if dx > 0 {
                self.press_key(KeyCode::Right).ok();
            } else if dx < 0 {
                self.press_key(KeyCode::Left).ok();
            } else if dy > 0 {
                self.press_key(KeyCode::Down).ok();
            } else if dy < 0 {
                self.press_key(KeyCode::Up).ok();
            }

            self.tick(1);
            current = self.player_overmap_position();

            // Safety: don't loop forever
            if (current.0 - target.0).abs() + (current.1 - target.1).abs() > 100 {
                break;
            }
        }
    }

    /// Get message log
    pub fn get_message_log(&self) -> Vec<String> {
        self.resources.log.recent(100).to_vec()
    }

    /// Advance time by a number of days
    pub fn advance_time_by_days(&mut self, days: i32) {
        for _ in 0..days {
            self.resources.world_time.advance_hours(24);
        }
    }

    /// Check if currently in a building
    pub fn is_in_building(&self) -> bool {
        // Could check if current map dimensions match building dimensions
        let map = self.resources.maps.active_map();
        map.width <= 20 && map.height <= 20 // Buildings are typically small
    }

    /// Damage player to specific HP (for testing rest/healing)
    pub fn damage_player(&mut self, target_hp: i32) {
        if let Some(player_entity) = self.resources.player_entity {
            if let Ok(mut stats) = self.world.get::<&mut ecs::CombatStats>(player_entity) {
                stats.hp = target_hp.min(stats.max_hp).max(0);
            }
        }
    }
}

#[cfg(test)]
mod harness_tests {
    use super::*;

    #[test]
    fn test_harness_creation() {
        let harness = GameTestHarness::new(12345);
        assert!(harness.player_position().is_some());
        assert_eq!(harness.frame_count, 0);
    }

    #[test]
    fn test_harness_tick() {
        let mut harness = GameTestHarness::new(12345);
        assert_eq!(harness.frame_count, 0);

        harness.tick(5);
        assert_eq!(harness.frame_count, 5);
    }

    #[test]
    fn test_harness_player_position() {
        let harness = GameTestHarness::new(12345);
        let pos = harness.player_position();
        assert!(pos.is_some());

        let (x, y) = pos.unwrap();
        assert!(x >= 0);
        assert!(y >= 0);
    }

    #[test]
    fn test_harness_snapshot() {
        let harness = GameTestHarness::new(12345);
        let snapshot = harness.snapshot();

        assert_eq!(snapshot.player_pos, harness.player_position().unwrap());
        assert!(!snapshot.in_overmap_mode);
    }

    #[test]
    fn test_harness_valid_state() {
        let harness = GameTestHarness::new(12345);
        harness.assert_valid_state(); // Should not panic
    }

    #[test]
    fn test_harness_movement_simulation() {
        let mut harness = GameTestHarness::new(12345);
        let initial_pos = harness.player_position().unwrap();

        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);

        let new_pos = harness.player_position().unwrap();

        // Position might not change if blocked by wall, but shouldn't panic
        assert!(new_pos.0 >= initial_pos.0 - 1);
        harness.assert_valid_state();
    }

    #[test]
    fn test_harness_time_query() {
        let harness = GameTestHarness::new(12345);
        let time = harness.current_time();

        assert!(time.total_minutes() >= 0);
        assert!(time.year >= 0);
    }

    #[test]
    fn test_harness_stats_query() {
        let harness = GameTestHarness::new(12345);
        let stats = harness.get_player_stats();

        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.hp, 100);
        assert_eq!(stats.max_hp, 100);
    }
}
