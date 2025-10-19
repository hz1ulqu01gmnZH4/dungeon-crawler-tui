use crate::ecs::{
    Inventory, Equipable, Consumable, EquipSlot, GameEvent,
};
use crate::ecs::resources::{Resources, RunMode, UiMode};
use crossterm::event::KeyCode;
use hecs::World;

/// Handle inventory screen input
pub(super) fn handle_inventory_input(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    // Extract current selection
    let mut selection = if let UiMode::Inventory { selection } = resources.ui.ui_mode {
        selection
    } else {
        return true; // Shouldn't happen, but handle gracefully
    };

    match key_code {
        // Close inventory
        KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Esc => {
            resources.ui.ui_mode = UiMode::InGame;
        }
        // Navigate up
        KeyCode::Up | KeyCode::Char('k') => {
            if selection > 0 {
                selection -= 1;
                resources.ui.ui_mode = UiMode::Inventory { selection };
            }
        }
        // Navigate down
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(player_entity) = resources.player.player_entity {
                if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                    if selection < inventory.items.len().saturating_sub(1) {
                        selection += 1;
                        resources.ui.ui_mode = UiMode::Inventory { selection };
                    }
                }
            }
        }
        // Equip/Unequip item
        KeyCode::Char('e') | KeyCode::Char('E') => {
            if let Some(player_entity) = resources.player.player_entity {
                // Extract item and equipable data first
                let equip_action = {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if selection < inventory.items.len() {
                            let item = inventory.items[selection];

                            // Check if item is equipable
                            if let Ok(equipable) = world.get::<&Equipable>(item) {
                                let slot = equipable.slot;
                                let is_equipped = inventory.get_equipped(slot) == Some(item);
                                Some((item, slot, is_equipped))
                            } else {
                                Some((item, EquipSlot::MainHand, false)) // Dummy - will be rejected
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                // Now perform action
                if let Some((item, slot, is_equipped)) = equip_action {
                    // Verify item is equipable
                    if world.get::<&Equipable>(item).is_ok() {
                        if is_equipped {
                            // Unequip
                            resources.events.send(GameEvent::UnequipItem { entity: player_entity, slot });
                        } else {
                            // Equip
                            resources.events.send(GameEvent::EquipItem { entity: player_entity, item });
                        }

                        resources.sim.mode = RunMode::PlayerTurn;
                        resources.ui.ui_mode = UiMode::InGame;
                    } else {
                        resources.ui.log.add("This item cannot be equipped.");
                    }
                }
            }
        }
        // Drop item
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if let Some(player_entity) = resources.player.player_entity {
                // Extract item first
                let item_to_drop = {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if selection < inventory.items.len() {
                            Some(inventory.items[selection])
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(item) = item_to_drop {
                    resources.events.send(GameEvent::DropItem { entity: player_entity, item });

                    resources.sim.mode = RunMode::PlayerTurn;
                    resources.ui.ui_mode = UiMode::InGame;
                }
            }
        }
        // Use item
        KeyCode::Char('u') | KeyCode::Char('U') => {
            if let Some(player_entity) = resources.player.player_entity {
                // Extract item first
                let item_to_use = {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if selection < inventory.items.len() {
                            let item = inventory.items[selection];
                            // Check if item is consumable
                            if world.get::<&Consumable>(item).is_ok() {
                                Some(item)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(item) = item_to_use {
                    resources.events.send(GameEvent::UseItem { entity: player_entity, item });

                    resources.sim.mode = RunMode::PlayerTurn;
                    resources.ui.ui_mode = UiMode::InGame;
                } else if let Some(player_entity) = resources.player.player_entity {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if selection < inventory.items.len() {
                            resources.ui.log.add("This item cannot be used.");
                        }
                    }
                }
            }
        }
        KeyCode::Char('w') => {
            // Wield weapon (quick equip to main hand) in inventory mode
            super::equipment::quick_wield_weapon(world, resources);
            resources.ui.ui_mode = UiMode::InGame;
        }
        KeyCode::Char('W') => {
            // Wear armor (quick equip to body) in inventory mode
            super::equipment::quick_wear_armor(world, resources);
            resources.ui.ui_mode = UiMode::InGame;
        }
        KeyCode::Char('T') => {
            // Take off (unequip) in inventory mode
            super::equipment::quick_take_off(world, resources);
            resources.ui.ui_mode = UiMode::InGame;
        }
        _ => {}
    }

    true
}
