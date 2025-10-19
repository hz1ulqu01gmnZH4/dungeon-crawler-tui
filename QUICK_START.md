# Quick Start Guide - Dungeon Crawler TUI

## ✅ Phase 1 is Fully Functional!

Yes, the game **works with `cargo run`**! All Phase 1 systems are integrated and playable.

## Running the Game

```bash
# Run the game
cargo run

# Or run in release mode (faster, smaller binary)
cargo run --release
```

## What Works in Phase 1

### 🎮 Gameplay Features

**Local Map Mode** (Default):
- ✅ Procedural dungeon generation
- ✅ Player movement (arrow keys or hjkl)
- ✅ Field of View (FOV) with lighting
- ✅ Monster spawning (Goblins, Orcs, Trolls)
- ✅ Combat system
- ✅ Health and stats display
- ✅ Message log

**Overmap Mode** (Press Tab):
- ✅ 256x256 world map with terrain biomes
- ✅ Settlements (Cities, Towns, Villages)
- ✅ Road network connecting settlements
- ✅ Travel with time progression
- ✅ Random travel events
- ✅ Tile discovery as you explore
- ✅ Settlement entry (Press Enter on settlement)

**Time & Weather System**:
- ✅ Dynamic time progression (year/season/day/hour)
- ✅ Day/night cycle with lighting effects
- ✅ Weather changes (Clear, Rain, Snow, Fog, Storm)
- ✅ Seasonal weather patterns
- ✅ Weather affects visibility

**Advanced Features**:
- ✅ Reality layer switching (Shift+L for Cosmic layer)
- ✅ Camping and rest (Press 'c')
- ✅ Minimap in bottom-right corner
- ✅ Save/Load system (automatic)
- ✅ TriMeter system (Sanity, Insight, Notice)

## Controls

### Movement
- **Arrow Keys** or **hjkl** - Move player (vim-style)
  - `h` = left, `j` = down, `k` = up, `l` = right
  - Diagonal movement available with arrow keys

### Mode Switching
- **Tab** - Toggle between local map and overmap
- **Shift+L** - Switch reality layer (Normal ↔ Cosmic)

### Actions
- **c** - Camp and rest (restores HP, advances time)
- **Enter** - Enter settlement/building (when on overmap)
- **q** - Quit game

### Information
- Status bar shows: Position, Time, Time of Day, Weather, HP, Stats
- Message log shows events and actions
- Minimap shows overview of current area

## What You'll See

### Starting the Game

When you first run `cargo run`, you'll see:

```
 Dungeon Clawler - Floor 1 [Normal Layer]
┌────────────────────────────────────────────────┐
│  ##########                                    │
│  #.........#                                   │
│  #..@......#    ##### ####                    │
│  #.........#    #...###..#                    │
│  ##########     #........#                    │
│                 #.....g..#                    │
│                 ##########                    │
└────────────────────────────────────────────────┘

 Status
Pos: (40, 25) | Year 1, Day 1, 08:00 | Day | Clear | HP: [████████████████████] 30/30

 Messages
Welcome to the Dungeon Clawler!
Use hjkl or arrow keys to move. Press 'q' to quit.
```

- `@` = You (the player)
- `#` = Walls
- `.` = Floor
- `g` = Goblin (monster)
- Minimap in bottom-right shows overview

### Pressing Tab (Overmap Mode)

```
 World Map
┌────────────────────────────────────────────────┐
│             . . . . . ^                        │
│           . . t t . . ^                        │
│         . . t t t . . ^                        │
│       . . . . . . . . ^                        │
│     . . = = = ⌂ = = .                         │
│   . . . = . . . = . .                         │
│ . . . . = . @ . = . .                         │
│   . . . = . . . = . .                         │
│     . . = = = = = .                           │
└────────────────────────────────────────────────┘

 Legend
@ You  . Plains  t Forest  ^ Mountains  ~ Water
= Road  ⌂ Settlement  ▼ Dungeon  % Swamp

 Controls
Arrow Keys Move  Enter Enter Location  Tab Toggle Map  Q Quit
```

- Navigate with arrow keys
- Tiles reveal as you explore
- Roads connect settlements
- Press Enter on `⌂` to enter a settlement

## Verifying It Works

### Test 1: Movement
```bash
cargo run
```
- Use arrow keys to move around
- You should see FOV update as you move
- Time advances slightly with each move
- HP bar visible in status

### Test 2: Combat
- Move towards a monster (g, o, or t)
- Combat should happen automatically
- Message log shows combat results
- HP decreases when hit

### Test 3: Overmap
- Press **Tab** to enter overmap mode
- Move around to discover tiles
- Find a settlement (⌂) and press Enter
- You'll be in the settlement's local map

### Test 4: Time Progression
- Watch the time in status bar
- Move around - time advances
- Press 'c' to camp - time jumps forward significantly
- Day/night cycle affects lighting

### Test 5: Weather & Lighting
- Watch weather in status bar
- Weather changes over time
- At night (DeepNight), map becomes darker
- Fog reduces visibility

## Technical Details

### Performance
- **Debug Build**: ~25MB binary, runs smoothly
- **Release Build**: Smaller, faster (`cargo run --release`)
- **Frame Rate**: Should feel responsive
- **100 Unit Tests**: All passing

### What's Generated
- **Dungeon**: 30 rooms, connected by corridors
- **Monsters**: Randomly placed in rooms (except first)
- **Overmap**: 256x256 tiles with varied terrain
- **Settlements**: ~10-20 settlements with road connections
- **POIs**: Dungeons, ruins, caves, towers scattered on overmap

### Saved Data
- Game state automatically saved
- Save location: `~/.local/share/dungeon-clawler-tui/` (Linux)
- Includes: player stats, position, world state, time, weather

## Troubleshooting

### "Terminal too small" error
- Resize terminal to at least 80x50
- Or adjust window size

### Game runs but looks wrong
- Ensure terminal supports Unicode characters
- Try a different terminal (iTerm2, Alacritty, Windows Terminal)
- Check terminal color support

### Movement doesn't work
- Make sure terminal is in focus
- Try arrow keys instead of hjkl (or vice versa)
- Check if terminal is intercepting key presses

### Performance issues
- Use release build: `cargo run --release`
- Reduce terminal size
- Close other applications

## Known Limitations

1. **Minimap Always Visible**: Minimap shows even in small terminals
2. **No Inventory UI**: Items exist but no UI yet (Phase 2)
3. **Simple Combat**: Automatic combat, no tactics yet (Phase 2)
4. **Building Interiors**: Generated but not yet enterable from settlements (Phase 2)
5. **Chunk Loading**: Implemented but disabled by default

## What's Not Yet Implemented (Phase 2)

- Inventory management UI
- Item pickup/use
- Tactical combat options
- NPC interactions
- Quest system
- Building interior exploration
- Multi-level dungeons
- Experience/leveling

## Confirmation

✅ **Yes, Phase 1 works with `cargo run`!**

The game is fully playable with:
- Dungeon exploration
- Combat with monsters
- Overmap travel
- Settlement discovery
- Time/weather simulation
- Save/load functionality

All 100 unit tests pass, and the integration is solid.

## Next Steps

After playing around:

1. Try exploring the overmap (Tab key)
2. Find and enter settlements
3. Watch day/night cycles change lighting
4. Camp to restore HP and advance time
5. Discover how weather affects gameplay
6. Try the Cosmic layer (Shift+L) - different reality!

Enjoy exploring the dungeon crawler! 🎮
