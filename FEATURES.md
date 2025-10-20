# Dungeon Crawler TUI - Feature Roadmap

## Current State

**Architecture Score**: 9.0/10
**Test Coverage**: 199 tests passing
**Phase**: Refactoring Complete (Phases 0-5)

### Implemented Systems
- ✅ Event-driven architecture with GameEvent system
- ✅ ECS with hecs framework
- ✅ Typed resource management (Sim, World, UI, Player)
- ✅ MapCache with unified map storage
- ✅ Save/load with compression (97% size reduction)
- ✅ Reality layer system (Normal/Cosmic)
- ✅ FOV and visibility system
- ✅ Basic monster AI (chase player)
- ✅ Inventory and equipment system
- ✅ Combat system with stats
- ✅ Dungeon generation with rooms and corridors
- ✅ Overworld navigation
- ✅ Item pickup and usage
- ✅ Stairs and multi-level dungeons

---

## Feature Categories

### 🎯 Phase 6: Core Gameplay Enhancement

#### 6.1 Experience & Leveling System
**Priority**: HIGH
**Complexity**: Medium
**Dependencies**: None

- [ ] XP component for player
- [ ] XP gain from killing monsters
- [ ] Level progression algorithm (exponential curve)
- [ ] Stat increases on level up (HP, Power, Defense)
- [ ] Level-up UI notification
- [ ] Monster XP values based on difficulty
- [ ] XP bar display in UI

**Estimated Implementation**: 2-3 hours
**Test Coverage Target**: +15 tests

#### 6.2 Enhanced Combat System
**Priority**: HIGH
**Complexity**: Medium
**Dependencies**: 6.1 (optional)

- [ ] Critical hit system (chance-based)
- [ ] Damage variance/randomization
- [ ] Status effects (poison, stun, burn)
- [ ] Damage types (physical, fire, ice, etc.)
- [ ] Armor penetration mechanics
- [ ] Weapon attack speed/cooldowns
- [ ] Combat log improvements

**Estimated Implementation**: 3-4 hours
**Test Coverage Target**: +20 tests

#### 6.3 Expanded Inventory System
**Priority**: MEDIUM
**Complexity**: Low
**Dependencies**: None

- [ ] Item stacking for consumables
- [ ] Item sorting (by type, name, value)
- [ ] Item filtering in UI
- [ ] Quick-use item slots (1-9 keys)
- [ ] Item comparison tooltip
- [ ] Item rarity system (common, rare, epic, legendary)
- [ ] Colored item names by rarity

**Estimated Implementation**: 2 hours
**Test Coverage Target**: +10 tests

---

### 🎲 Phase 7: Content Systems

#### 7.1 Magic/Spell System
**Priority**: HIGH
**Complexity**: High
**Dependencies**: 6.2 (status effects)

- [ ] Mana/Energy component
- [ ] Spell data structure (cost, power, effect)
- [ ] Spell book/grimoire system
- [ ] Spell casting input (m + direction or target)
- [ ] Spell types:
  - [ ] Direct damage (fireball, lightning bolt)
  - [ ] Healing (heal self, regeneration)
  - [ ] Utility (teleport, light, detect monsters)
  - [ ] Area of effect spells
  - [ ] Buff/debuff spells
- [ ] Mana regeneration over time
- [ ] Spell scrolls as consumable items
- [ ] Spell learning/discovery system

**Estimated Implementation**: 6-8 hours
**Test Coverage Target**: +30 tests

#### 7.2 Crafting System
**Priority**: MEDIUM
**Complexity**: High
**Dependencies**: 6.3 (inventory improvements)

- [ ] Recipe data structure
- [ ] Material items (ore, wood, leather, etc.)
- [ ] Crafting stations (forge, workbench, alchemy table)
- [ ] Crafting UI with recipe list
- [ ] Recipe discovery system
- [ ] Item enhancement/upgrading
- [ ] Disassembly of items for materials
- [ ] Crafting skill levels

**Estimated Implementation**: 8-10 hours
**Test Coverage Target**: +25 tests

#### 7.3 Quest System
**Priority**: HIGH
**Complexity**: High
**Dependencies**: 7.5 (NPC system recommended)

- [ ] Quest data structure (objectives, rewards, description)
- [ ] Quest types:
  - [ ] Kill quests (defeat X monsters)
  - [ ] Fetch quests (retrieve item)
  - [ ] Explore quests (discover location)
  - [ ] Escort quests (protect NPC)
  - [ ] Delivery quests (bring item to NPC)
- [ ] Quest log UI
- [ ] Quest tracking in HUD
- [ ] Objective completion detection
- [ ] Quest rewards (XP, gold, items)
- [ ] Quest chains (prerequisites)
- [ ] Quest markers on map

**Estimated Implementation**: 10-12 hours
**Test Coverage Target**: +35 tests

#### 7.4 Procedural Quest Generation
**Priority**: MEDIUM
**Complexity**: Very High
**Dependencies**: 7.3 (quest system)

- [ ] Quest generator algorithm
- [ ] Dynamic objective placement based on dungeon layout
- [ ] Randomized quest rewards scaled to difficulty
- [ ] Quest narrative generation (simple templates)
- [ ] Difficulty scaling system
- [ ] Quest variety balancing
- [ ] Replayability improvements

**Estimated Implementation**: 12-15 hours
**Test Coverage Target**: +30 tests

#### 7.5 NPC System
**Priority**: MEDIUM
**Complexity**: High
**Dependencies**: None

- [ ] NPC entity type
- [ ] NPC spawning in settlements
- [ ] Dialogue system:
  - [ ] Conversation tree data structure
  - [ ] Dialogue UI
  - [ ] Choice-based responses
  - [ ] Dialogue prerequisites (quests, items)
- [ ] NPC personalities/flavors
- [ ] Trading/merchant NPCs
- [ ] Quest-giving NPCs
- [ ] Named NPCs with unique dialogues
- [ ] Friendly vs hostile NPCs

**Estimated Implementation**: 8-10 hours
**Test Coverage Target**: +25 tests

---

### 🧠 Phase 8: AI & Behaviors

#### 8.1 Advanced Monster AI
**Priority**: MEDIUM
**Complexity**: Medium
**Dependencies**: None

- [ ] AI personality types:
  - [ ] Aggressive (always chase)
  - [ ] Cautious (flee when low health)
  - [ ] Defensive (guard area)
  - [ ] Pack hunter (coordinate with allies)
  - [ ] Ranged (maintain distance)
- [ ] Patrolling behavior with waypoints
- [ ] Idle animations/behaviors
- [ ] Alert system (noise, sight)
- [ ] Memory of player last position
- [ ] Group tactics and coordination
- [ ] AI difficulty settings

**Estimated Implementation**: 6-8 hours
**Test Coverage Target**: +20 tests

#### 8.2 NPC AI & Schedules
**Priority**: LOW
**Complexity**: Medium
**Dependencies**: 7.5 (NPC system)

- [ ] Daily schedules for NPCs
- [ ] Movement between locations
- [ ] Work/sleep cycles
- [ ] Dynamic dialogue based on time
- [ ] NPC relationships/factions
- [ ] Reactive behavior to events

**Estimated Implementation**: 6-8 hours
**Test Coverage Target**: +15 tests

---

### 🗺️ Phase 9: World Building

#### 9.1 Enhanced Dungeon Generation
**Priority**: MEDIUM
**Complexity**: High
**Dependencies**: None

- [ ] Multiple generation algorithms:
  - [ ] BSP tree dungeons
  - [ ] Cellular automata caves
  - [ ] Maze-like corridors
  - [ ] Open arena rooms
- [ ] Themed dungeon types (crypt, mine, temple, etc.)
- [ ] Special room types:
  - [ ] Treasure rooms
  - [ ] Boss arenas
  - [ ] Puzzle rooms
  - [ ] Trap rooms
- [ ] Environmental hazards (lava, water, chasms)
- [ ] Locked doors and keys
- [ ] Secret passages
- [ ] Dungeon modifiers (flooded, cursed, etc.)

**Estimated Implementation**: 12-15 hours
**Test Coverage Target**: +30 tests

#### 9.2 Overworld Enhancement
**Priority**: LOW
**Complexity**: Medium
**Dependencies**: None

- [ ] Biome system (forest, desert, mountains, swamp)
- [ ] Weather system (rain, snow, fog)
- [ ] Day/night cycle
- [ ] Random encounters on overworld
- [ ] Points of interest generation
- [ ] Roads and paths between locations
- [ ] Overworld map reveal system
- [ ] Fast travel system

**Estimated Implementation**: 8-10 hours
**Test Coverage Target**: +20 tests

#### 9.3 Settlement Generation
**Priority**: MEDIUM
**Complexity**: High
**Dependencies**: 7.5 (NPC system)

- [ ] Building generation (houses, shops, taverns)
- [ ] NPC population in settlements
- [ ] Shop system (buy/sell items)
- [ ] Inn/rest system (pay for healing)
- [ ] Settlement types (village, town, city)
- [ ] Settlement services (blacksmith, alchemist, etc.)
- [ ] Settlement reputation system

**Estimated Implementation**: 10-12 hours
**Test Coverage Target**: +25 tests

---

### ⚔️ Phase 10: Advanced Features

#### 10.1 Boss Encounters
**Priority**: MEDIUM
**Complexity**: High
**Dependencies**: 6.2, 8.1

- [ ] Boss entity type with unique stats
- [ ] Boss-specific AI patterns
- [ ] Multi-phase boss fights
- [ ] Boss special abilities/attacks
- [ ] Boss arena generation
- [ ] Unique boss loot tables
- [ ] Boss telegraphing (attack indicators)
- [ ] Victory celebrations/rewards

**Estimated Implementation**: 8-10 hours
**Test Coverage Target**: +20 tests

#### 10.2 Trap System
**Priority**: LOW
**Complexity**: Medium
**Dependencies**: None

- [ ] Trap component and types:
  - [ ] Spike trap (damage)
  - [ ] Pit trap (fall damage)
  - [ ] Arrow trap (ranged damage)
  - [ ] Magic trap (status effects)
  - [ ] Teleport trap (displacement)
- [ ] Trap detection system (perception check)
- [ ] Trap disarming mechanics
- [ ] Trap placement in dungeon generation
- [ ] Trap visual indicators (subtle vs obvious)

**Estimated Implementation**: 4-6 hours
**Test Coverage Target**: +15 tests

#### 10.3 Achievements & Statistics
**Priority**: LOW
**Complexity**: Low
**Dependencies**: None

- [ ] Achievement data structure
- [ ] Achievement tracking system
- [ ] Achievement types:
  - [ ] Kill milestones
  - [ ] Depth milestones
  - [ ] Collection achievements
  - [ ] Special action achievements
- [ ] Statistics tracking (kills, deaths, time played, etc.)
- [ ] Statistics screen UI
- [ ] Achievement notifications
- [ ] Persistent achievement storage

**Estimated Implementation**: 4-5 hours
**Test Coverage Target**: +10 tests

#### 10.4 Permadeath & Difficulty Modes
**Priority**: LOW
**Complexity**: Low
**Dependencies**: None

- [ ] Game mode selection (normal, hardcore, easy)
- [ ] Permadeath implementation
- [ ] Death screen with statistics
- [ ] Difficulty scaling:
  - [ ] Monster stat multipliers
  - [ ] Loot frequency adjustments
  - [ ] Trap density
- [ ] Ironman mode (no save/load)
- [ ] Score calculation system

**Estimated Implementation**: 3-4 hours
**Test Coverage Target**: +10 tests

---

### 🎨 Phase 11: Polish & UX

#### 11.1 Enhanced UI/UX
**Priority**: MEDIUM
**Complexity**: Medium
**Dependencies**: None

- [ ] Improved tooltips with more information
- [ ] Minimap display
- [ ] Status effect indicators
- [ ] Floating damage numbers
- [ ] Smooth scrolling message log
- [ ] Color-coded messages by severity
- [ ] Contextual help system
- [ ] Tutorial mode for new players
- [ ] Keybinding customization

**Estimated Implementation**: 6-8 hours
**Test Coverage Target**: +15 tests

#### 11.2 Audio System (Optional)
**Priority**: LOW
**Complexity**: High
**Dependencies**: External audio library

- [ ] Sound effect system
- [ ] Background music
- [ ] Ambient sounds
- [ ] Volume controls
- [ ] Audio settings

**Estimated Implementation**: 10-12 hours
**Test Coverage Target**: +10 tests

#### 11.3 Animation System
**Priority**: LOW
**Complexity**: Medium
**Dependencies**: None

- [ ] ASCII animation framework
- [ ] Attack animations
- [ ] Movement animations
- [ ] Spell effect animations
- [ ] Particle effects (ASCII-based)
- [ ] Animation timing system

**Estimated Implementation**: 8-10 hours
**Test Coverage Target**: +15 tests

---

### 🔧 Phase 12: Technical Improvements

#### 12.1 Performance Optimization
**Priority**: LOW
**Complexity**: Medium
**Dependencies**: None

- [ ] Benchmark suite
- [ ] Profiling integration
- [ ] Spatial partitioning for entity queries
- [ ] Lazy loading for map chunks
- [ ] Memory usage optimization
- [ ] Render caching

**Estimated Implementation**: 6-8 hours
**Test Coverage Target**: +10 tests

#### 12.2 Modding Support
**Priority**: LOW
**Complexity**: Very High
**Dependencies**: All systems stable

- [ ] Plugin API design
- [ ] Hot-reloading system
- [ ] Data-driven content (JSON/TOML)
- [ ] Custom monster definitions
- [ ] Custom item definitions
- [ ] Custom map generation scripts
- [ ] Mod loading system
- [ ] Mod compatibility checking

**Estimated Implementation**: 20-25 hours
**Test Coverage Target**: +30 tests

#### 12.3 Multiplayer Foundation (Future)
**Priority**: VERY LOW
**Complexity**: Very High
**Dependencies**: All systems refactored

- [ ] Network synchronization design
- [ ] Client/server architecture
- [ ] Event replication system
- [ ] Player-to-player interaction
- [ ] Shared world state
- [ ] Anti-cheat measures

**Estimated Implementation**: 40-60 hours
**Test Coverage Target**: +50 tests

---

## Recommended Implementation Order

### Short-term (Next Sprint)
1. **6.1 Experience & Leveling** - Adds immediate progression satisfaction
2. **6.2 Enhanced Combat** - Makes combat more interesting
3. **6.3 Expanded Inventory** - Quality of life improvements

### Mid-term (2-4 Weeks)
4. **7.1 Magic System** - Major new gameplay mechanic
5. **7.3 Quest System** - Provides goals and structure
6. **7.5 NPC System** - Brings world to life
7. **8.1 Advanced Monster AI** - Makes encounters more dynamic

### Long-term (1-2 Months)
8. **9.1 Enhanced Dungeon Generation** - Increases variety
9. **9.3 Settlement Generation** - Expands safe zones
10. **10.1 Boss Encounters** - Adds climactic challenges
11. **7.2 Crafting System** - Depth for item management
12. **11.1 Enhanced UI/UX** - Polish and accessibility

### Optional/Future
- 7.4 Procedural Quest Generation
- 9.2 Overworld Enhancement
- 10.2 Trap System
- 10.3 Achievements
- 12.1 Performance Optimization
- 12.2 Modding Support
- 12.3 Multiplayer

---

## Feature Metrics Summary

**Total Features**: 27
**High Priority**: 6
**Medium Priority**: 11
**Low Priority**: 10

**Estimated Total Implementation Time**: 200-270 hours
**Estimated Total Tests**: +450 tests (reaching ~650 total)

---

## Notes

- All features should maintain 9/10+ architecture score
- Each feature must include comprehensive tests
- Follow event-driven architecture patterns
- Maintain typed resource separation
- Document new systems in code
- Update this roadmap as features are completed

---

**Last Updated**: 2025-10-20
**Current Phase**: Ready for Phase 6
