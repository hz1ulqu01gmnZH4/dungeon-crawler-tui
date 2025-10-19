# Integration Test Plan - Direct Game Interface Testing

## Overview

Integration tests that directly interface with the running game, simulating player input and verifying game state transitions. These tests validate end-to-end functionality by exercising the full game loop.

## Test Architecture

### Test Framework Structure

```rust
// tests/integration/mod.rs
pub struct GameTestHarness {
    pub game: Game,
    pub world: World,
    pub resources: Resources,
    pub frame_count: u32,
}

impl GameTestHarness {
    /// Create a new test harness with deterministic seed
    pub fn new(seed: u64) -> Self;

    /// Simulate a key press and update game state
    pub fn press_key(&mut self, key: KeyCode) -> Result<(), String>;

    /// Run multiple game ticks
    pub fn tick(&mut self, count: u32);

    /// Get player position
    pub fn player_position(&self) -> Option<(i32, i32)>;

    /// Get current time
    pub fn current_time(&self) -> &WorldTime;

    /// Verify game state hasn't crashed
    pub fn assert_valid_state(&self);

    /// Save game state snapshot for comparison
    pub fn snapshot(&self) -> GameSnapshot;
}

pub struct GameSnapshot {
    player_pos: (i32, i32),
    time: WorldTime,
    in_overmap_mode: bool,
    // ... other relevant state
}
```

## Integration Test Suites

### 1. Movement & FOV Integration
**File**: `tests/integration/movement.rs`

```rust
#[test]
fn test_player_movement_updates_fov() {
    let mut harness = GameTestHarness::new(12345);

    // Get initial player position and FOV state
    let initial_pos = harness.player_position().unwrap();
    let initial_visible_count = harness.count_visible_tiles();

    // Move player right 5 times
    for _ in 0..5 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    // Verify position changed
    let new_pos = harness.player_position().unwrap();
    assert_eq!(new_pos.0, initial_pos.0 + 5);
    assert_eq!(new_pos.1, initial_pos.1);

    // Verify FOV recalculated (visible tiles should have changed)
    let new_visible_count = harness.count_visible_tiles();
    assert!(new_visible_count > 0, "FOV should reveal some tiles");

    // Verify revealed tiles persist
    assert!(harness.count_revealed_tiles() >= new_visible_count);
}

#[test]
fn test_movement_blocked_by_walls() {
    let mut harness = GameTestHarness::new(12345);
    let initial_pos = harness.player_position().unwrap();

    // Try to walk into a wall (find a wall first)
    let wall_direction = harness.find_direction_with_wall(initial_pos);

    if let Some(direction) = wall_direction {
        harness.press_key(direction);
        harness.tick(1);

        // Position should not change
        let new_pos = harness.player_position().unwrap();
        assert_eq!(new_pos, initial_pos, "Player should not move through walls");
    }
}

#[test]
fn test_movement_progresses_time() {
    let mut harness = GameTestHarness::new(12345);
    let initial_time = harness.current_time().total_minutes();

    // Move 10 times
    for _ in 0..10 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    let new_time = harness.current_time().total_minutes();
    assert!(new_time > initial_time, "Time should progress with movement");
}

#[test]
fn test_diagonal_movement() {
    let mut harness = GameTestHarness::new(12345);
    let initial_pos = harness.player_position().unwrap();

    // Move diagonally (up-right)
    harness.press_key(KeyCode::Up);
    harness.tick(1);
    harness.press_key(KeyCode::Right);
    harness.tick(1);

    let new_pos = harness.player_position().unwrap();
    assert_eq!(new_pos.0, initial_pos.0 + 1, "X should increase");
    assert_eq!(new_pos.1, initial_pos.1 - 1, "Y should decrease");
}
```

### 2. Overmap Integration
**File**: `tests/integration/overmap.rs`

```rust
#[test]
fn test_overmap_toggle() {
    let mut harness = GameTestHarness::new(12345);

    // Initially in local map mode
    assert!(!harness.is_in_overmap_mode());

    // Press Tab to toggle overmap
    harness.press_key(KeyCode::Tab);
    harness.tick(1);

    // Should now be in overmap mode
    assert!(harness.is_in_overmap_mode());

    // Press Tab again to return
    harness.press_key(KeyCode::Tab);
    harness.tick(1);

    assert!(!harness.is_in_overmap_mode());
}

#[test]
fn test_overmap_movement() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab);
    harness.tick(1);

    let initial_overmap_pos = harness.player_overmap_position();

    // Move on overmap
    for _ in 0..5 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    let new_overmap_pos = harness.player_overmap_position();
    assert_eq!(new_overmap_pos.0, initial_overmap_pos.0 + 5);

    // Verify time progressed more (overmap movement is faster)
    // Each overmap tile = significant time
}

#[test]
fn test_overmap_discovery() {
    let mut harness = GameTestHarness::new(12345);
    harness.press_key(KeyCode::Tab);
    harness.tick(1);

    let initial_discovered = harness.count_discovered_overmap_tiles();

    // Move around to discover tiles
    for _ in 0..10 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    let new_discovered = harness.count_discovered_overmap_tiles();
    assert!(new_discovered > initial_discovered, "Moving should discover new tiles");
}

#[test]
fn test_settlement_entry() {
    let mut harness = GameTestHarness::new(12345);

    // Find nearest settlement on overmap
    let settlement_pos = harness.find_nearest_settlement();

    // Navigate to settlement
    harness.press_key(KeyCode::Tab);
    harness.tick(1);
    harness.navigate_overmap_to(settlement_pos);

    // Enter settlement with Enter key
    harness.press_key(KeyCode::Enter);
    harness.tick(1);

    // Should now be in local map mode inside settlement
    assert!(!harness.is_in_overmap_mode());
    assert!(harness.current_location().is_some());
}
```

### 3. Time & Weather Integration
**File**: `tests/integration/time_weather.rs`

```rust
#[test]
fn test_day_night_cycle() {
    let mut harness = GameTestHarness::new(12345);

    // Set initial time to just before dawn
    harness.set_time_of_day(TimeOfDay::DeepNight);

    let initial_tod = harness.current_time().time_of_day();
    assert_eq!(initial_tod, TimeOfDay::DeepNight);

    // Advance time significantly by moving
    for _ in 0..1000 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    let new_tod = harness.current_time().time_of_day();
    assert_ne!(new_tod, initial_tod, "Time of day should have changed");
}

#[test]
fn test_weather_changes_over_time() {
    let mut harness = GameTestHarness::new(12345);

    let initial_weather = harness.current_weather();

    // Advance time by many days
    harness.advance_time_by_days(30);

    // Weather should have changed at some point
    let weather_history = harness.get_weather_history();
    assert!(weather_history.len() > 1, "Weather should change over time");
}

#[test]
fn test_seasonal_weather_patterns() {
    let mut harness = GameTestHarness::new(12345);

    // Advance through all seasons
    let seasons_weather = vec![];
    for season_idx in 0..4 {
        harness.set_season(season_idx);
        let weather = harness.sample_weather_for_days(30);
        seasons_weather.push(weather);
    }

    // Verify weather patterns differ by season
    // e.g., more snow in winter, more rain in spring
}

#[test]
fn test_camping_and_rest() {
    let mut harness = GameTestHarness::new(12345);

    let initial_time = harness.current_time().clone();

    // Press 'c' to camp
    harness.press_key(KeyCode::Char('c'));
    harness.tick(1);

    // Time should have advanced by rest duration
    let new_time = harness.current_time();
    assert!(new_time.total_minutes() > initial_time.total_minutes() + 60);

    // Player stats should be restored
    let player_stats = harness.get_player_stats();
    assert_eq!(player_stats.hp, player_stats.max_hp);
}
```

### 4. Save/Load Integration
**File**: `tests/integration/save_load.rs`

```rust
#[test]
fn test_save_and_load_preserves_state() {
    let mut harness = GameTestHarness::new(12345);

    // Make some changes to game state
    for _ in 0..20 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    let snapshot_before = harness.snapshot();

    // Save game
    let save_path = harness.save_game("test_save");

    // Modify state further
    for _ in 0..10 {
        harness.press_key(KeyCode::Left);
        harness.tick(1);
    }

    let snapshot_after_changes = harness.snapshot();
    assert_ne!(snapshot_before.player_pos, snapshot_after_changes.player_pos);

    // Load saved game
    harness.load_game(&save_path);

    let snapshot_after_load = harness.snapshot();

    // State should match pre-save snapshot
    assert_eq!(snapshot_before.player_pos, snapshot_after_load.player_pos);
    assert_eq!(snapshot_before.time, snapshot_after_load.time);
}

#[test]
fn test_save_preserves_world_state() {
    let mut harness = GameTestHarness::new(12345);

    // Discover some overmap tiles
    harness.press_key(KeyCode::Tab);
    harness.tick(1);
    for _ in 0..50 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    let discovered_count = harness.count_discovered_overmap_tiles();

    // Save and reload
    let save_path = harness.save_game("world_test");
    harness.load_game(&save_path);

    // Discovered tiles should be preserved
    let new_discovered_count = harness.count_discovered_overmap_tiles();
    assert_eq!(discovered_count, new_discovered_count);
}

#[test]
fn test_multiple_save_slots() {
    let mut harness = GameTestHarness::new(12345);

    // Create save slot 1
    harness.press_key(KeyCode::Right);
    harness.tick(1);
    let pos1 = harness.player_position().unwrap();
    harness.save_game("slot1");

    // Create save slot 2
    harness.press_key(KeyCode::Left);
    harness.tick(1);
    let pos2 = harness.player_position().unwrap();
    harness.save_game("slot2");

    // Load slot 1
    harness.load_game("slot1");
    assert_eq!(harness.player_position().unwrap(), pos1);

    // Load slot 2
    harness.load_game("slot2");
    assert_eq!(harness.player_position().unwrap(), pos2);
}
```

### 5. Travel Events Integration
**File**: `tests/integration/travel_events.rs`

```rust
#[test]
fn test_travel_events_trigger() {
    let mut harness = GameTestHarness::new(12345);

    // Move on overmap to trigger travel events
    harness.press_key(KeyCode::Tab);
    harness.tick(1);

    let initial_event_count = harness.get_message_log().len();

    // Move extensively
    for _ in 0..100 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);
    }

    // Should have triggered at least some travel events
    let new_event_count = harness.get_message_log().len();
    assert!(new_event_count > initial_event_count, "Travel should generate events");
}

#[test]
fn test_travel_event_terrain_dependency() {
    let mut harness = GameTestHarness::new(12345);

    // Navigate to different terrain types and collect events
    let forest_events = harness.collect_events_in_terrain(TerrainType::Forest, 50);
    let mountain_events = harness.collect_events_in_terrain(TerrainType::Mountains, 50);

    // Events should differ based on terrain
    assert!(forest_events != mountain_events);
}

#[test]
fn test_dangerous_event_affects_stats() {
    let mut harness = GameTestHarness::new(12345);

    let initial_hp = harness.get_player_stats().hp;

    // Force a dangerous event
    harness.trigger_specific_event(EventType::Danger);

    let new_hp = harness.get_player_stats().hp;
    assert!(new_hp < initial_hp, "Dangerous events should damage player");
}
```

### 6. Building Entry Integration
**File**: `tests/integration/buildings.rs`

```rust
#[test]
fn test_building_entry_and_exit() {
    let mut harness = GameTestHarness::new(12345);

    // Navigate to a settlement with buildings
    let settlement_pos = harness.find_settlement_with_buildings();
    harness.navigate_to_settlement(settlement_pos);

    // Find a building entrance
    let building_pos = harness.find_nearest_building_entrance();
    harness.navigate_to_position(building_pos);

    let before_location = harness.current_location();

    // Enter building
    harness.press_key(KeyCode::Enter);
    harness.tick(1);

    // Should be in building interior
    assert_ne!(harness.current_location(), before_location);
    assert!(harness.is_in_building());

    // Exit building
    harness.navigate_to_building_exit();
    harness.press_key(KeyCode::Enter);
    harness.tick(1);

    // Should be back in settlement
    assert_eq!(harness.current_location(), before_location);
}

#[test]
fn test_different_building_types_load() {
    let mut harness = GameTestHarness::new(12345);

    let building_types = vec![
        BuildingType::House,
        BuildingType::Inn,
        BuildingType::Shop,
        BuildingType::Temple,
        BuildingType::Blacksmith,
        BuildingType::Library,
        BuildingType::Warehouse,
    ];

    for building_type in building_types {
        let building_pos = harness.find_building_of_type(building_type);
        harness.navigate_and_enter_building(building_pos);

        // Verify correct interior loaded
        assert!(harness.is_in_building());
        let interior_map = harness.get_current_map();
        let (width, height) = building_type.dimensions();
        assert_eq!(interior_map.width, width);
        assert_eq!(interior_map.height, height);

        // Exit
        harness.exit_building();
    }
}
```

### 7. Layer Switching Integration
**File**: `tests/integration/reality_layers.rs`

```rust
#[test]
fn test_layer_switching() {
    let mut harness = GameTestHarness::new(12345);

    // Initially in Normal layer
    assert_eq!(harness.current_layer(), RealityLayer::Normal);

    // Switch to Cosmic layer
    harness.press_key(KeyCode::Char('L')); // Shift+L
    harness.tick(1);

    assert_eq!(harness.current_layer(), RealityLayer::Cosmic);

    // Switch back
    harness.press_key(KeyCode::Char('L'));
    harness.tick(1);

    assert_eq!(harness.current_layer(), RealityLayer::Normal);
}

#[test]
fn test_layer_preserves_position() {
    let mut harness = GameTestHarness::new(12345);

    let pos_normal = harness.player_position().unwrap();

    // Switch to Cosmic
    harness.press_key(KeyCode::Char('L'));
    harness.tick(1);

    let pos_cosmic = harness.player_position().unwrap();
    assert_eq!(pos_normal, pos_cosmic, "Position should be preserved across layers");

    // Move in cosmic layer
    harness.press_key(KeyCode::Right);
    harness.tick(1);

    // Switch back to Normal
    harness.press_key(KeyCode::Char('L'));
    harness.tick(1);

    // Position should match where we moved in Cosmic
    let pos_back_to_normal = harness.player_position().unwrap();
    assert_eq!(pos_back_to_normal.0, pos_normal.0 + 1);
}
```

### 8. Stress & Performance Integration
**File**: `tests/integration/stress.rs`

```rust
#[test]
fn test_extended_gameplay_session() {
    let mut harness = GameTestHarness::new(12345);

    // Simulate 1000 game ticks worth of gameplay
    for i in 0..1000 {
        // Alternate between different actions
        match i % 5 {
            0 => harness.press_key(KeyCode::Right),
            1 => harness.press_key(KeyCode::Up),
            2 => harness.press_key(KeyCode::Tab), // Toggle overmap
            3 => harness.press_key(KeyCode::Char('c')), // Camp
            4 => harness.press_key(KeyCode::Left),
            _ => {}
        }
        harness.tick(1);

        // Periodically verify state is valid
        if i % 100 == 0 {
            harness.assert_valid_state();
        }
    }

    // After extended play, game should still be functional
    harness.assert_valid_state();
}

#[test]
fn test_rapid_input() {
    let mut harness = GameTestHarness::new(12345);

    // Rapidly press keys without ticking
    for _ in 0..100 {
        harness.press_key(KeyCode::Right);
        harness.press_key(KeyCode::Left);
        harness.press_key(KeyCode::Up);
        harness.press_key(KeyCode::Down);
    }

    // Process all inputs
    harness.tick(1);

    // Game should handle this gracefully
    harness.assert_valid_state();
}

#[test]
fn test_save_load_under_stress() {
    let mut harness = GameTestHarness::new(12345);

    // Repeatedly save and load while moving
    for i in 0..50 {
        harness.press_key(KeyCode::Right);
        harness.tick(1);

        if i % 10 == 0 {
            let save_path = harness.save_game(&format!("stress_{}", i));
            harness.load_game(&save_path);
            harness.assert_valid_state();
        }
    }
}
```

## Test Execution Strategy

### Test Organization

```
tests/
├── integration/
│   ├── mod.rs           # Test harness implementation
│   ├── movement.rs      # Movement & FOV tests
│   ├── overmap.rs       # Overmap navigation tests
│   ├── time_weather.rs  # Time/weather tests
│   ├── save_load.rs     # Persistence tests
│   ├── travel_events.rs # Event system tests
│   ├── buildings.rs     # Building entry tests
│   ├── reality_layers.rs # Layer switching tests
│   └── stress.rs        # Stress & performance tests
└── test_helpers.rs      # Shared utilities
```

### Running Tests

```bash
# Run all integration tests
cargo test --test integration

# Run specific test suite
cargo test --test integration movement

# Run with output
cargo test --test integration -- --nocapture

# Run stress tests (longer)
cargo test --test integration stress -- --ignored
```

## Expected Outcomes

### Success Criteria

1. **All movement tests pass**: Player moves correctly, FOV updates, walls block
2. **Overmap navigation works**: Toggle, movement, discovery, settlement entry
3. **Time advances correctly**: Movement, camping, day/night cycles
4. **Save/load reliable**: State persists, multiple slots work
5. **Events trigger appropriately**: Based on terrain, time, conditions
6. **Buildings accessible**: Enter/exit, correct interiors load
7. **Layers switch cleanly**: Position preserved, separate maps
8. **No crashes under stress**: Extended play, rapid input, save/load spam

### Performance Targets

- Each integration test completes in < 1 second
- Stress test completes in < 10 seconds
- No memory leaks during extended gameplay
- Save/load operations < 100ms

## Implementation Priority

1. **Phase 1**: Test harness infrastructure (GameTestHarness)
2. **Phase 2**: Core gameplay tests (movement, overmap, time)
3. **Phase 3**: Persistence tests (save/load)
4. **Phase 4**: Advanced features (buildings, events, layers)
5. **Phase 5**: Stress and performance tests

## Maintenance

- Run integration tests on every commit
- Add new integration tests for new features
- Update harness when game architecture changes
- Profile slow tests and optimize
- Keep tests deterministic (seeded RNG)

## Notes

- Integration tests should be **deterministic** - use seeded RNG
- Tests should be **isolated** - clean state between tests
- Tests should be **fast** - avoid unnecessary delays
- Tests should **fail clearly** - good error messages
- Tests should **cover edge cases** - not just happy paths
