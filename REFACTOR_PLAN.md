# Refactoring Plan - Dungeon Crawler TUI
**Date**: 2025-10-18
**Current Score**: 7/10
**Target Score**: 9/10
**Estimated Timeline**: 9-15 weeks

---

## 🎯 Overview

The codebase is **solid** but has accumulated technical debt that will impact Phase 2 development. This plan addresses 6 critical areas through incremental refactoring.

**Philosophy**: Small, safe changes with tests passing at each step.

---

## 📊 Current State Assessment

### Strengths ✅
- Clean ECS architecture (hecs)
- Good module separation
- 125 tests (100% passing)
- Deterministic save/load
- Rich world generation

### Critical Issues ⚠️
| Issue | Impact | Priority |
|-------|--------|----------|
| Resources god object (30+ fields) | Coupling, hard to test | HIGH |
| Boolean UI flags | State bugs | HIGH |
| Monolithic input.rs (900+ lines) | Maintainability | HIGH |
| Manual intent cleanup | Performance (archetype churn) | HIGH |
| Test gaps (FOV, inventory, AI) | Regression risk | HIGH |
| Manual serialization | Phase 2 scalability | MEDIUM |

---

## 🚀 Phase 0: Quick Wins (Week 1-2)

**Goal**: Low-risk, high-impact improvements
**Score**: 7.0 → 7.5

### Task 0.1: Add UiMode Enum (4 hours)
**Why**: Eliminates conflicting UI state bugs

**Before**:
```rust
// src/ecs/resources.rs
pub struct Resources {
    pub in_main_menu: bool,
    pub in_inventory_mode: bool,
    pub in_character_screen: bool,
    pub in_examine_mode: bool,
    pub in_overmap_mode: bool,
    pub menu_selection: MenuOption,
    // Bugs when multiple flags are true!
}
```

**After**:
```rust
// src/ecs/ui_mode.rs
#[derive(Debug, Clone, PartialEq)]
pub enum UiMode {
    MainMenu { selection: MenuOption },
    InGame(GameSubmode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameSubmode {
    Normal,
    Inventory { selection: usize },
    Character,
    Examine { cursor: Point },
    Overmap,
}

// For nested modals
pub struct UiState {
    stack: Vec<UiMode>,
}

impl UiState {
    pub fn current(&self) -> &UiMode {
        self.stack.last().unwrap()
    }

    pub fn push(&mut self, mode: UiMode) {
        self.stack.push(mode);
    }

    pub fn pop(&mut self) -> Option<UiMode> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None
        }
    }
}
```

**Changes Required**:
1. Create `src/ecs/ui_mode.rs`
2. Add `ui_state: UiState` to Resources
3. Remove 5 boolean fields from Resources
4. Update `src/ui/renderer.rs`:
   ```rust
   // Before
   if resources.in_main_menu { render_main_menu(); return; }
   if resources.in_inventory_mode { render_inventory(); }

   // After
   match resources.ui_state.current() {
       UiMode::MainMenu { selection } => render_main_menu(selection),
       UiMode::InGame(GameSubmode::Inventory { selection }) => render_inventory(selection),
       UiMode::InGame(GameSubmode::Normal) => render_game(),
       // ...
   }
   ```
5. Update `src/systems/input.rs` to route by UiMode

**Testing**:
- [ ] All 125 tests still pass
- [ ] Manual test: ESC from each UI mode
- [ ] Manual test: No conflicting states possible

---

### Task 0.2: Split input.rs (6 hours)
**Why**: Maintainability, testability, reduces merge conflicts

**Current Structure**:
```
src/systems/input.rs (900+ lines)
  - handle_input()
  - handle_key()
  - handle_movement()
  - handle_main_menu_input()
  - handle_inventory_input()
  - handle_character_screen_input()
  - handle_examine_input()
  - try_move_player()
  - try_pickup_items()
  - try_open_door()
  - try_use_stairs()
  - ... 18 functions total
```

**Target Structure**:
```
src/systems/input/
  ├── mod.rs              # Central dispatcher (100 lines)
  ├── gameplay.rs         # Normal mode (200 lines)
  ├── inventory.rs        # Inventory UI (150 lines)
  ├── character.rs        # Character screen (100 lines)
  ├── examine.rs          # Examine mode (100 lines)
  ├── main_menu.rs        # Main menu (80 lines)
  ├── overmap.rs          # Overmap mode (120 lines)
  └── actions.rs          # Shared action helpers (150 lines)
```

**Migration Steps**:
1. Create `src/systems/input/` directory
2. Create `mod.rs` with routing:
   ```rust
   pub fn handle_input(key: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
       match resources.ui_state.current() {
           UiMode::MainMenu { .. } => main_menu::handle_input(key, world, resources),
           UiMode::InGame(GameSubmode::Normal) => gameplay::handle_input(key, world, resources),
           UiMode::InGame(GameSubmode::Inventory { .. }) => inventory::handle_input(key, world, resources),
           // ...
       }
   }
   ```
3. Move `handle_main_menu_input()` → `main_menu.rs`
4. Move `handle_inventory_input()` → `inventory.rs`
5. Move gameplay logic → `gameplay.rs`
6. Move shared helpers → `actions.rs`
7. Update `src/systems/mod.rs`

**Testing**:
- [ ] All 125 tests still pass
- [ ] Manual test: All keybindings work
- [ ] Add unit tests for each input module

---

### Task 0.3: Add Domain Newtypes (2 hours)
**Why**: Type safety, prevents unit mistakes

**Create `src/domain_types.rs`**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Depth(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HitPoints(pub u16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxHitPoints(pub u16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Power(pub i16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Defense(pub i16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(pub u32);

impl HitPoints {
    pub fn saturating_sub(self, amount: u16) -> Self {
        HitPoints(self.0.saturating_sub(amount))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Depth {
    pub const SURFACE: Depth = Depth(0);

    pub fn descend(&self) -> Depth {
        Depth(self.0 + 1)
    }

    pub fn ascend(&self) -> Option<Depth> {
        if self.0 > 0 {
            Some(Depth(self.0 - 1))
        } else {
            None
        }
    }
}
```

**Changes Required**:
1. Update `CombatStats`:
   ```rust
   pub struct CombatStats {
       pub hp: HitPoints,
       pub max_hp: MaxHitPoints,
       pub power: Power,
       pub defense: Defense,
   }
   ```
2. Update `Resources` to use `Depth`:
   ```rust
   pub current_depth: Depth,
   pub dungeon_levels: HashMap<Depth, Map>,
   ```
3. Update all usage sites (compile errors will guide you)

**Testing**:
- [ ] All tests compile
- [ ] All 125 tests still pass
- [ ] Save/load still works

---

### Task 0.4: Add FOV Tests (3 hours)
**Why**: Critical gap - FOV affects gameplay, has zero tests

**Add to `src/systems/fov.rs`**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Map, Tile};

    fn create_empty_room(width: i32, height: i32) -> Map {
        let mut map = Map::new(width, height);
        for y in 1..height-1 {
            for x in 1..width-1 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
        map
    }

    #[test]
    fn test_fov_empty_room_sees_all() {
        let map = create_empty_room(10, 10);
        let mut viewshed = Viewshed::new(8);

        compute_fov(&map, 5, 5, &mut viewshed);

        // Should see most of the room
        assert!(viewshed.visible.len() > 50);
        assert!(viewshed.visible.iter().any(|p| p.x == 5 && p.y == 5));
    }

    #[test]
    fn test_fov_blocked_by_wall() {
        let mut map = create_empty_room(10, 10);
        // Place wall at (5, 3)
        let idx = map.xy_idx(5, 3);
        map.tiles[idx] = Tile::Wall;

        let mut viewshed = Viewshed::new(8);
        compute_fov(&map, 5, 5, &mut viewshed);

        // Should NOT see through wall to (5, 1)
        assert!(!viewshed.visible.iter().any(|p| p.x == 5 && p.y == 1));
    }

    #[test]
    fn test_fov_range_limit() {
        let map = create_empty_room(20, 20);
        let mut viewshed = Viewshed::new(5);

        compute_fov(&map, 10, 10, &mut viewshed);

        // Should not see beyond range
        assert!(!viewshed.visible.iter().any(|p| p.x == 1 && p.y == 1));
        assert!(!viewshed.visible.iter().any(|p| p.x == 19 && p.y == 19));

        // Should see within range
        assert!(viewshed.visible.iter().any(|p| p.x == 10 && p.y == 14));
    }

    #[test]
    fn test_fov_origin_always_visible() {
        let map = create_empty_room(10, 10);
        let mut viewshed = Viewshed::new(3);

        compute_fov(&map, 5, 5, &mut viewshed);

        // Origin should always be visible
        assert!(viewshed.visible.iter().any(|p| p.x == 5 && p.y == 5));
    }

    #[test]
    fn test_fov_doors_block_when_closed() {
        let mut map = create_empty_room(10, 10);
        let idx = map.xy_idx(5, 3);
        map.tiles[idx] = Tile::ClosedDoor;

        let mut viewshed = Viewshed::new(8);
        compute_fov(&map, 5, 5, &mut viewshed);

        // Closed door should block vision
        assert!(!viewshed.visible.iter().any(|p| p.x == 5 && p.y == 1));
    }
}
```

**Testing**:
- [ ] Run `cargo test fov`
- [ ] All 5 new tests pass
- [ ] Total: 130 tests

---

### Task 0.5: Add Inventory Tests (3 hours)
**Why**: Critical gap - affects save/load, has minimal tests

**Add to `src/systems/inventory.rs`**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::{Player, Position, RealityLayer, Item, ItemData};

    #[test]
    fn test_pickup_adds_to_inventory() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let player = world.spawn((
            Player,
            Position::new(5, 5, RealityLayer::Normal),
            Inventory::new(20),
            Name("Player".to_string()),
        ));

        let item = world.spawn((
            Item,
            Position::new(5, 5, RealityLayer::Normal),
            ItemData::default(),
            Name("Sword".to_string()),
        ));

        resources.player_entity = Some(player);
        world.insert_one(player, WantsToPickupItem { item }).unwrap();

        pickup_system(&mut world, &mut resources);

        let inv = world.get::<&Inventory>(player).unwrap();
        assert!(inv.items.contains(&item));
        assert!(world.get::<&Position>(item).is_err()); // Item removed from ground
    }

    #[test]
    fn test_pickup_respects_capacity() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let player = world.spawn((
            Player,
            Position::new(5, 5, RealityLayer::Normal),
            Inventory::new(1), // Capacity of 1
            Name("Player".to_string()),
        ));

        // Add one item already
        let item1 = world.spawn((Item, ItemData::default()));
        world.get::<&mut Inventory>(player).unwrap().items.push(item1);

        // Try to pick up second item
        let item2 = world.spawn((
            Item,
            Position::new(5, 5, RealityLayer::Normal),
            ItemData::default(),
        ));

        resources.player_entity = Some(player);
        world.insert_one(player, WantsToPickupItem { item: item2 }).unwrap();

        pickup_system(&mut world, &mut resources);

        // Should fail - inventory full
        let inv = world.get::<&Inventory>(player).unwrap();
        assert!(!inv.items.contains(&item2));
        assert!(resources.log.messages.iter().any(|m| m.contains("full")));
    }

    #[test]
    fn test_drop_removes_from_inventory() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let player = world.spawn((
            Player,
            Position::new(5, 5, RealityLayer::Normal),
            Inventory::new(20),
        ));

        let item = world.spawn((Item, ItemData::default()));
        world.get::<&mut Inventory>(player).unwrap().items.push(item);

        world.insert_one(player, WantsToDropItem { item }).unwrap();

        drop_system(&mut world, &mut resources);

        let inv = world.get::<&Inventory>(player).unwrap();
        assert!(!inv.items.contains(&item));

        // Item should be on ground
        let pos = world.get::<&Position>(item).unwrap();
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 5);
    }

    #[test]
    fn test_equip_weapon_to_mainhand() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let player = world.spawn((
            Player,
            Inventory::new(20),
        ));

        let weapon = world.spawn((
            Item,
            ItemData::weapon(),
            Equipable { slot: EquipSlot::MainHand },
        ));

        world.get::<&mut Inventory>(player).unwrap().items.push(weapon);
        world.insert_one(player, WantsToEquipItem { item: weapon }).unwrap();

        equip_system(&mut world, &mut resources);

        let inv = world.get::<&Inventory>(player).unwrap();
        assert_eq!(inv.equipped.get(&EquipSlot::MainHand), Some(&weapon));
    }

    #[test]
    fn test_inventory_serialization_preserves_items() {
        let mut world = World::new();
        let mut resources = Resources::new(80, 50, 12345);

        let player = world.spawn((
            Player,
            Position::new(5, 5, RealityLayer::Normal),
            Inventory::new(20),
            Name("Player".to_string()),
        ));

        let item = world.spawn((
            Item,
            ItemData::weapon(),
            Name("Sword".to_string()),
        ));

        world.get::<&mut Inventory>(player).unwrap().items.push(item);
        resources.player_entity = Some(player);

        // Save and load
        let save = SaveGame::from_game(&world, &resources, 12345);
        let mut new_world = World::new();
        let mut new_resources = Resources::new(80, 50, 99999);
        save.restore_game(&mut new_world, &mut new_resources).unwrap();

        // Verify inventory preserved
        let mut found_player = false;
        for (_, inv) in new_world.query::<&Inventory>().iter() {
            assert_eq!(inv.items.len(), 1);
            found_player = true;
        }
        assert!(found_player);
    }
}
```

**Testing**:
- [ ] Run `cargo test inventory`
- [ ] All 5 new tests pass
- [ ] Total: 135 tests

---

### Phase 0 Success Criteria
- [x] UiMode enum implemented
- [x] input.rs split into modules
- [x] Domain newtypes added
- [x] FOV tests added (5 tests)
- [x] Inventory tests added (5 tests)
- [x] All 135+ tests passing
- [x] Build time unchanged or better
- [x] No regressions in manual testing

**Estimated Time**: 18 hours (1-2 weeks part-time)
**Score After**: 7.0 → 7.5

---

## 📈 Phase 1: Event System (Week 3-5)

**Goal**: Eliminate archetype churn, centralize cleanup
**Score**: 7.5 → 8.0

### Overview
Replace intent components (WantsToMove, WantsToMelee) with event queues. This eliminates the performance cost of adding/removing components every turn.

### Task 1.1: Add Events Infrastructure (4 hours)

**Create `src/ecs/events.rs`**:
```rust
use hecs::Entity;
use crate::ecs::Position;

pub struct Events<T> {
    pub queue: Vec<T>,
}

impl<T> Events<T> {
    pub fn new() -> Self {
        Events { queue: Vec::new() }
    }

    pub fn send(&mut self, event: T) {
        self.queue.push(event);
    }

    pub fn drain(&mut self) -> std::vec::Drain<T> {
        self.queue.drain(..)
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[derive(Debug, Clone)]
pub enum SimEvent {
    Move { entity: Entity, to: Position },
    Melee { attacker: Entity, target: Entity },
    Pickup { entity: Entity, item: Entity },
    Drop { entity: Entity, item: Entity },
    OpenDoor { entity: Entity, pos: Position },
    CloseDoor { entity: Entity, pos: Position },
    UseStairs { entity: Entity, ascending: bool },
    Die { entity: Entity },
}
```

**Add to Resources**:
```rust
pub struct Resources {
    // ... existing fields
    pub sim_events: Events<SimEvent>,
}
```

### Task 1.2: Create Schedule (6 hours)

**Create `src/sim/schedule.rs`**:
```rust
use hecs::World;
use crate::ecs::Resources;

pub type SystemFn = fn(&mut World, &mut Resources);

pub struct Schedule {
    pub pre_update: Vec<SystemFn>,
    pub simulation: Vec<SystemFn>,
    pub post_simulation: Vec<SystemFn>,
    pub cleanup: Vec<SystemFn>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            pre_update: vec![
                // Input processing (converts input to events)
            ],
            simulation: vec![
                // AI system (generates move/attack events)
                crate::systems::ai::ai_system,
                // Movement system (processes move events)
                crate::systems::movement::movement_system,
                // FOV system (updates visibility)
                crate::systems::fov::fov_system,
                // Combat system (processes melee events)
                crate::systems::combat::melee_combat_system,
            ],
            post_simulation: vec![
                // Death system
                crate::systems::combat::death_system,
                // Loot generation
            ],
            cleanup: vec![
                // Clear event queues
                cleanup_events,
            ],
        }
    }

    pub fn tick(&mut self, world: &mut World, resources: &mut Resources) {
        for system in &self.pre_update {
            system(world, resources);
        }
        for system in &self.simulation {
            system(world, resources);
        }
        for system in &self.post_simulation {
            system(world, resources);
        }
        for system in &self.cleanup {
            system(world, resources);
        }
    }
}

fn cleanup_events(world: &mut World, resources: &mut Resources) {
    resources.sim_events.clear();
}
```

### Task 1.3: Migrate Movement to Events (4 hours)

**Before** (`src/systems/movement.rs`):
```rust
pub fn movement_system(world: &mut World, resources: &mut Resources) {
    // Collect WantsToMove components
    for (entity, wants_move) in world.query::<&WantsToMove>().iter() {
        // ...
    }

    // Manual cleanup
    let entities: Vec<_> = world.query::<&WantsToMove>().iter().map(|(e, _)| e).collect();
    for entity in entities {
        world.remove_one::<WantsToMove>(entity);
    }
}
```

**After**:
```rust
pub fn movement_system(world: &mut World, resources: &mut Resources) {
    let mut moves = Vec::new();
    let mut attacks = Vec::new();

    // Process movement events
    for event in resources.sim_events.queue.iter() {
        if let SimEvent::Move { entity, to } = event {
            let map = resources.maps.active_map();

            if !map.is_walkable(to.x, to.y) {
                continue;
            }

            // Check for blocking entities
            let mut blocked = false;
            let mut target = None;
            for (other, (other_pos, _)) in world.query::<(&Position, &BlocksMovement)>().iter() {
                if other_pos.x == to.x && other_pos.y == to.y && other_pos.layer == to.layer {
                    blocked = true;
                    target = Some(other);
                    break;
                }
            }

            if blocked {
                if let Some(target) = target {
                    attacks.push((*entity, target));
                }
            } else {
                moves.push((*entity, *to));
            }
        }
    }

    // Apply movements
    for (entity, new_pos) in moves {
        if let Ok(mut pos) = world.get::<&mut Position>(entity) {
            *pos = new_pos;
        }
        if let Ok(mut viewshed) = world.get::<&mut Viewshed>(entity) {
            viewshed.dirty = true;
        }
    }

    // Generate melee events
    for (attacker, target) in attacks {
        resources.sim_events.send(SimEvent::Melee { attacker, target });
    }
}
```

**Update input to generate events**:
```rust
// In gameplay.rs
KeyCode::Char('h') => {
    if let Some(player) = resources.player_entity {
        if let Ok(pos) = world.get::<&Position>(player) {
            let new_pos = Position::new(pos.x - 1, pos.y, pos.layer);
            resources.sim_events.send(SimEvent::Move {
                entity: player,
                to: new_pos
            });
        }
    }
}
```

### Task 1.4: Migrate Combat to Events (4 hours)

Similar pattern - process `SimEvent::Melee` instead of `WantsToMelee` component.

### Task 1.5: Remove Old Intent Components (2 hours)

After all systems migrated:
1. Remove `WantsToMove` from components.rs
2. Remove `WantsToMelee` from components.rs
3. Remove any remaining manual cleanup code
4. Update tests

### Phase 1 Success Criteria
- [x] Events<SimEvent> in use
- [x] Movement uses events (no WantsToMove)
- [x] Combat uses events (no WantsToMelee)
- [x] Schedule with 4 stages implemented
- [x] All tests passing
- [x] Benchmark shows 2x+ improvement in turn processing

**Estimated Time**: 20 hours (2-3 weeks part-time)
**Score After**: 7.5 → 8.0

---

## 📊 Phase 2: Resource Decomposition (Week 6-8)

**Goal**: Break up god object, improve testability
**Score**: 8.0 → 8.5

### Task 2.1: Define Resource Categories (2 hours)

**Create `src/ecs/typed_resources.rs`**:
```rust
pub struct SimResources {
    pub world_time: WorldTime,
    pub rng: StdRng,
    pub weather: WeatherSystem,
    pub perf: PerformanceMetrics,
}

pub struct WorldResources {
    pub overmap: Overmap,
    pub map_cache: MapCache,
    pub current_location: Location,
}

pub struct UiResources {
    pub ui_state: UiState,
    pub camera: Camera,
    pub log: MessageLog,
}

pub struct PlayerResources {
    pub player_entity: Option<Entity>,
    pub last_target: Option<Entity>,
}
```

### Task 2.2: Implement ResourceMap (4 hours)

Generic typed storage for resources:
```rust
pub struct ResourceMap {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceMap {
    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|r| r.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|r| r.downcast_mut::<T>())
    }
}
```

### Task 2.3: Migrate Systems (8 hours)

Update systems to request specific resources:
```rust
// Before
pub fn movement_system(world: &mut World, resources: &mut Resources) {
    let map = resources.maps.active_map();
    let rng = &mut resources.rng;
    // ...
}

// After
pub fn movement_system(
    world: &mut World,
    world_res: &WorldResources,
    sim: &mut SimResources,
    events: &mut Events<SimEvent>,
) {
    let map = world_res.map_cache.current();
    let rng = &mut sim.rng;
    // ...
}
```

### Phase 2 Success Criteria
- [x] Resources split into 4 categories
- [x] ResourceMap implemented
- [x] All systems use typed resources
- [x] Tests easier to write (inject only needed resources)
- [x] All tests passing

**Estimated Time**: 14 hours (2-3 weeks part-time)
**Score After**: 8.0 → 8.5

---

## 🗺️ Phase 3: Map Unification (Week 9-10)

**Goal**: Single map storage abstraction
**Score**: 8.5 → 8.7

### Task 3.1: Define MapId (2 hours)

```rust
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum MapId {
    Overworld,
    Settlement(SettlementId),
    Dungeon(Depth),
}

pub struct Location {
    pub map_id: MapId,
    pub pos: Point,
}
```

### Task 3.2: Implement MapCache (6 hours)

```rust
pub struct MapCache {
    active: MapId,
    overworld: Map,
    dungeons: Vec<Option<Map>>,
    settlements: SlotMap<SettlementId, Map>,
    lru: LruCache<MapId, ()>,  // Track access order
    generator: MapGenerator,
}

impl MapCache {
    pub fn get_mut(&mut self, id: MapId) -> &mut Map {
        // Generate on demand, cache with LRU
    }

    pub fn current(&mut self) -> &mut Map {
        self.get_mut(self.active)
    }
}
```

### Task 3.3: Migrate Map Access (4 hours)

Update all map access to go through MapCache.

### Phase 3 Success Criteria
- [x] MapId enum defined
- [x] MapCache with LRU implemented
- [x] All map access unified
- [x] Tests passing

**Estimated Time**: 12 hours (1-2 weeks part-time)
**Score After**: 8.5 → 8.7

---

## 💾 Phase 4: Serialization Upgrade (Week 11-13)

**Goal**: Scalable save/load
**Score**: 8.7 → 9.0

### Task 4.1: Add Uid Component (2 hours)

```rust
#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Uid(pub u64);

static NEXT_UID: AtomicU64 = AtomicU64::new(1);

pub fn next_uid() -> Uid {
    Uid(NEXT_UID.fetch_add(1, Ordering::Relaxed))
}
```

### Task 4.2: Component Registry (8 hours)

### Task 4.3: Migration System (4 hours)

### Task 4.4: Add Compression (2 hours)

### Phase 4 Success Criteria
- [x] Uid system working
- [x] ComponentRegistry implemented
- [x] Migration tests pass
- [x] Compression reduces file size

**Estimated Time**: 16 hours (2-3 weeks part-time)
**Score After**: 8.7 → 9.0

---

## 🧪 Phase 5: Test Coverage (Week 14-15)

**Goal**: Close remaining gaps
**Score**: Maintain 9.0

### Tasks
- [ ] AI pathfinding tests (5 tests)
- [ ] Input action mapping tests (10 tests)
- [ ] Property-based dungeon tests
- [ ] Fuzzing for save/load
- [ ] Benchmark suite

**Estimated Time**: 12 hours (1-2 weeks part-time)
**Score After**: 9.0 (maintained)

---

## 📋 Tracking Progress

### Completion Checklist

#### Phase 0: Quick Wins
- [ ] 0.1: UiMode enum
- [ ] 0.2: Split input.rs
- [ ] 0.3: Domain newtypes
- [ ] 0.4: FOV tests (5 tests)
- [ ] 0.5: Inventory tests (5 tests)

#### Phase 1: Event System
- [ ] 1.1: Events infrastructure
- [ ] 1.2: Schedule
- [ ] 1.3: Migrate movement
- [ ] 1.4: Migrate combat
- [ ] 1.5: Remove old intents

#### Phase 2: Resources
- [ ] 2.1: Resource categories
- [ ] 2.2: ResourceMap
- [ ] 2.3: Migrate systems

#### Phase 3: Maps
- [ ] 3.1: MapId enum
- [ ] 3.2: MapCache
- [ ] 3.3: Migrate access

#### Phase 4: Serialization
- [ ] 4.1: Uid component
- [ ] 4.2: Component registry
- [ ] 4.3: Migrations
- [ ] 4.4: Compression

#### Phase 5: Tests
- [ ] AI tests
- [ ] Input tests
- [ ] Property tests
- [ ] Fuzzing
- [ ] Benchmarks

---

## 🎯 Success Metrics

| Metric | Current | Phase 0 | Phase 1 | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|--------|---------|---------|---------|---------|---------|---------|---------|
| **Score** | 7.0 | 7.5 | 8.0 | 8.5 | 8.7 | 9.0 | 9.0 |
| **Tests** | 125 | 135 | 135 | 135 | 135 | 140 | 155 |
| **LOC** | 8921 | ~9000 | ~9200 | ~9400 | ~9500 | ~9700 | ~9800 |
| **Turn Speed** | 1x | 1x | 2-3x | 2-3x | 2-3x | 2-3x | 2-3x |
| **Save Size** | 100% | 100% | 100% | 100% | 100% | 40% | 40% |

---

## 💡 Implementation Tips

1. **One phase at a time** - Don't rush
2. **Keep tests passing** - After every task
3. **Document changes** - Update REFACTORING_LOG.md
4. **Benchmark critical paths** - Before and after
5. **Review with fresh eyes** - Take breaks between phases
6. **Ask for help** - When stuck

---

## 📚 Next Steps

1. **Read**: Full review in `ARCHITECTURE_REVIEW_GPT5.md`
2. **Choose**: Pick Task 0.1 (UiMode) or 0.4 (FOV tests)
3. **Branch**: `git checkout -b refactor/phase-0-quick-wins`
4. **Execute**: Follow task steps
5. **Test**: `cargo test --lib`
6. **Review**: Check against success criteria
7. **Commit**: Small, focused commits
8. **Repeat**: Next task

---

**Target**: 9/10 architecture score
**Timeline**: 9-15 weeks
**Philosophy**: Incremental, safe, tested

**You've got this! Start with Phase 0, take it one task at a time.**
