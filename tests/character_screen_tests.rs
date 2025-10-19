/// Automated tests for character screen (@)
///
/// Tests the character screen display and resource flags

use dungeon_clawler_tui::ecs::*;
use dungeon_clawler_tui::ecs::resources::Resources;
use dungeon_clawler_tui::{spawn_iron_sword, spawn_leather_armor};
use hecs::World;

#[test]
fn test_character_screen_flag_initializes_false() {
    use dungeon_clawler_tui::ecs::resources::UiMode;

    let resources = Resources::new(80, 50, 12345);
    assert!(matches!(resources.ui_mode, UiMode::MainMenu { .. }));
}

#[test]
fn test_combat_stats_display_values() {
    let stats = CombatStats::new(30, 5, 2);

    assert_eq!(stats.hp.value(), 30);
    assert_eq!(stats.max_hp.value(), 30);
    assert_eq!(stats.power.value(), 5);
    assert_eq!(stats.defense.value(), 2);
}

#[test]
fn test_trimeter_initializes_correctly() {
    let trimeter = TriMeter::new();

    assert_eq!(trimeter.insight, 0);
    assert_eq!(trimeter.sanity, 100);
    assert_eq!(trimeter.notice, 0);
}

#[test]
fn test_all_equipment_slots_available() {
    let slots = EquipSlot::all();

    assert_eq!(slots.len(), 9);

    // Verify each slot has a name
    for slot in slots {
        assert!(!slot.name().is_empty());
    }
}

#[test]
fn test_equipment_slot_names() {
    assert_eq!(EquipSlot::MainHand.name(), "Main Hand");
    assert_eq!(EquipSlot::OffHand.name(), "Off Hand");
    assert_eq!(EquipSlot::Head.name(), "Head");
    assert_eq!(EquipSlot::Body.name(), "Body");
    assert_eq!(EquipSlot::Legs.name(), "Legs");
    assert_eq!(EquipSlot::Feet.name(), "Feet");
    assert_eq!(EquipSlot::Ring1.name(), "Ring 1");
    assert_eq!(EquipSlot::Ring2.name(), "Ring 2");
    assert_eq!(EquipSlot::Amulet.name(), "Amulet");
}

#[test]
fn test_player_with_full_stats() {
    let mut world = World::new();

    let player = world.spawn((
        Position::new(10, 10, RealityLayer::Normal),
        CombatStats::new(30, 5, 2),
        TriMeter::new(),
        Inventory::default_player(),
        Name("Test Player".to_string()),
    ));

    // Verify all components exist
    assert!(world.get::<&Position>(player).is_ok());
    assert!(world.get::<&CombatStats>(player).is_ok());
    assert!(world.get::<&TriMeter>(player).is_ok());
    assert!(world.get::<&Inventory>(player).is_ok());
    assert!(world.get::<&Name>(player).is_ok());
}

#[test]
fn test_equipment_bonuses_calculation() {
    let mut world = World::new();

    // Create player
    let player = world.spawn((
        CombatStats::new(30, 5, 2),
        Inventory::new(10),
    ));

    // Create and equip items
    let sword = spawn_iron_sword(&mut world, 0, 0);
    let armor = spawn_leather_armor(&mut world, 0, 0);

    let _ = world.remove_one::<OnGround>(sword);
    let _ = world.remove_one::<OnGround>(armor);

    // Get bonuses
    let sword_power = if let Ok(eq) = world.get::<&Equipable>(sword) {
        eq.power_bonus.value()
    } else {
        0
    };

    let armor_defense = if let Ok(eq) = world.get::<&Equipable>(armor) {
        eq.defense_bonus.value()
    } else {
        0
    };

    // Add to inventory and equip
    if let Ok(mut inv) = world.get::<&mut Inventory>(player) {
        inv.add_item(sword).unwrap();
        inv.add_item(armor).unwrap();
        inv.equip(EquipSlot::MainHand, sword);
        inv.equip(EquipSlot::Body, armor);
    }

    // Apply bonuses
    if let Ok(mut stats) = world.get::<&mut CombatStats>(player) {
        stats.power += dungeon_clawler_tui::Power::new(sword_power);
        stats.defense += dungeon_clawler_tui::Defense::new(armor_defense);
    }

    // Verify final stats
    let stats = world.get::<&CombatStats>(player).unwrap();
    assert_eq!(stats.power.value(), 5 + sword_power); // Base + weapon
    assert_eq!(stats.defense.value(), 2 + armor_defense); // Base + armor
}

#[test]
fn test_hp_percentage_calculation() {
    let stats = CombatStats::new(30, 5, 2);

    // Full HP
    let percent = (stats.hp.value() as f32 / stats.max_hp.value() as f32 * 100.0) as i32;
    assert_eq!(percent, 100);

    // Test at 50% HP
    let mut damaged_stats = CombatStats::new(30, 5, 2);
    damaged_stats.hp = dungeon_clawler_tui::HitPoints::new(15);
    let percent = (damaged_stats.hp.value() as f32 / damaged_stats.max_hp.value() as f32 * 100.0) as i32;
    assert_eq!(percent, 50);

    // Test at 25% HP
    let mut low_hp_stats = CombatStats::new(30, 5, 2);
    low_hp_stats.hp = dungeon_clawler_tui::HitPoints::new(7);
    let percent = (low_hp_stats.hp.value() as f32 / low_hp_stats.max_hp.value() as f32 * 100.0) as i32;
    assert_eq!(percent, 23);
}

#[test]
fn test_sanity_levels() {
    let mut trimeter = TriMeter::new();

    // High sanity (>70)
    assert!(trimeter.sanity > 70);

    // Medium sanity (30-70)
    trimeter.sanity = 50;
    assert!(trimeter.sanity >= 30 && trimeter.sanity <= 70);

    // Low sanity (<30)
    trimeter.sanity = 20;
    assert!(trimeter.sanity < 30);
}

#[test]
fn test_inventory_default_player_capacity() {
    let inventory = Inventory::default_player();

    assert_eq!(inventory.capacity, 26); // a-z
    assert_eq!(inventory.items.len(), 0);
    assert_eq!(inventory.equipped.len(), 0);
}

#[test]
fn test_equipped_items_retrievable() {
    let mut world = World::new();
    let mut inventory = Inventory::new(10);

    let sword = spawn_iron_sword(&mut world, 0, 0);
    let armor = spawn_leather_armor(&mut world, 0, 0);

    let _ = world.remove_one::<OnGround>(sword);
    let _ = world.remove_one::<OnGround>(armor);

    inventory.add_item(sword).unwrap();
    inventory.add_item(armor).unwrap();

    inventory.equip(EquipSlot::MainHand, sword);
    inventory.equip(EquipSlot::Body, armor);

    // Retrieve equipped items
    assert_eq!(inventory.get_equipped(EquipSlot::MainHand), Some(sword));
    assert_eq!(inventory.get_equipped(EquipSlot::Body), Some(armor));

    // Empty slots return None
    assert_eq!(inventory.get_equipped(EquipSlot::Head), None);
    assert_eq!(inventory.get_equipped(EquipSlot::Legs), None);
}

#[test]
fn test_equipment_in_inventory_values() {
    let mut world = World::new();

    let sword = spawn_iron_sword(&mut world, 0, 0);
    let equipable = world.get::<&Equipable>(sword).unwrap();

    // Iron sword should have power bonus, no defense
    assert!(equipable.power_bonus.value() > 0);
    assert_eq!(equipable.defense_bonus.value(), 0);
    assert_eq!(equipable.slot, EquipSlot::MainHand);
}

#[test]
fn test_armor_in_inventory_values() {
    let mut world = World::new();

    let armor = spawn_leather_armor(&mut world, 0, 0);
    let equipable = world.get::<&Equipable>(armor).unwrap();

    // Leather armor should have defense bonus, no power
    assert_eq!(equipable.power_bonus.value(), 0);
    assert!(equipable.defense_bonus.value() > 0);
    assert_eq!(equipable.slot, EquipSlot::Body);
}

#[test]
fn test_multiple_equipment_slots_occupied() {
    let mut world = World::new();
    let mut inventory = Inventory::new(10);

    // Create full equipment set
    let sword = spawn_iron_sword(&mut world, 0, 0);
    let armor = spawn_leather_armor(&mut world, 0, 0);

    let _ = world.remove_one::<OnGround>(sword);
    let _ = world.remove_one::<OnGround>(armor);

    inventory.add_item(sword).unwrap();
    inventory.add_item(armor).unwrap();

    // Equip multiple items
    inventory.equip(EquipSlot::MainHand, sword);
    inventory.equip(EquipSlot::Body, armor);

    // Count equipped slots
    let equipped_count = inventory.equipped.len();
    assert_eq!(equipped_count, 2);
}

#[test]
fn test_empty_equipment_slots_are_none() {
    let inventory = Inventory::new(10);

    // All slots should be empty initially
    for slot in EquipSlot::all() {
        assert_eq!(inventory.get_equipped(slot), None);
    }
}

#[test]
fn test_stats_damage_reduces_hp() {
    let mut stats = CombatStats::new(30, 5, 2);

    let initial_hp = stats.hp.value();

    // Take damage
    stats.hp = dungeon_clawler_tui::HitPoints::new(stats.hp.value() - 10);

    assert_eq!(stats.hp.value(), initial_hp - 10);
    assert_eq!(stats.hp.value(), 20);
    assert_eq!(stats.max_hp.value(), 30); // Max HP unchanged
}

#[test]
fn test_stats_healing_capped_at_max() {
    let mut stats = CombatStats::new(30, 5, 2);
    stats.hp = dungeon_clawler_tui::HitPoints::new(20);

    // Heal beyond max
    let new_hp = (stats.hp.value() + 20).min(stats.max_hp.value());
    stats.hp = dungeon_clawler_tui::HitPoints::new(new_hp);

    // Should be capped at max_hp
    assert_eq!(stats.hp.value(), 30);
}

#[test]
fn test_trimeter_values_can_change() {
    let mut trimeter = TriMeter::new();

    // Modify values
    trimeter.insight = 50;
    trimeter.sanity = 75;
    trimeter.notice = 25;

    assert_eq!(trimeter.insight, 50);
    assert_eq!(trimeter.sanity, 75);
    assert_eq!(trimeter.notice, 25);
}

#[test]
fn test_character_screen_data_completeness() {
    let mut world = World::new();

    // Create fully-equipped player
    let player = world.spawn((
        CombatStats::new(30, 5, 2),
        TriMeter::new(),
        Inventory::default_player(),
        Name("Hero".to_string()),
    ));

    // Verify can retrieve all data needed for character screen
    assert!(world.get::<&CombatStats>(player).is_ok());
    assert!(world.get::<&TriMeter>(player).is_ok());
    assert!(world.get::<&Inventory>(player).is_ok());
    assert!(world.get::<&Name>(player).is_ok());

    // Verify values
    let stats = world.get::<&CombatStats>(player).unwrap();
    assert_eq!(stats.hp.value(), 30);
    assert_eq!(stats.max_hp.value(), 30);

    let trimeter = world.get::<&TriMeter>(player).unwrap();
    assert_eq!(trimeter.sanity, 100);

    let inventory = world.get::<&Inventory>(player).unwrap();
    assert_eq!(inventory.capacity, 26);

    let name = world.get::<&Name>(player).unwrap();
    assert_eq!(name.0, "Hero");
}
