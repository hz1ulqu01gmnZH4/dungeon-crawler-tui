/// Settlement entry and exploration integration tests
///
/// Tests that validate settlement entry, exit, map generation,
/// and settlement interior navigation.

mod common;
use common::*;
use crossterm::event::KeyCode;

#[test]
fn test_enter_settlement() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap mode
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Navigate to nearest settlement
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // Should be in overmap mode at settlement
    assert!(harness.is_in_overmap_mode());
    assert_eq!(harness.player_overmap_position(), settlement_pos);

    // Press Enter to enter settlement
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Should now be in local map mode (inside settlement)
    assert!(!harness.is_in_overmap_mode());

    // Should have a current location set
    assert!(harness.current_location().is_some(),
        "Should have current_location set when in settlement");

    // Message log should mention entering
    let messages = harness.get_message_log();
    let has_enter_message = messages.iter().any(|m| m.contains("You enter"));
    assert!(has_enter_message, "Should log settlement entry");
}

#[test]
fn test_exit_settlement() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap and navigate to settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // Enter settlement
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    assert!(harness.current_location().is_some());
    let location_id = harness.current_location();

    // Press Tab to exit settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should be back in overmap mode
    assert!(harness.is_in_overmap_mode());

    // Current location should be cleared
    assert!(harness.current_location().is_none(),
        "Should clear current_location when exiting settlement");

    // Message log should mention leaving
    let messages = harness.get_message_log();
    let has_leave_message = messages.iter().any(|m| m.contains("leave"));
    assert!(has_leave_message, "Should log settlement exit");
}

#[test]
fn test_settlement_map_generation() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap and navigate to settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // Get settlement ID
    let settlement = harness.resources.settlements
        .iter()
        .find(|s| s.position == settlement_pos)
        .expect("Should find settlement");
    let settlement_id = settlement.id;

    // Initially no settlement map generated
    assert!(!harness.resources.settlement_maps.contains_key(&settlement_id));

    // Enter settlement (triggers map generation)
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Settlement map should now exist
    assert!(harness.resources.settlement_maps.contains_key(&settlement_id),
        "Should generate settlement map on first entry");
}

#[test]
fn test_settlement_map_persistence() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap and navigate to settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // Enter settlement twice
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    let location_id = harness.current_location().unwrap();

    // Exit
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Re-enter
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Should still have the same location ID
    assert_eq!(harness.current_location(), Some(location_id),
        "Re-entering settlement should use same location ID");

    // Settlement map should be persisted (not regenerated)
    assert!(harness.resources.settlement_maps.contains_key(&location_id));
}

#[test]
fn test_player_spawns_at_settlement_entrance() {
    let mut harness = GameTestHarness::new(12345);

    // Enter overmap and navigate to settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);

    // Enter settlement
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    // Get player position in settlement
    let player_pos = harness.player_position().unwrap();
    let map = harness.resources.maps.active_map();

    // Player should be near bottom of map (entrance area)
    assert!(player_pos.1 > map.height / 2,
        "Player should spawn near entrance (bottom of map). Position: {:?}, Map height: {}",
        player_pos, map.height);

    // Player should be standing on floor, not wall
    let player_idx = map.xy_idx(player_pos.0, player_pos.1);
    assert_eq!(map.tiles[player_idx], dungeon_clawler_tui::map::Tile::Floor,
        "Player should spawn on floor tile");
}

#[test]
fn test_movement_inside_settlement() {
    let mut harness = GameTestHarness::new(12345);

    // Enter settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    let initial_pos = harness.player_position().unwrap();

    // Try moving around
    let moves = [KeyCode::Up, KeyCode::Left, KeyCode::Right, KeyCode::Down];
    for key in &moves {
        harness.press_key(*key).ok();
        harness.tick(1);
    }

    // Should still have valid position
    let final_pos = harness.player_position().unwrap();
    harness.assert_valid_state();

    // Should be in same settlement
    assert!(harness.current_location().is_some());
}

#[test]
fn test_different_settlement_types() {
    let mut harness = GameTestHarness::new(11111);

    // Enter overmap
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);

    // Should have different types of settlements
    use dungeon_clawler_tui::world::SettlementType;

    let has_city = harness.resources.settlements.iter()
        .any(|s| s.settlement_type == SettlementType::City);
    let has_town = harness.resources.settlements.iter()
        .any(|s| s.settlement_type == SettlementType::Town);
    let has_village = harness.resources.settlements.iter()
        .any(|s| s.settlement_type == SettlementType::Village);

    // Should have at least 2 different types
    let type_count = [has_city, has_town, has_village].iter().filter(|&&x| x).count();
    assert!(type_count >= 2,
        "World should have at least 2 different settlement types. Found: City:{}, Town:{}, Village:{}",
        has_city, has_town, has_village);
}

#[test]
fn test_settlement_map_is_traversable() {
    let mut harness = GameTestHarness::new(12345);

    // Enter settlement
    harness.press_key(KeyCode::Tab).ok();
    harness.tick(1);
    let settlement_pos = harness.find_nearest_settlement();
    harness.navigate_overmap_to(settlement_pos);
    harness.press_key(KeyCode::Enter).ok();
    harness.tick(1);

    let map = harness.resources.maps.active_map();

    // Count floor tiles (settlement maps are currently floor-based)
    let floor_count = map.tiles.iter()
        .filter(|&&t| t == dungeon_clawler_tui::map::Tile::Floor)
        .count();

    // Settlement should be mostly traversable (placeholder implementation)
    let total_tiles = map.tiles.len();
    let floor_ratio = floor_count as f32 / total_tiles as f32;

    assert!(floor_ratio > 0.5,
        "Settlement should be mostly traversable. Floor: {}/{} ({:.1}%)",
        floor_count, total_tiles, floor_ratio * 100.0);

    // Map should have reasonable dimensions
    assert!(map.width >= 30 && map.width <= 60,
        "Settlement map should have reasonable width: {}", map.width);
    assert!(map.height >= 30 && map.height <= 60,
        "Settlement map should have reasonable height: {}", map.height);
}
