# Architecture Review - GPT-5 Analysis
**Date**: 2025-10-18
**Reviewer**: GPT-5 (High Reasoning)
**Project**: Dungeon Crawler TUI Roguelike
**Codebase**: 8,921 LOC, 49 files, 125 tests

---

## Executive Summary

**Overall Architecture Quality**: **7/10**

### Strengths ✅
- Coherent ECS core with hecs
- Clear separation of modules (map/world/ui/systems)
- Deterministic save/load system
- Healthy test count (125 tests, 100% passing)
- Surprisingly rich overworld for a terminal roguelike
- Good test coverage for world generation systems

### Critical Issues ⚠️
1. **Overgrown Resources struct** - 30+ fields creating implicit global state
2. **Monolithic input handler** - 900+ lines in single file
3. **Ad-hoc UI mode management** - Multiple boolean flags instead of state machine
4. **Manual intent cleanup** - Causes archetype churn and leak risk
5. **Manual serialization** - Won't scale as components proliferate
6. **Test gaps** - Core systems (inventory, AI, FOV) untested

---

## 🎯 High-Impact Refactors (Priority Order)

### 1. Replace Boolean UI Flags with State Machine
**Current Problem**:
```rust
// Resources has multiple boolean flags
in_main_menu: bool,
in_inventory_mode: bool,
in_character_screen: bool,
in_examine_mode: bool,
in_overmap_mode: bool,
```

**Recommended Solution**:
```rust
enum UiMode {
    MainMenu,
    InGame(GameSubmode),
}

enum GameSubmode {
    Normal,
    Inventory,
    Character,
    Examine,
    Overmap,
}

// For nested modals, use a stack
struct UiState {
    mode_stack: Vec<UiMode>,
}

impl UiState {
    fn current(&self) -> &UiMode { self.mode_stack.last().unwrap() }
    fn push(&mut self, mode: UiMode) { self.mode_stack.push(mode); }
    fn pop(&mut self) -> Option<UiMode> { self.mode_stack.pop(); }
}
```

**Benefits**:
- Eliminates conflicting state bugs
- Makes rendering logic clear
- Simplifies input routing
- Enables modal nesting (e.g., Inventory → Confirm Drop)

**Split Input Handling by Mode**:
```
ui/input/
  ├── mod.rs          # Central routing
  ├── gameplay.rs     # Normal mode
  ├── inventory.rs    # Inventory UI
  ├── character.rs    # Character screen
  ├── examine.rs      # Examine mode
  ├── main_menu.rs    # Main menu
  └── keymap.rs       # Key → Action mapping
```

---

### 2. Event/Command Pattern + Schedule Stages
**Current Problem**:
- Intent components (WantsToMove, WantsToMelee) cause archetype churn
- Manual cleanup after every system is error-prone
- System ordering rules scattered across files

**Recommended Solution**:
```rust
// Event queue for transient intentions
struct Events<T> {
    queue: Vec<T>,
}

enum SimEvent {
    Move { entity: Entity, to: Point },
    Melee { attacker: Entity, target: Entity },
    Pickup { entity: Entity, item: Entity },
    OpenDoor { entity: Entity, pos: Point },
    Die { entity: Entity },
}

// Schedule with explicit stages
struct Schedule {
    stages: Vec<(&'static str, Vec<SystemFn>)>,
}

// Stages:
// 1. PreUpdate    - Input → Actions → Events
// 2. Simulation   - AI, movement, FOV, combat
// 3. PostSim      - Death/loot, status effects
// 4. Cleanup      - Sweep remaining events, apply CommandBuffer
// 5. Render       - UI update
```

**Benefits**:
- Reduces archetype churn dramatically
- Centralizes cleanup (one place, not scattered)
- Makes system dependencies explicit
- Easier to test individual systems
- Better performance (fewer structural changes)

**Migration Path**:
1. Keep existing intent components initially
2. Add Events<SimEvent> resource
3. Convert one system at a time to read events instead of components
4. Remove old intent components once all systems migrated

---

### 3. Decompose Resources Into Typed Resources
**Current Problem**:
```rust
pub struct Resources {
    // Simulation state
    pub world_time: WorldTime,
    pub rng: StdRng,
    pub weather: WeatherSystem,

    // World data
    pub overmap: Overmap,
    pub settlements: Vec<Settlement>,
    pub dungeon_levels: HashMap<i32, Map>,

    // UI state
    pub camera: Camera,
    pub in_inventory_mode: bool,
    pub menu_selection: MenuOption,

    // Player state
    pub player_entity: Option<Entity>,
    pub examine_cursor: (i32, i32),

    // ... 30+ total fields mixing concerns
}
```

**Recommended Solution**:
```rust
// Typed resource storage
struct ResourceMap {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

// Separate resource types
struct SimResources {
    world_time: WorldTime,
    rng: StdRng,
    weather: WeatherSystem,
    perf_metrics: PerformanceMetrics,
}

struct WorldResources {
    overmap: Overmap,
    map_cache: MapCache,
    current_location: Location,
}

struct UiResources {
    ui_mode: UiState,
    camera: Camera,
    message_log: MessageLog,
    keymap: Keymap,
}

struct PlayerResources {
    player_entity: Entity,
    target: Option<Entity>,
    inspect_cursor: Option<Point>,
}

// Systems request specific resources
fn movement_system(
    world: &mut World,
    sim: Res<SimResources>,
    maps: ResMut<WorldResources>,
    events: ResMut<Events<SimEvent>>,
) {
    // Only access what's needed
}
```

**Benefits**:
- Clear separation of concerns
- Systems advertise dependencies explicitly
- Easier to test (inject only needed resources)
- Prevents accidental coupling
- Better encapsulation

---

### 4. Unified Map Storage with MapId
**Current Problem**:
```rust
// Multiple storage locations for different map types
dungeon_levels: HashMap<i32, Map>,        // Dungeons by depth
settlement_maps: HashMap<usize, Map>,     // Settlements by ID
maps: MapSet,                             // Surface map

// Scattered selection logic
if current_depth > 0 {
    dungeon_levels.get(&current_depth)
} else if let Some(location) = current_location {
    settlement_maps.get(&location)
} else {
    surface_map
}
```

**Recommended Solution**:
```rust
// Unified map identifier
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum MapId {
    Overworld,
    Settlement(SettlementId),
    Dungeon(Depth),
}

// Unified storage with on-demand generation
struct MapCache {
    active: MapId,
    cache: LruCache<MapId, Map>,
    generator: MapGenerator,
}

impl MapCache {
    fn get_mut(&mut self, id: MapId) -> &mut Map {
        if !self.cache.contains(&id) {
            let map = self.generator.generate(id);
            self.cache.put(id, map);
        }
        self.cache.get_mut(&id).unwrap()
    }

    fn current(&mut self) -> &mut Map {
        self.get_mut(self.active)
    }
}

// Location tracking
struct Location {
    map_id: MapId,
    pos: Point,
}
```

**Benefits**:
- Single source of truth for map storage
- LRU cache prevents memory bloat
- On-demand generation is explicit
- Easy to serialize active maps only
- Cleaner APIs for systems

**Storage Optimization**:
```rust
// For sequential dungeon depths, use Vec instead of HashMap
struct DungeonCache {
    levels: Vec<Option<Map>>,  // Index = depth
}

// For sparse IDs (settlements), use SlotMap
use slotmap::{SlotMap, DefaultKey as SettlementId};
struct SettlementCache {
    maps: SlotMap<SettlementId, Map>,
}
```

---

### 5. Component Serialization Registry
**Current Problem**:
```rust
// Manual extraction of every component in EntityData
struct EntityData {
    position: Option<Position>,
    renderable: Option<Renderable>,
    name: Option<String>,
    player: Option<Player>,
    monster: Option<Monster>,
    combat_stats: Option<CombatStats>,
    // ... must update for every new component
}

// Manual reconstruction on load
if let Some(pos) = entity_data.position {
    builder.add(pos);
}
if let Some(rend) = entity_data.renderable {
    builder.add(rend);
}
// ... repeated for all components
```

**Recommended Solution**:
```rust
// Stable UID for entity references
#[derive(Serialize, Deserialize, Copy, Clone)]
struct Uid(u64);

// Component registry
trait SerializableComponent: Serialize + DeserializeOwned + 'static {
    fn type_name() -> &'static str;
}

struct ComponentRegistry {
    serializers: HashMap<TypeId, Box<dyn ComponentSerializer>>,
}

// Serialize entire world
fn serialize_world(world: &World, registry: &ComponentRegistry) -> WorldData {
    let mut entities = Vec::new();

    for entity_ref in world.iter() {
        let entity = entity_ref.entity();
        let uid = *world.get::<&Uid>(entity).unwrap();
        let components = registry.serialize_entity(world, entity);
        entities.push((uid, components));
    }

    WorldData { entities }
}

// Deserialize with UID remapping
fn deserialize_world(data: WorldData, registry: &ComponentRegistry) -> (World, HashMap<Uid, Entity>) {
    let mut world = World::new();
    let mut uid_map = HashMap::new();

    for (uid, components) in data.entities {
        let entity = registry.deserialize_entity(&mut world, components);
        uid_map.insert(uid, entity);
    }

    // Fix up entity references using uid_map
    fix_entity_references(&mut world, &uid_map);

    (world, uid_map)
}
```

**Benefits**:
- Add components without touching save code
- Stable entity references across sessions
- Version migrations per component
- Reduced boilerplate

**Alternative**: Use `hecs::serialize` row format if it fits your needs.

---

### 6. Close Test Gaps
**Priority Order**:
1. **FOV** (highest risk - affects gameplay)
2. **Inventory** (affects save/load)
3. **AI/Pathfinding** (prevents soft locks)
4. **Input** (prevents UX regressions)

**Recommended Tests**:

```rust
// FOV - Snapshot tests
#[test]
fn test_fov_empty_room() {
    let map = create_empty_room(10, 10);
    let visible = compute_fov(&map, Point::new(5, 5), 8);
    insta::assert_debug_snapshot!(visible);
}

// FOV - Property tests
#[quickcheck]
fn fov_is_symmetric(origin: Point, radius: u8) -> bool {
    // Visibility should be symmetric under rotation/reflection
}

// Inventory - Round-trip serialization
#[test]
fn test_inventory_serialization() {
    let mut world = World::new();
    let player = world.spawn((
        Inventory::new(20),
        Position::new(0, 0, RealityLayer::Normal),
    ));

    // Add items to inventory
    let sword = world.spawn((Item, ItemData::weapon()));
    world.get::<&mut Inventory>(player).unwrap().add(sword);

    // Save and load
    let save = SaveGame::from_game(&world, &resources, seed);
    let (mut new_world, _) = save.restore();

    // Verify inventory preserved
    let inv = new_world.query::<&Inventory>().iter().next().unwrap().1;
    assert_eq!(inv.items.len(), 1);
}

// AI - Path finding correctness
#[test]
fn test_monster_reaches_player() {
    // Place monster and player, run AI for N turns
    // Assert monster gets closer or finds valid path
}

// Input - Action mapping
#[test]
fn test_keymap_normal_mode() {
    let keymap = Keymap::default();
    assert_eq!(
        keymap.translate(UiMode::InGame(Normal), KeyCode::Char('h')),
        Some(Action::Move(Direction::West))
    );
}
```

**Property-based tests for generators**:
```rust
#[quickcheck]
fn dungeon_all_rooms_connected(seed: u64) -> bool {
    let mut rng = StdRng::seed_from_u64(seed);
    let map = generate_dungeon_level(50, 50, &mut rng);

    // Find stairs up and down
    let stairs_up = find_tile(&map, Tile::StairsUp);
    let stairs_down = find_tile(&map, Tile::StairsDown);

    // Assert path exists between them
    pathfind(&map, stairs_up, stairs_down).is_some()
}
```

---

## 🔍 Detailed Analysis by Area

### Architecture & Systems

#### Scheduling and Data Flow
**Current Issues**:
- Intent components cause archetype churn
- Manual cleanup is error-prone and scattered
- Ordering rules implicit

**Recommendations**:
- Add explicit schedule with stages (PreUpdate, Simulation, PostSim, Cleanup, Render)
- Replace intent components with event queues (Events<T>)
- Use `hecs::CommandBuffer` for entity/component mutations
- Apply buffered changes at end of each stage

**Example Schedule**:
```rust
struct GameSchedule {
    pre_update: Vec<SystemFn>,
    simulation: Vec<SystemFn>,
    post_simulation: Vec<SystemFn>,
    cleanup: Vec<SystemFn>,
}

impl GameSchedule {
    fn tick(&mut self, world: &mut World, resources: &mut Resources) {
        for system in &self.pre_update { system(world, resources); }
        for system in &self.simulation { system(world, resources); }
        for system in &self.post_simulation { system(world, resources); }
        for system in &self.cleanup { system(world, resources); }
    }
}
```

#### Determinism and Ordering
**Issue**: Query iteration order is not stable; combat with simultaneous actors can vary.

**Solutions**:
- Add Initiative or TurnOrder component
- Sort entity IDs deterministically when order matters
- Split RNG by system or derive substreams (seed + system_id)

```rust
// Deterministic combat order
let mut combatants: Vec<_> = world.query::<(&Initiative, &CombatStats)>()
    .iter()
    .collect();
combatants.sort_by_key(|(entity, (init, _))| (Reverse(init.value), *entity));

for (entity, (_, stats)) in combatants {
    // Process combat in deterministic order
}
```

---

### Map & Location Model

**Current Issues**:
- Special-casing for depth and current_location
- Multiple HashMaps for different map types
- Scattered selection logic

**Recommendations**:
- Define `MapId` enum unifying all map types
- Implement `MapService` with LRU cache
- Use `Vec<Option<Map>>` for sequential dungeon depths
- Use `SlotMap` for sparse settlement IDs

**Example**:
```rust
struct MapService {
    active: MapId,
    overworld: Map,
    dungeons: Vec<Option<Map>>,
    settlements: SlotMap<SettlementId, Map>,
    generator: MapGenerator,
}

impl MapService {
    fn get_mut(&mut self, id: MapId) -> &mut Map {
        match id {
            MapId::Overworld => &mut self.overworld,
            MapId::Dungeon(depth) => {
                let d = depth.0 as usize;
                if d >= self.dungeons.len() {
                    self.dungeons.resize_with(d + 1, || None);
                }
                self.dungeons[d].get_or_insert_with(|| {
                    self.generator.generate_dungeon(depth)
                })
            }
            MapId::Settlement(id) => {
                if !self.settlements.contains_key(id) {
                    let map = self.generator.generate_settlement(id);
                    self.settlements.insert(id, map);
                }
                &mut self.settlements[id]
            }
        }
    }
}
```

---

### Combat, AI, FOV

#### Combat
**Status**: Reasonably featured
**Recommendations**:
- Keep deterministic by using separate RNG stream
- Consider extraction to sub-modules if it grows:
  - `damage.rs` - Damage calculation
  - `death.rs` - Death system
  - `loot.rs` - Drop system

#### AI
**Current**: 1 function only
**Priority**: HIGH - Extract behavior patterns

**Recommended Architecture**:
```rust
#[derive(Clone)]
enum Behavior {
    Idle,
    Chase { target: Entity },
    Flee { from: Entity },
    Patrol { waypoints: Vec<Point> },
    Guard { post: Point, radius: u8 },
}

struct BehaviorState {
    current: Behavior,
    blackboard: HashMap<String, Value>,  // Shared state
}

fn ai_system(world: &mut World, resources: &mut Resources) {
    for (entity, (behavior, pos, stats)) in world.query::<(&mut BehaviorState, &Position, &CombatStats)>().iter() {
        match &mut behavior.current {
            Behavior::Idle => {
                // Check for player in range
                if player_visible(world, entity, pos) {
                    behavior.current = Behavior::Chase { target: player };
                }
            }
            Behavior::Chase { target } => {
                // Move toward target
                let target_pos = world.get::<&Position>(*target).ok()?;
                let path = pathfind(map, pos, target_pos);
                // ...
            }
            // ...
        }
    }
}
```

#### FOV
**Recommendation**: Centralize as system with change detection

```rust
struct VisibleTiles {
    tiles: HashSet<Point>,
    dirty: bool,
}

fn fov_system(world: &mut World, resources: &mut Resources) {
    // Only recompute if something changed
    let needs_update = resources.fov_dirty_flag;

    if !needs_update { return; }

    for (entity, (pos, viewshed)) in world.query::<(&Position, &mut Viewshed)>().iter() {
        if viewshed.dirty {
            viewshed.visible = compute_fov(map, pos, viewshed.radius);
            viewshed.dirty = false;
        }
    }

    resources.fov_dirty_flag = false;
}

// Mark dirty on events that affect visibility
fn movement_system(...) {
    // ... move entity
    resources.fov_dirty_flag = true;
}

fn open_door_system(...) {
    // ... open door
    resources.fov_dirty_flag = true;
}
```

---

### Save/Load and Versioning

**Current Issues**:
- Manual EntityData that must be updated for every component
- No stable entity references (Inventory holds raw Entity)
- No per-component versioning

**Recommendations**:

1. **Add Stable UIDs**:
```rust
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
struct Uid(u64);

static NEXT_UID: AtomicU64 = AtomicU64::new(1);

fn spawn_with_uid(world: &mut World, bundle: impl DynamicBundle) -> (Entity, Uid) {
    let uid = Uid(NEXT_UID.fetch_add(1, Ordering::Relaxed));
    let entity = world.spawn((uid, bundle));
    (entity, uid)
}
```

2. **Component Registry**:
```rust
trait ComponentSerializer: Send + Sync {
    fn serialize(&self, world: &World, entity: Entity) -> Option<Box<dyn erased_serde::Serialize>>;
    fn deserialize(&self, world: &mut World, entity: Entity, data: &mut dyn erased_serde::Deserializer);
}

struct ComponentRegistry {
    serializers: HashMap<&'static str, Box<dyn ComponentSerializer>>,
}

// Register components at startup
fn register_components() -> ComponentRegistry {
    let mut registry = ComponentRegistry::new();
    registry.register::<Position>("Position");
    registry.register::<CombatStats>("CombatStats");
    registry.register::<Inventory>("Inventory");
    // ...
    registry
}
```

3. **Migration System**:
```rust
struct SaveGame {
    version: u32,
    component_versions: HashMap<String, u32>,
    world_data: WorldData,
}

// Per-component migrations
fn migrate_combat_stats_v1_to_v2(data: &mut serde_json::Value) {
    // Add new field with default
    data["dodge"] = json!(0);
}

// Test migrations with golden files
#[test]
fn test_migration_v1_to_v2() {
    let v1_save = include_str!("../tests/saves/v1_sample.json");
    let save = SaveGame::load_from_str(v1_save).unwrap();
    assert_eq!(save.version, 2);
    // Verify migration succeeded
}
```

4. **Compression**:
```rust
use zstd::stream::{encode_all, decode_all};

pub fn save_to_file(&self, path: &Path) -> Result<()> {
    let json = serde_json::to_vec(self)?;
    let compressed = encode_all(&json[..], 3)?;  // Level 3 compression
    fs::write(path, compressed)?;
    Ok(())
}

pub fn load_from_file(path: &Path) -> Result<Self> {
    let compressed = fs::read(path)?;
    let json = decode_all(&compressed[..])?;
    let save: SaveGame = serde_json::from_slice(&json)?;
    Ok(save)
}
```

---

### Performance Considerations

**Likely Bottlenecks**:

1. **Structural Churn from Intents**
   - **Impact**: HIGH
   - **Fix**: Replace with Events + CommandBuffer
   - **Expected improvement**: 2-3x faster turn processing

2. **FOV Recomputation**
   - **Impact**: MEDIUM
   - **Fix**: Recompute only on change; cache by origin + radius
   - **Expected improvement**: 10x faster when static

3. **Query Setup in Hot Loops**
   - **Impact**: LOW-MEDIUM
   - **Fix**: Reuse QueryBorrow across frames
   ```rust
   // Bad
   for _ in 0..100 {
       for (e, pos) in world.query::<&Position>().iter() { /* ... */ }
   }

   // Good
   let mut query = world.query::<&Position>();
   for _ in 0..100 {
       for (e, pos) in query.iter() { /* ... */ }
   }
   ```

4. **Pathfinding**
   - **Impact**: MEDIUM (will be HIGH when AI expands)
   - **Fix**:
     - Use goal maps/flow fields for "move toward player"
     - Cache paths with TTL
     - Recompute only on topology change
     - Consider hierarchical navigation for large maps

5. **Map Memory**
   - **Impact**: LOW (currently), MEDIUM (Phase 2+)
   - **Fix**: LRU cache with serialization of inactive levels

**Profiling Recommendations**:
```bash
# Add criterion benchmarks
cargo install cargo-criterion
cargo criterion

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bin dungeon-clawler-tui

# Add performance budgets to CI
# Example: 95th percentile FOV < 1ms on 50x50 map
```

---

### Maintainability & Code Organization

**File Splitting Recommendations**:

```
src/
├── systems/
│   ├── input/
│   │   ├── mod.rs
│   │   ├── gameplay.rs
│   │   ├── inventory.rs
│   │   ├── character.rs
│   │   ├── examine.rs
│   │   ├── main_menu.rs
│   │   └── keymap.rs
│   ├── combat/
│   │   ├── mod.rs
│   │   ├── damage.rs
│   │   ├── death.rs
│   │   └── loot.rs
│   └── ...
├── map/
│   ├── generator/
│   │   ├── mod.rs
│   │   ├── rooms.rs
│   │   ├── corridors.rs
│   │   └── decorators.rs
│   └── ...
└── sim/
    ├── mod.rs           # Schedule definition
    ├── events.rs        # Event types
    └── stages.rs        # Stage implementations
```

**Introduce Newtypes**:
```rust
// Instead of raw primitives
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct HitPoints(u16);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Power(i16);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Defense(i16);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Depth(i32);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Money(u32);

// Benefits: type safety, prevents unit mistakes, clearer intent
```

**Structured Logging**:
```rust
enum LogCategory {
    Combat,
    Movement,
    Inventory,
    World,
    System,
}

enum LogSeverity {
    Debug,
    Info,
    Warning,
    Error,
}

struct LogEntry {
    category: LogCategory,
    severity: LogSeverity,
    message: String,
    color: Color,
    timestamp: WorldTime,
}

struct MessageLog {
    entries: VecDeque<LogEntry>,
    capacity: usize,
}

// Usage
log.add(LogEntry {
    category: LogCategory::Combat,
    severity: LogSeverity::Info,
    message: format!("{} hits {} for {} damage", attacker, target, dmg),
    color: Color::Red,
    timestamp: world_time.clone(),
});
```

---

### Best Practice Violations

**Current Issues**:
1. ❌ Boolean UI flags instead of state machine
2. ❌ Global Resources mixing simulation, UI, and IO
3. ❌ Manual transient component cleanup
4. ❌ HashMap<i32, Map> for sequential depths (use Vec)
5. ❌ Manual entity serialization without stable UIDs
6. ❌ Potential nondeterminism from unspecified iteration order
7. ❌ Public fields everywhere (should encapsulate)

**Recommended Fixes**:
1. ✅ UiMode enum with stack for nested modals
2. ✅ Split Resources into typed resources
3. ✅ Event queues + CommandBuffer + Cleanup stage
4. ✅ Vec<Option<Map>> for dungeons, SlotMap for settlements
5. ✅ Uid component + ComponentRegistry
6. ✅ Sort by Initiative/TurnOrder or entity ID
7. ✅ Add accessor methods for Resources fields

---

### Testing Strategy Improvements

**Gap Closure Priority**:
1. FOV (snapshot + property tests)
2. Inventory (serialization + weight + equip)
3. AI/Pathfinding (correctness + soft lock prevention)
4. Input (action mapping per mode)

**Property-Based Testing**:
```bash
cargo add --dev proptest quickcheck quickcheck_macros
```

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn dungeon_stairs_reachable(seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        let map = generate_dungeon_level(50, 50, &mut rng);

        let up = find_tile(&map, Tile::StairsUp);
        let down = find_tile(&map, Tile::StairsDown);

        prop_assert!(pathfind(&map, up, down).is_some());
    }

    #[test]
    fn no_buildings_overlap(seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        let settlement = generate_settlement(30, 30, &mut rng);

        for (i, b1) in settlement.buildings.iter().enumerate() {
            for b2 in settlement.buildings.iter().skip(i + 1) {
                prop_assert!(!b1.intersects(b2));
            }
        }
    }
}
```

**Fuzzing**:
```bash
cargo install cargo-fuzz
cargo fuzz init

# Create fuzz target for save/load
# fuzz/fuzz_targets/save_load.rs
```

**Determinism Tests**:
```rust
#[test]
fn test_deterministic_combat() {
    // Same seed should produce same outcomes
    let outcomes1 = run_simulation(12345, 100);
    let outcomes2 = run_simulation(12345, 100);
    assert_eq!(outcomes1.log, outcomes2.log);
    assert_eq!(outcomes1.player_hp, outcomes2.player_hp);
}
```

**Snapshot Testing**:
```bash
cargo add --dev insta
```

```rust
#[test]
fn test_fov_snapshot() {
    let map = create_test_map();
    let visible = compute_fov(&map, Point::new(5, 5), 8);
    insta::assert_debug_snapshot!(visible);
}
```

---

### Scalability for Phase 2 Features

**Merchants & Economy**:
```rust
struct Vendor {
    inventory: Vec<(ItemId, u32)>,  // (item, quantity)
    pricing_rules: PricingStrategy,
    faction: FactionId,
}

struct Wallet {
    gold: Money,
}

struct Price(Money);
struct Quality(u8);  // 0-100

// Trade events
enum TradeEvent {
    Offer { buyer: Entity, seller: Entity, item: ItemId, price: Money },
    Accept { trade_id: TradeId },
    Decline { trade_id: TradeId },
    Barter { /* ... */ },
}
```

**Quests**:
```rust
struct QuestLog {
    active: Vec<Quest>,
    completed: Vec<QuestId>,
}

struct Quest {
    id: QuestId,
    title: String,
    objectives: Vec<Objective>,
    rewards: Vec<Reward>,
    state: QuestState,
}

enum Objective {
    Kill { target: MonsterType, count: u32, current: u32 },
    Acquire { item: ItemId, count: u32, current: u32 },
    Visit { location: MapId },
    Talk { npc: Uid },
}

// Event-driven progression
fn quest_system(world: &World, events: &Events<GameEvent>, quests: &mut QuestLog) {
    for event in &events.queue {
        match event {
            GameEvent::EntityKilled(uid) => {
                for quest in &mut quests.active {
                    quest.update_kill_objective(uid);
                }
            }
            GameEvent::ItemAcquired(item) => { /* ... */ }
            GameEvent::LocationVisited(loc) => { /* ... */ }
        }
    }
}
```

**Crafting**:
```rust
struct Recipe {
    id: RecipeId,
    inputs: Vec<(ItemId, u32)>,
    output: (ItemId, u32),
    skill_required: Option<(Skill, u8)>,
    tools_required: Vec<ItemTag>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum ItemTag {
    Weapon,
    Armor,
    Tool,
    Ingredient,
    Craftable,
}

struct CraftingStation {
    recipes: Vec<RecipeId>,
    bonuses: HashMap<ItemTag, f32>,
}

fn crafting_system(
    world: &mut World,
    recipes: &Recipes,
    events: &mut Events<CraftEvent>,
) {
    for CraftEvent::Attempt { crafter, recipe_id, station } in &events.queue {
        if let Some(recipe) = recipes.get(recipe_id) {
            if has_ingredients(world, crafter, &recipe.inputs) {
                consume_ingredients(world, crafter, &recipe.inputs);
                create_item(world, crafter, &recipe.output);
                events.queue.push(CraftEvent::Success { crafter, item });
            }
        }
    }
}
```

**Magic & Effects**:
```rust
struct Effect {
    kind: EffectKind,
    magnitude: i32,
    duration: Duration,
    source: Uid,
    stacks: u8,
    dispellable: bool,
}

enum EffectKind {
    Damage { element: Element },
    Heal,
    Buff { stat: Stat, amount: i32 },
    Debuff { stat: Stat, amount: i32 },
    StatusEffect { status: StatusKind },
}

enum Duration {
    Instant,
    Turns(u32),
    Permanent,
}

fn effect_system(world: &mut World, time: &WorldTime) {
    for (entity, effects) in world.query::<&mut Effects>().iter() {
        effects.active.retain_mut(|effect| {
            // Apply effect
            apply_effect(world, entity, effect);

            // Tick duration
            effect.duration.tick();

            // Remove if expired
            !effect.duration.is_expired()
        });
    }
}
```

---

## 📋 Implementation Roadmap

### Phase 0: Foundation (Week 1-2)
- [ ] Add UiMode enum and refactor boolean flags
- [ ] Split input.rs into modules by mode
- [ ] Add Action enum and Keymap
- [ ] Create basic Schedule structure
- [ ] Add Events<T> resource type
- [ ] Implement deterministic RNG per system

### Phase 1: Resource Refactor (Week 2-3)
- [ ] Create typed resource categories (Sim, World, UI, Player)
- [ ] Add ResourceMap with typed get/get_mut
- [ ] Migrate systems to use typed resources
- [ ] Remove god object fields from Resources

### Phase 2: Map Unification (Week 3-4)
- [ ] Add MapId enum
- [ ] Implement MapCache/MapService
- [ ] Convert dungeon_levels to Vec<Option<Map>>
- [ ] Unify map selection logic
- [ ] Add LRU cache for memory management

### Phase 3: Event/Command Pattern (Week 4-5)
- [ ] Define SimEvent enum
- [ ] Add Events<SimEvent> resource
- [ ] Migrate movement system to events
- [ ] Migrate combat system to events
- [ ] Add Cleanup stage to schedule
- [ ] Remove old intent components

### Phase 4: Serialization Upgrade (Week 5-6)
- [ ] Add Uid component
- [ ] Implement ComponentRegistry
- [ ] Add stable entity reference system
- [ ] Implement component versioning
- [ ] Add migration tests
- [ ] Add compression (zstd)

### Phase 5: Test Coverage (Week 6-7)
- [ ] Add FOV tests (snapshot + property)
- [ ] Add Inventory tests (serialization + logic)
- [ ] Add AI/pathfinding tests
- [ ] Add input mapping tests
- [ ] Add property-based dungeon tests
- [ ] Set up fuzzing targets
- [ ] Add benchmark suite (criterion)

### Phase 6: Performance (Week 7-8)
- [ ] Profile with flamegraph
- [ ] Optimize hot paths
- [ ] Add query reuse where beneficial
- [ ] Implement FOV caching
- [ ] Add performance regression tests

---

## 🎯 Quick Wins (Start Here)

These changes provide immediate value with minimal risk:

### 1. Add UiMode Enum (2-4 hours)
```rust
// Easy, high impact, low risk
enum UiMode { MainMenu, InGame(GameSubmode) }
enum GameSubmode { Normal, Inventory, Character, Examine, Overmap }
```

### 2. Extract Newtypes (1-2 hours)
```rust
// Simple refactor, prevents future bugs
struct Depth(i32);
struct HitPoints(u16);
struct Money(u32);
```

### 3. Add Structured Logging (2-3 hours)
```rust
// Improves UX and debugging
struct LogEntry {
    category: LogCategory,
    severity: LogSeverity,
    message: String,
    color: Color,
}
```

### 4. Split input.rs by Mode (4-6 hours)
```rust
// Improves maintainability immediately
ui/input/
  ├── gameplay.rs
  ├── inventory.rs
  ├── character.rs
  └── examine.rs
```

### 5. Add FOV Tests (2-3 hours)
```rust
// Closes critical test gap
#[test]
fn test_fov_empty_room() { /* ... */ }
```

---

## 📊 Summary Table

| Area | Current Score | Target Score | Effort | Priority |
|------|---------------|--------------|--------|----------|
| UI State Management | 4/10 | 9/10 | Medium | HIGH |
| Resource Architecture | 5/10 | 9/10 | High | HIGH |
| Event System | 3/10 | 9/10 | High | HIGH |
| Map Storage | 6/10 | 9/10 | Medium | MEDIUM |
| Serialization | 5/10 | 8/10 | High | MEDIUM |
| Test Coverage | 6/10 | 9/10 | High | HIGH |
| Performance | 7/10 | 8/10 | Medium | LOW |
| Code Organization | 6/10 | 9/10 | Medium | MEDIUM |

**Overall**: 7/10 → Target: 9/10

---

## 🔗 Additional Resources

- **hecs Documentation**: https://docs.rs/hecs/
- **ECS Patterns**: https://github.com/SanderMertens/ecs-faq
- **Rust Game Dev Patterns**: https://arewegameyet.rs/
- **Property Testing**: https://github.com/proptest-rs/proptest
- **Benchmark Suite**: https://github.com/bheisler/criterion.rs

---

**Generated**: 2025-10-18
**Reviewer**: GPT-5 High Reasoning
**Next Review**: After Phase 0-1 refactors (recommend 4-6 weeks)
