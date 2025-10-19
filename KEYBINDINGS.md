# Dungeon Crawler TUI - Modal Keybindings

Inspired by Cataclysm: Dark Days Ahead (CDDA) modal design philosophy.

## Modal Design Philosophy

Like CDDA, keys change behavior based on **context/mode**:
- **World Mode** (default) - Moving, fighting, interacting with world
- **Inventory Mode** - Managing items, equipment
- **Overmap Mode** - World map navigation
- **Character Mode** - Viewing stats, leveling up
- **Dialogue Mode** - Talking to NPCs

---

# World Mode (Default)

## Movement
- `h` / `←` - Move West
- `j` / `↓` - Move South
- `k` / `↑` - Move North
- `l` / `→` - Move East
- `y` - Move Northwest
- `u` - Move Northeast
- `b` - Move Southwest
- `n` - Move Southeast
- `.` / `5` - Wait/Pass time

## Combat
- Bump into enemy - Melee attack
- **[ESSENTIAL]** `F` - Auto-fight (attack until enemy dead or you move)

## Items (World Context)
- `g` / `,` - Get/Pickup items at current location
- **[ESSENTIAL]** `w` - Wield weapon (quick equip to main hand)
- **[ESSENTIAL]** `W` - Wear armor (quick equip body)
- **[ESSENTIAL]** `T` - Take off armor/weapon
- **[ESSENTIAL]** `a` - Apply/Use item (opens item menu if multiple)
- **[ESSENTIAL]** `q` - Quaff potion (quick use consumable)
- **[ESSENTIAL]** `E` - Eat food

## World Interaction
- **[ESSENTIAL]** `x` - Examine tile/object (look at description)
- **[ESSENTIAL]** `o` - Open door
- **[ESSENTIAL]** `c` - Close door
- **[ESSENTIAL]** `>` - Go down stairs / Enter building
- **[ESSENTIAL]** `<` - Go up stairs / Exit building
- **[PLANNED]** `s` - Smash object/furniture
- **[PLANNED]** `/` - Search area for hidden items
- **[PLANNED]** `V` - View/List items on ground (if multiple)

## Mode Switches
- `i` - Open **Inventory Mode**
- `@` - Open **Character Mode** (stats, skills, equipment overview)
- `Tab` - Toggle **Overmap Mode**
- **[PLANNED]** `m` - Toggle minimap view
- `Esc` - Cancel/Close current action

## Rest & Recovery
- `r` / `R` - Rest/Camp (only in settlements, restores HP, advances time)

## System
- `S` (Shift+s) - Save game
- `q` - Quit (lowercase q in world - needs confirmation)
- `Esc` - Cancel current action

---

# Inventory Mode

**Enter with:** `i` key from World Mode
**Exit with:** `i`, `Esc`, or `Tab`

## Navigation
- `↑` / `k` - Previous item
- `↓` / `j` - Next item
- `PgUp` / `K` - Previous page (if more than screen)
- `PgDn` / `J` - Next page
- `/` - Search/Filter items

## Item Actions (Modal - Different from World!)
- `e` - Equip/Unequip selected item
- `d` - Drop selected item
- `u` - Use selected item (consume/apply)
- `w` - Wield (equip to main hand)
- `W` - Wear (equip to body)
- `T` - Take off (unequip from current slot)
- `a` - Apply item (same as use)
- `=` - Reassign letter (change inventory letter)

## Information
- `x` / `Enter` - Examine selected item (detailed view)
- `@` - Jump to character screen (keep inventory context)
- `Tab` - Close and return to world

## Quick Actions
- `q` - Quaff (use potion) - **Modal difference!** (quit in world, quaff in inventory)
- `E` - Eat (use food)

## Exit
- `i` - Close inventory
- `Esc` - Close inventory
- `Tab` - Close inventory

**Key Difference from World:** In inventory, `q` means "quaff potion", not "quit"!

---

# Character Mode (@)

**Enter with:** `@` key from World or Inventory Mode
**Exit with:** `Esc`, `Tab`, or `@` again

## Navigation
- `↑` / `k` - Scroll up
- `↓` / `j` - Scroll down
- `Tab` - Cycle between tabs (Stats / Skills / Traits / Equipment)

## Display Sections
- **Stats Tab** - HP, Power, Defense, Speed, Level
- **Skills Tab** - **[PLANNED]** Skill list and levels
- **Traits Tab** - **[PLANNED]** Permanent traits/mutations
- **Equipment Tab** - All 9 equipment slots with bonuses

## Actions
- `+` - **[PLANNED]** Level up (if enough XP)
- `i` - Switch to inventory mode
- `Esc` / `@` - Close and return to world

---

# Overmap Mode (Tab)

**Enter with:** `Tab` key from World Mode
**Exit with:** `Tab` or `Esc`

## Navigation
- `hjkl` / Arrow keys - Navigate overmap
- `yubn` - Diagonal movement on overmap

## Actions
- `Enter` - Enter location (settlement/POI)
- `x` - Examine tile (show info)
- **[PLANNED]** `/` - Search for location type
- **[PLANNED]** `N` - Add note to tile

## Display
- **[PLANNED]** `m` - Toggle different map views (terrain/political/etc)

## Exit
- `Tab` - Return to local map
- `Esc` - Return to local map

---

# Dialogue Mode (When talking to NPC)

**Enter with:** `D` key when standing next to NPC
**Exit with:** `Esc` or selecting "Goodbye"

## Navigation
- `↑` / `k` - Previous dialogue option
- `↓` / `j` - Next dialogue option
- Number keys `1-9` - Quick select option
- `Enter` - Select current option

## Actions
- `t` - Trade (if available)
- `q` - Ask about quests
- `Esc` - End conversation

---

# Context-Specific Key Meanings

## Same Key, Different Meanings

| Key | World Mode | Inventory Mode | Character Mode |
|-----|------------|----------------|----------------|
| `q` | Quit game | Quaff potion | - |
| `e` | **[Future: Examine nearby]** | Equip item | - |
| `x` | Examine tile | Examine item | - |
| `d` | **[Future: Drop mode]** | Drop item | - |
| `/` | Search area | Filter items | - |
| `Tab` | → Overmap | Close inventory | Close character |
| `@` | → Character | → Character | Close character |
| `i` | → Inventory | Close inventory | → Inventory |

## Modal Benefits

1. **More keys available** - `q` does different things in different contexts
2. **Intuitive** - Actions make sense in context (e.g., `e` for equip in inventory)
3. **Less conflicts** - Same letter can mean different things
4. **CDDA-like** - Familiar to roguelike players

---

# Implementation Priority

## Phase 1: Essential Modal Actions (Current Sprint)

### High Priority - Must Have
1. **[ESSENTIAL]** Quick equipment (`w`, `W`, `T`)
   - `w` - Wield weapon (world & inventory)
   - `W` - Wear armor (world & inventory)
   - `T` - Take off (world & inventory)

2. **[ESSENTIAL]** Character screen (`@`)
   - View all stats
   - See all equipment with bonuses
   - Check HP/status

3. **[ESSENTIAL]** Examine (`x`)
   - World: Look at tiles, see descriptions
   - Inventory: Item details (already works)

4. **[ESSENTIAL]** Doors & Stairs (`o`, `c`, `<`, `>`)
   - `o` - Open door
   - `c` - Close door
   - `>` - Descend stairs / Enter building
   - `<` - Ascend stairs / Exit building

### Medium Priority - Important
5. **[WANTED]** Auto-fight (`F`)
   - Keep attacking current target until dead
   - Stop if player moves or takes too much damage

6. **[ESSENTIAL]** Apply/Use quick actions (`a`, `q`, `E`)
   - `a` - Apply item from inventory
   - `q` - Quick quaff potion
   - `E` - Quick eat food

## Phase 2: Convenience Features

7. **[PLANNED]** Better inventory navigation
   - Filter/search (`/`)
   - Multi-drop
   - Stack splitting

8. **[PLANNED]** Enhanced world interaction
   - `s` - Smash furniture/doors
   - `/` - Search for hidden items
   - `V` - List all ground items

## Phase 3: Social & Progression (Lower Priority per user)

9. **[PLANNED]** NPC Dialogue (`D`)
   - Talk to NPCs
   - Trade
   - Get quests

10. **[PLANNED]** Quest System (`J`)
    - Journal of active quests
    - Track objectives
    - Quest rewards

11. **[PLANNED]** Leveling (`+` in character screen)
    - Spend skill points
    - Increase stats
    - Learn abilities

---

# Main Menu (Startup)

**Before game starts**, show menu:

```
┌─────────────────────────────────┐
│     DUNGEON CRAWLER TUI         │
│                                 │
│  [C]ontinue (if save exists)    │
│  [N]ew Game                     │
│  [Q]uit                         │
│                                 │
│  Arrow keys to select           │
│  Enter to confirm               │
└─────────────────────────────────┘
```

## Main Menu Keys
- `↑` / `↓` / `k` / `j` - Navigate menu
- `c` - Continue (load save)
- `n` - New game
- `q` - Quit
- `Enter` - Confirm selection
- `Esc` - Quit

**This solves:** Load game conflict - no runtime load needed!

---

# Quick Reference Card

## World Mode Quick Keys
```
Movement:  hjkl yubn        Combat:    Bump enemy, F (auto)
Items:     g,wWT aqE        Interact:  x oc <>
Modes:     i @ Tab          System:    S (save), q (quit)
```

## Inventory Mode Quick Keys
```
Navigate:  ↑↓ kj            Actions:   e d u
Equipment: w W T            Info:      x
Exit:      i Esc Tab
```

## Context Guide
- **Lowercase** = common actions (g, w, x, etc.)
- **Uppercase** = shift variant (S=save, W=wear, T=take off)
- **Special** = mode switches (i, @, Tab, Esc)

---

# Summary of Changes from Original

## Fixed Conflicts
- ✅ `l` - Now only movement (was: load game)
- ✅ `S` - Save game (was: `s`, freed up for smash)
- ✅ `L` - Load game moved to main menu

## New Modal Behaviors
- ✅ `q` - Quit in world, Quaff in inventory
- ✅ `e` - Equip in inventory (freed up for examine in world)
- ✅ `d` - Drop in inventory (freed up for drop mode in world)
- ✅ `x` - Examine tile in world, Examine item in inventory

## Benefits
- More keys available through context
- No conflicts between movement and actions
- Familiar to CDDA players
- Clear mode indicators in UI

---

# Next Steps

1. **Implement main menu** (Continue/New Game/Quit)
2. **Add quick equipment** (`w`, `W`, `T`) - Most essential
3. **Add character screen** (`@`) - View stats/equipment
4. **Add examine** (`x`) - World tile descriptions
5. **Add doors & stairs** (`o`, `c`, `<`, `>`) - Building navigation
6. **Add auto-fight** (`F`) - Combat QoL
7. Polish modal UI indicators

**Questions resolved:**
- ✅ Load game: Main menu on startup
- ✅ Priority: Quick equipment, character screen, doors/stairs (all essential)
- ✅ Modal design: Like CDDA, context-sensitive keys
- ✅ Lower priority: NPC dialogue, quest system

Ready to implement! Should I start with the main menu or jump straight to quick equipment?
