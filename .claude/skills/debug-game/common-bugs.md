# Common Bugs to Watch For

This document lists bug patterns commonly found in this dungeon crawler, organized by system.

## Event System Bugs

### Symptoms
- Actions happen twice
- Actions don't happen
- Delayed reactions
- Duplicate events

### Root Causes
```rust
// ❌ BAD: Events not cleared after processing
pickup_system(&mut world, &mut resources);
// Events still in queue!

// ✅ GOOD: Events cleared in cleanup phase
pickup_system(&mut world, &mut resources);
resources.events.clear();  // Schedule does this
```

### What to Check
- `resources.events.clear()` called in schedule
- Systems use `events.iter()` not `events.drain()` during processing
- Event queue processed in correct order

---

## Entity/Component Bugs

### Duplicate Entities
**Symptom**: Same entity appears multiple times

**Example**: Item pickup bug (FIXED)
```rust
// ❌ BAD: No duplicate check
fn add_item(&mut self, item: Entity) {
    self.items.push(item);  // Can add same item twice!
}

// ✅ GOOD: Check for duplicates
fn add_item(&mut self, item: Entity) {
    if self.items.contains(&item) {
        return Err("Already in inventory");
    }
    self.items.push(item);
}
```

### Missing Components
**Symptom**: System skips entity or crashes

**What to Check**:
- Entity has all required components
- Components not removed accidentally
- Queries match actual component combinations

### Self-Referencing Entities
**Symptom**: Entity interacts with itself

**Example**: Player self-attack (FIXED)
```rust
// ❌ BAD: Doesn't skip self
for (other, pos) in world.query::<(&Position, &BlocksMovement)>() {
    if pos matches destination {
        attack(other);  // Might attack self!
    }
}

// ✅ GOOD: Skip the moving entity
for (other, pos) in world.query::<(&Position, &BlocksMovement)>() {
    if other == entity { continue; }  // Skip self
    if pos matches destination {
        attack(other);
    }
}
```

---

## Movement System Bugs

### Camera Desync
**Symptom**: Player moves but camera doesn't follow

**What to Check**:
- `camera.center_on(x, y)` called after movement
- Camera position updated in correct system order

### Collision Issues
**Symptom**: Player walks through walls/entities

**What to Check**:
```rust
// Verify all checks:
if !map.is_walkable(x, y) { return; }  // Wall check
if has_blocking_entity(x, y) { return; }  // Entity check
if out_of_bounds(x, y) { return; }  // Bounds check
```

### Position Desync
**Symptom**: Player appears in wrong location after map change

**What to Check**:
- Position component updated when changing maps
- Active layer (Normal/Cosmic) set correctly
- Overmap position synced with local position

---

## Inventory System Bugs

### Item Not Removed from Ground
**Symptom**: Item stays on ground after pickup

**What to Check**:
```rust
// Must remove OnGround marker:
let _ = world.remove_one::<OnGround>(item);
```

### Stat Bonuses Not Applied
**Symptom**: Equipped item doesn't increase stats

**What to Check**:
```rust
// Apply bonuses when equipping:
if let Ok(mut stats) = world.get::<&mut CombatStats>(player) {
    stats.power += equipable.power_bonus;
    stats.defense += equipable.defense_bonus;
}

// Remove bonuses when unequipping:
stats.power -= equipable.power_bonus;
stats.defense -= equipable.defense_bonus;
```

### Stacking Issues
**Symptom**: Stackable items don't stack or stack incorrectly

**What to Check**:
- Quantity updated correctly
- Item name comparison works
- Max stack respected
- Item despawned after merging into stack

---

## Combat System Bugs

### Damage Calculation Errors
**Symptom**: Weird damage numbers, negative damage

**What to Check**:
```rust
// Ensure minimum 1 damage:
let damage = (power - defense).max(1);

// Check for overflow/underflow
let new_hp = current_hp.saturating_sub(damage);
```

### Death Not Triggering
**Symptom**: Entity at 0 HP still alive

**What to Check**:
- Death event sent when HP <= 0
- Death system runs after combat
- Player vs Monster death handled differently

### Combat Stats Desync
**Symptom**: Stats don't match displayed values

**What to Check**:
- Base stats vs modified stats clear
- Equipment bonuses tracked separately
- Buffs/debuffs expire correctly

---

## Map Generation Bugs

### Invalid Maps
**Symptom**: Impassable dungeons, no rooms, no stairs

**What to Check**:
```rust
// Verify generation:
assert!(rooms.len() > 0, "Must have rooms");
assert!(has_stairs_down, "Must have exit");
assert!(all_rooms_connected(), "Must be traversable");
```

### Seed Issues
**Symptom**: Non-deterministic generation

**What to Check**:
- Same seed produces same map
- RNG state properly seeded
- No external randomness sources

### Out of Bounds
**Symptom**: Generation places things outside map

**What to Check**:
```rust
assert!(x >= 0 && x < width);
assert!(y >= 0 && y < height);
```

---

## Save/Load Bugs

### Incomplete Serialization
**Symptom**: Some data not restored after load

**What to Check**:
- All components serialized
- All resources saved
- UIDs properly restored
- References maintained

### Save Corruption
**Symptom**: Load fails or crashes

**What to Check**:
- Valid JSON/binary format
- Schema version compatible
- File not truncated
- Compression/decompression works

### State Mismatch
**Symptom**: Game state inconsistent after load

**What to Check**:
```rust
// Verify state after load:
assert_eq!(saved_position, loaded_position);
assert_eq!(saved_inventory.len(), loaded_inventory.len());
assert_eq!(saved_stats, loaded_stats);
```

---

## UI/Mode Switching Bugs

### Mode Stuck
**Symptom**: Can't switch between InGame/Overmap/Inventory

**What to Check**:
- UI mode properly set: `resources.ui.ui_mode = UiMode::Overmap;`
- Input routing checks mode correctly
- Modal states handled

### Overmap Not Generating
**Symptom**: Tab shows blank or crashes

**What to Check**:
```rust
// Must generate on first access:
if !resources.world.overmap.tiles.iter().any(|t| t.discovered) {
    generate_terrain(&mut resources.world.overmap);
    resources.world.settlements = place_settlements(...);
    resources.world.roads = generate_roads(...);
    resources.world.pois = place_pois(...);
}
```

---

## Performance Bugs

### Memory Leaks
**Symptom**: Memory usage grows over time

**What to Check**:
- Despawned entities fully cleaned up
- Event queue cleared each tick
- No growing vectors/hashmaps
- Resources freed properly

### FPS Drops
**Symptom**: Game becomes laggy

**What to Check**:
- FOV recalculated only when needed (`viewshed.dirty`)
- Rendering optimized (don't redraw everything)
- No O(n²) algorithms in hot paths
- Spatial partitioning for entity queries

---

## Testing Checklist for Each Bug Type

When you find a bug, verify:

- [ ] Can you reproduce it consistently?
- [ ] What are the minimal steps to reproduce?
- [ ] Does it happen in debug and release builds?
- [ ] Does it happen with different seeds?
- [ ] Is there an error message or panic?
- [ ] What was the game state when it occurred?
- [ ] Is there a related unit test that should catch this?
- [ ] Has this bug been fixed before and regressed?

---

## Bug Severity Guide

### Critical (Fix Immediately)
- Game crashes
- Data corruption
- Player unable to progress
- Infinite loops
- Example: Player self-attack, save corruption

### High (Fix Soon)
- Major feature broken
- Exploitable issues
- Frequent annoyances
- Example: Items duplicating, stats not applying

### Medium (Fix This Sprint)
- Minor feature issues
- Edge cases
- Polish issues
- Example: UI glitches, minor stat bugs

### Low (Fix When Possible)
- Cosmetic issues
- Rare edge cases
- Nice-to-haves
- Example: Typos, minor visual issues

---

## Debugging Tools

### Enable Backtraces
```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run  # Even more detail
```

### Debug Build
```bash
cargo run  # Not --release, has better error messages
```

### Print Debugging
```rust
eprintln!("DEBUG: entity={:?}, pos={:?}", entity, pos);
dbg!(&inventory.items);
```

### Test Specific System
```bash
cargo test --lib combat_system
cargo test --lib movement_system
cargo test --lib inventory
```

### Check Recent Changes
```bash
git diff HEAD~1  # What changed in last commit?
git log --oneline -10  # Recent commits
```

---

## Known Fixed Bugs (Regression Watch)

Keep an eye on these - they've been fixed but could regress:

1. ✅ **Player Self-Attack** (c5f9b3c)
   - Watch: Movement system entity collision check

2. ✅ **Duplicate Item Pickup** (946100c)
   - Watch: Inventory add_item duplicate check

3. ✅ **Integration Test Mode** (80211dd)
   - Watch: Test harness initialization

4. ✅ **Resource Field Access** (51b354a)
   - Watch: Typed resource structure (sim/world/ui/player)

If any of these bugs reappear, check the related commits for the fixes.
