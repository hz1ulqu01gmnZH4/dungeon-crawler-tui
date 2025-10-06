use crate::ecs::{WantsToMelee, CombatStats, Name};
use crate::ecs::resources::Resources;
use hecs::World;

pub fn melee_combat_system(world: &mut World, resources: &mut Resources) {
    let mut combat_results = Vec::new();

    // Collect combat intents
    for (attacker, wants_melee) in world.query::<&WantsToMelee>().iter() {
        let target = wants_melee.target;

        // Get attacker stats
        let attacker_stats = match world.get::<&CombatStats>(attacker) {
            Ok(stats) => *stats,
            Err(_) => continue,
        };

        // Get target stats
        let target_stats = match world.get::<&CombatStats>(target) {
            Ok(stats) => *stats,
            Err(_) => continue,
        };

        // Calculate damage
        let damage = (attacker_stats.power - target_stats.defense).max(0);

        // Get names for logging
        let attacker_name = world
            .get::<&Name>(attacker)
            .map(|n| n.0.clone())
            .unwrap_or_else(|_| "Someone".to_string());

        let target_name = world
            .get::<&Name>(target)
            .map(|n| n.0.clone())
            .unwrap_or_else(|_| "something".to_string());

        if damage > 0 {
            resources.log.add(format!(
                "{} attacks {} for {} damage",
                attacker_name, target_name, damage
            ));
            combat_results.push((target, damage));
        } else {
            resources.log.add(format!(
                "{} attacks {} but it has no effect",
                attacker_name, target_name
            ));
        }
    }

    // Apply damage
    for (target, damage) in combat_results {
        if let Ok(mut stats) = world.get::<&mut CombatStats>(target) {
            stats.hp -= damage;
        }
    }

    // Clean up combat intents
    let entities_to_clean: Vec<_> = world
        .query::<&WantsToMelee>()
        .iter()
        .map(|(e, _)| e)
        .collect();
    for entity in entities_to_clean {
        let _ = world.remove_one::<WantsToMelee>(entity);
    }
}

pub fn death_system(world: &mut World, resources: &mut Resources) {
    let mut dead = Vec::new();

    for (entity, stats) in world.query::<&CombatStats>().iter() {
        if stats.hp <= 0 {
            dead.push(entity);
        }
    }

    for entity in dead {
        let name = world
            .get::<&Name>(entity)
            .map(|n| n.0.clone())
            .unwrap_or_else(|_| "Something".to_string());

        resources.log.add(format!("{} has died!", name));

        // Check if player died
        if world.get::<&crate::ecs::Player>(entity).is_ok() {
            resources.mode = crate::ecs::RunMode::GameOver;
        } else {
            // Despawn non-player entities
            let _ = world.despawn(entity);
        }
    }
}
