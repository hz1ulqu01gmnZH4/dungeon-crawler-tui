---
name: Debug Game by Playing
description: Debug the dungeon crawler TUI game by running it, playing through features, and identifying bugs or issues. Use this when the user wants to test the game, find bugs, or verify that features work correctly during actual gameplay.
allowed-tools: Bash, Read, Grep, Write
---

# Debug Game by Playing

This skill helps debug the dungeon crawler TUI game by running it in interactive mode and systematically testing features through actual gameplay.

## When to Use This Skill

- User asks to "test the game" or "play the game to find bugs"
- User wants to verify a feature works correctly by playing
- User reports a bug and wants you to reproduce it
- After implementing a new feature and want to verify it works
- User says "can you try playing the game to see if..."

## Prerequisites

Before running the game, ensure:
1. The game compiles: `cargo build --release`
2. All tests pass: `cargo test`
3. Save the current state if needed

## How to Debug Through Gameplay

### 1. Start the Game

Run the game in release mode for better performance:

```bash
cargo run --release
```

### 2. Systematic Testing Approach

Test features in this order:

#### A. Basic Movement & UI
- **Test**: hjkl or arrow keys for movement
- **Verify**:
  - Player moves correctly in all 8 directions
  - Camera follows player
  - FOV updates properly
  - No crashes or freezes
- **Common bugs**: Movement lag, camera drift, FOV not updating

#### B. Combat System
- **Test**: Move into a monster to attack
- **Verify**:
  - Damage calculation works
  - HP decreases correctly
  - Combat messages appear
  - Monster dies at 0 HP
  - Player death triggers game over
- **Common bugs**: Self-damage, incorrect damage formulas, death not triggering

#### C. Item System
- **Test**:
  - Press 'g' to pick up items
  - Press 'i' for inventory
  - Press 'd' to drop items
  - Press 'e' to equip items
  - Press 'u' to use consumables
- **Verify**:
  - Items added to inventory
  - Items removed from ground
  - Equipped items show bonuses
  - Consumables restore HP
  - Inventory capacity respected
- **Common bugs**: Duplicate pickups, items not disappearing, stat bonuses not applying

#### D. Overworld & Travel
- **Test**:
  - Press Tab to enter overmap
  - Move around overmap
  - Enter settlements/dungeons
  - Exit back to overmap
- **Verify**:
  - Overmap generates correctly
  - Settlements/POIs visible
  - Roads rendered
  - Can enter/exit locations
  - Time advances during travel
- **Common bugs**: Mode switching issues, generation failures, position sync

#### E. Dungeon Exploration
- **Test**:
  - Enter dungeon from overmap
  - Explore multiple floors
  - Use stairs (> to descend, < to ascend)
  - Find items and monsters
- **Verify**:
  - Levels generate properly
  - Stairs work correctly
  - Monsters spawn appropriately
  - Items spawn on floors
- **Common bugs**: Stairs not working, generation fails, empty levels

#### F. Save/Load System
- **Test**:
  - Press 'S' to save
  - Quit with 'q'
  - Restart and load game
- **Verify**:
  - Save completes successfully
  - Load restores all state
  - Player position correct
  - Inventory preserved
  - World state intact
- **Common bugs**: Save corruption, incomplete restoration, position reset

### 3. Report Findings

After testing, create a report with:

```markdown
## Gameplay Test Report

**Date**: [timestamp]
**Build**: [commit hash]
**Test Duration**: [minutes]

### ✅ Working Features
- Feature 1: Description of what works
- Feature 2: Description of what works

### ❌ Bugs Found
1. **Bug Name**: Description
   - **Steps to Reproduce**:
     1. Step 1
     2. Step 2
   - **Expected**: What should happen
   - **Actual**: What actually happens
   - **Severity**: Critical/High/Medium/Low

### 📝 Observations
- Performance: [smooth/laggy/stuttering]
- UI: [clear/confusing]
- Feel: [fun/tedious/broken]

### 🎯 Recommendations
- Fix X before Y
- Consider adding Z
```

## Testing Specific Features

### Movement Bug Testing
```bash
# Test player movement in all directions
# Verify:
# - No self-attack when moving to same position
# - No collision issues
# - No camera glitches
```

### Item Pickup Bug Testing
```bash
# Test item pickup
# 1. Drop an item
# 2. Pick it up with 'g'
# 3. Check inventory (i)
# 4. Try picking up again
# Verify: Item appears once, not duplicated
```

### Combat Bug Testing
```bash
# Test combat mechanics
# 1. Find a weak monster
# 2. Attack multiple times
# 3. Check HP values
# Verify: Damage calculation correct, no self-damage
```

## Quick Bug Hunt Procedure

If user reports a specific bug:

1. **Reproduce**: Follow user's exact steps
2. **Observe**: Watch what actually happens
3. **Compare**: Check against expected behavior
4. **Capture**: Note error messages, logs, state
5. **Isolate**: Find minimal reproduction steps
6. **Report**: Document findings clearly

## Automated Testing After Playing

After manual testing, run automated tests to verify:

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --tests

# Specific system
cargo test --lib combat
cargo test --lib inventory
cargo test --lib movement
```

## Known Issues to Watch For

Based on recent fixes:

- ✅ FIXED: Player attacking self on movement
- ✅ FIXED: Items can be picked up multiple times
- ⚠️ WATCH: Event queue not clearing properly
- ⚠️ WATCH: Stacking items edge cases
- ⚠️ WATCH: Map generation edge cases

## Tips for Effective Debugging

1. **Take Notes**: Write down every action and result
2. **Be Systematic**: Test one feature at a time
3. **Try Edge Cases**: Empty inventory, full inventory, zero HP
4. **Stress Test**: Spam keys, rapid actions
5. **Look for Patterns**: Does bug happen always or sometimes?
6. **Check Logs**: Look for error messages or warnings
7. **Compare Versions**: Does bug exist in older versions?

## Example Session

```
$ cargo run --release
[Game starts]

> Try hjkl movement - ✅ works
> Try picking up item - ✅ works
> Try picking up same item - ✅ correctly prevented (recent fix)
> Move into monster - ✅ combat works
> Check inventory - ✅ items listed
> Equip weapon - ✅ stats increase
> Tab to overmap - ✅ overmap renders
> Enter settlement - ✅ loads correctly
> Save game (S) - ✅ save successful
> Quit (q) and reload - ✅ state restored

VERDICT: All core features working correctly
```

## Emergency Debugging

If the game crashes or hangs:

1. **Check build logs**: Compilation warnings might hint at issues
2. **Run with backtrace**: `RUST_BACKTRACE=1 cargo run`
3. **Check for panics**: Look for `thread panicked at` messages
4. **Test in debug mode**: `cargo run` (not --release) for better error info
5. **Inspect recent changes**: `git diff HEAD~1` to see what changed

## Reporting to User

Always provide:
- Clear summary (working/broken)
- Reproduction steps for any bugs
- Severity assessment
- Suggested next steps

Keep testing focused and time-boxed (5-15 minutes typically).
