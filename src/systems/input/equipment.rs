use crate::ecs::{Inventory, EquipSlot, GameEvent};
use crate::ecs::resources::{Resources, RunMode};
use crate::systems::inventory::find_equipable_for_slot;
use hecs::World;

/// Quick wield weapon - equip first unequipped weapon in inventory to main hand
pub(super) fn quick_wield_weapon(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player.player_entity {
        let weapon_to_equip = {
            if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                find_equipable_for_slot(world, &*inventory, EquipSlot::MainHand)
            } else {
                None
            }
        };

        if let Some(weapon) = weapon_to_equip {
            resources.events.send(GameEvent::EquipItem { entity: player_entity, item: weapon });
            resources.sim.mode = RunMode::PlayerTurn;
        } else {
            // Check if already have something equipped
            let already_equipped = {
                if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                    inventory.get_equipped(EquipSlot::MainHand).is_some()
                } else {
                    false
                }
            };

            if already_equipped {
                resources.ui.log.add("You already have a weapon equipped.");
            } else {
                resources.ui.log.add("You have no weapon to wield.");
            }
        }
    }
}

/// Quick wear armor - equip first unequipped armor in inventory to body
pub(super) fn quick_wear_armor(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player.player_entity {
        let armor_to_equip = {
            if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                find_equipable_for_slot(world, &*inventory, EquipSlot::Body)
            } else {
                None
            }
        };

        if let Some(armor) = armor_to_equip {
            resources.events.send(GameEvent::EquipItem { entity: player_entity, item: armor });
            resources.sim.mode = RunMode::PlayerTurn;
        } else {
            // Check if already have something equipped
            let already_equipped = {
                if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                    inventory.get_equipped(EquipSlot::Body).is_some()
                } else {
                    false
                }
            };

            if already_equipped {
                resources.ui.log.add("You already have armor equipped.");
            } else {
                resources.ui.log.add("You have no armor to wear.");
            }
        }
    }
}

/// Quick take off - unequip currently equipped item (prioritize: weapon > armor > other)
pub(super) fn quick_take_off(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player.player_entity {
        // Find which slot to unequip (priority: MainHand > Body > OffHand > others)
        let slot_to_unequip = {
            if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                if inventory.get_equipped(EquipSlot::MainHand).is_some() {
                    Some(EquipSlot::MainHand)
                } else if inventory.get_equipped(EquipSlot::Body).is_some() {
                    Some(EquipSlot::Body)
                } else if inventory.get_equipped(EquipSlot::OffHand).is_some() {
                    Some(EquipSlot::OffHand)
                } else if inventory.get_equipped(EquipSlot::Head).is_some() {
                    Some(EquipSlot::Head)
                } else if inventory.get_equipped(EquipSlot::Legs).is_some() {
                    Some(EquipSlot::Legs)
                } else if inventory.get_equipped(EquipSlot::Feet).is_some() {
                    Some(EquipSlot::Feet)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(slot) = slot_to_unequip {
            resources.events.send(GameEvent::UnequipItem { entity: player_entity, slot });
            resources.sim.mode = RunMode::PlayerTurn;
        } else {
            resources.ui.log.add("You have nothing equipped to take off.");
        }
    }
}
