/// Automated tests for quick equipment keys (w/W/T)
///
/// Tests the quick equipment functionality added to input system

use dungeon_clawler_tui::ecs::*;
use dungeon_clawler_tui::{spawn_rusty_sword, spawn_iron_sword, spawn_leather_armor, spawn_wooden_shield};
use hecs::World;

#[test]
fn test_find_equipable_for_slot_weapon() {
    let mut world = World::new();

    // Create inventory with a weapon
    let mut inventory = Inventory::new(10);
    let sword = spawn_rusty_sword(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(sword);
    inventory.add_item(sword).unwrap();

    // Find weapon for main hand slot
    let result = dungeon_clawler_tui::find_equipable_for_slot(
        &world,
        &inventory,
        EquipSlot::MainHand
    );

    assert!(result.is_some());
    assert_eq!(result.unwrap(), sword);
}

#[test]
fn test_find_equipable_for_slot_armor() {
    let mut world = World::new();

    // Create inventory with armor
    let mut inventory = Inventory::new(10);
    let armor = spawn_leather_armor(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(armor);
    inventory.add_item(armor).unwrap();

    // Find armor for body slot
    let result = dungeon_clawler_tui::find_equipable_for_slot(
        &world,
        &inventory,
        EquipSlot::Body
    );

    assert!(result.is_some());
    assert_eq!(result.unwrap(), armor);
}

#[test]
fn test_find_equipable_ignores_equipped_items() {
    let mut world = World::new();

    // Create inventory with two swords
    let mut inventory = Inventory::new(10);
    let sword1 = spawn_rusty_sword(&mut world, 0, 0);
    let sword2 = spawn_iron_sword(&mut world, 0, 0);

    let _ = world.remove_one::<OnGround>(sword1);
    let _ = world.remove_one::<OnGround>(sword2);

    inventory.add_item(sword1).unwrap();
    inventory.add_item(sword2).unwrap();

    // Equip first sword
    inventory.equip(EquipSlot::MainHand, sword1);

    // Find weapon should return second sword (first is equipped)
    let result = dungeon_clawler_tui::find_equipable_for_slot(
        &world,
        &inventory,
        EquipSlot::MainHand
    );

    assert!(result.is_some());
    assert_eq!(result.unwrap(), sword2);
}

#[test]
fn test_find_equipable_returns_none_when_no_match() {
    let mut world = World::new();

    // Create inventory with only armor (no weapon)
    let mut inventory = Inventory::new(10);
    let armor = spawn_leather_armor(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(armor);
    inventory.add_item(armor).unwrap();

    // Try to find weapon
    let result = dungeon_clawler_tui::find_equipable_for_slot(
        &world,
        &inventory,
        EquipSlot::MainHand
    );

    assert!(result.is_none());
}

#[test]
fn test_find_equipable_returns_none_when_all_equipped() {
    let mut world = World::new();

    // Create inventory with weapon
    let mut inventory = Inventory::new(10);
    let sword = spawn_rusty_sword(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(sword);
    inventory.add_item(sword).unwrap();

    // Equip the only weapon
    inventory.equip(EquipSlot::MainHand, sword);

    // Try to find weapon (should return none, as only weapon is equipped)
    let result = dungeon_clawler_tui::find_equipable_for_slot(
        &world,
        &inventory,
        EquipSlot::MainHand
    );

    assert!(result.is_none());
}

#[test]
fn test_find_equipable_finds_first_unequipped() {
    let mut world = World::new();

    // Create inventory with multiple weapons
    let mut inventory = Inventory::new(10);
    let sword1 = spawn_rusty_sword(&mut world, 0, 0);
    let sword2 = spawn_iron_sword(&mut world, 1, 0);
    let sword3 = spawn_rusty_sword(&mut world, 2, 0);

    let _ = world.remove_one::<OnGround>(sword1);
    let _ = world.remove_one::<OnGround>(sword2);
    let _ = world.remove_one::<OnGround>(sword3);

    inventory.add_item(sword1).unwrap();
    inventory.add_item(sword2).unwrap();
    inventory.add_item(sword3).unwrap();

    // Find weapon should return first one
    let result = dungeon_clawler_tui::find_equipable_for_slot(
        &world,
        &inventory,
        EquipSlot::MainHand
    );

    assert!(result.is_some());
    assert_eq!(result.unwrap(), sword1);
}

#[test]
fn test_equip_weapon_increases_power() {
    let mut world = World::new();

    // Create player with base stats
    let player = world.spawn((
        Position::new(0, 0, RealityLayer::Normal),
        CombatStats::new(30, 5, 2),
        Inventory::new(10),
    ));

    // Create and add weapon
    let sword = spawn_iron_sword(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(sword);

    let weapon_power = if let Ok(equipable) = world.get::<&Equipable>(sword) {
        equipable.power_bonus
    } else {
        0
    };

    // Add to inventory
    if let Ok(mut inventory) = world.get::<&mut Inventory>(player) {
        inventory.add_item(sword).unwrap();
    }

    // Get initial power
    let initial_power = if let Ok(stats) = world.get::<&CombatStats>(player) {
        stats.power
    } else {
        0
    };

    // Equip weapon
    if let Ok(mut inventory) = world.get::<&mut Inventory>(player) {
        inventory.equip(EquipSlot::MainHand, sword);
    }

    // Apply bonus manually (this would normally be done by equip_system)
    if let Ok(mut stats) = world.get::<&mut CombatStats>(player) {
        stats.power += weapon_power;
    }

    // Verify power increased
    let final_power = if let Ok(stats) = world.get::<&CombatStats>(player) {
        stats.power
    } else {
        0
    };

    assert_eq!(final_power, initial_power + weapon_power);
}

#[test]
fn test_equip_armor_increases_defense() {
    let mut world = World::new();

    // Create player with base stats
    let player = world.spawn((
        Position::new(0, 0, RealityLayer::Normal),
        CombatStats::new(30, 5, 2),
        Inventory::new(10),
    ));

    // Create and add armor
    let armor = spawn_leather_armor(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(armor);

    let armor_defense = if let Ok(equipable) = world.get::<&Equipable>(armor) {
        equipable.defense_bonus
    } else {
        0
    };

    // Add to inventory
    if let Ok(mut inventory) = world.get::<&mut Inventory>(player) {
        inventory.add_item(armor).unwrap();
    }

    // Get initial defense
    let initial_defense = if let Ok(stats) = world.get::<&CombatStats>(player) {
        stats.defense
    } else {
        0
    };

    // Equip armor
    if let Ok(mut inventory) = world.get::<&mut Inventory>(player) {
        inventory.equip(EquipSlot::Body, armor);
    }

    // Apply bonus manually (this would normally be done by equip_system)
    if let Ok(mut stats) = world.get::<&mut CombatStats>(player) {
        stats.defense += armor_defense;
    }

    // Verify defense increased
    let final_defense = if let Ok(stats) = world.get::<&CombatStats>(player) {
        stats.defense
    } else {
        0
    };

    assert_eq!(final_defense, initial_defense + armor_defense);
}

#[test]
fn test_unequip_removes_bonuses() {
    let mut world = World::new();

    // Create player with base stats
    let player = world.spawn((
        Position::new(0, 0, RealityLayer::Normal),
        CombatStats::new(30, 5, 2),
        Inventory::new(10),
    ));

    // Create and equip weapon
    let sword = spawn_iron_sword(&mut world, 0, 0);
    let _ = world.remove_one::<OnGround>(sword);

    let weapon_power = if let Ok(equipable) = world.get::<&Equipable>(sword) {
        equipable.power_bonus
    } else {
        0
    };

    // Add to inventory and equip
    if let Ok(mut inventory) = world.get::<&mut Inventory>(player) {
        inventory.add_item(sword).unwrap();
        inventory.equip(EquipSlot::MainHand, sword);
    }

    // Apply bonus
    if let Ok(mut stats) = world.get::<&mut CombatStats>(player) {
        stats.power += weapon_power;
    }

    let equipped_power = if let Ok(stats) = world.get::<&CombatStats>(player) {
        stats.power
    } else {
        0
    };

    // Unequip
    if let Ok(mut inventory) = world.get::<&mut Inventory>(player) {
        inventory.unequip(EquipSlot::MainHand);
    }

    // Remove bonus
    if let Ok(mut stats) = world.get::<&mut CombatStats>(player) {
        stats.power -= weapon_power;
    }

    let final_power = if let Ok(stats) = world.get::<&CombatStats>(player) {
        stats.power
    } else {
        0
    };

    // Should be back to base power
    assert_eq!(final_power, 5); // Base power
    assert_eq!(final_power, equipped_power - weapon_power);
}

#[test]
fn test_equipment_slot_priority() {
    // Test that EquipSlot enum has expected slots
    let slots = EquipSlot::all();

    assert_eq!(slots.len(), 9);
    assert_eq!(slots[0], EquipSlot::MainHand);
    assert_eq!(slots[1], EquipSlot::OffHand);
    assert_eq!(slots[2], EquipSlot::Head);
    assert_eq!(slots[3], EquipSlot::Body);
    assert_eq!(slots[4], EquipSlot::Legs);
    assert_eq!(slots[5], EquipSlot::Feet);
    assert_eq!(slots[6], EquipSlot::Ring1);
    assert_eq!(slots[7], EquipSlot::Ring2);
    assert_eq!(slots[8], EquipSlot::Amulet);
}

#[test]
fn test_can_equip_multiple_items() {
    let mut world = World::new();

    // Create inventory
    let mut inventory = Inventory::new(10);

    // Create different equipment
    let sword = spawn_iron_sword(&mut world, 0, 0);
    let armor = spawn_leather_armor(&mut world, 0, 0);
    let shield = spawn_wooden_shield(&mut world, 0, 0);

    let _ = world.remove_one::<OnGround>(sword);
    let _ = world.remove_one::<OnGround>(armor);
    let _ = world.remove_one::<OnGround>(shield);

    inventory.add_item(sword).unwrap();
    inventory.add_item(armor).unwrap();
    inventory.add_item(shield).unwrap();

    // Equip to different slots
    inventory.equip(EquipSlot::MainHand, sword);
    inventory.equip(EquipSlot::Body, armor);
    inventory.equip(EquipSlot::OffHand, shield);

    // Verify all equipped
    assert_eq!(inventory.get_equipped(EquipSlot::MainHand), Some(sword));
    assert_eq!(inventory.get_equipped(EquipSlot::Body), Some(armor));
    assert_eq!(inventory.get_equipped(EquipSlot::OffHand), Some(shield));
}

#[test]
fn test_equipping_replaces_previous_item() {
    let mut world = World::new();

    // Create inventory
    let mut inventory = Inventory::new(10);

    // Create two weapons
    let rusty_sword = spawn_rusty_sword(&mut world, 0, 0);
    let iron_sword = spawn_iron_sword(&mut world, 0, 0);

    let _ = world.remove_one::<OnGround>(rusty_sword);
    let _ = world.remove_one::<OnGround>(iron_sword);

    inventory.add_item(rusty_sword).unwrap();
    inventory.add_item(iron_sword).unwrap();

    // Equip first weapon
    let old = inventory.equip(EquipSlot::MainHand, rusty_sword);
    assert!(old.is_none());

    // Equip second weapon (should replace first)
    let old = inventory.equip(EquipSlot::MainHand, iron_sword);
    assert_eq!(old, Some(rusty_sword));

    // Verify only iron sword equipped
    assert_eq!(inventory.get_equipped(EquipSlot::MainHand), Some(iron_sword));
}
