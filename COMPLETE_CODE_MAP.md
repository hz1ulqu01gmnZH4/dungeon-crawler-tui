# Complete Code Map - Dungeon Crawler TUI
**Generated**: Sat Oct 18 07:04:40 PM JST 2025

## Project Overview

- **Total Lines of Code**: 8,921
- **Total Rust Files**: 49
- **Total Tests**: 125
- **Test Coverage**: Combat ✅ | Movement ✅ | Save/Load ✅ | World Gen ✅

## Module Structure

### ECS (Entity Component System)

**Components** (src/ecs/components.rs):
- Position
- Renderable
- Viewshed
- CombatStats
- TriMeter
- Name(pub String);
- Player;
- Monster;
- BlocksMovement;
- WantsToMove
- WantsToMelee
- WantsToWait;
- Inventory
- Item;
- OnGround;
- ItemData
- Stackable
- Equipable
- Consumable
- WantsToPickupItem
- WantsToDropItem
- WantsToEquipItem
- WantsToUnequipItem
- WantsToUseItem

**Resources** (src/ecs/resources.rs):
- Camera
- GameLog
- Resources

### Systems

**ai** (`src/systems/ai.rs`):
- monster_ai_system

**combat** (`src/systems/combat.rs`):
- melee_combat_system
- death_system

**fov** (`src/systems/fov.rs`):
- update_fov

**input** (`src/systems/input.rs`):
- handle_input
- handle_key

**inventory** (`src/systems/inventory.rs`):
- pickup_system
- drop_system
- equip_system
- unequip_system
- use_item_system
- get_items_at_position
- get_inventory_weight
- find_equipable_for_slot

**item_spawner** (`src/systems/item_spawner.rs`):
- spawn_healing_potion
- spawn_rusty_sword
- spawn_iron_sword
- spawn_wooden_shield
- spawn_leather_armor
- spawn_random_item<R: Rng>
- spawn_items_in_room<R: Rng>
- spawn_random_weapon<R: Rng>
- spawn_random_armor<R: Rng>

**mod** (`src/systems/mod.rs`):

**movement** (`src/systems/movement.rs`):
- movement_system

### World Generation

- **building** (`src/world/building.rs`): 8 functions
- **generator** (`src/world/generator.rs`): 1 functions
- **mod** (`src/world/mod.rs`): 0 functions
- **overmap** (`src/world/overmap.rs`): 0 functions
- **placement** (`src/world/placement.rs`): 1 functions
- **poi** (`src/world/poi.rs`): 1 functions
- **roads** (`src/world/roads.rs`): 5 functions
- **settlement_gen** (`src/world/settlement_gen.rs`): 5 functions
- **settlement** (`src/world/settlement.rs`): 0 functions
- **terrain** (`src/world/terrain.rs`): 0 functions
- **time** (`src/world/time.rs`): 0 functions
- **travel_events** (`src/world/travel_events.rs`): 0 functions
- **weather** (`src/world/weather.rs`): 0 functions

### UI Modules

- **character_screen** (`src/ui/character_screen.rs`): 6 functions
- **examine_renderer** (`src/ui/examine_renderer.rs`): 2 functions
- **input** (`src/ui/input.rs`): 1 functions
- **inventory_renderer** (`src/ui/inventory_renderer.rs`): 7 functions
- **main_menu** (`src/ui/main_menu.rs`): 1 functions
- **minimap** (`src/ui/minimap.rs`): 1 functions
- **mod** (`src/ui/mod.rs`): 0 functions
- **overmap_renderer** (`src/ui/overmap_renderer.rs`): 0 functions
- **renderer** (`src/ui/renderer.rs`): 6 functions

## Key Functions by Module

### Input Handling (src/systems/input.rs)
- handle_input - input handler
- handle_key - input handler
- handle_movement - input handler
- try_move_player - input handler
- try_move_overmap - input handler
- try_enter_location - input handler
- try_rest - input handler
- try_pickup_items - input handler
- quick_wield_weapon - input handler
- quick_wear_armor - input handler
- quick_take_off - input handler
- handle_character_screen_input - input handler
- handle_examine_input - input handler
- handle_inventory_input - input handler
- try_open_door - input handler
- try_close_door - input handler
- try_use_stairs - input handler
- handle_main_menu_input - input handler

### Combat (src/systems/combat.rs)
- calculate_attack - combat logic
- melee_combat_system - combat logic
- death_system - combat logic

