/// Inventory management systems
///
/// Handles item pickup, drop, equip/unequip, and usage

use crate::ecs::{
    Consumable, CombatStats, Equipable, Inventory, Item, ItemData, OnGround, Player, Position,
    Stackable, WantsToDropItem, WantsToEquipItem, WantsToPickupItem, WantsToUnequipItem,
    WantsToUseItem,
};
use crate::ecs::resources::Resources;
use hecs::World;

/// Process item pickup intents
pub fn pickup_system(world: &mut World, resources: &mut Resources) {
    let mut pickups = Vec::new();

    // Collect all pickup intents
    for (entity, intent) in world.query::<&WantsToPickupItem>().iter() {
        pickups.push((entity, intent.item));
    }

    // Process pickups
    for (entity, item) in pickups {
        // Check if this is a stackable item
        let is_stackable = world.get::<&Stackable>(item).is_ok();

        let mut stacked = false;

        if is_stackable {
            // Extract item data first (clone to avoid borrow issues)
            let item_info = {
                if let Ok(item_data) = world.get::<&ItemData>(item) {
                    let pickup_qty = if let Ok(stackable) = world.get::<&Stackable>(item) {
                        stackable.quantity
                    } else {
                        1
                    };
                    Some((item_data.name.clone(), item_data.max_stack, pickup_qty))
                } else {
                    None
                }
            };

            if let Some((item_name, max_stack, pickup_qty)) = item_info {
                // Search inventory for existing stack
                let stack_target = {
                    if let Ok(inventory) = world.get::<&Inventory>(entity) {
                        let mut target = None;
                        for &inv_item in &inventory.items {
                            if let Ok(inv_data) = world.get::<&ItemData>(inv_item) {
                                if inv_data.name == item_name {
                                    if let Ok(inv_stackable) = world.get::<&Stackable>(inv_item) {
                                        let space_available = max_stack - inv_stackable.quantity;
                                        if space_available > 0 {
                                            target = Some((inv_item, space_available));
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        target
                    } else {
                        None
                    }
                };

                // Now perform the stacking (all immutable borrows dropped)
                if let Some((inv_item, space_available)) = stack_target {
                    let qty_to_add = pickup_qty.min(space_available);

                    // Update stack quantity in separate scope
                    let new_quantity = {
                        if let Ok(mut inv_stackable) = world.get::<&mut Stackable>(inv_item) {
                            inv_stackable.quantity += qty_to_add;
                            Some(inv_stackable.quantity)
                        } else {
                            None
                        }
                    }; // Mutable borrow dropped here

                    // Now safe to despawn
                    if new_quantity.is_some() {
                        let _ = world.despawn(item);
                        stacked = true;

                        resources.log.add(format!(
                            "You pick up {} {} (stack: {}/{}).",
                            qty_to_add, item_name, new_quantity.unwrap(), max_stack
                        ));
                    }
                }
            }
        }

        // If not stacked, add normally
        if !stacked {
            // Try to add item to inventory
            let pickup_result = {
                if let Ok(mut inventory) = world.get::<&mut Inventory>(entity) {
                    inventory.add_item(item)
                } else {
                    Err("No inventory")
                }
            };

            // Handle result
            match pickup_result {
                Ok(_) => {
                    // Remove OnGround marker
                    let _ = world.remove_one::<OnGround>(item);

                    // Get item name for message
                    if let Ok(item_data) = world.get::<&ItemData>(item) {
                        if let Ok(stackable) = world.get::<&Stackable>(item) {
                            resources.log.add(format!(
                                "You pick up {} (qty: {}).",
                                item_data.name, stackable.quantity
                            ));
                        } else {
                            resources.log.add(format!("You pick up {}.", item_data.name));
                        }
                    } else {
                        resources.log.add("You pick up an item.");
                    }
                }
                Err(msg) => {
                    resources.log.add(msg);
                }
            }
        }

        // Remove intent
        let _ = world.remove_one::<WantsToPickupItem>(entity);
    }
}

/// Process item drop intents
pub fn drop_system(world: &mut World, resources: &mut Resources) {
    let mut drops = Vec::new();

    // Collect all drop intents
    for (entity, intent) in world.query::<&WantsToDropItem>().iter() {
        drops.push((entity, intent.item));
    }

    // Process drops
    for (entity, item) in drops {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            continue;
        };

        // Check if item is equipped and find which slot
        let equipped_slot = {
            if let Ok(inventory) = world.get::<&Inventory>(entity) {
                inventory.equipped.iter()
                    .find(|(_, &equipped_item)| equipped_item == item)
                    .map(|(slot, _)| *slot)
            } else {
                None
            }
        };

        // Unequip if needed
        let was_equipped = if let Some(slot) = equipped_slot {
            if let Ok(mut inventory) = world.get::<&mut Inventory>(entity) {
                inventory.unequip(slot);
                true
            } else {
                false
            }
        } else {
            false
        };

        // Remove from inventory
        let removed = {
            if let Ok(mut inventory) = world.get::<&mut Inventory>(entity) {
                inventory.remove_item(item)
            } else {
                false
            }
        };

        if removed {
            // Place on ground at player position
            if let Ok(mut item_pos) = world.get::<&mut Position>(item) {
                item_pos.x = player_pos.0;
                item_pos.y = player_pos.1;
                item_pos.layer = player_pos.2;
            }

            // Add OnGround marker
            let _ = world.insert_one(item, OnGround);

            // Get item name for message
            if let Ok(item_data) = world.get::<&ItemData>(item) {
                if was_equipped {
                    resources.log.add(format!("You unequip and drop {}.", item_data.name));
                } else {
                    resources.log.add(format!("You drop {}.", item_data.name));
                }
            } else {
                resources.log.add("You drop an item.");
            }
        }

        // Remove intent
        let _ = world.remove_one::<WantsToDropItem>(entity);
    }
}

/// Process equip item intents
pub fn equip_system(world: &mut World, resources: &mut Resources) {
    let mut equips = Vec::new();

    // Collect all equip intents
    for (entity, intent) in world.query::<&WantsToEquipItem>().iter() {
        equips.push((entity, intent.item));
    }

    // Process equips
    for (entity, item) in equips {
        // Extract equip data first
        let equip_data = if let Ok(equipable) = world.get::<&Equipable>(item) {
            Some((equipable.slot, equipable.power_bonus, equipable.defense_bonus))
        } else {
            None
        };

        if let Some((slot, power_bonus, defense_bonus)) = equip_data {
            // Check if item is in inventory and get old equipped item
            let (has_item, old_item_data) = {
                if let Ok(inventory) = world.get::<&Inventory>(entity) {
                    let has = inventory.has_item(item);
                    let old = inventory.get_equipped(slot);
                    let old_bonuses = if let Some(old_item) = old {
                        if let Ok(old_equipable) = world.get::<&Equipable>(old_item) {
                            Some((old_equipable.power_bonus, old_equipable.defense_bonus))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    (has, old_bonuses)
                } else {
                    (false, None)
                }
            };

            if !has_item {
                resources.log.add("You must pick up the item first.");
                let _ = world.remove_one::<WantsToEquipItem>(entity);
                continue;
            }

            // Remove old item bonuses
            if let Some((old_power, old_defense)) = old_item_data {
                if let Ok(mut stats) = world.get::<&mut CombatStats>(entity) {
                    stats.power -= old_power;
                    stats.defense -= old_defense;
                }
            }

            // Equip new item
            if let Ok(mut inventory) = world.get::<&mut Inventory>(entity) {
                inventory.equip(slot, item);
            }

            // Apply new item bonuses
            if let Ok(mut stats) = world.get::<&mut CombatStats>(entity) {
                stats.power += power_bonus;
                stats.defense += defense_bonus;
            }

            // Get item name for message
            if let Ok(item_data) = world.get::<&ItemData>(item) {
                resources.log.add(format!(
                    "You equip {} in {}. (+{} power, +{} defense)",
                    item_data.name,
                    slot.name(),
                    power_bonus,
                    defense_bonus
                ));
            } else {
                resources.log.add(format!("You equip an item in {}.", slot.name()));
            }
        } else {
            resources.log.add("This item cannot be equipped.");
        }

        // Remove intent
        let _ = world.remove_one::<WantsToEquipItem>(entity);
    }
}

/// Process unequip item intents
pub fn unequip_system(world: &mut World, resources: &mut Resources) {
    let mut unequips = Vec::new();

    // Collect all unequip intents
    for (entity, intent) in world.query::<&WantsToUnequipItem>().iter() {
        unequips.push((entity, intent.slot));
    }

    // Process unequips
    for (entity, slot) in unequips {
        if let Ok(mut inventory) = world.get::<&mut Inventory>(entity) {
            if let Some(item) = inventory.get_equipped(slot) {
                // Remove stat bonuses
                if let Ok(equipable) = world.get::<&Equipable>(item) {
                    if let Ok(mut stats) = world.get::<&mut CombatStats>(entity) {
                        stats.power -= equipable.power_bonus;
                        stats.defense -= equipable.defense_bonus;
                    }
                }

                // Unequip
                inventory.unequip(slot);

                // Get item name for message
                if let Ok(item_data) = world.get::<&ItemData>(item) {
                    resources.log.add(format!("You unequip {} from {}.", item_data.name, slot.name()));
                } else {
                    resources.log.add(format!("You unequip an item from {}.", slot.name()));
                }
            } else {
                resources.log.add(format!("Nothing equipped in {}.", slot.name()));
            }
        }

        // Remove intent
        let _ = world.remove_one::<WantsToUnequipItem>(entity);
    }
}

/// Process use item intents (consumables)
pub fn use_item_system(world: &mut World, resources: &mut Resources) {
    let mut uses = Vec::new();

    // Collect all use intents
    for (entity, intent) in world.query::<&WantsToUseItem>().iter() {
        uses.push((entity, intent.item));
    }

    // Process uses
    for (entity, item) in uses {
        // Extract consumable data
        let consumable_data = {
            if let Ok(consumable) = world.get::<&Consumable>(item) {
                Some((consumable.hp_restore, consumable.uses))
            } else {
                None
            }
        };

        if let Some((hp_restore, uses_left)) = consumable_data {
            // Check if item has uses left
            if uses_left <= 0 {
                resources.log.add("This item has no uses left.");
                let _ = world.remove_one::<WantsToUseItem>(entity);
                continue;
            }

            // Apply effect
            let actual_restore = {
                if let Ok(mut stats) = world.get::<&mut CombatStats>(entity) {
                    let old_hp = stats.hp;
                    stats.hp = (stats.hp + hp_restore).min(stats.max_hp);
                    stats.hp - old_hp
                } else {
                    0
                }
            };

            // Get item name for message
            if let Ok(item_data) = world.get::<&ItemData>(item) {
                if actual_restore > 0 {
                    resources.log.add(format!(
                        "You use {}. Restored {} HP.",
                        item_data.name, actual_restore
                    ));
                } else {
                    resources.log.add(format!("You use {} but are already at full HP.", item_data.name));
                }
            } else if actual_restore > 0 {
                resources.log.add(format!("Restored {} HP.", actual_restore));
            }

            // Decrease uses
            let new_uses = {
                if let Ok(mut consumable) = world.get::<&mut Consumable>(item) {
                    consumable.uses -= 1;
                    consumable.uses
                } else {
                    uses_left - 1
                }
            };

            // Handle stackable items
            let is_stackable = world.get::<&Stackable>(item).is_ok();
            let should_remove_item = if is_stackable && new_uses <= 0 {
                // Reduce stack quantity
                if let Ok(mut stackable) = world.get::<&mut Stackable>(item) {
                    stackable.quantity = stackable.quantity.saturating_sub(1);

                    if stackable.quantity > 0 {
                        // Reset uses for next item in stack
                        if let Ok(mut consumable) = world.get::<&mut Consumable>(item) {
                            consumable.uses = 1; // Reset to default (healing potions have 1 use)
                        }
                        false // Don't remove, still items in stack
                    } else {
                        true // Stack empty, remove item
                    }
                } else {
                    true // Not stackable, remove
                }
            } else if new_uses <= 0 {
                true // Non-stackable item with no uses
            } else {
                false // Still has uses
            };

            // Remove item if depleted
            if should_remove_item {
                if let Ok(mut inventory) = world.get::<&mut Inventory>(entity) {
                    inventory.remove_item(item);
                }
                let _ = world.despawn(item);
                resources.log.add("The item is consumed.");
            }
        } else {
            resources.log.add("This item cannot be used.");
        }

        // Remove intent
        let _ = world.remove_one::<WantsToUseItem>(entity);
    }
}

/// Get all items at a position
pub fn get_items_at_position(world: &World, x: i32, y: i32) -> Vec<hecs::Entity> {
    let mut items = Vec::new();

    for (entity, (pos, _, _)) in world.query::<(&Position, &Item, &OnGround)>().iter() {
        if pos.x == x && pos.y == y {
            items.push(entity);
        }
    }

    items
}

/// Get total weight of items in inventory
pub fn get_inventory_weight(world: &World, inventory: &Inventory) -> i32 {
    let mut total_weight = 0;

    for &item in &inventory.items {
        if let Ok(item_data) = world.get::<&ItemData>(item) {
            total_weight += item_data.weight;
        }
    }

    // Add equipped items
    for &item in inventory.equipped.values() {
        if let Ok(item_data) = world.get::<&ItemData>(item) {
            total_weight += item_data.weight;
        }
    }

    total_weight
}

/// Find first unequipped item in inventory that can be equipped in the given slot
pub fn find_equipable_for_slot(
    world: &World,
    inventory: &Inventory,
    slot: crate::ecs::EquipSlot,
) -> Option<hecs::Entity> {
    for &item in &inventory.items {
        // Check if already equipped
        if inventory.equipped.values().any(|&e| e == item) {
            continue;
        }

        // Check if item can be equipped in this slot
        if let Ok(equipable) = world.get::<&crate::ecs::Equipable>(item) {
            if equipable.slot == slot {
                return Some(item);
            }
        }
    }
    None
}
