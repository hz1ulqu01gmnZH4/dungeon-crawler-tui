# Code Structure Analysis Report
**Generated**: Sat Oct 18 07:04:24 PM JST 2025

## Code Statistics

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Rust                   49        11159         8921          720         1518
 |- Markdown            27          197            0          192            5
 (Total)                          11356         8921          912         1523
===============================================================================
 Total                  49        11159         8921          720         1518
===============================================================================
```

## Structs by Module

- `src/components/position.rs`: 1 structs
- `src/components/stats.rs`: 1 structs
- `src/ecs/components.rs`: 24 structs
- `src/ecs/resources.rs`: 3 structs
- `src/game/app.rs`: 1 structs
- `src/game/state.rs`: 1 structs
- `src/game/world.rs`: 1 structs
- `src/generation/dungeon.rs`: 1 structs
- `src/map/chunks.rs`: 2 structs
- `src/map/generator.rs`: 1 structs
- `src/map/mod.rs`: 2 structs
- `src/perf.rs`: 3 structs
- `src/save.rs`: 2 structs
- `src/ui/minimap.rs`: 2 structs
- `src/ui/overmap_renderer.rs`: 1 structs
- `src/world/building.rs`: 1 structs
- `src/world/generator.rs`: 1 structs
- `src/world/overmap.rs`: 2 structs
- `src/world/placement.rs`: 1 structs
- `src/world/poi.rs`: 3 structs
- `src/world/roads.rs`: 2 structs
- `src/world/settlement.rs`: 2 structs
- `src/world/time.rs`: 2 structs
- `src/world/travel_events.rs`: 3 structs
- `src/world/weather.rs`: 1 structs

## Functions by Module

- `src/components/position.rs`: 3 functions
- `src/components/stats.rs`: 1 functions
- `src/ecs/components.rs`: 34 functions
- `src/ecs/resources.rs`: 6 functions
- `src/game/app.rs`: 8 functions
- `src/game/state.rs`: 2 functions
- `src/game/world.rs`: 1 functions
- `src/generation/dungeon.rs`: 2 functions
- `src/main.rs`: 3 functions
- `src/map/chunks.rs`: 25 functions
- `src/map/fov.rs`: 1 functions
- `src/map/generator.rs`: 8 functions
- `src/map/mod.rs`: 11 functions
- `src/map/tile.rs`: 6 functions
- `src/perf.rs`: 27 functions
- `src/save.rs`: 15 functions
- `src/systems/ai.rs`: 1 functions
- `src/systems/combat.rs`: 12 functions
- `src/systems/fov.rs`: 1 functions
- `src/systems/input.rs`: 18 functions
- `src/systems/inventory.rs`: 8 functions
- `src/systems/item_spawner.rs`: 10 functions
- `src/systems/movement.rs`: 11 functions
- `src/ui/character_screen.rs`: 6 functions
- `src/ui/examine_renderer.rs`: 2 functions
- `src/ui/input.rs`: 1 functions
- `src/ui/inventory_renderer.rs`: 7 functions
- `src/ui/main_menu.rs`: 5 functions
- `src/ui/minimap.rs`: 10 functions
- `src/ui/overmap_renderer.rs`: 10 functions
- `src/ui/renderer.rs`: 6 functions
- `src/world/building.rs`: 27 functions
- `src/world/generator.rs`: 10 functions
- `src/world/overmap.rs`: 18 functions
- `src/world/placement.rs`: 9 functions
- `src/world/poi.rs`: 17 functions
- `src/world/roads.rs`: 19 functions
- `src/world/settlement_gen.rs`: 9 functions
- `src/world/settlement.rs`: 10 functions
- `src/world/terrain.rs`: 4 functions
- `src/world/time.rs`: 28 functions
- `src/world/travel_events.rs`: 24 functions
- `src/world/weather.rs`: 18 functions

## Test Coverage by Module

- ✅ `src/map/chunks.rs`: 10 tests
- ✅ `src/perf.rs`: 8 tests
- ✅ `src/save.rs`: 9 tests
- ✅ `src/systems/combat.rs`: 9 tests
- ✅ `src/systems/movement.rs`: 9 tests
- ✅ `src/ui/minimap.rs`: 5 tests
- ✅ `src/ui/overmap_renderer.rs`: 3 tests
- ✅ `src/world/building.rs`: 16 tests
- ✅ `src/world/generator.rs`: 3 tests
- ✅ `src/world/overmap.rs`: 6 tests
- ✅ `src/world/placement.rs`: 4 tests
- ✅ `src/world/poi.rs`: 5 tests
- ✅ `src/world/roads.rs`: 9 tests
- ✅ `src/world/settlement_gen.rs`: 4 tests
- ✅ `src/world/settlement.rs`: 4 tests
- ✅ `src/world/time.rs`: 10 tests
- ✅ `src/world/travel_events.rs`: 5 tests
- ✅ `src/world/weather.rs`: 6 tests

## Module Organization

```
.
./components
./ecs
./game
./generation
./map
./systems
./ui
./world
```
