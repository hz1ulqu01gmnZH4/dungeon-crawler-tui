# Test Scenarios for Dungeon Crawler TUI

## Quick Smoke Tests (2 minutes)

Basic sanity check to ensure game is playable:

1. ✅ **Game Launches**: `cargo run --release` starts without errors
2. ✅ **Movement Works**: hjkl moves player in all directions
3. ✅ **Combat Works**: Bump into monster, damage dealt
4. ✅ **Inventory Works**: Pick up item with 'g', view with 'i'
5. ✅ **Save/Load Works**: Save with 'S', reload preserves state

**Pass Criteria**: All 5 work without crashes

---

## Comprehensive Test Suite (15 minutes)

### 1. Movement System Test

**Objective**: Verify all movement mechanics

| Action | Expected Result | Status |
|--------|----------------|--------|
| Press h (left) | Move left 1 tile | ⬜ |
| Press l (right) | Move right 1 tile | ⬜ |
| Press k (up) | Move up 1 tile | ⬜ |
| Press j (down) | Move down 1 tile | ⬜ |
| Press y (up-left) | Move diagonally | ⬜ |
| Press u (up-right) | Move diagonally | ⬜ |
| Press b (down-left) | Move diagonally | ⬜ |
| Press n (down-right) | Move diagonally | ⬜ |
| Move into wall | No movement, no crash | ⬜ |
| Move into monster | Combat initiated | ⬜ |
| Move into same spot | No self-attack (CRITICAL) | ⬜ |

**Bug Watch**: Self-attack on wait, camera not following, stuck on walls

### 2. Combat System Test

**Objective**: Verify combat mechanics and damage calculation

| Test Case | Steps | Expected | Status |
|-----------|-------|----------|--------|
| Basic Attack | Move into goblin | Damage dealt, HP reduced | ⬜ |
| Take Damage | Let goblin hit you | Player HP reduced | ⬜ |
| Kill Monster | Attack until 0 HP | Monster despawns, maybe loot | ⬜ |
| Player Death | Let HP reach 0 | Game over screen | ⬜ |
| Multiple Enemies | Fight 2+ monsters | Combat with each works | ⬜ |
| Stat Bonuses | Equip weapon, attack | Damage increases | ⬜ |

**Bug Watch**: Self-damage, negative HP, crashes on death, damage overflow

### 3. Inventory System Test

**Objective**: Verify item management

| Test Case | Steps | Expected | Status |
|-----------|-------|----------|--------|
| Pick Up Item | Stand on item, press 'g' | Item in inventory | ⬜ |
| Duplicate Pickup | Press 'g' twice on item | Only 1 copy (CRITICAL) | ⬜ |
| Drop Item | Press 'd', select item | Item on ground | ⬜ |
| Equip Weapon | Press 'e', select weapon | Stats increase | ⬜ |
| Equip Armor | Press 'e', select armor | Defense increases | ⬜ |
| Unequip Item | Unequip equipped item | Stats decrease | ⬜ |
| Use Potion | Press 'u', select potion | HP restored | ⬜ |
| Stack Items | Pick up 2 potions | Stack quantity = 2 | ⬜ |
| Full Inventory | Fill capacity, try pickup | Error message shown | ⬜ |

**Bug Watch**: Duplicate items, items not removed from ground, stat bugs, stack overflow

### 4. World Navigation Test

**Objective**: Verify overmap and travel

| Test Case | Steps | Expected | Status |
|-----------|-------|----------|--------|
| Enter Overmap | Press Tab | Overmap displayed | ⬜ |
| Exit Overmap | Press Tab again | Return to local view | ⬜ |
| Overmap Movement | Move on overmap | Position updates | ⬜ |
| See Settlements | Look for city icons | Settlements visible | ⬜ |
| See Roads | Look for road lines | Roads rendered | ⬜ |
| See POIs | Look for POI markers | Dungeons/caves visible | ⬜ |
| Enter Settlement | Move to settlement | Load settlement map | ⬜ |
| Exit Settlement | Exit settlement | Return to overmap | ⬜ |
| Time Passes | Travel on overmap | Time advances | ⬜ |

**Bug Watch**: Mode switching fails, generation errors, position desync, roads missing

### 5. Dungeon Exploration Test

**Objective**: Verify dungeon generation and navigation

| Test Case | Steps | Expected | Status |
|-----------|-------|----------|--------|
| Find Dungeon | Look for dungeon on overmap | Dungeon POI exists | ⬜ |
| Enter Dungeon | Move into dungeon | Dungeon level 1 loads | ⬜ |
| Dungeon Layout | Explore level | Rooms and corridors | ⬜ |
| Find Stairs Down | Look for '>' | Stairs present | ⬜ |
| Descend Stairs | Press '>' on stairs | Go to next level | ⬜ |
| Find Stairs Up | Look for '<' | Return stairs present | ⬜ |
| Ascend Stairs | Press '<' on stairs | Return to previous level | ⬜ |
| Monsters Spawn | Explore levels | Monsters present | ⬜ |
| Items Spawn | Explore levels | Items on floor | ⬜ |

**Bug Watch**: Empty dungeons, stairs not working, generation crashes, infinite loops

### 6. Save/Load Test

**Objective**: Verify persistence

| Test Case | Steps | Expected | Status |
|-----------|-------|----------|--------|
| Save Game | Press 'S' | "Game saved" message | ⬜ |
| Quit Game | Press 'q' | Clean exit | ⬜ |
| Restart Game | `cargo run --release` | Main menu appears | ⬜ |
| Load Game | Select Continue | Game state restored | ⬜ |
| Check Position | Look at player position | Same as before save | ⬜ |
| Check Inventory | Press 'i' | Same items as before | ⬜ |
| Check Stats | View character screen | Same HP/stats as before | ⬜ |
| Check World | Press Tab | Same overmap state | ⬜ |
| Multiple Saves | Save, play, save again | Both saves work | ⬜ |

**Bug Watch**: Save corruption, incomplete load, position reset, lost items

---

## Stress Tests (5 minutes)

### Rapid Input Test
**Goal**: Ensure game handles spam input
```
1. Spam movement keys rapidly (hjkl)
2. Spam 'g' repeatedly on empty ground
3. Spam Tab to switch modes rapidly
4. Spam inventory keys (i, e, d, u)

PASS: No crashes, no freezes, no weird state
```

### Edge Case Tests

#### Empty Inventory
- Try to drop with empty inventory → Should show message
- Try to equip with nothing → Should show message
- Try to use item with none → Should show message

#### Full Inventory
- Fill inventory to capacity
- Try to pick up item → Should fail gracefully
- Drop item, try again → Should work

#### Zero HP
- Reduce HP to exactly 0 → Game over screen
- Try to continue playing → Should not allow

#### Long Play Session
- Play for 10+ in-game days
- Check for memory leaks
- Check for performance degradation

---

## Regression Tests (Recent Bug Fixes)

### ✅ Player Self-Attack (Fixed in commit c5f9b3c)

**Test**: Verify player doesn't attack themselves
```
1. Start game
2. Try to move to current position (if possible)
3. Wait in place (. key or invalid move)

EXPECTED: No self-damage, no combat with self
STATUS: ✅ FIXED
```

### ✅ Duplicate Item Pickup (Fixed in commit 946100c)

**Test**: Verify items can't be picked up twice
```
1. Drop an item on the ground
2. Press 'g' to pick it up
3. Check inventory - should have 1 copy
4. Press 'g' again
5. Check inventory - should still have 1 copy

EXPECTED: Item appears once, not duplicated
STATUS: ✅ FIXED
```

### ✅ Integration Tests (Fixed in commit 80211dd)

**Test**: Verify game initializes in InGame mode
```
1. Start game
2. Press Tab immediately
3. Should enter Overmap mode
4. Should generate world (roads, settlements, POIs)

EXPECTED: World generation happens, Tab works
STATUS: ✅ FIXED
```

---

## Performance Tests

### Frame Rate Test
```
1. Run game: cargo run --release
2. Move around dungeon rapidly
3. Observe: Smooth/laggy?
4. Check CPU usage: Should be low when idle
```

### Large World Test
```
1. Press Tab to overmap
2. Explore entire overmap (50x50 tiles)
3. Visit multiple settlements
4. Enter/exit many dungeons

EXPECTED: No slowdown, no memory issues
```

### Long Session Test
```
1. Play for 30 real-world minutes
2. Monitor: Memory usage, CPU usage
3. Check for: Memory leaks, performance degradation

PASS: Stable performance throughout
```

---

## Test Result Template

```markdown
## Test Session Report

**Date**: [YYYY-MM-DD]
**Commit**: [git hash]
**Tester**: Claude/Human
**Duration**: [X minutes]

### Summary
- ✅ Tests Passed: X/Y
- ❌ Tests Failed: Y/Y
- ⚠️ Tests Partial: Z/Y

### Smoke Tests
- [✅/❌] Game Launches
- [✅/❌] Movement
- [✅/❌] Combat
- [✅/❌] Inventory
- [✅/❌] Save/Load

### Comprehensive Tests
- Movement: ✅ (11/11 tests passed)
- Combat: ✅ (6/6 tests passed)
- Inventory: ⚠️ (8/9 tests passed, 1 issue)
- Navigation: ✅ (9/9 tests passed)
- Dungeons: ✅ (9/9 tests passed)
- Save/Load: ✅ (9/9 tests passed)

### Bugs Found

#### Bug #1: [Title]
- **Severity**: Critical/High/Medium/Low
- **Steps**:
  1. Step 1
  2. Step 2
- **Expected**: [behavior]
- **Actual**: [behavior]
- **Frequency**: Always/Sometimes/Rare

### Performance
- FPS: Smooth/Laggy
- Memory: Stable/Leaking
- CPU: Low/High

### Recommendations
1. [Action item]
2. [Action item]

### Overall Verdict
✅ READY FOR RELEASE / ⚠️ MINOR ISSUES / ❌ MAJOR ISSUES
```

---

## Quick Reference: Game Controls

| Key | Action |
|-----|--------|
| hjkl / arrows | Move (8 directions with yubn) |
| g | Pick up item |
| i | Inventory |
| d | Drop item |
| e | Equip item |
| u | Use item |
| Tab | Toggle overmap |
| > | Descend stairs |
| < | Ascend stairs |
| S | Save game |
| q | Quit |
| c | Character screen |
