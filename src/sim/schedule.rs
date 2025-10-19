use hecs::World;
use crate::ecs::Resources;

/// A system function that operates on the world and resources
pub type SystemFn = fn(&mut World, &mut Resources);

/// System execution schedule with ordered phases
///
/// The schedule ensures systems run in the correct order:
/// 1. Pre-update: Prep work (currently unused, reserved for future use)
/// 2. Simulation: Core game logic (AI, movement, combat, etc.)
/// 3. Post-simulation: Effects and reactions (death, loot, etc.)
/// 4. Cleanup: Clear temporary data (event queues, etc.)
#[derive(Default)]
pub struct Schedule {
    /// Systems that run before simulation (reserved for future use)
    pub pre_update: Vec<SystemFn>,

    /// Core simulation systems (AI, movement, FOV, combat)
    pub simulation: Vec<SystemFn>,

    /// Post-simulation effects (death handling, etc.)
    pub post_simulation: Vec<SystemFn>,

    /// Cleanup systems (clear events, remove dead entities, etc.)
    pub cleanup: Vec<SystemFn>,
}

impl Schedule {
    /// Create a new empty schedule
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the standard game schedule with all systems
    pub fn build_game_schedule() -> Self {
        use crate::systems;

        Self {
            pre_update: vec![
                // Reserved for future use (e.g., input event generation)
            ],
            simulation: vec![
                // AI generates movement/attack intents (will be migrated to events)
                systems::monster_ai_system,

                // Movement processes move intents and updates positions
                systems::movement_system,

                // FOV updates visibility based on new positions
                systems::update_fov,

                // Combat processes melee intents
                systems::melee_combat_system,

                // Inventory systems process pickup/drop/equip/use intents
                systems::pickup_system,
                systems::drop_system,
                systems::equip_system,
                systems::unequip_system,
                systems::use_item_system,
            ],
            post_simulation: vec![
                // Handle entity deaths
                systems::death_system,
            ],
            cleanup: vec![
                // Clear event queues
                cleanup_events,
            ],
        }
    }

    /// Run all systems in the schedule in order
    pub fn run(&self, world: &mut World, resources: &mut Resources) {
        // Pre-update phase
        for system in &self.pre_update {
            system(world, resources);
        }

        // Simulation phase
        for system in &self.simulation {
            system(world, resources);
        }

        // Post-simulation phase
        for system in &self.post_simulation {
            system(world, resources);
        }

        // Cleanup phase
        for system in &self.cleanup {
            system(world, resources);
        }
    }

    /// Run only the simulation phase (useful for player turns)
    pub fn run_simulation(&self, world: &mut World, resources: &mut Resources) {
        for system in &self.simulation {
            system(world, resources);
        }
    }

    /// Run only the cleanup phase
    pub fn run_cleanup(&self, world: &mut World, resources: &mut Resources) {
        for system in &self.cleanup {
            system(world, resources);
        }
    }
}

/// Cleanup system that clears the event queue
fn cleanup_events(_world: &mut World, resources: &mut Resources) {
    resources.events.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_creation() {
        let schedule = Schedule::new();
        assert_eq!(schedule.pre_update.len(), 0);
        assert_eq!(schedule.simulation.len(), 0);
        assert_eq!(schedule.post_simulation.len(), 0);
        assert_eq!(schedule.cleanup.len(), 0);
    }

    #[test]
    fn test_game_schedule_has_systems() {
        let schedule = Schedule::build_game_schedule();

        // Should have core simulation systems
        assert!(schedule.simulation.len() >= 3, "Should have AI, movement, FOV at minimum");

        // Should have cleanup
        assert_eq!(schedule.cleanup.len(), 1, "Should have event cleanup");
    }

    #[test]
    fn test_cleanup_events() {
        use crate::ecs::GameEvent;

        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Add some events
        let entity = world.spawn(());
        resources.events.send(GameEvent::Move { entity, dest_x: 5, dest_y: 5 });
        resources.events.send(GameEvent::Wait { entity });

        assert_eq!(resources.events.len(), 2);

        // Run cleanup
        cleanup_events(&mut world, &mut resources);

        // Events should be cleared
        assert_eq!(resources.events.len(), 0);
        assert!(resources.events.is_empty());
    }

    #[test]
    fn test_schedule_run_phases() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        // Track which phases ran
        static mut PRE_RAN: bool = false;
        static mut SIM_RAN: bool = false;
        static mut POST_RAN: bool = false;
        static mut CLEANUP_RAN: bool = false;

        fn pre_system(_world: &mut World, _resources: &mut Resources) {
            unsafe { PRE_RAN = true; }
        }

        fn sim_system(_world: &mut World, _resources: &mut Resources) {
            unsafe { SIM_RAN = true; }
        }

        fn post_system(_world: &mut World, _resources: &mut Resources) {
            unsafe { POST_RAN = true; }
        }

        fn cleanup_system(_world: &mut World, _resources: &mut Resources) {
            unsafe { CLEANUP_RAN = true; }
        }

        let schedule = Schedule {
            pre_update: vec![pre_system],
            simulation: vec![sim_system],
            post_simulation: vec![post_system],
            cleanup: vec![cleanup_system],
        };

        schedule.run(&mut world, &mut resources);

        unsafe {
            assert!(PRE_RAN, "Pre-update should have run");
            assert!(SIM_RAN, "Simulation should have run");
            assert!(POST_RAN, "Post-simulation should have run");
            assert!(CLEANUP_RAN, "Cleanup should have run");
        }
    }
}
