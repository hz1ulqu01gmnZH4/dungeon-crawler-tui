use hecs::Entity;
use crate::ecs::EquipSlot;

/// Generic event queue container
#[derive(Debug, Clone)]
pub struct Events<T> {
    queue: Vec<T>,
}

impl<T> Events<T> {
    pub fn new() -> Self {
        Events { queue: Vec::new() }
    }

    /// Add an event to the queue
    pub fn send(&mut self, event: T) {
        self.queue.push(event);
    }

    /// Get an iterator over all events (for processing)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.queue.iter()
    }

    /// Drain all events from the queue (for processing and clearing)
    pub fn drain(&mut self) -> std::vec::Drain<T> {
        self.queue.drain(..)
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Check if there are any events
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get the number of events in the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Game simulation events
/// These replace the WantsTo* intent components to avoid archetype churn
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// Entity wants to move to a destination
    Move {
        entity: Entity,
        dest_x: i32,
        dest_y: i32,
    },

    /// Entity wants to attack another entity in melee
    Melee {
        attacker: Entity,
        target: Entity,
    },

    /// Entity wants to wait/skip turn
    Wait {
        entity: Entity,
    },

    /// Entity wants to pick up an item
    PickupItem {
        entity: Entity,
        item: Entity,
    },

    /// Entity wants to drop an item
    DropItem {
        entity: Entity,
        item: Entity,
    },

    /// Entity wants to equip an item
    EquipItem {
        entity: Entity,
        item: Entity,
    },

    /// Entity wants to unequip an item from a slot
    UnequipItem {
        entity: Entity,
        slot: EquipSlot,
    },

    /// Entity wants to use/consume an item
    UseItem {
        entity: Entity,
        item: Entity,
    },

    /// Entity wants to open a door at position
    OpenDoor {
        entity: Entity,
        x: i32,
        y: i32,
    },

    /// Entity wants to close a door at position
    CloseDoor {
        entity: Entity,
        x: i32,
        y: i32,
    },

    /// Entity wants to use stairs
    UseStairs {
        entity: Entity,
        ascending: bool,
    },

    /// Entity has died (for cleanup)
    Death {
        entity: Entity,
    },
}

impl GameEvent {
    /// Get the primary entity involved in this event
    pub fn entity(&self) -> Entity {
        match self {
            GameEvent::Move { entity, .. } => *entity,
            GameEvent::Melee { attacker, .. } => *attacker,
            GameEvent::Wait { entity } => *entity,
            GameEvent::PickupItem { entity, .. } => *entity,
            GameEvent::DropItem { entity, .. } => *entity,
            GameEvent::EquipItem { entity, .. } => *entity,
            GameEvent::UnequipItem { entity, .. } => *entity,
            GameEvent::UseItem { entity, .. } => *entity,
            GameEvent::OpenDoor { entity, .. } => *entity,
            GameEvent::CloseDoor { entity, .. } => *entity,
            GameEvent::UseStairs { entity, .. } => *entity,
            GameEvent::Death { entity } => *entity,
        }
    }

    /// Check if this event involves a specific entity
    pub fn involves(&self, target: Entity) -> bool {
        match self {
            GameEvent::Melee { attacker, target: t } => {
                *attacker == target || *t == target
            }
            _ => self.entity() == target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hecs::World;

    #[test]
    fn test_events_send_and_drain() {
        let mut events = Events::<GameEvent>::new();
        assert!(events.is_empty());

        // Create a test entity properly using hecs World
        let mut world = World::new();
        let entity = world.spawn(());

        events.send(GameEvent::Move { entity, dest_x: 5, dest_y: 10 });
        events.send(GameEvent::Wait { entity });

        assert_eq!(events.len(), 2);
        assert!(!events.is_empty());

        let drained: Vec<_> = events.drain().collect();
        assert_eq!(drained.len(), 2);
        assert!(events.is_empty());
    }

    #[test]
    fn test_events_clear() {
        let mut events = Events::<GameEvent>::new();
        let mut world = World::new();
        let entity = world.spawn(());

        events.send(GameEvent::Move { entity, dest_x: 0, dest_y: 0 });
        events.send(GameEvent::Wait { entity });

        assert_eq!(events.len(), 2);
        events.clear();
        assert!(events.is_empty());
    }

    #[test]
    fn test_game_event_entity() {
        let mut world = World::new();
        let entity = world.spawn(());
        let event = GameEvent::Move { entity, dest_x: 5, dest_y: 10 };

        assert_eq!(event.entity(), entity);
    }

    #[test]
    fn test_game_event_involves() {
        let mut world = World::new();
        let attacker = world.spawn(());
        let target = world.spawn(());
        let other = world.spawn(());

        let event = GameEvent::Melee { attacker, target };

        assert!(event.involves(attacker));
        assert!(event.involves(target));
        assert!(!event.involves(other));
    }
}
