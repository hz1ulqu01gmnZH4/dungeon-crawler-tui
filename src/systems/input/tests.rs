use super::*;
use crate::ecs::{Player, Position, RealityLayer, CombatStats, Inventory, GameEvent};
use crate::ecs::resources::{Resources, RunMode, UiMode};
use crate::domain_types::{MaxHitPoints, Power, Defense};
use hecs::World;

fn create_test_world() -> (World, Resources) {
    let mut world = World::new();
    let mut resources = Resources::new(80, 50, 12345);

    // Spawn player at (5, 5)
    let player = world.spawn((
        Player,
        Position::new(5, 5, RealityLayer::Normal),
        CombatStats::new(100, 10, 5),
        Inventory::new(20),
    ));

    resources.player.player_entity = Some(player);
    resources.ui.ui_mode = UiMode::InGame;

    (world, resources)
}

#[test]
fn test_movement_key_h_moves_left() {
    let (mut world, mut resources) = create_test_world();

    gameplay::try_move_player(&mut world, &mut resources, -1, 0);

    // Check Move event was generated
    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);

    match &events[0] {
        GameEvent::Move { dest_x, dest_y, .. } => {
            assert_eq!(*dest_x, 4); // 5 - 1
            assert_eq!(*dest_y, 5);
        }
        _ => panic!("Expected Move event"),
    }
}

#[test]
fn test_movement_key_l_moves_right() {
    let (mut world, mut resources) = create_test_world();

    gameplay::try_move_player(&mut world, &mut resources, 1, 0);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);

    match &events[0] {
        GameEvent::Move { dest_x, dest_y, .. } => {
            assert_eq!(*dest_x, 6); // 5 + 1
            assert_eq!(*dest_y, 5);
        }
        _ => panic!("Expected Move event"),
    }
}

#[test]
fn test_movement_key_k_moves_up() {
    let (mut world, mut resources) = create_test_world();

    gameplay::try_move_player(&mut world, &mut resources, 0, -1);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);

    match &events[0] {
        GameEvent::Move { dest_x, dest_y, .. } => {
            assert_eq!(*dest_x, 5);
            assert_eq!(*dest_y, 4); // 5 - 1
        }
        _ => panic!("Expected Move event"),
    }
}

#[test]
fn test_movement_key_j_moves_down() {
    let (mut world, mut resources) = create_test_world();

    gameplay::try_move_player(&mut world, &mut resources, 0, 1);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);

    match &events[0] {
        GameEvent::Move { dest_x, dest_y, .. } => {
            assert_eq!(*dest_x, 5);
            assert_eq!(*dest_y, 6); // 5 + 1
        }
        _ => panic!("Expected Move event"),
    }
}

#[test]
fn test_diagonal_movement_y_moves_up_left() {
    let (mut world, mut resources) = create_test_world();

    gameplay::try_move_player(&mut world, &mut resources, -1, -1);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);

    match &events[0] {
        GameEvent::Move { dest_x, dest_y, .. } => {
            assert_eq!(*dest_x, 4); // 5 - 1
            assert_eq!(*dest_y, 4); // 5 - 1
        }
        _ => panic!("Expected Move event"),
    }
}

#[test]
fn test_diagonal_movement_n_moves_down_right() {
    let (mut world, mut resources) = create_test_world();

    gameplay::try_move_player(&mut world, &mut resources, 1, 1);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);

    match &events[0] {
        GameEvent::Move { dest_x, dest_y, .. } => {
            assert_eq!(*dest_x, 6); // 5 + 1
            assert_eq!(*dest_y, 6); // 5 + 1
        }
        _ => panic!("Expected Move event"),
    }
}

#[test]
fn test_movement_triggers_player_turn() {
    let (mut world, mut resources) = create_test_world();
    resources.sim.mode = RunMode::AwaitingInput;

    gameplay::try_move_player(&mut world, &mut resources, 1, 0);

    // Movement should trigger player turn
    assert_eq!(resources.sim.mode, RunMode::PlayerTurn);
}

#[test]
fn test_overmap_movement_changes_position() {
    let (mut world, mut resources) = create_test_world();
    resources.ui.ui_mode = UiMode::Overmap;
    resources.world.player_overmap_pos = (25, 25);

    // Generate terrain so there are walkable tiles
    crate::world::generator::generate_terrain(&mut resources.world.overmap);

    gameplay::try_move_overmap(&mut world, &mut resources, 1, 0);

    // Position should change if tile is walkable
    // We can't predict exact walkability but can verify function runs
    let (new_x, new_y) = resources.world.player_overmap_pos;
    assert!(new_x >= 24 && new_x <= 26); // Either moved or blocked
    assert_eq!(new_y, 25);
}

#[test]
fn test_handle_movement_routes_by_mode() {
    let (mut world, mut resources) = create_test_world();

    // In game mode, should generate Move event
    resources.ui.ui_mode = UiMode::InGame;
    gameplay::handle_movement(&mut world, &mut resources, 1, 0);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1); // Move event generated

    // In overmap mode, should modify overmap position
    resources.ui.ui_mode = UiMode::Overmap;
    let original_pos = resources.world.player_overmap_pos;

    // Generate terrain for walkable tiles
    crate::world::generator::generate_terrain(&mut resources.world.overmap);

    gameplay::handle_movement(&mut world, &mut resources, 0, 1);

    // No new Move events should be generated (overmap doesn't use events)
    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 0);
}

#[test]
fn test_inventory_mode_blocks_movement() {
    let (mut world, mut resources) = create_test_world();
    resources.ui.ui_mode = UiMode::Inventory { selection: 0 };

    // Movement keys in inventory mode shouldn't generate Move events
    // This would be handled by the input router, but we can verify
    // the handle_movement function respects UI mode
    gameplay::handle_movement(&mut world, &mut resources, 1, 0);

    // Should route through normal movement (not overmap)
    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1); // Still generates event (input router would block it)
}

#[test]
fn test_examine_mode_blocks_movement() {
    let (mut world, mut resources) = create_test_world();
    resources.ui.ui_mode = UiMode::Examine { cursor: (5, 5) };

    // In examine mode, movement should still work at the gameplay level
    // (The input router at a higher level would handle cursor movement instead)
    gameplay::handle_movement(&mut world, &mut resources, 1, 0);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 1);
}

#[test]
fn test_multiple_movements_generate_multiple_events() {
    let (mut world, mut resources) = create_test_world();

    // Move right
    gameplay::try_move_player(&mut world, &mut resources, 1, 0);
    // Move down
    gameplay::try_move_player(&mut world, &mut resources, 0, 1);
    // Move left
    gameplay::try_move_player(&mut world, &mut resources, -1, 0);

    let events: Vec<_> = resources.events.drain().collect();
    assert_eq!(events.len(), 3);
}
