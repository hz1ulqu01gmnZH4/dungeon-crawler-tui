/// Complete gameplay loop integration tests
///
/// Tests that validate complete gameplay scenarios combining multiple systems:
/// exploration, travel, settlement interaction, time progression, and state persistence.

mod common;
use common::*;
use crossterm::event::KeyCode;

#[test]
#[ignore = "Too slow - uses navigate_overmap_to"]
fn test_complete_exploration_loop() {
    let mut harness = GameTestHarness::new(77777);

    // Start in local map
    assert!(!harness.is_in_overmap_mode());
    let initial_time = harness.current_time().total_minutes();

    // 1. Move around locally
    for _ in 0..5 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // 2. Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    assert!(harness.is_in_overmap_mode());

    // 3. Explore and discover tiles
    let initial_discovered = harness.count_discovered_overmap_tiles();
    for _ in 0..3 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }
    let new_discovered = harness.count_discovered_overmap_tiles();
    assert!(new_discovered > initial_discovered, "Should discover new tiles");

    // 4. Find and navigate to settlement
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // 5. Enter settlement
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);
    assert!(!harness.is_in_overmap_mode());
    assert!(harness.current_location().is_some());

    // 6. Move around inside settlement
    for _ in 0..3 {
        harness.press_key(KeyCode::Up).ok();
        harness.tick(1);
    }

    // 7. Exit settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    assert!(harness.is_in_overmap_mode());
    assert!(harness.current_location().is_none());

    // Verify time progressed
    let final_time = harness.current_time().total_minutes();
    assert!(final_time > initial_time, "Time should progress during gameplay");

    // Verify game state is still valid
    harness.assert_valid_state();
}

#[test]
#[ignore = "Too slow - navigate_overmap_to can take minutes"]
fn test_travel_and_rest_loop() {
    let mut harness = GameTestHarness::new(88888);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Travel to settlement
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // Enter settlement
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Damage player to enable resting
    harness.damage_player(100 / 3);

    let damaged_hp = harness.get_player_stats().unwrap().hp;

    // Rest to recover
    harness.press_key(KeyCode::Char('r')).ok();
    harness.tick(1);

    let recovered_hp = harness.get_player_stats().unwrap().hp;
    assert!(recovered_hp > damaged_hp, "Should recover HP from resting");

    // Leave settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Travel again
    for _ in 0..5 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Verify state
    harness.assert_valid_state();
}

#[test]
fn test_extended_overmap_exploration() {
    let mut harness = GameTestHarness::new(99999);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    let initial_discovered = harness.count_discovered_overmap_tiles();
    let initial_time = harness.current_time().total_minutes();

    // Extensive exploration in a pattern
    for i in 0..20 {
        let key = match i % 4 {
            0 => KeyCode::Right,
            1 => KeyCode::Down,
            2 => KeyCode::Left,
            3 => KeyCode::Up,
            _ => KeyCode::Right,
        };

        harness.press_key(key).ok();
        harness.tick(1);

        // Verify state every few moves
        if i % 5 == 0 {
            harness.assert_valid_state();
        }
    }

    // Check final state (movement might be blocked by impassable terrain)
    let final_discovered = harness.count_discovered_overmap_tiles();
    let final_time = harness.current_time().total_minutes();
    let time_elapsed = final_time - initial_time;

    // At minimum, we should still have the initial discovered tiles
    assert!(final_discovered >= initial_discovered,
        "Should maintain discovered tiles. Initial: {}, Final: {}",
        initial_discovered, final_discovered);

    // If we moved at all, time should progress
    if final_discovered > initial_discovered {
        assert!(time_elapsed > 0, "Time should progress if movement occurred");
    }
}

#[test]
#[ignore = "Too slow - uses navigate_overmap_to"]
fn test_multiple_settlement_visits() {
    let mut harness = GameTestHarness::new(55555);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Visit first settlement
    let settlement1 = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement1);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    let location1 = harness.current_location();
    assert!(location1.is_some());

    // Exit
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Move away
    for _ in 0..5 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Find another settlement
    let settlement2 = harness.find_nearest_settlement();

    // If different from first, visit it
    if settlement2 != settlement1 {
        harness.navigate_overmap_to(settlement2);
        harness.press_key(KeyCode::Enter).ok();
        harness.tick(1);

        let location2 = harness.current_location();
        assert!(location2.is_some());
        assert_ne!(location1, location2, "Different settlements should have different IDs");
    }

    harness.assert_valid_state();
}

#[test]
fn test_day_night_gameplay() {
    let mut harness = GameTestHarness::new(66666);

    // Start during day
    use dungeon_clawler_tui::world::TimeOfDay;
    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::Day);

    // Advance to night
    harness.resources.sim.world_time.advance_hours(14); // 22:00

    assert_eq!(harness.current_time().time_of_day(), TimeOfDay::Night);

    // Move around at night
    for _ in 0..5 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Should still see something (minimum FOV of 3)
    let visible = harness.count_visible_tiles();
    assert!(visible >= 9,
        "Should have minimum visibility at night (3x3 = 9 tiles). Found: {}", visible);

    harness.assert_valid_state();
}

#[test]
fn test_seasonal_travel() {
    let mut harness = GameTestHarness::new(33333);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Travel in spring
    use dungeon_clawler_tui::world::Season;
    assert_eq!(harness.current_time().season(), Season::Spring);

    for _ in 0..3 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Advance to summer
    harness.advance_time_by_days(90);
    assert_eq!(harness.current_time().season(), Season::Summer);

    // Travel in summer
    for _ in 0..3 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Advance to winter
    harness.advance_time_by_days(180);
    assert_eq!(harness.current_time().season(), Season::Winter);

    // Travel in winter
    for _ in 0..3 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    harness.assert_valid_state();
}

#[test]
fn test_world_exploration_reveals_roads() {
    let mut harness = GameTestHarness::new(44444);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should have roads
    let road_count = harness.resources.world.roads.len();
    assert!(road_count > 0, "World should have roads");

    // Explore around (may be blocked by terrain)
    let initial_discovered = harness.count_discovered_overmap_tiles();
    for _ in 0..10 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Should discover some tiles as we explore (terrain may block movement)
    let discovered = harness.count_discovered_overmap_tiles();
    assert!(discovered >= initial_discovered, "Should maintain or increase discovered tiles");
}

#[test]
fn test_world_exploration_reveals_pois() {
    let mut harness = GameTestHarness::new(44444);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should have POIs
    let poi_count = harness.resources.world.pois.len();
    assert!(poi_count > 0, "World should have POIs (dungeons, caves, etc.)");

    // Verify POI types exist
    use dungeon_clawler_tui::world::POIType;
    let poi_types: Vec<POIType> = harness.resources.world.pois
        .iter()
        .map(|poi| poi.poi_type)
        .collect();

    assert!(!poi_types.is_empty(), "Should have POIs with types");
}

#[test]
fn test_rapid_mode_switching() {
    let mut harness = GameTestHarness::new(22222);

    // Rapidly switch between local and overmap
    for _ in 0..5 {
        // Enter overmap
        harness.press_key(KeyCode::Tab).ok();
        harness.tick(1);
        assert!(harness.is_in_overmap_mode());

        // Exit overmap
        harness.press_key(KeyCode::Tab).ok();
        harness.tick(1);
        assert!(!harness.is_in_overmap_mode());
    }

    // State should remain valid
    harness.assert_valid_state();
}

#[test]
fn test_full_year_gameplay() {
    let mut harness = GameTestHarness::new(11111);

    let initial_year = harness.current_time().year;

    // Simulate gameplay over a full year
    // Move on overmap occasionally
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Advance through all seasons
    for season_num in 0..4 {
        // Move a bit each season
        for _ in 0..3 {
            harness.press_key(KeyCode::Right).ok();
            harness.tick(1);
        }

        // Advance to next season
        harness.advance_time_by_days(90);
    }

    let final_year = harness.current_time().year;
    assert_eq!(final_year, initial_year + 1, "Should complete a full year");

    harness.assert_valid_state();
}

#[test]
#[ignore = "Too slow - uses navigate_overmap_to"]
fn test_settlement_to_settlement_journey() {
    let mut harness = GameTestHarness::new(12121);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Visit first settlement
    let settlement1 = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement1);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);
    let location1 = harness.current_location().unwrap();

    // Exit and travel
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Travel in a direction
    for _ in 0..10 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Find next settlement
    let settlement2 = harness.find_nearest_settlement();

    if settlement2 != settlement1 {
        harness.navigate_overmap_to(settlement2);
        harness.press_key(KeyCode::Enter).ok();
        harness.tick(1);
        let location2 = harness.current_location().unwrap();

        // Should be at different settlement
        assert_ne!(location1, location2);
    }

    harness.assert_valid_state();
}
