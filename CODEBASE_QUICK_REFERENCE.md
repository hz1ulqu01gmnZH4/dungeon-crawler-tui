# Codebase Quick Reference
**Generated**: 2025-10-18
**Project**: Dungeon Crawler TUI - Rust Roguelike
**Version**: Phase 1 Complete + Phase 1.5 Tests

---

## 📊 Project Metrics

| Metric | Value |
|--------|-------|
| **Lines of Code** | 8,921 |
| **Rust Files** | 49 |
| **Total Tests** | 125 (100% passing) |
| **Structs** | 63 |
| **Functions** | 380+ |
| **Test Execution** | 20ms |

---

## 🗂️ Module Organization

```
dungeon-clawler-tui/
├── src/
│   ├── ecs/                   # Entity Component System
│   │   ├── components.rs      # 24 components (Position, CombatStats, Inventory, etc.)
│   │   └── resources.rs       # Global game resources
│   ├── systems/               # Game logic systems
│   │   ├── combat.rs          # Combat & death (9 tests) ✅
│   │   ├── movement.rs        # Movement & collision (9 tests) ✅
│   │   ├── input.rs           # Input handling (18 functions)
│   │   ├── inventory.rs       # Item management
│   │   ├── item_spawner.rs    # Loot generation
│   │   ├── ai.rs              # Monster AI
│   │   └── fov.rs             # Field of view
│   ├── map/                   # Map & tiles
│   │   ├── mod.rs             # Map struct
│   │   ├── tile.rs            # Tile enum (Floor, Wall, Doors, Stairs)
│   │   ├── generator.rs       # Dungeon generation
│   │   ├── chunks.rs          # Chunk system (10 tests)
│   │   └── fov.rs             # FOV algorithm
│   ├── world/                 # Overworld generation
│   │   ├── overmap.rs         # Overworld map (6 tests)
│   │   ├── generator.rs       # World generation
│   │   ├── settlement.rs      # Towns & villages (4 tests)
│   │   ├── building.rs        # Building generation (16 tests)
│   │   ├── poi.rs             # Points of interest (5 tests)
│   │   ├── roads.rs           # Road network (9 tests)
│   │   ├── time.rs            # Day/night cycle (10 tests)
│   │   ├── weather.rs         # Weather system (6 tests)
│   │   └── travel_events.rs   # Random encounters (5 tests)
│   ├── ui/                    # User interface
│   │   ├── renderer.rs        # Main renderer
│   │   ├── main_menu.rs       # Main menu (Continue/New/Quit)
│   │   ├── inventory_renderer.rs  # Inventory UI
│   │   ├── character_screen.rs    # Character stats (@)
│   │   ├── examine_renderer.rs    # Examine mode (x)
│   │   ├── overmap_renderer.rs    # Overworld UI (3 tests)
│   │   └── minimap.rs         # Minimap (5 tests)
│   ├── save.rs                # Save/Load system (9 tests) ✅
│   ├── game/                  # Core game loop
│   │   ├── app.rs             # Main application
│   │   └── state.rs           # Game state
│   ├── perf.rs                # Performance metrics (8 tests)
│   └── main.rs                # Entry point
```

---

## 🎯 Key Systems

### 1. **ECS Architecture** (hecs)
- **24 Components**: Position, CombatStats, Inventory, Item, Monster, Player, etc.
- **Pattern**: Intent components (WantsToMove, WantsToMelee) → System execution → Intent cleanup
- **Resources**: Global state (maps, camera, RNG, logs, etc.)

### 2. **Multi-Level Dungeons**
- **Surface**: Depth 0 (overworld)
- **Dungeons**: Depth 1+ (HashMap storage)
- **On-demand generation**: Levels created when first visited
- **Stairs**: `<` to ascend, `>` to descend
- **Persistence**: All levels saved with SaveGame v2

### 3. **Combat System**
```rust
AttackResult {
    Miss,
    Hit(damage),      // Normal hit with ±25% variance
    Critical(damage), // 2x damage (10% chance)
}
```
- Base hit chance: 90% (50-95% range)
- Damage: (power - defense).max(1)
- Player death → Game Over (not despawned)
- Monster death → Despawn + 40% loot drop

### 4. **Movement & Collision**
- Reality Layers: Normal & Cosmic (no collision between layers)
- Wall blocking: !walkable() tiles block movement
- Entity blocking: BlocksMovement component → converts to melee attack
- Doors: Closed doors block, open doors allow passage
- Viewshed: Marked dirty on movement

### 5. **Save/Load System** (Version 2)
```rust
SaveGame {
    version: 2,
    overmap: Overmap,
    settlements: Vec<Settlement>,
    dungeon_levels: HashMap<i32, Map>,  // NEW in v2
    current_depth: i32,                  // NEW in v2
    entities: Vec<EntityData>,
    message_log: Vec<String>,
    seed: u64,
}
```
- **Format**: JSON (serde)
- **Location**: `savegame.json`
- **Keybindings**:
  - `S` = Quick save
  - `L` = Quick load
- **Main menu**: Continue option (loads save)

### 6. **Inventory System**
```rust
Inventory {
    items: Vec<Entity>,
    capacity: usize,
    equipped: HashMap<EquipSlot, Entity>,
}
```
- **Slots**: MainHand, OffHand, Head, Torso, Legs
- **Weight**: Stackable items (potions, food)
- **Keybindings**:
  - `g` = Pickup
  - `i` = Open inventory
  - `w` = Quick wield weapon
  - `a` = Quick wear armor
  - `t` = Quick take off

---

## 🧪 Test Coverage

| Module | Tests | Coverage |
|--------|-------|----------|
| **Combat** | 9 | ✅ Damage, death, multiple attacks, logging |
| **Movement** | 9 | ✅ Collision, walls, entities, doors, layers |
| **Save/Load** | 9 | ✅ Entities, dungeons, time, file I/O |
| **World Gen** | 72 | ✅ Settlements, roads, buildings, POIs, time, weather |
| **Map** | 10 | ✅ Chunks, generation |
| **UI** | 8 | ✅ Minimap, overmap |
| **Performance** | 8 | ✅ Metrics tracking |
| **TOTAL** | **125** | **100% pass rate** |

### Test Gaps (Future Work)
- ❌ Inventory system (pickup/drop/equip)
- ❌ AI/pathfinding
- ❌ FOV calculation
- ❌ Input handling

---

## 🎮 Keybindings Reference

### Core Movement
- `hjkl` or Arrow keys: Move
- `y u b n`: Diagonal movement
- `.` or Space: Wait/rest

### Combat
- Move into enemy: Attack

### Dungeon Navigation
- `<`: Ascend stairs (go up)
- `>`: Descend stairs (go down)

### Overworld
- `m`: Toggle overworld map
- `Enter`: Enter settlement/location
- `Tab`: Leave settlement (return to overworld)

### Inventory
- `g`: Pickup items
- `i`: Inventory menu
- `w`: Quick wield weapon
- `a`: Quick wear armor
- `t`: Quick take off equipment

### UI Modes
- `@`: Character screen
- `x`: Examine mode (look around)
- `ESC`: Close UI / Return to game

### Doors
- `o`: Open door
- `c`: Close door

### Save/Load
- `S`: Quick save
- `L`: Quick load

### System
- `q`: Quit to main menu
- `Enter` (in menu): Confirm selection
- `↑/↓` or `k/j` (in menu): Navigate

---

## 📦 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| **hecs** | 0.10.5 | Entity Component System |
| **ratatui** | 0.28.1 | Terminal UI framework |
| **crossterm** | 0.28.1 | Terminal control |
| **rand** | 0.8.5 | Random number generation |
| **rand_pcg** | 0.3.1 | PCG RNG algorithm |
| **serde** | 1.0.219 | Serialization framework |
| **serde_json** | 1.0.143 | JSON save format |
| **noise** | 0.9.0 | Perlin noise (terrain) |
| **anyhow** | 1.0.99 | Error handling |
| **thiserror** | 1.0.69 | Error derive macros |

---

## 🔧 Development Tools

### Code Analysis
```bash
# View module structure
cargo modules generate tree

# Code statistics
tokei src

# Generate documentation
cargo doc --document-private-items --no-deps --open

# Dependency tree
cargo tree
```

### Testing
```bash
# Run all tests
cargo test --lib

# Run specific test
cargo test test_movement_basic_success

# Run with output
cargo test -- --nocapture

# Test timing
cargo test --release
```

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run --release
```

---

## 🏗️ Architecture Patterns

### 1. **Intent Pattern** (Combat & Movement)
```rust
// Player input → Add intent component
world.insert_one(player, WantsToMove { dest_x, dest_y });

// System processes intent
movement_system(&mut world, &mut resources);

// Intent cleaned up automatically
assert!(world.get::<&WantsToMove>(player).is_err());
```

### 2. **Map Selection**
```rust
let map = if resources.current_depth > 0 {
    resources.dungeon_levels.get(&resources.current_depth)
} else if let Some(location_id) = resources.current_location {
    resources.settlement_maps.get(&location_id)
} else {
    Some(resources.maps.active_map())
};
```

### 3. **Deterministic Testing**
```rust
use rand::rngs::StdRng;
use rand::SeedableRng;

let mut rng = StdRng::seed_from_u64(12345);
// Reproducible random behavior for testing
```

### 4. **Modal UI States**
```rust
if resources.in_main_menu {
    render_main_menu(frame, resources.menu_selection, save_exists);
    return;
}
if resources.in_inventory_mode {
    render_inventory(frame, world, resources);
}
if resources.in_examine_mode {
    render_examine_cursor(frame, world, resources);
}
```

---

## 📋 Phase Completion Status

### ✅ Phase 1 (COMPLETE)
1. ✅ Basic player movement (8-way + diagonals)
2. ✅ Dungeon generation (rooms + corridors)
3. ✅ FOV/visibility system
4. ✅ Monster spawning
5. ✅ Basic combat (attack, death)
6. ✅ Overworld map system
7. ✅ Settlement generation
8. ✅ Overworld movement with time progression
9. ✅ Settlement transitions
10. ✅ Inventory system (pickup/drop/equip)
11. ✅ Item spawning & loot
12. ✅ Character screen (@)
13. ✅ Examine mode (x)
14. ✅ Doors (open/close)
15. ✅ Save/Load system
16. ✅ Main menu (Continue/New/Quit)
17. ✅ Multi-level dungeons with stairs

### ✅ Phase 1.5 (COMPLETE)
- ✅ Combat system tests (9 tests)
- ✅ Movement system tests (9 tests)
- ✅ Save/Load system tests (7 new tests)
- ✅ Total: 125 tests (100% passing)

### 📋 Phase 2 (PLANNED)
- Merchants & trading
- Quest system
- Crafting
- Magic/spells
- More monster types
- More item types
- Dungeon types (caves, crypts, etc.)

---

## 🎨 UI Layout

```
┌─────────────────────────────────────────────────────────┐
│  Dungeon - Depth 1 [Normal Layer]                       │
│  ######################                                  │
│  #....................#                                  │
│  #.....@..............#                                  │
│  #..........g.........#                                  │
│  #....................#                                  │
│  ######################                                  │
└─────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────┐
│ Stats │ HP: 25/30 ████████░░ │ Pow: 5 │ Def: 2         │
└─────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────┐
│ Log                                                      │
│ > You hit Goblin for 3 damage                           │
│ > Goblin attacks you but misses!                        │
│ > You descend the stairs                                │
└─────────────────────────────────────────────────────────┘
```

---

## 🔍 Finding Things in the Code

### "Where is the player movement code?"
- **Input**: `src/systems/input.rs` → `handle_movement()`
- **Movement logic**: `src/systems/movement.rs` → `movement_system()`

### "How does combat work?"
- **File**: `src/systems/combat.rs`
- **Functions**: `calculate_attack()`, `melee_combat_system()`, `death_system()`

### "Where are maps generated?"
- **Dungeons**: `src/map/generator.rs` → `generate_dungeon_level()`
- **Overworld**: `src/world/generator.rs` → `generate_overmap()`
- **Settlements**: `src/world/settlement_gen.rs`

### "How do stairs work?"
- **Input**: `src/systems/input.rs` → `try_use_stairs()`
- **Depth tracking**: `resources.current_depth`
- **Map storage**: `resources.dungeon_levels: HashMap<i32, Map>`

### "Where is save/load?"
- **File**: `src/save.rs`
- **Functions**: `SaveGame::from_game()`, `SaveGame::restore_game()`
- **Quick functions**: `quick_save()`, `quick_load()`

### "How does the inventory work?"
- **Component**: `src/ecs/components.rs` → `Inventory` struct
- **Systems**: `src/systems/inventory.rs` → pickup/drop/equip functions
- **UI**: `src/ui/inventory_renderer.rs`

### "Where is the main menu?"
- **UI**: `src/ui/main_menu.rs` → `render_main_menu()`
- **Input**: `src/systems/input.rs` → `handle_main_menu_input()`
- **State**: `resources.in_main_menu`, `resources.menu_selection`

---

## 📚 Additional Documentation

- **PHASE_1_COMPLETION.md** - Phase 1 feature completion report
- **PHASE_1.5_COMPLETION.md** - Test coverage expansion report
- **TEST_COVERAGE_REPORT.md** - Detailed test analysis
- **STAIRS_IMPLEMENTATION.md** - Multi-level dungeon documentation
- **STATIC_ANALYSIS_TOOLS.md** - Code analysis tools guide
- **CODE_STRUCTURE_ANALYSIS.md** - Generated structure report
- **COMPLETE_CODE_MAP.md** - Function-level code map
- **DETAILED_API_REFERENCE.md** - API documentation

---

## 🚀 Quick Start for Developers

1. **Clone and build**
   ```bash
   cargo build --release
   ```

2. **Run tests**
   ```bash
   cargo test --lib
   ```

3. **Generate docs**
   ```bash
   cargo doc --document-private-items --no-deps --open
   ```

4. **Run the game**
   ```bash
   cargo run --release
   ```

5. **View code structure**
   ```bash
   cargo install cargo-modules tokei
   cargo modules generate tree
   tokei src
   ```

---

## 🎯 Code Conventions

- **Component naming**: Nouns (Position, CombatStats, Item)
- **Intent components**: "WantsTo" prefix (WantsToMove, WantsToMelee)
- **Systems**: Verb suffix (movement_system, combat_system)
- **Tests**: "test_" prefix, descriptive names (test_movement_wall_collision)
- **Resource access**: `resources.field` (not `self.field`)
- **Error handling**: `anyhow::Result<T>` for recoverable errors
- **RNG**: Seeded `StdRng` for tests, resources.rng for game

---

**Last Updated**: 2025-10-18
**Status**: Phase 1 + 1.5 Complete ✅
**Next Phase**: Phase 2 Development
