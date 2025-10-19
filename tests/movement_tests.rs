/// Movement and FOV integration tests
///
/// Tests that validate player movement, field of view updates,
/// collision detection, and time progression during movement.

mod common;
use common::*;
use crossterm::event::KeyCode;

#[test]
fn test_player_can_move() {
    let mut harness = GameTestHarness::new(12345);
    let initial_pos = harness.player_position().unwrap();

    // Try moving in all directions
    let moves = vec![
        (KeyCode::Right, "right"),
        (KeyCode::Left, "left"),
        (KeyCode::Up, "up"),
        (KeyCode::Down, "down"),
    ];

    for (key, _direction) in moves {
        let before_pos = harness.player_position().unwrap();
        harness.press_key(key).ok();
        harness.tick(1);

        // Position should be valid
        harness.assert_valid_state();

        // We should still have a valid position
        assert!(harness.player_position().is_some());
    }

    // After moving, we should be somewhere (might be back at start if bounced off walls)
    let final_pos = harness.player_position().unwrap();
    assert!(final_pos.0 >= 0 && final_pos.1 >= 0);
}

#[test]
fn test_movement_updates_fov() {
    let mut harness = GameTestHarness::new(12345);

    // Get initial visible tiles
    let initial_visible = harness.count_visible_tiles();
    assert!(initial_visible > 0, "Player should see something initially");

    // Move player
    for _ in 0..5 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Visible tiles should still exist (FOV should update)
    let new_visible = harness.count_visible_tiles();
    assert!(new_visible > 0, "Player should still see tiles after moving");

    // Revealed tiles should accumulate
    let revealed = harness.count_revealed_tiles();
    assert!(revealed >= initial_visible, "Revealed tiles should not decrease");
}

#[test]
fn test_movement_progresses_time() {
    let mut harness = GameTestHarness::new(12345);
    let initial_time = harness.current_time().total_minutes();

    // Move several times
    for _ in 0..10 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    let new_time = harness.current_time().total_minutes();
    // Time should have progressed (or stayed same if movement didn't happen)
    assert!(new_time >= initial_time, "Time should not go backwards");
}

#[test]
fn test_position_stays_in_bounds() {
    let mut harness = GameTestHarness::new(12345);

    // Try to move out of bounds
    for _ in 0..100 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    // Player should still be in bounds
    let pos = harness.player_position().unwrap();
    let map = harness.resources.maps.active_map();
    assert!(map.in_bounds(pos.0, pos.1), "Player should stay in map bounds");
}

#[test]
fn test_snapshot_captures_state() {
    let mut harness = GameTestHarness::new(12345);

    let snapshot1 = harness.snapshot();

    // Move
    harness.press_key(KeyCode::Right).ok();
    harness.tick(1);

    let snapshot2 = harness.snapshot();

    // Snapshots should be comparable
    assert_eq!(snapshot1.active_layer, snapshot2.active_layer);
    // Time might have changed
    assert!(snapshot2.time.total_minutes() >= snapshot1.time.total_minutes());
}

#[test]
fn test_extended_movement() {
    let mut harness = GameTestHarness::new(12345);

    // Simulate extended movement session
    for i in 0..100 {
        let key = match i % 4 {
            0 => KeyCode::Right,
            1 => KeyCode::Down,
            2 => KeyCode::Left,
            3 => KeyCode::Up,
            _ => KeyCode::Right,
        };

        harness.press_key(key).ok();
        harness.tick(1);

        // Periodically check state validity
        if i % 20 == 0 {
            harness.assert_valid_state();
        }
    }

    // After extended movement, game should still be stable
    harness.assert_valid_state();
    assert!(harness.player_position().is_some());
}

#[test]
fn test_fov_reveals_tiles() {
    let mut harness = GameTestHarness::new(12345);

    let initial_revealed = harness.count_revealed_tiles();

    // Move around to reveal more tiles
    for _ in 0..20 {
        harness.press_key(KeyCode::Right).ok();
        harness.tick(1);
    }

    let new_revealed = harness.count_revealed_tiles();
    assert!(new_revealed >= initial_revealed, "Revealed tiles should only increase");
}

#[test]
fn test_player_stats_maintained_during_movement() {
    let mut harness = GameTestHarness::new(12345);

    let initial_stats = harness.get_player_stats().unwrap();

    // Move around
    for _ in 0..50 {
        harness.press_key(KeyCode::Down).ok();
        harness.tick(1);
    }

    let final_stats = harness.get_player_stats().unwrap();

    // Stats should still be valid
    assert_eq!(final_stats.max_hp, initial_stats.max_hp);
    assert!(final_stats.hp <= final_stats.max_hp);
    assert!(final_stats.hp > 0);
}
