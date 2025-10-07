# The Unraveling of Kándavael - Implementation Tasks & Progress

**Project Type**: Open World Cosmic Horror Roguelike
**Target Platform**: Linux, Windows, macOS (Terminal)
**Tech Stack**: Rust, Ratatui, Hecs ECS
**Development Start**: 2025-10-06
**Current Phase**: Foundation

---

## Progress Overview

| Phase | Status | Progress | Est. Duration | Actual Duration |
|-------|--------|----------|---------------|-----------------|
| Phase 0: Current State | ✅ Complete | 100% | - | - |
| Phase 1: Open World Foundation | 🔴 Not Started | 0% | 8-12 weeks | - |
| Phase 2: Content Systems | 🔴 Not Started | 0% | 6-8 weeks | - |
| Phase 3: Survival & Base Building | 🔴 Not Started | 0% | 4-6 weeks | - |
| Phase 4: World Simulation | 🔴 Not Started | 0% | 6-8 weeks | - |
| Phase 5: Polish & Content | 🔴 Not Started | 0% | 8-12 weeks | - |

**Legend**: ✅ Complete | 🟢 In Progress | 🟡 Blocked | 🔴 Not Started

---

## Phase 0: Current State (Baseline)

### ✅ Completed Features

#### Core Systems
- [x] **ECS Architecture** (hecs)
  - Components: Position, Renderable, CombatStats, Player, Monster
  - Resources: Map, Camera, RunMode, GameLog
  - Systems: Movement, Combat, AI, FOV, Input

- [x] **Basic Game Loop**
  - Turn-based execution
  - Player turn → Monster turn alternation
  - Input handling with crossterm
  - Terminal rendering with ratatui

- [x] **Map & Generation**
  - Procedural dungeon generation (rooms + corridors)
  - Single floor dungeons
  - Wall/floor tiles
  - Collision detection

- [x] **Combat System**
  - Melee combat with damage calculation
  - HP tracking
  - Death detection
  - Combat log messages

- [x] **AI & Monsters**
  - Simple chase AI
  - Monster types: Goblin, Orc, Troll
  - Vision-based detection
  - Basic pathfinding (manhattan distance)

- [x] **Field of View**
  - Ray-casting FOV algorithm
  - Fog of war
  - Explored/visible tile tracking
  - Camera following player

- [x] **UI/UX**
  - Map rendering
  - Status bar (HP, position, meters)
  - Message log
  - Game over screen

#### Partial Systems (Architecture Only)
- [x] **Tri-Meter System** (UI only, not functional)
  - Insight, Sanity, Notice displayed
  - No gameplay effects yet

- [x] **Dual-Layer Reality** (Structure only)
  - RealityLayer enum defined
  - MapSet with Normal/Cosmic maps
  - No layer switching implemented

### 📊 Code Statistics
- **Total Lines**: ~2,500
- **Modules**: 8 (ecs, systems, map, ui, game)
- **Components**: 11
- **Systems**: 6
- **Entities**: Player + 3 monster types

---

## Phase 1: Open World Foundation (8-12 weeks)

**Goal**: Transform from single-dungeon crawler to explorable overworld

### Week 1-2: Overmap System

#### Task 1.1: Overmap Data Structure
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: None

**Subtasks**:
- [ ] Create `OvermapTile` struct
  - [ ] Position (world coordinates)
  - [ ] Terrain type enum
  - [ ] Discovered/visited flags
  - [ ] Corruption level
  - [ ] Location reference
- [ ] Create `Overmap` struct
  - [ ] 2D grid of tiles (50x50 for MVP, 200x200 for full)
  - [ ] Tile access methods
  - [ ] Bounds checking
- [ ] Add to Resources
- [ ] Unit tests for overmap operations

**Files to Create**:
- `src/world/mod.rs`
- `src/world/overmap.rs`
- `src/world/terrain.rs`

**Success Criteria**:
- Can create 50x50 overmap
- Can read/write tiles
- Tests pass

---

#### Task 1.2: Terrain Generation
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.1

**Subtasks**:
- [ ] Implement noise-based terrain generation
  - [ ] Perlin/Simplex noise for elevation
  - [ ] Moisture map generation
  - [ ] Temperature map generation
- [ ] Terrain type assignment
  - [ ] Plains (low elevation, medium moisture)
  - [ ] Forest (medium elevation, high moisture)
  - [ ] Mountains (high elevation)
  - [ ] Rivers (drainage paths)
  - [ ] Lakes (low areas)
- [ ] Biome boundaries
- [ ] Smooth transitions between terrain types
- [ ] Seed-based deterministic generation

**Files to Modify/Create**:
- `src/world/terrain.rs`
- `src/world/generator.rs`

**Success Criteria**:
- Generated overmap looks natural
- Same seed produces same map
- Rivers flow downhill
- Biomes make sense geographically

---

#### Task 1.3: Road Network Generation
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: Task 1.2

**Subtasks**:
- [ ] Implement A* pathfinding for roads
- [ ] Generate roads between major settlements
- [ ] Road quality levels (king road, trade route, path)
- [ ] Cost function considers terrain
- [ ] Visual representation on overmap

**Files to Create**:
- `src/world/roads.rs`

**Success Criteria**:
- Roads connect settlements
- Prefer flat terrain
- Avoid mountains when possible
- Look natural

---

#### Task 1.4: Overmap Rendering
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: Task 1.1, 1.2

**Subtasks**:
- [ ] Create overmap renderer
  - [ ] ASCII symbols for terrain
  - [ ] Color coding
  - [ ] Player position indicator
  - [ ] Legend/key
- [ ] Viewport system (can't show 200x200 at once)
- [ ] Mini-map in corner
- [ ] Toggle between overmap and local map (Tab key)

**Files to Create**:
- `src/ui/overmap_renderer.rs`

**Success Criteria**:
- Can see overmap
- Player position clear
- Can scroll/pan
- Smooth transitions

---

### Week 3-4: Settlement System

#### Task 1.5: Settlement Placement
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.2

**Subtasks**:
- [ ] Settlement placement algorithm
  - [ ] City placement (1-3 in 50x50)
  - [ ] Town placement (3-5)
  - [ ] Village placement (10-15)
  - [ ] Distance requirements
  - [ ] Terrain suitability
- [ ] `Settlement` struct
  - [ ] Name generation
  - [ ] Size/type
  - [ ] Population
  - [ ] Features
- [ ] Settlement data storage

**Files to Create**:
- `src/world/settlement.rs`
- `src/world/names.rs` (name generator)

**Success Criteria**:
- Settlements placed logically
- Not too close together
- Names are reasonable
- Population scales with size

---

#### Task 1.6: Basic Building Generation
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 1.5

**Subtasks**:
- [ ] Building types enum
  - [ ] House, Inn, Shop, Temple, etc.
- [ ] Building interior generation
  - [ ] Simple rectangular rooms
  - [ ] Furniture placement
  - [ ] Doors and windows
- [ ] Building data structure
- [ ] Link buildings to settlements

**Files to Create**:
- `src/world/building.rs`
- `src/map/building_gen.rs`

**Success Criteria**:
- Can generate 5+ building types
- Interiors are navigable
- Buildings have appropriate furniture
- Can enter/exit buildings

---

#### Task 1.7: Dungeon Placement
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: Task 1.2

**Subtasks**:
- [ ] Scatter dungeons on overmap
  - [ ] Prefer remote areas
  - [ ] Distance from settlements
  - [ ] Terrain considerations
- [ ] Dungeon entrance markers
- [ ] Connect to existing dungeon generator

**Files to Modify**:
- `src/world/generator.rs`
- `src/map/dungeon.rs`

**Success Criteria**:
- 5-10 dungeons per overmap
- Placed in reasonable locations
- Can enter from overmap
- Existing dungeon gen works

---

### Week 5-6: Travel System

#### Task 1.8: Overworld Movement
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.4

**Subtasks**:
- [ ] Player movement on overmap
  - [ ] Arrow keys/vim keys
  - [ ] Collision with impassable terrain
  - [ ] Movement cost by terrain
- [ ] Time progression during travel
  - [ ] 1 hour per tile
  - [ ] Day/night cycle
  - [ ] Update world time
- [ ] Stamina/fatigue from travel

**Files to Create**:
- `src/systems/travel.rs`
- `src/systems/time.rs`

**Success Criteria**:
- Can move on overmap
- Time passes correctly
- Mountains slow you down
- Roads speed you up

---

#### Task 1.9: Transition System
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.8, 1.6

**Subtasks**:
- [ ] Enter location from overmap
  - [ ] Press Enter to enter
  - [ ] Load/generate location
  - [ ] Spawn player at entrance
- [ ] Exit location to overmap
  - [ ] Special exit tiles
  - [ ] Return to overmap position
  - [ ] Save location state
- [ ] Smooth camera transitions

**Files to Create**:
- `src/systems/transition.rs`

**Success Criteria**:
- Can enter settlements
- Can enter dungeons
- Can exit back to overmap
- No crashes or data loss

---

#### Task 1.10: Random Encounters
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.8

**Subtasks**:
- [ ] Encounter trigger system
  - [ ] Roll for encounter each tile
  - [ ] Chance modified by terrain
  - [ ] Chance modified by time of day
- [ ] Encounter types
  - [ ] Travelers (friendly)
  - [ ] Bandits (hostile)
  - [ ] Wild animals
  - [ ] Strange events
- [ ] Mini-battle maps for encounters
- [ ] Escape option

**Files to Create**:
- `src/world/encounters.rs`

**Success Criteria**:
- Encounters happen occasionally
- Different types
- Can fight or flee
- Rewards for winning

---

### Week 7-8: Time & Basic World State

#### Task 1.11: Time System
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: Task 1.8

**Subtasks**:
- [ ] `WorldTime` struct
  - [ ] Year, season, day, hour
  - [ ] Progression logic
  - [ ] Calendar system
- [ ] Time display in UI
- [ ] Actions consume time
  - [ ] Travel: 1h per tile
  - [ ] Combat: 5 minutes
  - [ ] Crafting: varies
  - [ ] Rest: 8 hours

**Files to Create**:
- `src/world/time.rs`

**Success Criteria**:
- Time advances correctly
- Days turn to months
- UI shows current time
- Time affects gameplay

---

#### Task 1.12: Day/Night Cycle
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: Task 1.11

**Subtasks**:
- [ ] Time of day effects
  - [ ] Visibility changes
  - [ ] NPC activity changes
  - [ ] Monster spawn changes
- [ ] Lighting system
  - [ ] Torches/light sources
  - [ ] Darkness penalties
- [ ] Night rendering (darker colors)

**Files to Create**:
- `src/systems/lighting.rs`

**Success Criteria**:
- Night is darker
- NPCs sleep at night
- Monsters more active at night
- Can use light sources

---

#### Task 1.13: Basic Weather
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.11

**Subtasks**:
- [ ] Weather types (clear, rain, storm, fog)
- [ ] Weather generation
  - [ ] Random with patterns
  - [ ] Regional variation
  - [ ] Seasonal influence
- [ ] Weather effects
  - [ ] Visibility changes
  - [ ] Travel speed changes
  - [ ] Visual indicators
- [ ] Weather display in UI

**Files to Create**:
- `src/world/weather.rs`

**Success Criteria**:
- Weather changes over time
- Affects visibility
- Displayed clearly
- Feels natural

---

#### Task 1.14: Chunk Loading System
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 1.9

**Subtasks**:
- [ ] Chunk system
  - [ ] Define chunk size (24x24 tiles)
  - [ ] Chunk coordinates
  - [ ] Chunk data structure
- [ ] Loading/unloading
  - [ ] Load chunks in radius
  - [ ] Unload distant chunks
  - [ ] Save modified chunks
- [ ] LRU cache for chunks
- [ ] Async loading (optional)

**Files to Create**:
- `src/world/chunks.rs`

**Success Criteria**:
- Only nearby chunks loaded
- Seamless loading
- No memory leaks
- Performance acceptable

---

### Week 9-10: Save System & Polish

#### Task 1.15: Enhanced Save System
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 1.14

**Subtasks**:
- [ ] Save overmap state
- [ ] Save all locations (or chunks)
- [ ] Save world time
- [ ] Save player position (overmap + local)
- [ ] Save discovered/visited flags
- [ ] Compression for large saves
- [ ] Multiple save slots
- [ ] Auto-save system

**Files to Create**:
- `src/save/world_save.rs`
- `src/save/compression.rs`

**Success Criteria**:
- Can save entire world state
- Can load and continue
- Save file reasonable size
- No corruption

---

#### Task 1.16: Map & Navigation UI
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.4

**Subtasks**:
- [ ] Full-screen map view (M key)
- [ ] Zoom in/out
- [ ] Player markers
- [ ] Quest markers
- [ ] Custom waypoints
- [ ] Distance calculator
- [ ] Legend

**Files to Create**:
- `src/ui/map_screen.rs`

**Success Criteria**:
- Can view full map
- Clear navigation
- Can set waypoints
- Useful for planning

---

#### Task 1.17: Performance Optimization
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: All Week 1-9 tasks

**Subtasks**:
- [ ] Profile rendering
- [ ] Profile world simulation
- [ ] Optimize hot paths
- [ ] Reduce allocations
- [ ] Entity culling
- [ ] Target 60 FPS

**Success Criteria**:
- Smooth gameplay
- No stuttering
- Low memory usage
- Fast save/load

---

#### Task 1.18: Testing & Bug Fixes
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: All previous tasks

**Subtasks**:
- [ ] Unit tests for all systems
- [ ] Integration tests
- [ ] Playtesting
  - [ ] Generate multiple worlds
  - [ ] Travel between locations
  - [ ] Enter/exit buildings
  - [ ] Save/load cycles
- [ ] Fix all critical bugs
- [ ] Fix major bugs

**Success Criteria**:
- No crashes
- No game-breaking bugs
- Smooth experience
- All tests pass

---

## Phase 2: Content Systems (6-8 weeks)

**Goal**: Add NPCs, factions, quests, and dynamic events

### Week 11-12: NPC Foundation

#### Task 2.1: NPC Data Structure
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Phase 1 complete

**Subtasks**:
- [ ] `NPC` struct
  - [ ] Name, profession, home
  - [ ] Stats, inventory
  - [ ] AI state
  - [ ] Relationship tracking
- [ ] NPC spawning
  - [ ] Spawn in settlements
  - [ ] Population based on size
  - [ ] Profession distribution
- [ ] NPC storage and management

**Files to Create**:
- `src/npc/mod.rs`
- `src/npc/npc.rs`
- `src/npc/profession.rs`

**Success Criteria**:
- NPCs spawn in towns
- Have names and professions
- Can be interacted with
- Save/load correctly

---

#### Task 2.2: Basic NPC AI & Schedules
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 2.1

**Subtasks**:
- [ ] Daily schedule system
  - [ ] Sleep (night)
  - [ ] Work (day)
  - [ ] Eat (meals)
  - [ ] Socialize (evening)
- [ ] Pathfinding for NPCs
- [ ] Activity locations
- [ ] NPC movement simulation

**Files to Create**:
- `src/npc/schedule.rs`
- `src/npc/ai.rs`

**Success Criteria**:
- NPCs follow schedules
- Move between locations
- Sleep at night
- Work during day

---

#### Task 2.3: NPC Dialogue System
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 2.2

**Subtasks**:
- [ ] Dialogue tree structure
- [ ] Dialogue UI
  - [ ] Text display
  - [ ] Choice menu
  - [ ] Speaker name
- [ ] Context-aware responses
  - [ ] Based on profession
  - [ ] Based on relationship
  - [ ] Based on world state
- [ ] Dialogue data files (JSON)

**Files to Create**:
- `src/npc/dialogue.rs`
- `src/ui/dialogue_ui.rs`
- `data/dialogue/`

**Success Criteria**:
- Can talk to NPCs
- Responses make sense
- UI is clear
- Multiple dialogue paths

---

### Week 13-14: Faction System

#### Task 2.4: Faction Data Structure
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.1

**Subtasks**:
- [ ] `Faction` struct
  - [ ] Name, type, goals
  - [ ] Territory
  - [ ] Power level
  - [ ] Relationships with other factions
  - [ ] Relationship with player
- [ ] Define major factions
  - [ ] Royal Crown
  - [ ] Holy Church
  - [ ] Wardens Order
  - [ ] The Stitchers
  - [ ] Cult of Dæl
  - [ ] Merchants Guild
- [ ] Faction initialization

**Files to Create**:
- `src/faction/mod.rs`
- `src/faction/faction.rs`
- `data/factions.json`

**Success Criteria**:
- 6+ factions defined
- Each has unique identity
- Relationships make sense
- Data-driven

---

#### Task 2.5: Faction Territory System
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.4

**Subtasks**:
- [ ] Assign settlements to factions
- [ ] Territory visualization on overmap
  - [ ] Color coding
  - [ ] Borders
  - [ ] Contested zones
- [ ] Territory influence calculation
- [ ] Dynamic territory changes

**Files to Create**:
- `src/faction/territory.rs`

**Success Criteria**:
- Clear faction territories
- Visualized on map
- Can change over time
- Makes strategic sense

---

#### Task 2.6: Reputation System
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.4

**Subtasks**:
- [ ] Reputation tracking per faction
- [ ] Actions affect reputation
  - [ ] Quest completion
  - [ ] Killing faction members
  - [ ] Trading
  - [ ] Dialogue choices
- [ ] Reputation levels
  - [ ] Hated → Hostile → Neutral → Friendly → Ally
- [ ] Reputation effects
  - [ ] Shop prices
  - [ ] Quest availability
  - [ ] NPC reactions
  - [ ] Territory access

**Files to Create**:
- `src/faction/reputation.rs`

**Success Criteria**:
- Actions affect reputation
- Clear feedback
- Meaningful consequences
- Can ally with factions

---

### Week 15-16: Quest System

#### Task 2.7: Quest Data Structure
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.3

**Subtasks**:
- [ ] `Quest` struct
  - [ ] ID, title, description
  - [ ] Giver NPC
  - [ ] Objectives
  - [ ] Rewards
  - [ ] Stage tracking
- [ ] Quest types
  - [ ] Kill target
  - [ ] Fetch item
  - [ ] Escort NPC
  - [ ] Investigate location
  - [ ] Delivery
- [ ] Quest journal UI

**Files to Create**:
- `src/quest/mod.rs`
- `src/quest/quest.rs`
- `src/ui/quest_journal.rs`

**Success Criteria**:
- Can receive quests
- Track active quests
- View in journal
- Complete quests

---

#### Task 2.8: Dynamic Quest Generation
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 2.7

**Subtasks**:
- [ ] Quest generator
  - [ ] Bounty quests (kill monster)
  - [ ] Delivery quests (transport item)
  - [ ] Escort quests (protect NPC)
  - [ ] Investigation quests (find clues)
- [ ] Context-aware generation
  - [ ] Based on location
  - [ ] Based on faction
  - [ ] Based on world state
- [ ] Reward calculation
  - [ ] Scale with difficulty
  - [ ] Include gold, items, reputation

**Files to Create**:
- `src/quest/generator.rs`

**Success Criteria**:
- Quests generate dynamically
- Feel unique
- Rewards appropriate
- Always available

---

#### Task 2.9: Main Quest Line
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 5 days
**Dependencies**: Task 2.7

**Subtasks**:
- [ ] Design main quest stages
  - [ ] Act 1: Hero's Welcome (5 quests)
  - [ ] Act 2: Growing Doubts (7 quests)
  - [ ] Act 3: Horrifying Truth (5 quests)
  - [ ] Act 4: Desperate Measures (5 quests)
  - [ ] Act 5: Endings (3 paths)
- [ ] Progressive revelation system
- [ ] Quest scripting
- [ ] Special encounters
- [ ] Narrative integration

**Files to Create**:
- `src/quest/main_quest.rs`
- `data/quests/main_story.json`

**Success Criteria**:
- 25+ story quests
- Clear narrative arc
- Multiple endings
- Emotional impact

---

### Week 17-18: Dynamic Events

#### Task 2.10: World Event System
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.4

**Subtasks**:
- [ ] Event types
  - [ ] Faction warfare
  - [ ] Monster raid
  - [ ] Festival
  - [ ] Plague
  - [ ] Reality storm
  - [ ] Cult ritual
- [ ] Event triggers
  - [ ] Time-based
  - [ ] Player action-based
  - [ ] Random
- [ ] Event consequences
  - [ ] Settlement changes
  - [ ] NPC changes
  - [ ] Faction changes

**Files to Create**:
- `src/world/events.rs`

**Success Criteria**:
- Events happen dynamically
- Affect world state
- Player can interact
- Create emergent stories

---

#### Task 2.11: Corruption Spreading System
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 2.10

**Subtasks**:
- [ ] Corruption tracking per region
- [ ] Player actions increase corruption
  - [ ] Light beacons
  - [ ] Defeat wardens
  - [ ] Perform rituals
  - [ ] Use lexemes
- [ ] Corruption spread algorithm
  - [ ] Adjacent regions
  - [ ] Over time
  - [ ] Accelerating
- [ ] Corruption effects per level
  - [ ] 10%: Whispers, nightmares
  - [ ] 30%: Visible changes
  - [ ] 60%: Reality collapse
  - [ ] 100%: Void consumed

**Files to Create**:
- `src/world/corruption.rs`

**Success Criteria**:
- Corruption spreads visibly
- Player actions matter
- Progressive transformation
- Affects gameplay

---

#### Task 2.12: Regional Transformation
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 2.11

**Subtasks**:
- [ ] Terrain changes with corruption
  - [ ] Normal → Corrupted variants
  - [ ] Visual changes
  - [ ] Mechanical changes
- [ ] Settlement transformation
  - [ ] Buildings change
  - [ ] NPCs affected
  - [ ] Faction control shifts
- [ ] Monster changes
  - [ ] New corrupted variants
  - [ ] Stronger enemies
  - [ ] Reality entities

**Files to Modify**:
- `src/world/terrain.rs`
- `src/world/settlement.rs`
- `src/monsters/`

**Success Criteria**:
- Regions visibly transform
- NPCs react to changes
- Monsters evolve
- Feels apocalyptic

---

---

## Phase 3: Survival & Base Building (4-6 weeks)

**Goal**: Add survival mechanics, resource gathering, and base construction

### Week 19-20: Survival Systems

#### Task 3.1: Hunger & Thirst
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.11

**Subtasks**:
- [ ] Hunger/thirst meters
- [ ] Decay over time
- [ ] Food/water items
- [ ] Consumption mechanics
- [ ] Starvation/dehydration effects
  - [ ] Stat penalties
  - [ ] Health drain
  - [ ] Death

**Files to Create**:
- `src/survival/mod.rs`
- `src/survival/hunger.rs`

**Success Criteria**:
- Must eat/drink
- Clear feedback
- Not too tedious
- Strategic resource

---

#### Task 3.2: Food & Cooking
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 3.1

**Subtasks**:
- [ ] Food types
  - [ ] Raw ingredients
  - [ ] Cooked meals
  - [ ] Preserved food
- [ ] Cooking system
  - [ ] Recipes
  - [ ] Cooking locations (fire, stove)
  - [ ] Skill bonuses
- [ ] Food spoilage
  - [ ] Time-based decay
  - [ ] Preservation methods

**Files to Create**:
- `src/items/food.rs`
- `src/crafting/cooking.rs`
- `data/recipes/food.json`

**Success Criteria**:
- Can cook meals
- Recipes make sense
- Food management interesting
- Not too complex

---

#### Task 3.3: Camping & Rest
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 3.1

**Subtasks**:
- [ ] Fatigue system
- [ ] Sleep mechanics
  - [ ] Bedroll in wilderness
  - [ ] Beds in buildings
  - [ ] Quality affects recovery
- [ ] Camp setup
  - [ ] Campfire
  - [ ] Tent
  - [ ] Watch shifts (with NPCs)
- [ ] Rest interruptions
  - [ ] Monster attacks
  - [ ] Weather
  - [ ] Nightmares (sanity)

**Files to Create**:
- `src/survival/rest.rs`
- `src/survival/camping.rs`

**Success Criteria**:
- Must rest regularly
- Camping is strategic
- Sleep quality matters
- Risk/reward balance

---

#### Task 3.4: Resource Gathering
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: None

**Subtasks**:
- [ ] Resource nodes
  - [ ] Trees (wood)
  - [ ] Rocks (stone)
  - [ ] Herbs (alchemy)
  - [ ] Animals (meat, hide)
- [ ] Gathering mechanics
  - [ ] Time required
  - [ ] Tool requirements
  - [ ] Skill checks
  - [ ] Yield amounts
- [ ] Resource respawning
- [ ] Inventory management

**Files to Create**:
- `src/world/resources.rs`
- `src/systems/gathering.rs`

**Success Criteria**:
- Can gather materials
- Requires appropriate tools
- Takes time
- Yields make sense

---

### Week 21-22: Crafting System

#### Task 3.5: Crafting Framework
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 3.4

**Subtasks**:
- [ ] `Recipe` data structure
  - [ ] Requirements (items, tools, location)
  - [ ] Time cost
  - [ ] Skill requirements
  - [ ] Result
- [ ] Crafting UI
  - [ ] Recipe browser
  - [ ] Ingredient check
  - [ ] Progress bar
- [ ] Crafting execution
- [ ] Recipe discovery
  - [ ] Find in world
  - [ ] Learn from NPCs
  - [ ] Experiment

**Files to Create**:
- `src/crafting/mod.rs`
- `src/crafting/recipe.rs`
- `src/ui/crafting_ui.rs`
- `data/recipes/`

**Success Criteria**:
- Can craft items
- Clear UI
- Recipe system flexible
- Data-driven

---

#### Task 3.6: Crafting Categories
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 5 days
**Dependencies**: Task 3.5

**Subtasks**:
- [ ] Weapon crafting
  - [ ] Melee weapons
  - [ ] Ranged weapons
  - [ ] Ammunition
- [ ] Armor crafting
  - [ ] Leather
  - [ ] Metal
  - [ ] Cloth
- [ ] Tool crafting
  - [ ] Gathering tools
  - [ ] Utility items
- [ ] Alchemy
  - [ ] Potions
  - [ ] Poisons
  - [ ] Elixirs
- [ ] Blood magic (ritual crafting)
  - [ ] Requires lexemes
  - [ ] Sanity costs
  - [ ] Special ingredients

**Files to Create**:
- `data/recipes/weapons.json`
- `data/recipes/armor.json`
- `data/recipes/alchemy.json`
- `data/recipes/rituals.json`

**Success Criteria**:
- 50+ craftable items
- Multiple paths (combat, magic, survival)
- Progression through tiers
- Interesting choices

---

### Week 23-24: Base Building

#### Task 3.7: Construction System
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 5 days
**Dependencies**: Task 3.5

**Subtasks**:
- [ ] Structure types
  - [ ] Walls, doors, floors
  - [ ] Furniture
  - [ ] Workbenches
  - [ ] Defenses
- [ ] Construction mechanics
  - [ ] Placement system
  - [ ] Resource requirements
  - [ ] Build time
  - [ ] Cancel/deconstruct
- [ ] Construction UI
  - [ ] Building menu
  - [ ] Blueprint mode
  - [ ] Material check

**Files to Create**:
- `src/building/mod.rs`
- `src/building/construction.rs`
- `src/ui/construction_ui.rs`

**Success Criteria**:
- Can build structures
- Clear UI
- Resource management
- Visible progress

---

#### Task 3.8: Player Base System
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 3.7

**Subtasks**:
- [ ] Claim location as base
- [ ] Base types (house, fortress, tower)
- [ ] Base features
  - [ ] Storage
  - [ ] Crafting stations
  - [ ] Bed/rest area
  - [ ] Defense rating
- [ ] NPC residents/guards
- [ ] Base management UI

**Files to Create**:
- `src/building/base.rs`

**Success Criteria**:
- Can establish base
- Functional storage
- Safe rest location
- Feels like home

---

---

## Phase 4: World Simulation (6-8 weeks)

**Goal**: Make the world feel alive and reactive

### Week 25-26: Advanced Weather & Environment

#### Task 4.1: Expanded Weather System
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.13, Task 2.11

**Subtasks**:
- [ ] Corrupted weather types
  - [ ] Blood rain
  - [ ] Reality storm
  - [ ] Whisper fog
  - [ ] Void darkness
  - [ ] Flesh snow
- [ ] Weather progression based on corruption
- [ ] Extreme weather effects
  - [ ] Damage
  - [ ] Sanity drain
  - [ ] Forced layer switches
- [ ] Weather warnings

**Files to Modify**:
- `src/world/weather.rs`

**Success Criteria**:
- Corrupted weather appears
- Tied to corruption levels
- Dangerous and atmospheric
- Visual distinction

---

#### Task 4.2: Seasonal System
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 1.11

**Subtasks**:
- [ ] Season definitions
  - [ ] Normal seasons (Year 1)
  - [ ] Corrupted seasons (Year 2+)
- [ ] Seasonal effects
  - [ ] Temperature
  - [ ] Crop growth
  - [ ] Monster types
  - [ ] Weather patterns
- [ ] Season transitions
- [ ] Visual changes per season

**Files to Create**:
- `src/world/seasons.rs`

**Success Criteria**:
- Seasons change
- Affect gameplay
- Corrupted seasons scary
- Visual feedback

---

#### Task 4.3: Environmental Hazards
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 4.1

**Subtasks**:
- [ ] Hazard types
  - [ ] Fire/lava
  - [ ] Poison gas
  - [ ] Acid rain
  - [ ] Corruption zones
  - [ ] Reality rifts
- [ ] Damage over time
- [ ] Status effects from hazards
- [ ] Visual indicators

**Files to Create**:
- `src/world/hazards.rs`

**Success Criteria**:
- Hazards placed in world
- Deal damage appropriately
- Clear warnings
- Strategic obstacles

---

### Week 27-28: Faction Dynamics

#### Task 4.4: Faction Warfare
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 2.5

**Subtasks**:
- [ ] Faction conflict system
  - [ ] Declare war/peace
  - [ ] Battle calculations
  - [ ] Territory capture
- [ ] Settlement sieges
- [ ] Player can influence
  - [ ] Side with faction
  - [ ] Fight in battles
  - [ ] Negotiate peace
- [ ] Consequences of wars

**Files to Create**:
- `src/faction/warfare.rs`

**Success Criteria**:
- Factions fight over territory
- Player can participate
- Affects world state
- Creates drama

---

#### Task 4.5: Faction Response to Corruption
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.11, Task 4.4

**Subtasks**:
- [ ] Faction reactions to corruption
  - [ ] Some fight it
  - [ ] Some embrace it
  - [ ] Some go mad
  - [ ] Some flee
- [ ] Faction transformations
  - [ ] Holy Church → Cult of Dæl
  - [ ] Military → Corrupted Legion
- [ ] Dynamic alliances
  - [ ] Against corruption
  - [ ] For power
- [ ] Faction extinction

**Files to Modify**:
- `src/faction/faction.rs`
- `src/faction/warfare.rs`

**Success Criteria**:
- Factions react to corruption
- Some adapt, some fall
- Creates dynamic world
- Player sees consequences

---

### Week 29-30: NPC Simulation Depth

#### Task 4.6: NPC Needs & Morale
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.2

**Subtasks**:
- [ ] NPC needs
  - [ ] Hunger/thirst
  - [ ] Sleep
  - [ ] Safety
  - [ ] Social
- [ ] Morale system
  - [ ] Happiness affects behavior
  - [ ] Low morale → flee/rebel
- [ ] NPC death
  - [ ] From combat
  - [ ] From starvation
  - [ ] From corruption

**Files to Modify**:
- `src/npc/npc.rs`
- `src/npc/ai.rs`

**Success Criteria**:
- NPCs feel more alive
- React to hardship
- Can die permanently
- Population fluctuates

---

#### Task 4.7: NPC Corruption & Madness
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 4.6, Task 2.11

**Subtasks**:
- [ ] NPC corruption tracking
- [ ] Corruption effects on NPCs
  - [ ] Behavior changes
  - [ ] Dialogue changes
  - [ ] Visual changes (mutations)
- [ ] Stages of corruption
  - [ ] Suspicious → Aware → Enlightened → Mad → Possessed
- [ ] NPC can join cults
- [ ] NPC can attack player

**Files to Create**:
- `src/npc/corruption.rs`

**Success Criteria**:
- NPCs gradually corrupt
- Personality changes
- Some turn hostile
- Tragic transformations

---

### Week 31-32: Dynamic World Events

#### Task 4.8: Large-Scale Events
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 5 days
**Dependencies**: Task 2.10, Task 2.11

**Subtasks**:
- [ ] Event types
  - [ ] Portal storm (reality tears)
  - [ ] Mass corruption (region flips)
  - [ ] Refugee crisis (NPCs fleeing)
  - [ ] Cult uprising
  - [ ] Warden expedition (hunt player)
  - [ ] The Unraveling (apocalypse)
- [ ] Event triggers
  - [ ] Corruption thresholds
  - [ ] Player actions
  - [ ] Time-based
- [ ] Event consequences
  - [ ] Permanent changes
  - [ ] New quests
  - [ ] Faction shifts

**Files to Modify**:
- `src/world/events.rs`

**Success Criteria**:
- Epic-scale events
- World-changing
- Player can interact
- Memorable moments

---

#### Task 4.9: Apocalypse Timer
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 3 days
**Dependencies**: Task 2.11

**Subtasks**:
- [ ] Doomsday clock
  - [ ] Days until complete collapse
  - [ ] Accelerates with corruption
  - [ ] Can be slowed (rarely)
- [ ] Endgame warnings
  - [ ] Prophecies
  - [ ] NPC warnings
  - [ ] Visual indicators
- [ ] Final reckoning
  - [ ] World ends if timer reaches 0
  - [ ] Multiple ending scenarios

**Files to Create**:
- `src/world/apocalypse.rs`

**Success Criteria**:
- Clear time pressure
- Player can see progress
- Creates urgency
- Endings trigger

---

---

## Phase 5: Polish & Content (8-12 weeks)

**Goal**: Complete narrative, add unique locations, balance gameplay

### Week 33-36: Narrative & Lore

#### Task 5.1: Complete Main Story
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 8 days
**Dependencies**: Task 2.9

**Subtasks**:
- [ ] Write all 25+ main quests
- [ ] Multiple endings (3-5)
  - [ ] Triumph (seal the breach)
  - [ ] Sacrifice (stop it with death)
  - [ ] Transformation (become entity)
  - [ ] Apocalypse (fail completely)
  - [ ] Hidden ending (???)
- [ ] Cutscenes/special moments
- [ ] Final boss encounters
- [ ] Epilogue system

**Files to Modify**:
- `src/quest/main_quest.rs`
- `data/quests/main_story.json`
- `data/dialogue/main_quest/`

**Success Criteria**:
- Complete story arc
- Emotional payoff
- Multiple valid endings
- Replayability

---

#### Task 5.2: Lore System
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 5 days
**Dependencies**: None

**Subtasks**:
- [ ] Lore fragments
  - [ ] Books
  - [ ] Notes
  - [ ] Inscriptions
  - [ ] NPC stories
- [ ] Lore codex (player journal)
- [ ] Hidden lore
- [ ] Contradictory accounts (unreliable narrators)
- [ ] Discovery mechanics

**Files to Create**:
- `src/lore/mod.rs`
- `src/ui/lore_codex.rs`
- `data/lore/`

**Success Criteria**:
- Rich worldbuilding
- Discoverable through exploration
- Rewards curiosity
- Deepens narrative

---

#### Task 5.3: Unique Locations
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 10 days
**Dependencies**: Phase 1 complete

**Subtasks**:
- [ ] Design 10+ hand-crafted locations
  - [ ] Daelspire (capital city)
  - [ ] Saelcairn Academy (magic school)
  - [ ] The Wound (corruption epicenter)
  - [ ] Memory Archive (meta-hub)
  - [ ] Ancient temples
  - [ ] Warden strongholds
  - [ ] Cult hideouts
  - [ ] Reality tears
- [ ] Unique encounters per location
- [ ] Special loot
- [ ] Boss fights
- [ ] Secrets

**Files to Create**:
- `data/locations/unique/`

**Success Criteria**:
- 10+ memorable locations
- Hand-crafted quality
- Story integration
- Exploration rewards

---

### Week 37-40: Systems Refinement

#### Task 5.4: Lexeme Magic System
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 8 days
**Dependencies**: Task 3.6

**Subtasks**:
- [ ] Lexeme parser
  - [ ] Tokenization
  - [ ] Grammar rules
  - [ ] Validation
- [ ] 30+ lexemes
  - [ ] Base words (FIRE, ICE, LIGHT, etc.)
  - [ ] Modifiers (GREAT, LESSER, etc.)
  - [ ] Targets (SELF, FOE, AREA)
- [ ] Spell effects
  - [ ] Damage
  - [ ] Buffs/debuffs
  - [ ] Summons
  - [ ] Utility
  - [ ] Reality manipulation
- [ ] Sanity costs per spell
- [ ] Notice increases per cast
- [ ] Discovery system

**Files to Create**:
- `src/magic/mod.rs`
- `src/magic/lexeme.rs`
- `src/magic/parser.rs`
- `src/magic/spells.rs`
- `data/lexemes.json`

**Success Criteria**:
- 30+ valid combinations
- Intuitive grammar
- Balanced power
- Discovery is fun

---

#### Task 5.5: Mutation System
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 7 days
**Dependencies**: Task 5.4

**Subtasks**:
- [ ] Mutation categories
  - [ ] Cosmic Insight
  - [ ] Flesh Warping
  - [ ] Dimensional Taint
  - [ ] Parasitic Growth
  - [ ] Void Touched
- [ ] 50+ mutations
  - [ ] Tiers 1-3
  - [ ] Prerequisites
  - [ ] Conflicts
- [ ] Mutation triggers
  - [ ] Corruption threshold
  - [ ] Rituals
  - [ ] Items
  - [ ] Random events
- [ ] Mutation effects
  - [ ] Stats
  - [ ] Abilities
  - [ ] Appearance
  - [ ] NPC reactions
- [ ] Threshold transformations

**Files to Create**:
- `src/mutation/mod.rs`
- `src/mutation/mutation.rs`
- `data/mutations.json`

**Success Criteria**:
- 50+ mutations
- Body horror theme
- Meaningful choices
- Visual feedback

---

#### Task 5.6: Enhanced Combat
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 5 days
**Dependencies**: Task 5.4, Task 5.5

**Subtasks**:
- [ ] Status effects in combat
- [ ] Spell casting in combat
- [ ] Mutation abilities in combat
- [ ] Tactical positioning
- [ ] Cover system
- [ ] Flanking bonuses
- [ ] Critical hits
- [ ] Combat animations (ASCII)

**Files to Modify**:
- `src/systems/combat.rs`

**Success Criteria**:
- Combat feels tactical
- Multiple viable strategies
- Magic integrated
- Not too slow

---

#### Task 5.7: AI Improvements
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 5.6

**Subtasks**:
- [ ] Better monster AI
  - [ ] Use abilities
  - [ ] Flee when wounded
  - [ ] Call for help
  - [ ] Use environment
- [ ] Boss AI
  - [ ] Phases
  - [ ] Special attacks
  - [ ] Scripted behavior
- [ ] Corrupted monster variants
- [ ] Reality entity behavior

**Files to Modify**:
- `src/systems/ai.rs`
- `src/monsters/`

**Success Criteria**:
- Enemies feel smart
- Bosses memorable
- Challenging but fair
- Variety in behavior

---

### Week 41-44: Balance & Testing

#### Task 5.8: Game Balance
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 8 days
**Dependencies**: All previous tasks

**Subtasks**:
- [ ] Combat balance
  - [ ] Damage numbers
  - [ ] Enemy HP
  - [ ] Player progression
- [ ] Economy balance
  - [ ] Item costs
  - [ ] Loot drops
  - [ ] Crafting costs
- [ ] Progression curve
  - [ ] Level difficulty
  - [ ] Skill gains
  - [ ] Equipment tiers
- [ ] Sanity/Notice balance
  - [ ] Drain rates
  - [ ] Recovery rates
  - [ ] Costs vs benefits
- [ ] Corruption pace
  - [ ] Not too fast/slow
  - [ ] Player agency

**Success Criteria**:
- Feels fair
- Progression satisfying
- Multiple viable builds
- Challenging but not punishing

---

#### Task 5.9: Extensive Playtesting
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 12 days
**Dependencies**: All previous tasks

**Subtasks**:
- [ ] Full playthrough testing (multiple)
- [ ] Different playstyles
  - [ ] Combat focus
  - [ ] Magic focus
  - [ ] Stealth/social
  - [ ] Speedrun
  - [ ] Completionist
- [ ] Bug hunting
- [ ] Balance tweaking
- [ ] Quality of life improvements
- [ ] Performance testing

**Success Criteria**:
- No critical bugs
- All paths viable
- Smooth experience
- Ready for release

---

#### Task 5.10: Achievements & Meta-Progression
**Priority**: 🟢 Medium
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 5.9

**Subtasks**:
- [ ] Achievement system
  - [ ] 30+ achievements
  - [ ] Hidden achievements
  - [ ] Tracking
  - [ ] UI display
- [ ] Meta-progression unlocks
  - [ ] New classes
  - [ ] Starting bonuses
  - [ ] Special items
  - [ ] Lore entries
- [ ] Statistics tracking
  - [ ] Deaths
  - [ ] Kills
  - [ ] Distance traveled
  - [ ] Corruption spread
  - [ ] etc.

**Files to Create**:
- `src/meta/achievements.rs`
- `src/meta/statistics.rs`

**Success Criteria**:
- 30+ achievements
- Rewards meaningful
- Encourages replayability
- Stats interesting

---

### Week 45-46: Documentation & Release Prep

#### Task 5.11: In-Game Tutorial
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: Task 5.9

**Subtasks**:
- [ ] Tutorial sequence
  - [ ] Movement basics
  - [ ] Combat intro
  - [ ] Inventory management
  - [ ] Overworld navigation
  - [ ] Dialogue
  - [ ] Quests
- [ ] Tooltips
- [ ] Help screens
- [ ] Context hints
- [ ] Can skip for veterans

**Files to Create**:
- `src/tutorial/mod.rs`
- `src/ui/help.rs`

**Success Criteria**:
- New players understand basics
- Not intrusive
- Can be skipped
- Covers all core systems

---

#### Task 5.12: Documentation
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: None

**Subtasks**:
- [ ] README update
- [ ] Installation guide
- [ ] Controls reference
- [ ] Game mechanics guide
- [ ] Lore/story primer (spoiler-free)
- [ ] FAQ
- [ ] Troubleshooting
- [ ] Credits

**Files to Create/Update**:
- `README.md`
- `INSTALL.md`
- `CONTROLS.md`
- `GUIDE.md`
- `FAQ.md`

**Success Criteria**:
- Clear documentation
- Easy to get started
- Questions answered
- Professional presentation

---

#### Task 5.13: Final Polish
**Priority**: 🟡 High
**Status**: 🔴 Not Started
**Estimate**: 4 days
**Dependencies**: All previous tasks

**Subtasks**:
- [ ] UI polish
  - [ ] Consistent styling
  - [ ] Better colors
  - [ ] Animations
  - [ ] Transitions
- [ ] Audio (if possible)
  - [ ] Sound effects
  - [ ] Music (MIDI?)
  - [ ] Ambient sounds
- [ ] Accessibility
  - [ ] Colorblind modes
  - [ ] Text size options
  - [ ] Key rebinding
- [ ] Performance optimization
- [ ] Code cleanup
- [ ] Final bug fixes

**Success Criteria**:
- Polished feel
- Professional quality
- Accessible
- Optimized

---

#### Task 5.14: Release Build
**Priority**: 🔴 Critical
**Status**: 🔴 Not Started
**Estimate**: 2 days
**Dependencies**: All previous tasks

**Subtasks**:
- [ ] Version 1.0 tag
- [ ] Release builds
  - [ ] Linux
  - [ ] Windows
  - [ ] macOS
- [ ] Packaging
  - [ ] Archives
  - [ ] Installers
- [ ] Distribution
  - [ ] GitHub release
  - [ ] itch.io
  - [ ] Steam (optional)
- [ ] Launch announcement

**Success Criteria**:
- Builds work on all platforms
- Easy to install
- Publicly available
- Launch successful

---

## Quick Reference: Priority Guide

### 🔴 Critical Path (Must Have)
- Overmap system
- Travel & transitions
- Time system
- Save system
- NPCs & dialogue
- Factions
- Quests
- Corruption spreading
- Main story
- Lexeme magic
- Mutations
- Game balance

### 🟡 High Priority (Should Have)
- Weather
- Camping & rest
- Crafting
- Faction warfare
- NPC corruption
- Combat improvements
- AI improvements
- Unique locations

### 🟢 Medium Priority (Nice to Have)
- Advanced weather
- Seasons
- Base building
- Achievements
- Tutorial
- Polish features

---

## Development Tools & Resources

### Code Quality Checks
```bash
# Run before committing
cargo fmt              # Format code
cargo clippy           # Lint
cargo test            # Run tests
cargo build --release # Test release build
```

### Testing Commands
```bash
# Quick test
cargo run

# Profile performance
cargo build --release
time cargo run --release

# Memory check (with valgrind)
valgrind --leak-check=full ./target/release/dungeon-clawler-tui
```

### Git Workflow
```bash
# Feature branch
git checkout -b feature/task-X.Y

# Commit
git add .
git commit -m "Task X.Y: Brief description"

# Push
git push origin feature/task-X.Y

# Merge to main when complete
git checkout main
git merge feature/task-X.Y
git tag vX.Y.Z
```

---

## Progress Tracking Template

### Daily Log Format
```markdown
## YYYY-MM-DD

### Completed
- [x] Task X.Y.Z - Description

### In Progress
- [ ] Task X.Y.Z - Description (50%)

### Blocked
- [ ] Task X.Y.Z - Description (blocked by: ...)

### Next Up
- Task X.Y.Z - Description

### Notes
- Any issues, discoveries, or changes

### Time Spent
- X hours
```

---

## Milestones

| Milestone | Target Date | Tasks | Status |
|-----------|-------------|-------|--------|
| **M1: Playable Overworld** | Week 10 | 1.1 - 1.18 | 🔴 Not Started |
| **M2: Living World** | Week 18 | 2.1 - 2.12 | 🔴 Not Started |
| **M3: Survival Systems** | Week 24 | 3.1 - 3.8 | 🔴 Not Started |
| **M4: Dynamic World** | Week 32 | 4.1 - 4.9 | 🔴 Not Started |
| **M5: Content Complete** | Week 40 | 5.1 - 5.7 | 🔴 Not Started |
| **M6: Release Ready** | Week 46 | 5.8 - 5.14 | 🔴 Not Started |

---

## Risk Register

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Scope too large | High | High | Start with MVP, add features iteratively |
| Performance issues | High | Medium | Profile early, optimize hot paths |
| Save corruption | High | Low | Extensive testing, backup system |
| Complex AI | Medium | Medium | Start simple, iterate |
| Balance problems | Medium | High | Continuous playtesting |
| Burnout | High | Medium | Modular dev, take breaks |

---

**Document Status**: Living document
**Last Updated**: 2025-10-06
**Next Review**: Start of Phase 1

---

*"Every task completed brings us closer to The Unraveling..."*
