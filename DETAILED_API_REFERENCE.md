# Detailed API Reference
**Generated**: Sat Oct 18 07:04:53 PM JST 2025

## Core Components (src/ecs/components.rs)

### Position
```rust
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub layer: RealityLayer,
}

```

### CombatStats
```rust
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub power: i32,
    pub defense: i32,
}

```

### Inventory
```rust
pub struct Inventory {
    pub items: Vec<hecs::Entity>,
    pub capacity: usize,
    pub equipped: std::collections::HashMap<EquipSlot, hecs::Entity>,
```

## Map System (src/map/tile.rs)

### Tile Enum
```rust
pub enum Tile {
    Floor,
    Wall,
    ClosedDoor,
    OpenDoor,
    StairsUp,
    StairsDown,
}

impl Tile {
    pub fn walkable(&self) -> bool {
```

## Save System (src/save.rs)

### SaveGame Structure
```rust
pub struct SaveGame {
    version: u32,

    // World state
    overmap: Overmap,
    settlements: Vec<Settlement>,
    world_time: WorldTime,

    // Player state
    player_overmap_pos: (i32, i32),
    current_location: Option<usize>,

    // Dungeon state
    dungeon_levels: HashMap<i32, Map>,
    current_depth: i32,

```

## Resources (src/ecs/resources.rs)

### Resources Fields
```rust
pub struct Resources {
    pub maps: MapSet,
    pub overmap: Overmap,
    pub settlements: Vec<Settlement>,     // All settlements in the world
    pub pois: Vec<POI>,                   // All points of interest (dungeons, caves, etc.)
    pub roads: Vec<Road>,                 // Road network connecting settlements
    pub settlement_maps: HashMap<usize, Map>,  // Generated settlement maps by ID
    pub dungeon_levels: HashMap<i32, Map>,     // Dungeon levels by depth (1 = first level down, etc.)
    pub current_depth: i32,               // Current dungeon depth (0 = surface, 1+ = dungeon)
    pub world_time: WorldTime,
    pub weather: WeatherSystem,           // Current weather conditions
    pub camera: Camera,
    pub rng: StdRng,
    pub mode: RunMode,
    pub log: GameLog,
    pub player_entity: Option<hecs::Entity>,
    pub player_overmap_pos: (i32, i32),  // Player position on overmap
    pub in_overmap_mode: bool,            // True when viewing/navigating overmap
    pub current_location: Option<usize>,  // Current settlement/location ID (None = wilderness)
    pub in_inventory_mode: bool,          // True when viewing inventory UI
    pub inventory_selection: usize,       // Selected item index in inventory
    pub in_character_screen: bool,        // True when viewing character screen (@)
    pub in_examine_mode: bool,            // True when examining tiles/objects (x)
    pub examine_cursor: (i32, i32),       // Position of examine cursor
    pub last_combat_target: Option<hecs::Entity>,  // Last entity player attacked (for UI)
    pub seed: u64,                        // World seed for save/load
    pub in_main_menu: bool,               // True when in main menu (startup)
    pub menu_selection: crate::ui::MenuOption,  // Currently selected menu option
}

impl Resources {
```

## Test Distribution

| Module | Tests | Status |
|--------|-------|--------|
| map::chunks | 10 | ✅ |
| perf | 8 | ✅ |
| save | 9 | ✅ |
| systems::combat | 9 | ✅ |
| systems::movement | 9 | ✅ |
| ui::minimap | 5 | ✅ |
| ui::overmap_renderer | 3 | ✅ |
| world::building | 16 | ✅ |
| world::generator | 3 | ✅ |
| world::overmap | 6 | ✅ |
| world::placement | 4 | ✅ |
| world::poi | 5 | ✅ |
| world::roads | 9 | ✅ |
| world::settlement_gen | 4 | ✅ |
| world::settlement | 4 | ✅ |
| world::time | 10 | ✅ |
| world::travel_events | 5 | ✅ |
| world::weather | 6 | ✅ |

