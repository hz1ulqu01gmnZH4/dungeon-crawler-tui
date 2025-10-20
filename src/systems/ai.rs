use crate::ecs::{Position, Monster, Player, Viewshed, GameEvent, RealityLayer};
use crate::ecs::resources::Resources;
use hecs::World;

pub fn monster_ai_system(world: &mut World, resources: &mut Resources) {
    // Get player position
    let player_pos = {
        let mut player_pos = None;
        for (_, (pos, _)) in world.query::<(&Position, &Player)>().iter() {
            player_pos = Some((*pos).clone());
            break;
        }
        player_pos
    };

    let Some(player_pos) = player_pos else {
        return;
    };

    // For each monster, decide action
    let mut monster_moves = Vec::new();
    for (entity, (pos, viewshed, _monster)) in world.query::<(&Position, &Viewshed, &Monster)>().iter() {
        // Only act if same layer as player
        if pos.layer != player_pos.layer {
            continue;
        }

        // Check if player is visible
        let can_see_player = viewshed.visible.contains(&(player_pos.x, player_pos.y));

        if can_see_player {
            // Move towards player
            let dx = (player_pos.x - pos.x).signum();
            let dy = (player_pos.y - pos.y).signum();

            let dest_x = pos.x + dx;
            let dest_y = pos.y + dy;

            monster_moves.push((entity, dest_x, dest_y));
        }
    }

    // Send Move events for monster movement
    for (entity, dest_x, dest_y) in monster_moves {
        resources.events.send(GameEvent::Move { entity, dest_x, dest_y });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::{Name, Renderable};

    fn create_test_world() -> (World, Resources) {
        let world = World::new();
        let resources = Resources::new(80, 50, 12345);
        (world, resources)
    }

    #[test]
    fn test_monster_moves_toward_visible_player() {
        let (mut world, mut resources) = create_test_world();

        // Spawn player at (10, 10)
        world.spawn((
            Player,
            Position::new(10, 10, RealityLayer::Normal),
        ));

        // Spawn monster at (5, 5) that can see player
        let monster = world.spawn((
            Monster,
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed {
                range: 8,
                visible: vec![(10, 10)], // Player is visible
                dirty: false,
            },
            Name("Goblin".to_string()),
        ));

        // Run AI system
        monster_ai_system(&mut world, &mut resources);

        // Check that a Move event was generated
        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 1);

        match &events[0] {
            GameEvent::Move { entity, dest_x, dest_y } => {
                assert_eq!(*entity, monster);
                // Monster should move diagonally toward player: (5,5) -> (6,6)
                assert_eq!(*dest_x, 6);
                assert_eq!(*dest_y, 6);
            }
            _ => panic!("Expected Move event"),
        }
    }

    #[test]
    fn test_monster_does_not_move_when_player_not_visible() {
        let (mut world, mut resources) = create_test_world();

        // Spawn player
        world.spawn((
            Player,
            Position::new(10, 10, RealityLayer::Normal),
        ));

        // Spawn monster that cannot see player
        world.spawn((
            Monster,
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed {
                range: 8,
                visible: vec![], // Player not visible
                dirty: false,
            },
        ));

        // Run AI system
        monster_ai_system(&mut world, &mut resources);

        // Check that no Move events were generated
        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_monster_moves_horizontally_when_aligned() {
        let (mut world, mut resources) = create_test_world();

        // Spawn player at (10, 5)
        world.spawn((
            Player,
            Position::new(10, 5, RealityLayer::Normal),
        ));

        // Spawn monster at (5, 5) - same Y, different X
        let monster = world.spawn((
            Monster,
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed {
                range: 8,
                visible: vec![(10, 5)],
                dirty: false,
            },
        ));

        monster_ai_system(&mut world, &mut resources);

        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 1);

        match &events[0] {
            GameEvent::Move { entity, dest_x, dest_y } => {
                assert_eq!(*entity, monster);
                // Should move right: (5,5) -> (6,5)
                assert_eq!(*dest_x, 6);
                assert_eq!(*dest_y, 5);
            }
            _ => panic!("Expected Move event"),
        }
    }

    #[test]
    fn test_monster_moves_vertically_when_aligned() {
        let (mut world, mut resources) = create_test_world();

        // Spawn player at (5, 10)
        world.spawn((
            Player,
            Position::new(5, 10, RealityLayer::Normal),
        ));

        // Spawn monster at (5, 5) - same X, different Y
        let monster = world.spawn((
            Monster,
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed {
                range: 8,
                visible: vec![(5, 10)],
                dirty: false,
            },
        ));

        monster_ai_system(&mut world, &mut resources);

        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 1);

        match &events[0] {
            GameEvent::Move { entity, dest_x, dest_y } => {
                assert_eq!(*entity, monster);
                // Should move down: (5,5) -> (5,6)
                assert_eq!(*dest_x, 5);
                assert_eq!(*dest_y, 6);
            }
            _ => panic!("Expected Move event"),
        }
    }

    #[test]
    fn test_monster_ignores_player_in_different_reality_layer() {
        let (mut world, mut resources) = create_test_world();

        // Spawn player in Normal layer
        world.spawn((
            Player,
            Position::new(10, 10, RealityLayer::Normal),
        ));

        // Spawn monster in Cosmic layer
        world.spawn((
            Monster,
            Position::new(5, 5, RealityLayer::Cosmic),
            Viewshed {
                range: 8,
                visible: vec![(10, 10)],
                dirty: false,
            },
        ));

        monster_ai_system(&mut world, &mut resources);

        // Monster should not move because player is in different layer
        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_multiple_monsters_generate_multiple_moves() {
        let (mut world, mut resources) = create_test_world();

        // Spawn player
        world.spawn((
            Player,
            Position::new(10, 10, RealityLayer::Normal),
        ));

        // Spawn 3 monsters that can see player
        for i in 0..3 {
            world.spawn((
                Monster,
                Position::new(5 + i, 5, RealityLayer::Normal),
                Viewshed {
                    range: 8,
                    visible: vec![(10, 10)],
                    dirty: false,
                },
                Name(format!("Monster {}", i)),
            ));
        }

        monster_ai_system(&mut world, &mut resources);

        // Should have 3 move events
        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 3);

        // All should be Move events
        for event in events {
            match event {
                GameEvent::Move { .. } => {},
                _ => panic!("Expected Move event"),
            }
        }
    }

    #[test]
    fn test_ai_does_nothing_without_player() {
        let (mut world, mut resources) = create_test_world();

        // Spawn monster but no player
        world.spawn((
            Monster,
            Position::new(5, 5, RealityLayer::Normal),
            Viewshed {
                range: 8,
                visible: vec![],
                dirty: false,
            },
        ));

        monster_ai_system(&mut world, &mut resources);

        // No events should be generated
        let events: Vec<_> = resources.events.drain().collect();
        assert_eq!(events.len(), 0);
    }
}
