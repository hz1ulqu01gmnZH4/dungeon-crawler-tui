# Manual Testing Plan - Quick Equipment & Character Screen

## Prerequisites

```bash
# Build the project
cargo build --release

# Run the game
cargo run --release
```

**Note**: Game starts with seed 12345 (deterministic world generation)

---

## Test Suite 1: Quick Equipment Keys - Wield Weapon (w)

### Test 1.1: Basic Wield
**Steps:**
1. Start new game
2. Navigate to find a weapon (look for `/` or `|` symbols)
3. Stand on weapon tile
4. Press `g` to pick up
5. Press `w` to wield

**Expected Result:**
```
You pick up [Weapon Name].
You equip [Weapon Name] in Main Hand. (+X power, +0 defense)
```

**Verification:**
- Message log shows both pickup and equip messages
- Turn advances (monsters may move)
- HP/stats displayed correctly

### Test 1.2: Already Equipped
**Steps:**
1. With weapon already equipped
2. Press `w` again

**Expected Result:**
```
You already have a weapon equipped.
```

**Verification:**
- No turn consumed (monsters don't move)
- Weapon stays equipped

### Test 1.3: No Weapon Available
**Steps:**
1. Empty inventory or only armor/potions
2. Press `w`

**Expected Result:**
```
You have no weapon to wield.
```

**Verification:**
- No turn consumed
- No equipment changes

### Test 1.4: Wield from Inventory
**Steps:**
1. Pick up weapon but don't equip
2. Press `i` to open inventory
3. Press `w` while in inventory

**Expected Result:**
- Weapon equipped
- Inventory closes automatically
- Turn advances

---

## Test Suite 2: Quick Equipment Keys - Wear Armor (W)

### Test 2.1: Basic Wear
**Steps:**
1. Find armor (look for `]` symbols - leather armor, etc.)
2. Stand on armor tile
3. Press `g` to pick up
4. Press `Shift+W` (capital W) to wear

**Expected Result:**
```
You pick up [Armor Name].
You equip [Armor Name] in Body. (+0 power, +X defense)
```

**Verification:**
- Armor equipped to Body slot
- Defense stat increases
- Turn advances

### Test 2.2: Already Wearing Armor
**Steps:**
1. With armor already equipped
2. Press `W` again

**Expected Result:**
```
You already have armor equipped.
```

### Test 2.3: Wear from Inventory
**Steps:**
1. Pick up armor
2. Press `i` to open inventory
3. Navigate to armor
4. Press `W`

**Expected Result:**
- Armor equipped
- Inventory closes
- Defense stat updated

---

## Test Suite 3: Quick Equipment Keys - Take Off (T)

### Test 3.1: Take Off Weapon
**Steps:**
1. Equip weapon only (no armor)
2. Press `Shift+T` (capital T)

**Expected Result:**
```
You unequip [Weapon Name] from Main Hand.
```

**Verification:**
- Weapon returns to inventory
- Power stat decreases
- Turn advances

### Test 3.2: Take Off Priority Order
**Steps:**
1. Equip both weapon AND armor
2. Press `T`

**Expected Result:**
- Weapon unequipped first (Main Hand priority)

**Steps:**
3. Press `T` again

**Expected Result:**
- Armor unequipped second (Body priority)

### Test 3.3: Nothing Equipped
**Steps:**
1. With empty equipment slots
2. Press `T`

**Expected Result:**
```
You have nothing equipped to take off.
```

---

## Test Suite 4: Character Screen (@)

### Test 4.1: Open Character Screen
**Steps:**
1. From world view, press `Shift+2` or `@` (depending on keyboard)
2. Observe character screen displays

**Expected Layout:**
```
┌─────────────────────────────────────┐
│           Character                 │
├─────────────┬───────────────────────┤
│  Stats      │  Equipment            │
│  ─────      │  ─────────            │
│  HP: XX/XX  │  Main Hand: ...       │
│  Power: X   │  Off Hand: ...        │
│  Defense: X │  Head: ...            │
│             │  Body: ...            │
│  TriMeter   │  Legs: ...            │
│  ────────   │  Feet: ...            │
│  Insight: X │  Ring 1: ...          │
│  Sanity: XX │  Ring 2: ...          │
│  Notice: X  │  Amulet: ...          │
└─────────────┴───────────────────────┘
│         @/ESC: Close                │
└─────────────────────────────────────┘
```

**Verification:**
- HP shows current/max (e.g., 30/30)
- HP is green (>75% HP)
- Power and Defense match equipped items
- All 9 equipment slots visible
- Empty slots show "(empty)" in gray
- Equipped items show in green
- TriMeter shows Insight/Sanity/Notice

### Test 4.2: Close Character Screen
**Steps:**
1. With character screen open
2. Press `@` or `ESC`

**Expected Result:**
- Returns to world view
- No turn consumed

### Test 4.3: HP Color Coding
**Steps:**
1. Fight monsters to take damage
2. Reduce HP to ~50%
3. Open character screen

**Expected Result:**
- HP turns yellow (26-75% HP)

**Steps:**
4. Take more damage (HP <25%)
5. Open character screen

**Expected Result:**
- HP turns red (<25% HP)

### Test 4.4: Equipment Display with Bonuses
**Steps:**
1. Equip Iron Sword (+5 power)
2. Equip Leather Armor (+3 defense)
3. Open character screen

**Expected Result:**
```
Equipment:
  Main Hand: Iron Sword (+5 Pow)
  Body: Leather Armor (+3 Def)
  Off Hand: (empty)
  ...other slots (empty)
```

**Verification:**
- Bonuses shown inline in yellow
- Total Power = base + weapon bonus
- Total Defense = base + armor bonus

### Test 4.5: Cannot Open in Other Modes
**Steps:**
1. Press `Tab` to enter Overmap mode
2. Press `@`

**Expected Result:**
- Character screen does NOT open
- Overmap stays visible

**Steps:**
3. Return to world (`Tab` again)
4. Press `i` to enter Inventory mode
5. Press `@`

**Expected Result:**
- Inventory stays open
- Character screen does NOT open (inventory has priority)

---

## Test Suite 5: Integration Tests

### Test 5.1: Quick Equip → Character Screen
**Steps:**
1. Pick up sword and armor
2. Press `w` to wield sword
3. Press `W` to wear armor
4. Press `@` to open character screen

**Expected Result:**
- Stats show increased Power and Defense
- Equipment section shows both items equipped
- Bonuses calculated correctly

### Test 5.2: Take Off → Character Screen
**Steps:**
1. With equipment equipped
2. Open character screen, note stats
3. Close screen, press `T` to take off
4. Open character screen again

**Expected Result:**
- Stats decreased (weapon removed)
- Main Hand shows "(empty)"
- Power stat back to base value

### Test 5.3: Inventory → Quick Equip → Character Screen
**Steps:**
1. Pick up multiple weapons
2. Press `i` to open inventory
3. Examine first weapon (arrow keys)
4. Press `w` to wield
5. Press `@` to open character screen

**Expected Result:**
- Inventory closes
- Character screen shows equipped weapon
- Stats updated correctly

### Test 5.4: Combat → Damage → Character Screen
**Steps:**
1. Engage in combat with monster
2. Take damage
3. Open character screen

**Expected Result:**
- HP shows reduced value (e.g., 24/30)
- HP color changes based on percentage
- Other stats unchanged (unless equipment changed)

---

## Test Suite 6: Edge Cases

### Test 6.1: Multiple Weapons
**Steps:**
1. Pick up Rusty Sword and Iron Sword
2. Press `w`

**Expected Result:**
- Equips FIRST weapon in inventory (Rusty Sword)
- Not necessarily the best weapon

**Workaround Test:**
3. Press `T` to unequip
4. Press `i` to open inventory
5. Use `e` to manually equip Iron Sword
6. Verify better weapon equipped

### Test 6.2: Full Inventory
**Steps:**
1. Fill inventory with items (pick up 26 items)
2. Try to equip something

**Expected Result:**
- Quick equip still works (uses existing inventory)
- "Inventory is full" only for pickup

### Test 6.3: Stacked Items
**Steps:**
1. Pick up 5 healing potions (should stack)
2. Press `w`

**Expected Result:**
- "You have no weapon to wield." (potions aren't weapons)
- Stacking doesn't interfere with equipment

### Test 6.4: Character Screen During Combat
**Steps:**
1. Stand next to monster (don't attack)
2. Press `@` to open character screen
3. Close screen

**Expected Result:**
- No turn consumed by viewing character
- Monster doesn't move while screen open

---

## Test Suite 7: Save/Load Persistence

### Test 7.1: Equipment Persists
**Steps:**
1. Equip weapon and armor
2. Note current equipment in character screen
3. Press `Shift+S` to save
4. Quit game (press `q`)
5. Restart game (game auto-loads on startup)
6. Press `@` to open character screen

**Expected Result:**
- All equipment still equipped
- Stats match previous session
- Items still in inventory

### Test 7.2: Character Screen State
**Steps:**
1. Open character screen
2. Save game (`Shift+S`)
3. Close game
4. Reload

**Expected Result:**
- Character screen is closed on load
- Can reopen with `@`

---

## Success Criteria Checklist

### Quick Equipment (w/W/T)
- [ ] `w` wields first weapon to Main Hand
- [ ] `W` wears first armor to Body
- [ ] `T` unequips in priority order (weapon > armor > shield)
- [ ] Works in both World and Inventory modes
- [ ] Correct feedback messages for all states
- [ ] Turns consumed for successful actions
- [ ] No turns consumed for failed actions
- [ ] Stats update correctly after equipping/unequipping

### Character Screen (@)
- [ ] `@` opens/closes character screen
- [ ] HP displays with correct color coding
- [ ] Power and Defense show correct values
- [ ] TriMeter displays Insight/Sanity/Notice
- [ ] All 9 equipment slots visible
- [ ] Equipped items show name and bonuses
- [ ] Empty slots show "(empty)" in gray
- [ ] Equipment bonuses calculated correctly
- [ ] Can close with `@` or `ESC`
- [ ] Doesn't open in Overmap or Inventory modes

### Integration
- [ ] Quick equip updates character screen stats
- [ ] Character screen reflects real-time combat damage
- [ ] Save/load preserves all equipment and stats
- [ ] No conflicts with inventory system
- [ ] No conflicts with stacking system

---

## Known Issues (Expected Behavior)

1. **Quick equip always picks first item**: Cannot choose which weapon to wield
   - **Workaround**: Use inventory mode (`i`) to select specific item, then press `e`

2. **Take off priority is fixed**: Cannot select which slot to unequip
   - **Workaround**: Use inventory mode, select equipped item, press `e` to toggle

3. **No stat comparison preview**: Character screen doesn't show "what if" scenarios
   - **Future feature**: Stat comparison UI

4. **Character screen doesn't show XP/Level**: Not yet implemented
   - **Future feature**: Progression system

---

## Reporting Issues

If you encounter issues during testing:

1. **Note the exact steps** to reproduce
2. **Capture the error message** (if any)
3. **Check console output** (if running from terminal)
4. **Save game state** before and after issue
5. **Report with**:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Save file (if relevant)

---

## Next Steps After Testing

Once testing is complete and features verified:

1. **Examine (x)** - Essential for tile inspection
2. **Doors & Stairs (o/c/</>)** - Essential for dungeon navigation
3. **Stat Comparison UI** - Preview before equipping
4. **Main Menu** - Continue/New Game/Quit

---

## Quick Reference Card

**Quick Equipment:**
```
w - Wield weapon (Main Hand)
W - Wear armor (Body)
T - Take off (priority order)
```

**Character Screen:**
```
@ - Open/close character screen
ESC - Close character screen
```

**Other Important Keys:**
```
g - Get/pickup items
i - Inventory
hjkl - Movement
S - Save game
q - Quit
```

Happy testing! 🎮
