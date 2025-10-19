/// Integration tests for item stacking system
///
/// Tests the stacking behavior of consumable items

use dungeon_clawler_tui::ecs::*;
use dungeon_clawler_tui::spawn_healing_potion;
use hecs::World;

#[test]
fn test_stackable_component_creation() {
    let mut world = World::new();

    // Create a healing potion
    let potion = spawn_healing_potion(&mut world, 5, 5);

    // Verify it has Stackable component
    assert!(world.get::<&Stackable>(potion).is_ok());

    // Verify initial quantity is 1
    let stackable = world.get::<&Stackable>(potion).unwrap();
    assert_eq!(stackable.quantity, 1);
}

#[test]
fn test_item_data_has_max_stack() {
    let mut world = World::new();

    // Create a healing potion
    let potion = spawn_healing_potion(&mut world, 5, 5);

    // Verify ItemData has max_stack
    let item_data = world.get::<&ItemData>(potion).unwrap();
    assert_eq!(item_data.max_stack, 10);
    assert_eq!(item_data.category, ItemCategory::Consumable);
}

#[test]
fn test_stackable_quantity_increase() {
    let mut world = World::new();

    // Create a healing potion
    let potion = spawn_healing_potion(&mut world, 5, 5);

    // Increase stack quantity
    {
        let mut stackable = world.get::<&mut Stackable>(potion).unwrap();
        stackable.quantity += 5;
    }

    // Verify new quantity
    let stackable = world.get::<&Stackable>(potion).unwrap();
    assert_eq!(stackable.quantity, 6);
}

#[test]
fn test_stackable_quantity_decrease() {
    let mut world = World::new();

    // Create a healing potion with stack of 5
    let potion = spawn_healing_potion(&mut world, 5, 5);
    {
        let mut stackable = world.get::<&mut Stackable>(potion).unwrap();
        stackable.quantity = 5;
    }

    // Use one item (decrease stack)
    {
        let mut stackable = world.get::<&mut Stackable>(potion).unwrap();
        stackable.quantity = stackable.quantity.saturating_sub(1);
    }

    // Verify quantity decreased
    let stackable = world.get::<&Stackable>(potion).unwrap();
    assert_eq!(stackable.quantity, 4);
}

#[test]
fn test_detailed_description_exists() {
    let mut world = World::new();

    // Create a healing potion
    let potion = spawn_healing_potion(&mut world, 5, 5);

    // Verify detailed description
    let item_data = world.get::<&ItemData>(potion).unwrap();
    assert!(!item_data.detailed_description.is_empty());
    assert!(item_data.detailed_description.contains("crimson liquid"));
    assert!(item_data.detailed_description.contains("restorative magic"));
}

#[test]
fn test_item_category_system() {
    let mut world = World::new();

    // Create different item types
    let potion = spawn_healing_potion(&mut world, 5, 5);

    // Verify category
    let item_data = world.get::<&ItemData>(potion).unwrap();
    assert_eq!(item_data.category, ItemCategory::Consumable);
    assert_eq!(item_data.category.name(), "Consumable");
}

#[test]
fn test_inventory_can_hold_stacked_item() {
    let mut world = World::new();

    // Create inventory
    let mut inventory = Inventory::new(10);

    // Create stacked potion
    let potion = spawn_healing_potion(&mut world, 5, 5);
    {
        let mut stackable = world.get::<&mut Stackable>(potion).unwrap();
        stackable.quantity = 5;
    }

    // Add to inventory
    let result = inventory.add_item(potion);
    assert!(result.is_ok());

    // Verify inventory count
    assert_eq!(inventory.items.len(), 1);
    assert!(inventory.has_item(potion));
}

#[test]
fn test_max_stack_limit() {
    let mut world = World::new();

    // Create potion at max stack
    let potion = spawn_healing_potion(&mut world, 5, 5);

    let item_data = world.get::<&ItemData>(potion).unwrap();
    let max_stack = item_data.max_stack;

    {
        let mut stackable = world.get::<&mut Stackable>(potion).unwrap();
        stackable.quantity = max_stack;
    }

    // Verify at max
    let stackable = world.get::<&Stackable>(potion).unwrap();
    assert_eq!(stackable.quantity, max_stack);
    assert_eq!(stackable.quantity, 10); // Healing potion max is 10
}

#[test]
fn test_stack_depletion() {
    let mut world = World::new();

    // Create potion with quantity 1
    let potion = spawn_healing_potion(&mut world, 5, 5);

    // Reduce to 0
    {
        let mut stackable = world.get::<&mut Stackable>(potion).unwrap();
        stackable.quantity = stackable.quantity.saturating_sub(1);
    }

    // Verify depleted
    let stackable = world.get::<&Stackable>(potion).unwrap();
    assert_eq!(stackable.quantity, 0);
}

#[test]
fn test_non_stackable_items_have_max_stack_one() {
    let mut world = World::new();

    // Create non-stackable items (weapons, armor)
    use dungeon_clawler_tui::spawn_rusty_sword;
    let sword = spawn_rusty_sword(&mut world, 5, 5);

    // Verify max_stack is 1
    let item_data = world.get::<&ItemData>(sword).unwrap();
    assert_eq!(item_data.max_stack, 1);

    // Verify it's NOT stackable (no Stackable component)
    assert!(world.get::<&Stackable>(sword).is_err());
}
