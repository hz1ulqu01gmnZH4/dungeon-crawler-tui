/// Overmap navigation and world discovery integration tests
///
/// Tests that validate overmap mode, tile discovery, settlement finding,
/// and world exploration mechanics.

mod common;
use common::*;
use crossterm::event::KeyCode;

#[test]
fn test_toggle_overmap_mode() {
    let mut harness = GameTestHarness::new(12345);

    // Should start in local map mode
    assert!(!harness.is_in_overmap_mode());

    // Press Tab to enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should now be in overmap mode
    assert!(harness.is_in_overmap_mode());

    // Press Tab again to return
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    assert!(!harness.is_in_overmap_mode());
}

#[test]
fn test_overmap_world_generation() {
    let mut harness = GameTestHarness::new(12345);

    // Initially no tiles should be discovered
    let initial_discovered = harness.count_discovered_overmap_tiles();

    // Enter overmap mode (triggers world generation)
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should have discovered tiles around starting position
    let discovered = harness.count_discovered_overmap_tiles();
    assert!(discovered > initial_discovered,
        "Should discover tiles when entering overmap mode. Found: {}", discovered);

    // Should have at least starting area visible (7x7 = 49 tiles)
    assert!(discovered >= 49,
        "Should discover at least 7x7 starting area. Found: {}", discovered);

    // Should have settlements
    assert!(!harness.resources.world.settlements.is_empty(),
        "World should have settlements");

    // Should have roads
    assert!(!harness.resources.world.roads.is_empty(),
        "World should have road network");

    // Should have POIs
    assert!(!harness.resources.world.pois.is_empty(),
        "World should have points of interest");
}

#[test]
fn test_overmap_movement_discovers_tiles() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    let initial_discovered = harness.count_discovered_overmap_tiles();
    let initial_pos = harness.player_overmap_position();

    // Move in overmap mode (should discover new tiles)
    for _ in 0..5 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    let new_discovered = harness.count_discovered_overmap_tiles();
    let new_pos = harness.player_overmap_position();

    // Should have moved
    assert_ne!(initial_pos, new_pos, "Player should have moved on overmap");

    // Should have discovered more tiles
    assert!(new_discovered > initial_discovered,
        "Moving should discover new tiles. Was: {}, Now: {}",
        initial_discovered, new_discovered);
}

#[test]
fn test_overmap_time_progression() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    let initial_time = harness.current_time().total_minutes();

    // Move on overmap (each move takes ~60 minutes)
    harness.press_key(KeyCode::Right).ok();
    harness.tick(1);

    let new_time = harness.current_time().total_minutes();

    // Time should have advanced significantly
    let time_diff = new_time - initial_time;
    assert!(time_diff >= 30,
        "Overmap movement should take significant time. Time passed: {} minutes",
        time_diff);
}

#[test]
fn test_find_settlement() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should have settlements
    assert!(!harness.resources.world.settlements.is_empty());

    // Find nearest settlement
    let settlement_pos = harness.find_nearest_settlement();

    // Settlement position should be valid
    assert!(settlement_pos.0 >= 0 && settlement_pos.1 >= 0);

    // There should be a settlement at that position
    let settlement_exists = harness.resources.world.settlements
        .iter()
        .any(|s| s.position == settlement_pos);
    assert!(settlement_exists, "Should find a settlement at calculated position");
}

#[test]
fn test_overmap_navigation_to_settlement() {
    let mut harness = GameTestHarness::new(54321);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    let initial_pos = harness.player_overmap_position();

    // Find a nearby settlement
    let settlement_pos = harness.find_nearest_settlement();

    // Navigate towards it (limited moves to avoid timeout)
    for _ in 0..10 {
        let current = harness.player_overmap_position();
        if current == settlement_pos {
            break;
        }

        let dx = settlement_pos.0 - current.0;
        let dy = settlement_pos.1 - current.1;

        if dx > 0 {
            harness.press_key(KeyCode::Right).ok();
        } else if dx < 0 {
            harness.press_key(KeyCode::Left).ok();
        } else if dy > 0 {
            harness.press_key(KeyCode::Down).ok();
        } else if dy < 0 {
            harness.press_key(KeyCode::Up).ok();
        }

        harness.tick(1);
    }

    let final_pos = harness.player_overmap_position();

    // Should have moved closer to settlement
    let initial_dist = (settlement_pos.0 - initial_pos.0).abs() + (settlement_pos.1 - initial_pos.1).abs();
    let final_dist = (settlement_pos.0 - final_pos.0).abs() + (settlement_pos.1 - final_pos.1).abs();

    assert!(final_dist <= initial_dist,
        "Should move closer to settlement. Initial distance: {}, Final: {}",
        initial_dist, final_dist);
}

#[test]
fn test_overmap_tile_discovery_radius() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Move once
    harness.press_key(KeyCode::Right).ok();
    harness.tick(1);

    let player_pos = harness.player_overmap_position();

    // Check that tiles around player are discovered (5x5 area)
    let mut discovered_count = 0;
    for dy in -2..=2 {
        for dx in -2..=2 {
            let check_x = player_pos.0 + dx;
            let check_y = player_pos.1 + dy;

            if let Some(tile) = harness.resources.world.overmap.get_tile(check_x, check_y) {
                if tile.discovered {
                    discovered_count += 1;
                }
            }
        }
    }

    // Most tiles in 5x5 area should be discovered
    assert!(discovered_count >= 20,
        "Should discover tiles around player position. Found: {}/25",
        discovered_count);
}

#[test]
fn test_overmap_deterministic_generation() {
    // Create two harnesses with same seed
    let mut harness1 = GameTestHarness::new(99999);
    let mut harness2 = GameTestHarness::new(99999);

    // Enter overmap mode on both
    harness1.press_key(KeyCode::Tab).ok();
    harness1.tick(1);

    harness2.press_key(KeyCode::Tab).ok();
    harness2.tick(1);

    // Should have same number of settlements
    assert_eq!(harness1.resources.world.settlements.len(),
               harness2.resources.world.settlements.len(),
               "Same seed should generate same number of settlements");

    // Should have same number of roads
    assert_eq!(harness1.resources.world.roads.len(),
               harness2.resources.world.roads.len(),
               "Same seed should generate same road network");

    // Should have same number of POIs
    assert_eq!(harness1.resources.world.pois.len(),
               harness2.resources.world.pois.len(),
               "Same seed should generate same POIs");

    // First settlement should be at same position with same name
    if !harness1.resources.world.settlements.is_empty() {
        let s1 = &harness1.resources.world.settlements[0];
        let s2 = &harness2.resources.world.settlements[0];

        assert_eq!(s1.position, s2.position, "Settlement positions should match");
        assert_eq!(s1.name, s2.name, "Settlement names should match");
        assert_eq!(s1.settlement_type, s2.settlement_type, "Settlement types should match");
    }
}
