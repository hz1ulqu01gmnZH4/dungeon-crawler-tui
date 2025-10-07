# Current Implementation vs. Planned Architecture

**Generated**: 2025-10-07
**Status**: Phase 0 Complete, Phase 1 Not Started

---

## Executive Summary

The project has successfully completed **Phase 0** (baseline dungeon crawler) with ~1,330 lines of Rust code across 25 files. The foundation is solid with proper ECS architecture using `hecs`, but **Phase 1** (Open World Foundation) has not yet begun. This document compares what exists vs. what's planned.

---

## 1. Architecture Alignment

### ✅ Core Architecture - IMPLEMENTED

| Planned Component | Status | Implementation |
|------------------|--------|----------------|
| ECS Framework (hecs) | ✅ Complete | `Cargo.toml:12` |
| Terminal UI (ratatui) | ✅ Complete | `Cargo.toml:8` |
| Crossterm input | ✅ Complete | `Cargo.toml:9` |
| Serialization (serde) | ✅ Complete | `Cargo.toml:15-17` |
| Random generation | ✅ Complete | `Cargo.toml:20-22` |
| Turn-based game loop | ✅ Complete | `src/game/app.rs` |
| Intent-based actions | ✅ Complete | Components: `WantsToMove`, `WantsToMelee` |
| Resources struct | ✅ Complete | `src/ecs/resources.rs` |

**Assessment**: Core architecture follows planned design perfectly. ECS pattern properly implemented with intent components.

---

## 2. Components Implementation

### ✅ Core Components - IMPLEMENTED

| Component | Status | File | Notes |
|-----------|--------|------|-------|
| `Position` | ✅ | `src/ecs/components.rs:10` | Includes `RealityLayer` |
| `Renderable` | ✅ | `src/ecs/components.rs:22` | With z-ordering |
| `Viewshed` | ✅ | `src/ecs/components.rs:42` | FOV with dirty flag |
| `CombatStats` | ✅ | `src/ecs/components.rs:58` | HP, power, defense |
| `TriMeter` | ✅ | `src/ecs/components.rs:77` | Insight, sanity, notice |
| `Name` | ✅ | `src/ecs/components.rs:95` | String wrapper |
| `Player` | ✅ | `src/ecs/components.rs:99` | Marker |
| `Monster` | ✅ | `src/ecs/components.rs:102` | Marker |
| `BlocksMovement` | ✅ | `src/ecs/components.rs:105` | Collision marker |
| `WantsToMove` | ✅ | `src/ecs/components.rs:108` | Intent component |
| `WantsToMelee` | ✅ | `src/ecs/components.rs:120` | Intent component |
| `WantsToWait` | ✅ | `src/ecs/components.rs:132` | Wait intent |

### ❌ Missing Components - NOT IMPLEMENTED

According to `ARCHITECTURE.md`, the following are missing:

**NPC Components** (all missing):
- `NPCData` - profession, home, faction
- `Schedule` - daily activities
- `Dialogue` - dialogue tree
- `Inventory` - items carried
- `Reputation` - faction standings
- `Morale` - happiness, needs

**World Components** (all missing):
- `Settlement` - name, size, population, buildings
- `Building` - type, interior, owner
- `Faction` - name, territory, power, relationships
- `Quest` - objectives, rewards, stage

**Cosmic Horror Components** (partially implemented):
- ✅ `TriMeter` - implemented but not functional
- ❌ `Corruption` - not implemented
- ❌ `Mutation` - not implemented
- ❌ `Lexeme` - not implemented
- ❌ `MadnessEffect` - not implemented

**Assessment**: Core gameplay components exist. All Phase 1+ components missing.

---

## 3. Systems Implementation

### ✅ Implemented Systems

| System | Status | File | Notes |
|--------|--------|------|-------|
| Input System | ✅ | `src/systems/input.rs` | Player controls |
| Movement System | ✅ | `src/systems/movement.rs` | With collision |
| Combat System | ✅ | `src/systems/combat.rs` | Melee combat + death |
| AI System | ✅ | `src/systems/ai.rs` | Simple chase AI |
| FOV System | ✅ | `src/systems/fov.rs` | Ray-casting |

### ❌ Missing Systems - NOT IMPLEMENTED

According to `IMPLEMENTATION_TASKS.md` Phase 1, these are missing:

**Travel & World Systems**:
- Travel System - overworld movement
- Time System - world time, day/night
- Transition System - enter/exit locations
- Weather System - dynamic weather

**World Management**:
- Overmap System - 200x200 world map
- Chunk System - streaming world chunks
- Settlement System - towns, cities
- Building System - interiors
- Encounter System - random events

**Content Systems** (Phase 2+):
- NPC AI & Schedules
- Dialogue System
- Faction System
- Quest System
- Corruption Spreading

**Survival Systems** (Phase 3+):
- Hunger/Thirst
- Crafting
- Construction
- Rest/Camping

**Assessment**: Only baseline Phase 0 systems implemented. All Phase 1+ systems missing.

---

## 4. Module Structure Comparison

### Current Structure (Phase 0)

```
src/
├── main.rs
├── lib.rs
├── ecs/
│   ├── components.rs    ✅ Basic components
│   ├── resources.rs     ✅ Basic resources
│   └── mod.rs
├── systems/
│   ├── movement.rs      ✅ Movement
│   ├── combat.rs        ✅ Combat
│   ├── ai.rs            ✅ Basic AI
│   ├── fov.rs           ✅ FOV
│   ├── input.rs         ✅ Input
│   └── mod.rs
├── map/
│   ├── tile.rs          ✅ Tile types
│   ├── generator.rs     ✅ Dungeon gen
│   ├── fov.rs           ✅ FOV algorithm
│   └── mod.rs           ✅ Map + MapSet
├── ui/
│   ├── renderer.rs      ✅ Basic rendering
│   ├── input.rs         ✅ Input handling
│   └── mod.rs
├── game/
│   ├── app.rs           ✅ Game loop
│   ├── state.rs         ✅ (stub)
│   └── world.rs         ✅ (stub)
├── generation/
│   └── dungeon.rs       ✅ Room gen
└── components/
    ├── position.rs      ✅ (duplicate?)
    └── stats.rs         ✅ (duplicate?)
```

### Planned Structure (from ARCHITECTURE.md)

```
src/
├── main.rs
├── lib.rs
├── ecs/                 ✅ EXISTS
├── systems/             ✅ EXISTS (incomplete)
├── map/                 ✅ EXISTS (local only)
├── ui/                  ✅ EXISTS (basic)
├── game/                ✅ EXISTS (stubs)
├── world/               ❌ MISSING (entire directory)
│   ├── overmap.rs       ❌ Critical for Phase 1
│   ├── chunks.rs        ❌ Critical for Phase 1
│   ├── terrain.rs       ❌ Critical for Phase 1
│   ├── generator.rs     ❌ Critical for Phase 1
│   ├── settlement.rs    ❌ Phase 1
│   ├── time.rs          ❌ Phase 1
│   ├── weather.rs       ❌ Phase 1
│   ├── events.rs        ❌ Phase 2
│   ├── corruption.rs    ❌ Phase 2
│   └── resources.rs     ❌ Phase 3
├── npc/                 ❌ MISSING (entire directory)
├── faction/             ❌ MISSING (entire directory)
├── quest/               ❌ MISSING (entire directory)
├── magic/               ❌ MISSING (entire directory)
├── mutation/            ❌ MISSING (entire directory)
├── items/               ❌ MISSING (entire directory)
├── crafting/            ❌ MISSING (entire directory)
├── building/            ❌ MISSING (entire directory)
├── survival/            ❌ MISSING (entire directory)
├── save/                ❌ MISSING (entire directory)
├── lore/                ❌ MISSING (entire directory)
├── meta/                ❌ MISSING (entire directory)
└── tutorial/            ❌ MISSING (entire directory)
```

**Assessment**: Only Phase 0 modules exist. All Phase 1+ modules completely missing.

---

## 5. Map & World Systems

### ✅ Current Map Implementation

```rust
// src/map/mod.rs:11
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
}

// src/map/mod.rs:75
pub struct MapSet {
    pub active: RealityLayer,
    pub normal: Map,
    pub cosmic: Map,
}
```

**Status**: Single-dungeon map with dual-layer support (structure only)

### ❌ Planned World Systems - MISSING

From `IMPLEMENTATION_TASKS.md` Task 1.1-1.4:

**Overmap System** (Not Started):
```rust
// Should exist: src/world/overmap.rs
pub struct OvermapTile {
    pub position: (i32, i32),
    pub terrain: TerrainType,
    pub discovered: bool,
    pub visited: bool,
    pub corruption: u8,
    pub location: Option<LocationId>,
}

pub struct Overmap {
    pub tiles: Vec<OvermapTile>,
    pub width: i32,  // 200x200
    pub height: i32,
}
```

**Chunk System** (Not Started):
- Should load 24x24 tile chunks
- Should stream around player
- Should save/load modified chunks
- LRU cache for performance

**Terrain Generation** (Not Started):
- Perlin/Simplex noise for elevation
- Biome assignment (plains, forest, mountains, rivers)
- Seed-based deterministic generation
- Road network between settlements

**Assessment**: No world systems exist beyond single map.

---

## 6. Resources Comparison

### ✅ Current Resources

```rust
// src/ecs/resources.rs:62
pub struct Resources {
    pub maps: MapSet,
    pub camera: Camera,
    pub rng: StdRng,
    pub mode: RunMode,
    pub log: GameLog,
    pub player_entity: Option<hecs::Entity>,
}
```

### ❌ Planned Resources - MISSING

From `ARCHITECTURE.md:214`:

```rust
pub struct Resources {
    // World (MISSING)
    pub overmap: Overmap,
    pub chunks: ChunkManager,
    pub maps: MapSet,  // ✅ EXISTS

    // Rendering (EXISTS)
    pub camera: Camera,

    // Game State (PARTIAL)
    pub mode: RunMode,
    pub player_entity: Option<Entity>,
    pub time: WorldTime,  // ❌ MISSING

    // Systems (MISSING)
    pub log: GameLog,  // ✅ EXISTS
    pub rng: StdRng,   // ✅ EXISTS
    pub factions: FactionManager,  // ❌ MISSING
    pub quests: QuestManager,      // ❌ MISSING
    pub events: EventManager,      // ❌ MISSING

    // Meta (MISSING)
    pub save_data: SaveMetadata,  // ❌ MISSING
}
```

**Assessment**: Only 40% of planned resources exist.

---

## 7. Gameplay Features

### ✅ Phase 0 Features - IMPLEMENTED

From `IMPLEMENTATION_TASKS.md` Phase 0 checklist:

| Feature | Status | Evidence |
|---------|--------|----------|
| ECS Architecture | ✅ | hecs in use throughout |
| Basic Game Loop | ✅ | Turn-based player/monster |
| Map Generation | ✅ | Room + corridor dungeons |
| Combat System | ✅ | Melee with damage calc |
| AI & Monsters | ✅ | Chase AI, 3 types |
| Field of View | ✅ | Ray-casting FOV |
| UI/UX | ✅ | Map, status, messages |
| Dual-Layer Reality | 🟡 | Structure only, no switching |
| Tri-Meter System | 🟡 | UI only, not functional |

**Phase 0 Progress**: 100% complete (as stated in docs)

### ❌ Phase 1 Features - NOT IMPLEMENTED

From `IMPLEMENTATION_TASKS.md` Week 1-10:

| Feature | Status | Progress | Required For |
|---------|--------|----------|--------------|
| Overmap System | ❌ | 0% | Open world |
| Terrain Generation | ❌ | 0% | Open world |
| Road Networks | ❌ | 0% | Travel |
| Overmap Rendering | ❌ | 0% | Visualization |
| Settlement Placement | ❌ | 0% | Towns/cities |
| Building Generation | ❌ | 0% | Interiors |
| Dungeon Placement | ❌ | 0% | World dungeons |
| Overworld Movement | ❌ | 0% | Travel |
| Transition System | ❌ | 0% | Enter locations |
| Random Encounters | ❌ | 0% | Travel events |
| Time System | ❌ | 0% | Day/night |
| Day/Night Cycle | ❌ | 0% | Visibility |
| Basic Weather | ❌ | 0% | Atmosphere |
| Chunk Loading | ❌ | 0% | Performance |
| Enhanced Save System | ❌ | 0% | Persistence |
| Map & Navigation UI | ❌ | 0% | Player tools |
| Performance Optimization | ❌ | 0% | Scalability |
| Testing & Bug Fixes | ❌ | 0% | Quality |

**Phase 1 Progress**: 0% (not started)

---

## 8. Cosmic Horror Mechanics

### 🟡 Tri-Meter System - STRUCTURE ONLY

**Current**:
```rust
// src/ecs/components.rs:77
pub struct TriMeter {
    pub insight: i32,
    pub sanity: i32,
    pub notice: i32,
}
```

**Issues**:
- Component exists but is never modified
- No systems consume or update meters
- No gameplay effects from meter values
- UI displays but values don't change

**Planned** (from `SYSTEMS_SPECIFICATION.md:556`):
- Insight affects layer switching (not implemented)
- Sanity affects madness effects (not implemented)
- Notice affects enemy spawns (not implemented)
- Complex balance mechanics (not implemented)

### ❌ Reality Layer Switching - NOT FUNCTIONAL

**Current**:
```rust
// src/ecs/components.rs:4
pub enum RealityLayer {
    Normal,
    Cosmic,
}

// src/map/mod.rs:75
pub struct MapSet {
    pub active: RealityLayer,
    pub normal: Map,
    pub cosmic: Map,
}
```

**Issues**:
- Enum defined but never changed
- Both maps exist but cosmic never used
- No input to switch layers
- No insight requirement check

**Planned**:
- Switch with Tab key when insight > 50
- Cosmic layer shows hidden entities
- Different terrain in cosmic layer
- Forced switches in high corruption

### ❌ Missing Cosmic Systems

All missing:
- Corruption spreading
- Mutation system
- Lexeme magic
- Madness effects
- Reality entities
- Corruption weather

**Assessment**: Cosmic horror is architectural only, no functionality.

---

## 9. Dependency Alignment

### Current Dependencies (Cargo.toml)

| Dependency | Version | Planned | Status |
|------------|---------|---------|--------|
| ratatui | 0.28 | 0.28 | ✅ Match |
| crossterm | 0.28 | 0.28 | ✅ Match |
| hecs | 0.10 | 0.10 | ✅ Match |
| serde | 1.0 | 1.0 | ✅ Match |
| serde_json | - | 1.0 | ❌ Missing |
| ron | 0.8 | - | ⚠️ Extra (RON format) |
| bincode | 1.3 | - | ⚠️ Extra (for saves) |
| rand | 0.8 | 0.8 | ✅ Match |
| rand_pcg | 0.3 | - | ⚠️ Extra (PCG RNG) |
| noise | 0.9 | 0.9 | ✅ Match |
| anyhow | 1.0 | 1.0 | ✅ Match |
| thiserror | 1.0 | 1.0 | ✅ Match |
| smallvec | 1.11 | - | ⚠️ Extra (optimization) |

**Assessment**: Core dependencies correct. Some extras for optimization/formats.

---

## 10. Code Quality Metrics

### Current State

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Total Files | 25 | - | - |
| Total Lines | ~1,330 | - | Phase 0 |
| Modules | 8 | 15+ | ⚠️ Need more |
| Components | 11 | 20+ | ⚠️ Need more |
| Systems | 5 | 15+ | ⚠️ Need more |

### Missing Code

Based on `IMPLEMENTATION_TASKS.md` estimates:

| Phase | Est. Duration | Tasks | Status |
|-------|---------------|-------|--------|
| Phase 0 | - | - | ✅ 100% |
| Phase 1 | 8-12 weeks | 18 tasks | ❌ 0% |
| Phase 2 | 6-8 weeks | 12 tasks | ❌ 0% |
| Phase 3 | 4-6 weeks | 8 tasks | ❌ 0% |
| Phase 4 | 6-8 weeks | 9 tasks | ❌ 0% |
| Phase 5 | 8-12 weeks | 14 tasks | ❌ 0% |

**Total Remaining**: 61 tasks, 32-46 weeks estimated

---

## 11. Data Files

### ❌ All Data Files Missing

From `ARCHITECTURE.md:377`, none exist:

```
data/
├── items/           ❌ Not created
├── monsters/        ❌ Not created
├── recipes/         ❌ Not created
├── factions.json    ❌ Not created
├── lexemes.json     ❌ Not created
├── mutations.json   ❌ Not created
├── quests/          ❌ Not created
├── dialogue/        ❌ Not created
├── lore/            ❌ Not created
└── locations/       ❌ Not created
```

**Current**: All data is hardcoded in Rust. No JSON files exist.

**Planned**: Data-driven design with external JSON files for modding.

---

## 12. Critical Path Analysis

### To reach Phase 1 (Open World Foundation)

**Critical Tasks** (must be done in order):

1. **Task 1.1**: Overmap Data Structure (2 days)
   - Create `src/world/overmap.rs`
   - Define `OvermapTile` and `Overmap` structs

2. **Task 1.2**: Terrain Generation (3 days)
   - Implement noise-based generation
   - Biome assignment

3. **Task 1.4**: Overmap Rendering (2 days)
   - Create `src/ui/overmap_renderer.rs`
   - Toggle between local and overmap

4. **Task 1.8**: Overworld Movement (3 days)
   - Player movement on overmap
   - Time progression

5. **Task 1.9**: Transition System (3 days)
   - Enter/exit locations
   - Load/save state

6. **Task 1.11**: Time System (2 days)
   - WorldTime struct
   - Action time costs

**Estimated**: 15 days for minimal open world

---

## 13. Recommendations

### Immediate Priorities (Next 2 Weeks)

1. **Create `src/world/` module** with:
   - `overmap.rs` - 50x50 grid for MVP
   - `terrain.rs` - Basic terrain enum
   - `generator.rs` - Simple noise generation

2. **Implement basic overmap**:
   - Can view (Tab key to toggle)
   - Can move on overmap (arrow keys)
   - Single town to start

3. **Basic transition system**:
   - Enter town → load building
   - Exit building → return to overmap

4. **Time system**:
   - WorldTime struct
   - Advance time on actions

### Medium-term (Weeks 3-8)

5. **Settlement system**: Towns with NPCs
6. **NPC foundation**: Basic schedules
7. **Quest system**: Simple fetch quests
8. **Save/load**: Serialize world state

### Long-term (Months 3-6)

9. **Corruption spreading**: Core mechanic
10. **Lexeme magic**: Word-based spells
11. **Mutation system**: Physical changes
12. **Main story**: 25+ quests

---

## 14. Gap Summary

### What Exists ✅

- Solid ECS foundation (hecs)
- Turn-based game loop
- Basic dungeon generation
- Melee combat system
- Simple chase AI
- Ray-casting FOV
- Terminal rendering
- Dual-layer structure (no functionality)
- Tri-meter structure (no functionality)

### What's Missing ❌

**Phase 1** (0% complete):
- Entire `world/` module
- Overmap system
- Terrain generation
- Settlement system
- Travel system
- Time system
- Weather system
- Chunk streaming
- Save/load

**Phase 2** (0% complete):
- NPC system
- Dialogue system
- Faction system
- Quest system
- Corruption spreading
- Dynamic events

**Phase 3-5** (0% complete):
- Survival systems
- Crafting
- Magic (lexemes)
- Mutations
- Main story
- All content

### Critical Missing Features

1. **No open world** - Only single dungeons
2. **No persistence** - Can't save/load
3. **No NPCs** - World feels dead
4. **No quests** - No goals beyond survival
5. **No time** - No day/night, schedules
6. **No corruption** - Core mechanic missing
7. **No cosmic mechanics** - Theme not implemented

---

## 15. Conclusion

**Current State**: **Phase 0 Complete, Phase 1 Not Started**

The project has excellent foundations:
- Clean ECS architecture
- Proper component design
- Intent-based actions
- Dual-layer structure ready

But **0% of Phase 1** is implemented:
- No open world
- No overmap
- No settlements
- No world simulation

**Estimated Completion**:
- Minimal open world (Phase 1): 8-12 weeks
- Full game (Phase 5): 32-46 weeks

**Next Step**: Begin Task 1.1 (Overmap Data Structure)

---

*"The foundation is solid. The tower reaches toward madness. But we have only built the basement."*
