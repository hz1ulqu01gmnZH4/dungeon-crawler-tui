use crate::ecs::{CombatStats, Name, GameEvent};
use crate::ecs::resources::Resources;
use crate::domain_types::{Power, Defense};
use hecs::World;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum AttackResult {
    Miss,
    Hit(i32),      // Normal hit with damage amount
    Critical(i32), // Critical hit with damage amount
}

/// Calculate attack result with hit chance, dodge, and critical hits
fn calculate_attack(
    attacker_power: Power,
    attacker_level: i32,
    target_defense: Defense,
    target_level: i32,
    rng: &mut impl Rng,
) -> AttackResult {
    // Base hit chance: 90% - level difference affects hit chance
    let level_diff = attacker_level - target_level;
    let hit_chance = (90.0 + (level_diff as f32 * 5.0)).clamp(50.0, 95.0);

    // Roll to hit
    let roll = rng.gen_range(0.0..100.0);
    if roll > hit_chance {
        return AttackResult::Miss;
    }

    // Check for critical hit (10% base chance)
    let crit_chance = 10.0;
    let is_crit = rng.gen_range(0.0..100.0) < crit_chance;

    // Calculate base damage
    let base_damage = (attacker_power.value() - target_defense.value()).max(1);

    if is_crit {
        // Critical hits deal 2x damage
        AttackResult::Critical(base_damage * 2)
    } else {
        // Add variance: ±25% damage
        let variance = rng.gen_range(-0.25..=0.25);
        let final_damage = (base_damage as f32 * (1.0 + variance)).round() as i32;
        AttackResult::Hit(final_damage.max(1))
    }
}

pub fn melee_combat_system(world: &mut World, resources: &mut Resources) {
    let mut combat_results = Vec::new();

    // Process Melee events from the queue
    for event in resources.events.iter() {
        if let GameEvent::Melee { attacker, target } = event {
            // Track if player is attacking (for UI)
            if world.get::<&crate::ecs::Player>(*attacker).is_ok() {
                resources.player.last_combat_target = Some(*target);
            }

            // Get attacker stats
            let attacker_stats = match world.get::<&CombatStats>(*attacker) {
                Ok(stats) => *stats,
                Err(_) => continue,
            };

            // Get target stats
            let target_stats = match world.get::<&CombatStats>(*target) {
                Ok(stats) => *stats,
                Err(_) => continue,
            };

            // Get names for logging
            let attacker_name = world
                .get::<&Name>(*attacker)
                .map(|n| n.0.clone())
                .unwrap_or_else(|_| "Someone".to_string());

            let target_name = world
                .get::<&Name>(*target)
                .map(|n| n.0.clone())
                .unwrap_or_else(|_| "something".to_string());

            // Calculate attack with enhanced mechanics
            let result = calculate_attack(
                attacker_stats.power,
                1, // TODO: Add level component later
                target_stats.defense,
                1, // TODO: Add level component later
                &mut resources.sim.rng,
            );

            match result {
                AttackResult::Miss => {
                    resources.ui.log.add(format!("{} attacks {} but misses!", attacker_name, target_name));
                }
                AttackResult::Hit(damage) => {
                    resources.ui.log.add(format!(
                        "{} hits {} for {} damage",
                        attacker_name, target_name, damage
                    ));
                    combat_results.push((*target, damage));
                }
                AttackResult::Critical(damage) => {
                    resources.ui.log.add(format!(
                        "{} lands a CRITICAL HIT on {} for {} damage!",
                        attacker_name, target_name, damage
                    ));
                    combat_results.push((*target, damage));
                }
            }
        }
    }


    // Apply damage
    for (target, damage) in combat_results {
        if let Ok(mut stats) = world.get::<&mut CombatStats>(target) {
            stats.hp = stats.hp.saturating_sub(damage);
        }
    }
}

pub fn death_system(world: &mut World, resources: &mut Resources) {
    let mut dead = Vec::new();

    for (entity, stats) in world.query::<&CombatStats>().iter() {
        if stats.hp.is_dead() {
            dead.push(entity);
        }
    }

    for entity in dead {
        let name = world
            .get::<&Name>(entity)
            .map(|n| n.0.clone())
            .unwrap_or_else(|_| "Something".to_string());

        resources.ui.log.add(format!("{} has died!", name));

        // Check if player died
        if world.get::<&crate::ecs::Player>(entity).is_ok() {
            resources.sim.mode = crate::ecs::RunMode::GameOver;
        } else {
            // Drop loot at monster's position before despawning
            let drop_pos = {
                if let Ok(pos) = world.get::<&crate::ecs::Position>(entity) {
                    Some((pos.x, pos.y))
                } else {
                    None
                }
            };

            if let Some((x, y)) = drop_pos {
                // 40% chance to drop an item
                if resources.sim.rng.gen_bool(0.4) {
                    crate::systems::spawn_random_item(world, x, y, &mut resources.sim.rng);
                    resources.ui.log.add(format!("{} dropped something!", name));
                }
            }

            // Despawn non-player entities
            let _ = world.despawn(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::{Player, Position, RealityLayer};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_calculate_attack_basic_hit() {
        let mut rng = StdRng::seed_from_u64(100); // Seed for deterministic test

        // Power 5, Defense 2 should deal 3 damage (with variance)
        let result = calculate_attack(Power::new(5), 1, Defense::new(2), 1, &mut rng);

        match result {
            AttackResult::Hit(damage) => {
                // Base damage = 5-2 = 3, with variance should be close
                assert!(damage >= 2 && damage <= 4, "Damage {} out of expected range", damage);
            }
            AttackResult::Critical(damage) => {
                // Critical is also valid, should be ~6
                assert!(damage >= 5 && damage <= 7, "Crit damage {} out of expected range", damage);
            }
            AttackResult::Miss => {
                // Miss can happen, not a failure
            }
        }
    }

    #[test]
    fn test_calculate_attack_minimum_damage() {
        let mut rng = StdRng::seed_from_u64(200);

        // Even with 0 power and high defense, should deal at least 1 damage
        let result = calculate_attack(Power::new(0), 1, Defense::new(10), 1, &mut rng);

        match result {
            AttackResult::Hit(damage) => {
                assert!(damage >= 1, "Minimum damage should be 1, got {}", damage);
            }
            AttackResult::Critical(damage) => {
                assert!(damage >= 2, "Minimum crit damage should be 2, got {}", damage);
            }
            AttackResult::Miss => {
                // Miss is valid
            }
        }
    }

    #[test]
    fn test_calculate_attack_defense_reduces_damage() {
        let mut rng = StdRng::seed_from_u64(300);

        // Test multiple times to get hits (not misses)
        let mut found_hit = false;
        for _ in 0..20 {
            let result = calculate_attack(Power::new(10), 1, Defense::new(5), 1, &mut rng);
            if let AttackResult::Hit(damage) | AttackResult::Critical(damage) = result {
                // 10 power - 5 defense = 5 base damage
                // With variance, should be around 3-7 for normal hits
                // Critical would be ~10 (2x base)
                assert!(damage >= 1 && damage <= 12, "Damage {} out of reasonable range", damage);
                found_hit = true;
                break;
            }
        }
        assert!(found_hit, "Should have found at least one hit in 20 attempts");
    }

    #[test]
    fn test_combat_system_basic() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create attacker
        let attacker = world.spawn((
            Name("Attacker".to_string()),
            CombatStats::new(20, 5, 0),
            Position::new(0, 0, RealityLayer::Normal),
        ));

        // Create target
        let target = world.spawn((
            Name("Target".to_string()),
            CombatStats::new(10, 0, 2),
            Position::new(1, 0, RealityLayer::Normal),
        ));

        // Send combat event
        resources.events.send(GameEvent::Melee { attacker, target });

        // Run combat system
        melee_combat_system(&mut world, &mut resources);

        // Target should have taken damage
        let target_stats = world.get::<&CombatStats>(target).unwrap();
        assert!(target_stats.hp.value() < 10, "Target should have taken damage");
    }

    #[test]
    fn test_death_system_removes_entity() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create entity with 0 HP
        let dead_entity = world.spawn((
            Name("Dead Monster".to_string()),
            CombatStats::new(0, 0, 0), // 0 HP = dead
            Position::new(0, 0, RealityLayer::Normal),
        ));

        // Run death system
        death_system(&mut world, &mut resources);

        // Entity should be despawned
        assert!(!world.contains(dead_entity), "Dead entity should be despawned");

        // Log should mention death
        let has_death_message = resources.ui.log.messages.iter().any(|msg| msg.contains("died"));
        assert!(has_death_message, "Should log death message");
    }

    #[test]
    fn test_death_system_player_death() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create player with 0 HP
        let player = world.spawn((
            Name("Player".to_string()),
            Player,
            CombatStats::new(0, 0, 0), // 0 HP = dead
            Position::new(0, 0, RealityLayer::Normal),
        ));

        // Run death system
        death_system(&mut world, &mut resources);

        // Player should still exist (not despawned)
        assert!(world.contains(player), "Player should not be despawned");

        // Game should be over
        assert_eq!(resources.sim.mode, crate::ecs::RunMode::GameOver, "Game should be over when player dies");
    }

    #[test]
    fn test_death_system_negative_hp() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Create entity with negative HP (overkill)
        let dead_entity = world.spawn((
            Name("Overkilled Monster".to_string()),
            CombatStats::new(-5, 0, 0),
            Position::new(0, 0, RealityLayer::Normal),
        ));

        death_system(&mut world, &mut resources);

        // Should still be removed
        assert!(!world.contains(dead_entity), "Entity with negative HP should be despawned");
    }

    #[test]
    fn test_combat_system_multiple_attacks() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let target = world.spawn((
            Name("Target".to_string()),
            CombatStats::new(20, 0, 1),
            Position::new(0, 0, RealityLayer::Normal),
        ));

        // Multiple attackers
        for i in 0..3 {
            let attacker = world.spawn((
                Name(format!("Attacker {}", i)),
                CombatStats::new(10, 4, 0),
                Position::new(i, 0, RealityLayer::Normal),
            ));
            resources.events.send(GameEvent::Melee { attacker, target });
        }

        let initial_hp = world.get::<&CombatStats>(target).unwrap().hp;

        melee_combat_system(&mut world, &mut resources);

        let final_hp = world.get::<&CombatStats>(target).unwrap().hp;
        assert!(final_hp < initial_hp, "Target should have taken damage from multiple attacks");
    }

    #[test]
    fn test_combat_logs_messages() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 54321);

        let attacker = world.spawn((
            Name("Goblin".to_string()),
            CombatStats::new(10, 3, 0),
            Position::new(0, 0, RealityLayer::Normal),
        ));

        let target = world.spawn((
            Name("Player".to_string()),
            Player,
            CombatStats::new(10, 0, 1),
            Position::new(1, 0, RealityLayer::Normal),
        ));

        resources.events.send(GameEvent::Melee { attacker, target });

        let initial_log_count = resources.ui.log.messages.len();
        melee_combat_system(&mut world, &mut resources);

        // Should have logged an attack message
        assert!(resources.ui.log.messages.len() > initial_log_count, "Combat should log messages");
    }

    // NEW TEST: Event-based combat
    #[test]
    fn test_melee_combat_via_events() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let attacker = world.spawn((
            Name("Goblin".to_string()),
            CombatStats::new(10, 3, 0),
            Position::new(0, 0, RealityLayer::Normal),
        ));

        let target = world.spawn((
            Name("Player".to_string()),
            Player,
            CombatStats::new(10, 0, 1),
            Position::new(1, 0, RealityLayer::Normal),
        ));

        // Send Melee event instead of adding component
        resources.events.send(GameEvent::Melee { attacker, target });

        let initial_log_count = resources.ui.log.messages.len();
        melee_combat_system(&mut world, &mut resources);

        // Should have logged an attack message
        assert!(resources.ui.log.messages.len() > initial_log_count, "Combat should log messages via events");

        // Target should have taken damage
        let target_stats = world.get::<&CombatStats>(target).unwrap();
        assert!(target_stats.hp.value() < 10, "Target should have taken damage from event-based attack");
    }
}
