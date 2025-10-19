# Phase 1 Complete Test Plan

**Date**: 2025-10-18
**Build Status**: ✅ Successful (0 errors, 21 warnings - cosmetic only)
**Environment**: WSL2 Linux, Rust 1.83+

---

## Test Environment Setup

### Prerequisites
- Build the project: `cargo build --release`
- Run the game: `cargo run --release`
- Have a terminal with good color support
- Recommended size: 80x40 or larger

### Test Data
- Save file location: `./savegame.json`
- Test seed: Use default or note custom seed for reproducibility

---

## Phase 1 Feature Checklist

## 1. Basic Movement & Camera

**Test ID**: T1.1
**Feature**: Player movement with hjkl/arrows
**Priority**: CRITICAL

### Test Steps:
1. Launch game
2. Press `h` - should move left
3. Press `j` - should move down
4. Press `k` - should move up
5. Press `l` - should move right
6. Try arrow keys (←↓↑→) - should work same as hjkl
7. Try diagonal movement with `y`, `u`, `b`, `n`

### Expected Results:
- ✅ Player `@` symbol moves in correct direction
- ✅ Camera follows player smoothly
- ✅ Position updates in status bar
- ✅ Movement blocked by walls
- ✅ Each movement consumes 1 turn
- ✅ Message log shows turn progression

### Pass Criteria:
- All 8 directions work (hjklyubn or arrows)
- Camera keeps player visible
- Turn counter increases

---

## 2. Combat System

**Test ID**: T2.1
**Feature**: Melee combat with monsters
**Priority**: CRITICAL

### Test Steps:
1. Find a monster (look for colored letters like `g` for goblin)
2. Move into monster to attack
3. Observe combat messages
4. Fight until monster dies or player dies
5. Check HP changes in status bar

### Expected Results:
- ✅ Moving into monster initiates attack
- ✅ Damage values shown in message log
- ✅ HP bar decreases when hit
- ✅ Monster dies at 0 HP
- ✅ Combat consumes turns
- ✅ Monster attacks back

### Pass Criteria:
- Combat messages clear and accurate
- HP changes reflect damage
- Can defeat at least one monster

---

## 3. Field of View (FOV)

**Test ID**: T3.1
**Feature**: Vision and fog of war
**Priority**: CRITICAL

### Test Steps:
1. Move around dungeon
2. Observe tiles becoming visible
3. Move away and observe memory (revealed but not visible)
4. Check walls block vision
5. Open a door and see FOV expand

### Expected Results:
- ✅ Tiles in vision range are brightly colored
- ✅ Previously seen tiles are darker (revealed)
- ✅ Unseen tiles are black
- ✅ Walls block line of sight
- ✅ Closed doors block vision
- ✅ Open doors allow vision through

### Pass Criteria:
- FOV updates every turn
- Revealed tiles remain visible (memory)
- Vision blocked appropriately

---

## 4. Item System

**Test ID**: T4.1
**Feature**: Item pickup and inventory
**Priority**: HIGH

### Test Steps:
1. Find an item (colored `!` or `/` symbol)
2. Stand on item
3. Press `g` to pick up
4. Press `i` to open inventory
5. Navigate with `j`/`k`
6. Select item with `Enter`
7. Press `d` to drop item

### Expected Results:
- ✅ `g` picks up item from floor
- ✅ Message confirms pickup
- ✅ `i` opens inventory screen
- ✅ Items listed with letters (a, b, c...)
- ✅ Item descriptions visible
- ✅ Can select and view details
- ✅ `d` drops item back on floor
- ✅ `Esc` closes inventory

### Pass Criteria:
- Can pick up multiple items
- Inventory displays correctly
- Drop works as expected

---

## 5. Equipment System

**Test ID**: T5.1
**Feature**: Equip weapons and armor
**Priority**: HIGH

### Test Steps:
1. Pick up a weapon (e.g., sword `/`)
2. Press `w` (wield weapon) from world mode
3. Pick up armor (e.g., leather armor)
4. Press `W` (wear armor) from world mode
5. Check stats change
6. Press `T` to remove equipment
7. Try equipment from inventory mode (`i` then select item, choose equip)

### Expected Results:
- ✅ `w` in world mode equips last picked up weapon
- ✅ `W` in world mode equips last picked up armor
- ✅ `T` removes equipped items
- ✅ Power increases with weapon equipped
- ✅ Defense increases with armor equipped
- ✅ Status bar reflects stat changes
- ✅ Can equip from inventory menu

### Pass Criteria:
- Quick equip works (w/W)
- Stats update correctly
- Can unequip with T

---

## 6. Character Screen

**Test ID**: T6.1
**Feature**: Character stats display
**Priority**: MEDIUM

### Test Steps:
1. Press `@` to open character screen
2. Review all displayed information
3. Check equipment slots
4. Verify stats match status bar
5. Press `Esc` to close

### Expected Results:
- ✅ Shows HP, Max HP
- ✅ Shows Power and Defense
- ✅ Shows Sanity, Insight, Notice meters
- ✅ Lists equipped items in slots:
  - Weapon slot
  - Armor slot
  - Head, Hands, Feet (if implemented)
- ✅ Shows base stats vs. equipped bonuses
- ✅ Clear layout and formatting

### Pass Criteria:
- All stats visible
- Equipment shown correctly
- Screen easy to read

---

## 7. Examine Mode

**Test ID**: T7.1
**Feature**: Examine tiles and entities
**Priority**: MEDIUM

### Test Steps:
1. Press `x` to enter examine mode
2. Move cursor with `hjkl` or arrows
3. Examine different tile types:
   - Floor
   - Wall
   - Door (open and closed)
   - Stairs (up and down)
4. Examine monsters
5. Examine items
6. Press `x` or `Esc` to exit

### Expected Results:
- ✅ Yellow cursor appears
- ✅ Cursor moves with hjkl/arrows
- ✅ Bottom panel shows examined tile info
- ✅ Floor: "Stone floor - worn smooth..."
- ✅ Wall: "Solid stone wall..."
- ✅ Closed door: "Press 'o' to open"
- ✅ Open door: "Press 'c' to close"
- ✅ Stairs up: "Press '<' to ascend"
- ✅ Stairs down: "Press '>' to descend"
- ✅ Monsters show HP and stats
- ✅ Items show descriptions
- ✅ Shows visibility status (visible/remembered)

### Pass Criteria:
- Cursor navigation works
- All terrain types have descriptions
- Entity details shown

---

## 8. Door System

**Test ID**: T8.1
**Feature**: Open and close doors
**Priority**: MEDIUM

### Test Steps:
1. Find a closed door (`+` yellow symbol)
2. Stand adjacent to door
3. Press `o` to open
4. Verify door opens (`/` brown symbol)
5. Walk through door
6. Stand adjacent to open door
7. Press `c` to close
8. Verify door closes back to `+`
9. Test multiple doors

### Expected Results:
- ✅ Closed door: `+` yellow
- ✅ Open door: `/` brown
- ✅ `o` opens closed door (must be adjacent)
- ✅ `c` closes open door (must be adjacent)
- ✅ Can't open/close from distance
- ✅ Can walk through open doors
- ✅ Can't walk through closed doors
- ✅ Opening/closing consumes turn
- ✅ FOV blocked by closed, open by open doors

### Pass Criteria:
- Can open and close doors
- Vision changes appropriately
- Movement blocked by closed doors

---

## 9. Stairs & Multi-Level Dungeons

**Test ID**: T9.1
**Feature**: Navigate between dungeon levels
**Priority**: HIGH

### Test Steps:
1. Find stairs down (`>` magenta symbol)
2. Stand on stairs
3. Press `>` to descend
4. Verify depth increases (title shows "Depth 1")
5. Explore new level
6. Find stairs up (`<` cyan symbol)
7. Press `<` to ascend
8. Verify returns to previous level
9. Check level state preserved
10. Descend multiple times (Depth 2, 3, etc.)

### Expected Results:
- ✅ StairsDown: `>` magenta
- ✅ StairsUp: `<` cyan
- ✅ `>` on StairsDown descends one level
- ✅ `<` on StairsUp ascends one level
- ✅ Title bar shows "Dungeon - Depth N"
- ✅ Each level has different layout
- ✅ Levels persist when revisited
- ✅ Player appears at appropriate stairs
- ✅ Can descend to multiple depths
- ✅ Can't ascend from surface (Depth 0)
- ✅ Clear error messages for wrong actions

### Pass Criteria:
- Can navigate up and down
- Levels generate properly
- State preserved

---

## 10. Overmap & Settlement System

**Test ID**: T10.1
**Feature**: World map and settlements
**Priority**: HIGH

### Test Steps:
1. Press `m` to open overmap
2. Observe player position (white `@`)
3. Navigate with hjkl/arrows
4. Find settlements (various symbols)
5. Press `m` to close overmap
6. Walk to settlement on surface map
7. Press `Enter` to enter settlement
8. Explore settlement interior
9. Press `Enter` again to exit

### Expected Results:
- ✅ `m` toggles overmap
- ✅ Shows player position
- ✅ Shows settlements with colors
- ✅ Shows roads connecting settlements
- ✅ Shows POIs (points of interest)
- ✅ Can navigate large world
- ✅ `Enter` on settlement tile enters it
- ✅ Settlement interior loads
- ✅ Can exit settlement with `Enter`
- ✅ Returns to world map at settlement location

### Pass Criteria:
- Overmap displays correctly
- Can enter/exit settlements
- Navigation smooth

---

## 11. Save & Load System

**Test ID**: T11.1
**Feature**: Save game state
**Priority**: HIGH

### Test Steps:
1. Play for several turns
2. Note player position, HP, items
3. Press `S` (shift+s) to save
4. Verify save message
5. Quit game (`Q`)
6. Restart game
7. Press `L` (shift+l) to load
8. Verify all state restored:
   - Player position same
   - HP same
   - Items same
   - Map state same

### Expected Results:
- ✅ `S` saves game
- ✅ Message confirms save
- ✅ Creates `savegame.json`
- ✅ `L` loads game
- ✅ Player position restored
- ✅ HP/stats restored
- ✅ Inventory restored
- ✅ Equipment restored
- ✅ Map state restored
- ✅ Time/weather restored

### Pass Criteria:
- Save file created
- All state restored accurately
- No data loss

---

## 12. Time & Weather System

**Test ID**: T12.1
**Feature**: Time progression and weather
**Priority**: MEDIUM

### Test Steps:
1. Check status bar for time
2. Move several times
3. Observe time advancing
4. Wait for time of day changes:
   - Day
   - Dawn
   - Dusk
   - Night
   - DeepNight
5. Observe lighting changes on map
6. Check weather changes (Clear, Cloudy, Rainy, etc.)

### Expected Results:
- ✅ Status bar shows time (HH:MM format)
- ✅ Status bar shows time of day name
- ✅ Status bar shows weather
- ✅ Time advances with each turn
- ✅ Time of day transitions occur
- ✅ Map lighting changes with time:
  - Day: Full brightness
  - Dawn/Dusk: 70% brightness
  - Night: 40% brightness
  - DeepNight: 30% brightness
- ✅ Weather affects visibility
- ✅ Colors darker at night

### Pass Criteria:
- Time advances properly
- Lighting effects visible
- Weather displayed

---

## 13. Reality Layer System

**Test ID**: T13.1
**Feature**: Switch between Normal and Cosmic layers
**Priority**: MEDIUM

### Test Steps:
1. Check title bar shows "[Normal Layer]"
2. Press `R` to switch layer
3. Verify title shows "[Cosmic Layer]"
4. Observe any visual differences
5. Check if entities appear/disappear
6. Press `R` again to return to Normal
7. Verify layer switching works both ways

### Expected Results:
- ✅ `R` toggles between layers
- ✅ Title bar shows current layer
- ✅ Message confirms switch
- ✅ Entities filtered by layer
- ✅ Player position maintained
- ✅ Can switch freely

### Pass Criteria:
- Layer switching works
- Visual indication clear
- No crashes

---

## 14. Camping & Rest System

**Test ID**: T14.1
**Feature**: Rest to restore HP
**Priority**: MEDIUM

### Test Steps:
1. Take damage from combat
2. Find safe location
3. Press `5` or `.` to rest/camp
4. Observe HP restoration
5. Check time advancement
6. Verify resting can be interrupted

### Expected Results:
- ✅ Rest command works
- ✅ HP restores over time
- ✅ Time advances during rest
- ✅ Message shows rest progress
- ✅ Can cancel rest
- ✅ Monsters may interrupt

### Pass Criteria:
- HP restoration works
- Time advances appropriately
- System balanced

---

## 15. Minimap Display

**Test ID**: T15.1
**Feature**: Minimap overlay
**Priority**: LOW

### Test Steps:
1. Check top-right corner of screen
2. Verify minimap shows nearby area
3. Move around and observe minimap update
4. Check different tile types visible:
   - Walls
   - Floors
   - Doors
   - Stairs
   - Player position

### Expected Results:
- ✅ Minimap visible in corner
- ✅ Shows compressed view of area
- ✅ Player marked clearly
- ✅ Tile types distinguishable
- ✅ Updates in real-time

### Pass Criteria:
- Minimap readable
- Helps navigation
- Updates correctly

---

## Integration Tests

## INT-1: Combat + Items + Equipment

**Test**: Complete combat scenario with equipment
1. Start with no equipment
2. Find weapon, equip it
3. Fight monster with weapon
4. Find armor, equip it
5. Fight another monster
6. Verify bonuses apply

**Expected**: Combat calculations use equipment bonuses

---

## INT-2: Multi-Level + Save/Load

**Test**: Save/load with multi-level dungeon
1. Descend to Depth 3
2. Drop item on floor
3. Save game
4. Quit and reload
5. Verify at Depth 3
6. Verify item still on floor

**Expected**: Dungeon levels persist through save/load

---

## INT-3: Overmap + Settlement + Items

**Test**: Complete settlement visit
1. Find settlement on overmap
2. Travel to settlement
3. Enter settlement
4. Pick up items inside
5. Exit settlement
6. Verify items retained

**Expected**: Settlement items can be taken to world map

---

## INT-4: Time + Weather + FOV

**Test**: Visibility in different conditions
1. Note visibility during day
2. Wait for night
3. Check visibility reduction
4. Change to rainy weather (if possible)
5. Check further visibility reduction

**Expected**: Lighting system works correctly

---

## INT-5: Stairs + Combat + FOV

**Test**: Fight on different levels
1. Descend to Depth 1
2. Fight monster
3. Descend to Depth 2
4. Fight monster
5. Ascend back to Depth 1
6. Check if fought monster gone

**Expected**: Each level maintains separate monster state

---

## Performance Tests

## PERF-1: Deep Descent

**Test**: Descend to Depth 10+
- Verify no slowdown
- Check memory usage reasonable
- Ensure generation fast

**Expected**: No performance degradation

---

## PERF-2: Large Inventory

**Test**: Collect 20+ items
- Verify inventory responsive
- Check save/load with large inventory
- Ensure no lag

**Expected**: Smooth operation with many items

---

## Bug Checks

## BUG-1: Wall Walking

**Test**: Try to walk through walls
**Expected**: Blocked properly

---

## BUG-2: Item Stacking

**Test**: Drop two items on same tile
**Expected**: Both visible and pickable

---

## BUG-3: Monster Pathfinding

**Test**: Observe monster movement
**Expected**: Monsters can navigate around obstacles

---

## BUG-4: Door Edge Cases

**Test**: Try to open already open door
**Expected**: Appropriate message

**Test**: Try to close already closed door
**Expected**: Appropriate message

---

## BUG-5: Stair Edge Cases

**Test**: Try to use stairs while not standing on them
**Expected**: Clear error message

**Test**: Try to ascend from surface
**Expected**: Message: "Already on surface"

**Test**: Try wrong direction on stairs
**Expected**: Clear instruction message

---

## Known Issues & Limitations

### Expected Limitations (from STAIRS_IMPLEMENTATION.md):
1. ⚠️ Player position search: Linear O(w×h) when changing levels
2. ⚠️ No stair position memory: Doesn't remember exact entry point
3. ⚠️ Single path: One entrance/exit per level
4. ⚠️ Infinite depth: No maximum depth limit
5. ⚠️ No dungeon persistence: Lost on quit (needs save integration)

### Cosmetic Warnings (21 total):
- Unused imports
- Unused variables
- Deprecated buffer method
- Ambiguous glob re-exports

These do not affect functionality.

---

## Test Environment Notes

### WSL2 Limitations:
- TUI apps may show "No such device or address" when run in background
- Must run in foreground with proper TTY: `cargo run --release`
- Ensure terminal supports 256 colors

### Terminal Requirements:
- Minimum size: 80x40
- Color support: 256 colors recommended
- UTF-8 encoding for special characters

---

## Success Criteria Summary

### Must Pass (CRITICAL):
- ✅ T1.1: Movement
- ✅ T2.1: Combat
- ✅ T3.1: FOV
- ✅ T9.1: Stairs

### Should Pass (HIGH):
- ✅ T4.1: Items
- ✅ T5.1: Equipment
- ✅ T10.1: Overmap
- ✅ T11.1: Save/Load

### Nice to Pass (MEDIUM):
- ✅ T6.1: Character Screen
- ✅ T7.1: Examine Mode
- ✅ T8.1: Doors
- ✅ T12.1: Time/Weather
- ✅ T13.1: Reality Layers
- ✅ T14.1: Camping

### Optional (LOW):
- ✅ T15.1: Minimap

---

## Test Report Template

**Test Date**: ___________
**Tester**: ___________
**Build**: ___________

| Test ID | Feature | Status | Notes |
|---------|---------|--------|-------|
| T1.1 | Movement | ⬜ PASS / ⬜ FAIL | |
| T2.1 | Combat | ⬜ PASS / ⬜ FAIL | |
| T3.1 | FOV | ⬜ PASS / ⬜ FAIL | |
| T4.1 | Items | ⬜ PASS / ⬜ FAIL | |
| T5.1 | Equipment | ⬜ PASS / ⬜ FAIL | |
| T6.1 | Character | ⬜ PASS / ⬜ FAIL | |
| T7.1 | Examine | ⬜ PASS / ⬜ FAIL | |
| T8.1 | Doors | ⬜ PASS / ⬜ FAIL | |
| T9.1 | Stairs | ⬜ PASS / ⬜ FAIL | |
| T10.1 | Overmap | ⬜ PASS / ⬜ FAIL | |
| T11.1 | Save/Load | ⬜ PASS / ⬜ FAIL | |
| T12.1 | Time/Weather | ⬜ PASS / ⬜ FAIL | |
| T13.1 | Reality | ⬜ PASS / ⬜ FAIL | |
| T14.1 | Camping | ⬜ PASS / ⬜ FAIL | |
| T15.1 | Minimap | ⬜ PASS / ⬜ FAIL | |

**Overall Phase 1 Status**: ⬜ PASS / ⬜ FAIL

**Critical Issues Found**: ___________
**Recommendations**: ___________

---

**END OF TEST PLAN**
