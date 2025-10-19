# Refactoring Action Plan
**Date**: 2025-10-18
**Based on**: GPT-5 Architecture Review
**Current Score**: 7/10 → **Target**: 9/10

---

## 🎯 Executive Summary

The codebase is **solid** (7/10) but has technical debt that will slow Phase 2 development. Key issues:
- Resources struct has become a "god object" (30+ fields)
- UI state managed with boolean flags instead of state machine
- Input handling is monolithic (900+ lines)
- Manual intent cleanup causes performance issues
- Test gaps in core systems (FOV, inventory, AI)

**Good news**: All issues are fixable with incremental refactoring. No rewrites needed.

---

## 🚀 Quick Wins (Do These First)

These provide immediate value with minimal risk:

### 1. Add UiMode Enum ⏱️ 2-4 hours
**Impact**: HIGH | **Risk**: LOW

```rust
// Before: Multiple booleans
in_main_menu: bool,
in_inventory_mode: bool,
in_examine_mode: bool,
// ... conflicts and bugs

// After: Single state machine
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
```

**Benefits**:
- Eliminates conflicting state bugs
- Makes UI logic clearer
- Enables modal nesting
- Simplifies input routing

**Files to change**:
- `src/ecs/resources.rs` - Add UiMode, remove boolean flags
- `src/ui/renderer.rs` - Match on UiMode instead of if-checks
- `src/systems/input.rs` - Route by UiMode

---

### 2. Split input.rs ⏱️ 4-6 hours
**Impact**: HIGH | **Risk**: LOW

**Current**: 900+ lines in one file
**Target**: Modular structure

```
src/systems/input/
  ├── mod.rs          # Central routing
  ├── gameplay.rs     # Normal mode (200 lines)
  ├── inventory.rs    # Inventory UI (150 lines)
  ├── character.rs    # Character screen (100 lines)
  ├── examine.rs      # Examine mode (100 lines)
  ├── main_menu.rs    # Main menu (80 lines)
  └── keymap.rs       # Action mapping (100 lines)
```

**Benefits**:
- Easier to understand and maintain
- Can test each mode independently
- Reduces merge conflicts
- Clear separation of concerns

**Migration**:
1. Create `src/systems/input/` directory
2. Move main menu handler to `main_menu.rs`
3. Move inventory handler to `inventory.rs`
4. Continue for each mode
5. Update `mod.rs` to route by UiMode

---

### 3. Add Domain Newtypes ⏱️ 1-2 hours
**Impact**: MEDIUM | **Risk**: NONE

```rust
// Prevents unit mistakes and clarifies intent
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Depth(i32);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct HitPoints(u16);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Money(u32);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Power(i16);

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Defense(i16);

// Usage
combat_stats.hp = HitPoints(25);  // Clear intent
current_depth = Depth(3);         // Can't confuse with other i32s
```

**Files to change**:
- Create `src/domain_types.rs`
- Update `CombatStats` to use HitPoints, Power, Defense
- Update dungeon code to use Depth

---

### 4. Add FOV Tests ⏱️ 2-3 hours
**Impact**: HIGH | **Risk**: NONE

**Critical gap**: FOV affects gameplay but has zero tests

```rust
#[test]
fn test_fov_empty_room() {
    let map = create_empty_room(10, 10);
    let origin = Point::new(5, 5);
    let visible = compute_fov(&map, origin, 8);

    // Should see entire room
    assert!(visible.len() > 50);
    assert!(visible.contains(&origin));
}

#[test]
fn test_fov_blocked_by_wall() {
    let mut map = create_empty_room(10, 10);
    map.set_tile(5, 3, Tile::Wall);

    let visible = compute_fov(&map, Point::new(5, 5), 8);

    // Should NOT see through wall
    assert!(!visible.contains(&Point::new(5, 1)));
}

#[test]
fn test_fov_symmetry() {
    let map = create_empty_room(10, 10);

    let from_a = compute_fov(&map, Point::new(2, 2), 5);
    let from_b = compute_fov(&map, Point::new(7, 7), 5);

    // If A sees B, then B sees A
    if from_a.contains(&Point::new(7, 7)) {
        assert!(from_b.contains(&Point::new(2, 2)));
    }
}
```

**Files to change**:
- `src/systems/fov.rs` - Add tests module

---

### 5. Add Inventory Tests ⏱️ 2-3 hours
**Impact**: HIGH | **Risk**: NONE

**Critical gap**: Inventory affects save/load but minimal tests

```rust
#[test]
fn test_pickup_adds_to_inventory() {
    let mut world = World::new();
    let player = create_test_player(&mut world);
    let item = create_test_item(&mut world, Point::new(5, 5));

    world.insert_one(player, WantsToPickupItem { item }).unwrap();
    pickup_system(&mut world, &mut resources);

    let inv = world.get::<&Inventory>(player).unwrap();
    assert!(inv.items.contains(&item));
}

#[test]
fn test_equip_weapon_to_mainhand() {
    // Test equipping logic
}

#[test]
fn test_inventory_weight_limit() {
    // Test capacity constraints
}

#[test]
fn test_inventory_serialization() {
    // Save and load with items in inventory
}
```

**Files to change**:
- `src/systems/inventory.rs` - Add tests module

---

## 📋 Full Refactoring Roadmap

### Phase 0: Quick Wins (1-2 weeks)
- [x] Code review complete
- [ ] Add UiMode enum
- [ ] Split input.rs
- [ ] Add domain newtypes
- [ ] Add FOV tests
- [ ] Add Inventory tests

**Milestone**: 7/10 → 7.5/10

---

### Phase 1: Event System (2-3 weeks)
- [ ] Add Events<T> generic resource
- [ ] Define SimEvent enum
- [ ] Migrate movement to events
- [ ] Migrate combat to events
- [ ] Add hecs::CommandBuffer usage
- [ ] Create Schedule with stages
- [ ] Remove manual intent cleanup

**Milestone**: 7.5/10 → 8/10

**Benefits**:
- 2-3x faster turn processing
- No archetype churn
- Centralized cleanup
- Better testability

---

### Phase 2: Resource Decomposition (2-3 weeks)
- [ ] Create ResourceMap
- [ ] Split into typed categories:
  - SimResources (time, RNG, weather)
  - WorldResources (maps, overworld)
  - UiResources (camera, log, mode)
  - PlayerResources (entity, cursor, target)
- [ ] Update systems to request specific resources
- [ ] Add Res<T> and ResMut<T> wrappers

**Milestone**: 8/10 → 8.5/10

**Benefits**:
- Clear dependencies
- Better encapsulation
- Easier testing
- Prevents coupling

---

### Phase 3: Map Unification (1-2 weeks)
- [ ] Add MapId enum
- [ ] Implement MapCache with LRU
- [ ] Convert dungeon_levels to Vec<Option<Map>>
- [ ] Unify map selection logic
- [ ] Add MapService abstraction

**Milestone**: 8.5/10 → 8.7/10

**Benefits**:
- Single source of truth
- Memory management
- Cleaner APIs

---

### Phase 4: Serialization Upgrade (2-3 weeks)
- [ ] Add Uid component
- [ ] Implement ComponentRegistry
- [ ] Add stable entity references
- [ ] Add per-component versioning
- [ ] Add migration tests
- [ ] Add compression (zstd)

**Milestone**: 8.7/10 → 9/10

**Benefits**:
- Add components without touching save code
- Stable references across sessions
- Smaller save files
- Future-proof

---

### Phase 5: Test Coverage (1-2 weeks)
- [ ] Add AI pathfinding tests
- [ ] Add input action tests
- [ ] Add property-based dungeon tests
- [ ] Add fuzzing for save/load
- [ ] Add benchmark suite (criterion)

**Milestone**: 9/10 → 9/10 (maintained)

**Benefits**:
- Catch regressions early
- Performance budgets
- Confidence in changes

---

## 🔥 Critical Issues (Do Not Ignore)

### 1. Test Gaps in Core Systems
**Risk**: HIGH
**Priority**: Immediate

Missing tests:
- ❌ FOV calculation (affects gameplay)
- ❌ Inventory (affects save/load)
- ❌ AI/pathfinding (can cause soft locks)
- ❌ Input handling (UX regressions)

**Action**: Add FOV and Inventory tests in Phase 0

---

### 2. Intent Component Archetype Churn
**Risk**: MEDIUM
**Priority**: Phase 1

Every intent component add/remove causes entity archetype changes, which is expensive.

**Impact**:
- Slower turn processing
- Memory fragmentation
- Cache misses

**Fix**: Event system (Phase 1)

---

### 3. Resources God Object
**Risk**: MEDIUM
**Priority**: Phase 2

30+ fields mixing simulation, UI, IO, and world state violates separation of concerns.

**Impact**:
- Hard to understand dependencies
- Difficult to test systems
- Accidental coupling

**Fix**: Split into typed resources (Phase 2)

---

### 4. Manual Entity Serialization
**Risk**: MEDIUM-HIGH (for Phase 2+)
**Priority**: Phase 4

Every new component requires updating EntityData and serialization code.

**Impact**:
- High maintenance burden
- Easy to forget components
- Brittle save/load

**Fix**: Component registry (Phase 4)

---

## 🎓 Learning Opportunities

### Recommended Reading

**ECS Patterns**:
- https://github.com/SanderMertens/ecs-faq
- https://csherratt.github.io/blog/posts/scheduling-ecs/

**Rust Game Dev**:
- https://arewegameyet.rs/
- https://github.com/amethyst/specs (for scheduling ideas)

**Property Testing**:
- https://github.com/proptest-rs/proptest
- https://www.youtube.com/watch?v=jdTeQM6iQq8

**Benchmarking**:
- https://github.com/bheisler/criterion.rs

---

## 📊 Progress Tracking

| Phase | Status | Score Improvement | Time Estimate |
|-------|--------|-------------------|---------------|
| Phase 0 (Quick Wins) | 🟡 Pending | 7.0 → 7.5 | 1-2 weeks |
| Phase 1 (Events) | ⚪ Not Started | 7.5 → 8.0 | 2-3 weeks |
| Phase 2 (Resources) | ⚪ Not Started | 8.0 → 8.5 | 2-3 weeks |
| Phase 3 (Maps) | ⚪ Not Started | 8.5 → 8.7 | 1-2 weeks |
| Phase 4 (Serialization) | ⚪ Not Started | 8.7 → 9.0 | 2-3 weeks |
| Phase 5 (Tests) | ⚪ Not Started | 9.0 → 9.0 | 1-2 weeks |

**Total Estimated Time**: 9-15 weeks

**Recommended Pace**: One phase at a time, with testing and review between each.

---

## 🛠️ Implementation Tips

### Start Small
- Don't refactor everything at once
- One system at a time
- Keep tests passing

### Use Feature Flags
```rust
#[cfg(feature = "new_event_system")]
fn movement_system_v2(...) { /* new implementation */ }

#[cfg(not(feature = "new_event_system"))]
fn movement_system(...) { /* old implementation */ }
```

### Create Adapters
When migrating to new patterns, create temporary adapters:
```rust
// Adapter during migration
fn legacy_resources_to_typed(res: &Resources) -> (SimResources, WorldResources) {
    // Extract and convert
}
```

### Document Decisions
Keep a REFACTORING_LOG.md:
```markdown
## 2025-10-20: Added UiMode enum

**Motivation**: Eliminate boolean UI flags
**Changes**: Added UiMode/GameSubmode, removed 5 boolean fields
**Migration**: Updated 12 call sites
**Result**: -40 lines, +20 lines (net -20), clearer state
**Tests**: All 125 tests still passing
```

### Test Before and After
```bash
# Before refactoring
cargo test --lib > before_tests.txt
cargo build --release --timings

# After refactoring
cargo test --lib > after_tests.txt
cargo build --release --timings

# Compare
diff before_tests.txt after_tests.txt
```

---

## ⚠️ Risks & Mitigation

### Risk: Breaking Save Compatibility
**Mitigation**:
- Keep version number in SaveGame
- Add migration tests
- Test with real save files before releasing

### Risk: Performance Regression
**Mitigation**:
- Add benchmarks before refactoring
- Compare before/after
- Set performance budgets in CI

### Risk: Scope Creep
**Mitigation**:
- Stick to one phase at a time
- Don't add features during refactoring
- Review PRs carefully

### Risk: Merge Conflicts (if team)
**Mitigation**:
- Communicate refactoring plans
- Small, incremental PRs
- Refactor one subsystem at a time

---

## 🎯 Success Criteria

### Phase 0 Complete When:
- ✅ UiMode enum implemented
- ✅ input.rs split into modules
- ✅ Domain newtypes added
- ✅ FOV tests added (5+ tests)
- ✅ Inventory tests added (5+ tests)
- ✅ All 130+ tests passing
- ✅ Build time unchanged or better
- ✅ Code review approved

### Phase 1 Complete When:
- ✅ Events<T> in use
- ✅ Movement and combat use events
- ✅ No manual intent cleanup
- ✅ Schedule with stages implemented
- ✅ Turn processing 2x faster
- ✅ All tests passing

### Final Success (9/10) When:
- ✅ All 6 phases complete
- ✅ 150+ tests passing
- ✅ No god objects
- ✅ Clear architecture
- ✅ <5 minutes to onboard new dev
- ✅ Phase 2 features can be added easily

---

## 📞 Next Steps

1. **Read full review**: `ARCHITECTURE_REVIEW_GPT5.md`
2. **Pick a quick win**: Start with UiMode enum or FOV tests
3. **Create branch**: `git checkout -b refactor/phase-0-quick-wins`
4. **Make changes**: One file at a time, tests passing
5. **Measure**: Before/after benchmarks
6. **Document**: Update REFACTORING_LOG.md
7. **Review**: Check changes against success criteria

---

**Remember**: The goal is not perfection, it's **sustainable improvement**. Each phase makes the next feature easier to add.

**Current**: 7/10 (Good)
**Target**: 9/10 (Excellent)
**Timeline**: 9-15 weeks at steady pace

You've built a solid foundation. These refactors will make Phase 2 development much smoother.

---

**Document Created**: 2025-10-18
**Last Updated**: 2025-10-18
**Next Review**: After Phase 0 completion
