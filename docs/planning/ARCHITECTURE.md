# The Unraveling of Kándavael - Architecture Documentation

**Version**: 1.0
**Last Updated**: 2025-10-07
**Status**: Design Phase

---

## Table of Contents

1. [Overview](#overview)
2. [Technology Stack](#technology-stack)
3. [System Architecture](#system-architecture)
4. [ECS Architecture](#ecs-architecture)
5. [Module Structure](#module-structure)
6. [Data Flow](#data-flow)
7. [World Management](#world-management)
8. [Performance Considerations](#performance-considerations)
9. [Design Patterns](#design-patterns)

---

## Overview

The Unraveling of Kándavael is an open-world cosmic horror roguelike built with a modern Entity-Component-System (ECS) architecture. The game features:

- **Persistent Open World**: Large overmap with streaming chunk-based loading
- **Living World Simulation**: NPCs with schedules, factions with territories, dynamic events
- **Reality Layers**: Dual-layer system (Normal/Cosmic) with seamless transitions
- **Data-Driven Design**: JSON-based content for easy modding and iteration
- **Terminal UI**: Cross-platform TUI using Ratatui

### Design Philosophy

1. **Modularity**: Systems are independent and communicate through ECS
2. **Data-Driven**: Game content in JSON files, not hardcoded
3. **Performance**: Chunk streaming, entity culling, efficient queries
4. **Extensibility**: Easy to add new components, systems, and content
5. **Testability**: Pure functions, dependency injection, unit tests

---

## Technology Stack

### Core Libraries

```toml
[dependencies]
# ECS Framework
hecs = "0.10"           # Entity-Component-System

# Terminal UI
ratatui = "0.28"        # TUI rendering
crossterm = "0.28"      # Terminal control

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Random & Generation
rand = "0.8"
noise = "0.9"           # Perlin/Simplex noise for terrain

# Utilities
anyhow = "1.0"          # Error handling
thiserror = "1.0"       # Error derive macros
```

### Development Tools

- **rustfmt**: Code formatting
- **clippy**: Linting
- **cargo-watch**: Development workflow
- **criterion**: Benchmarking (optional)

---

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Application Layer                     │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │   Main     │  │  Game App  │  │   Event    │            │
│  │   Loop     │──│   State    │──│   Handler  │            │
│  └────────────┘  └────────────┘  └────────────┘            │
└───────────────────────────────┬─────────────────────────────┘
                                │
┌───────────────────────────────┴─────────────────────────────┐
│                        Systems Layer                         │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │  Input   │ │ Movement │ │  Combat  │ │   FOV    │       │
│  │  System  │ │  System  │ │  System  │ │  System  │       │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘       │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │   AI     │ │  Travel  │ │   Time   │ │  Death   │       │
│  │  System  │ │  System  │ │  System  │ │  System  │       │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘       │
└───────────────────────────────┬─────────────────────────────┘
                                │
┌───────────────────────────────┴─────────────────────────────┐
│                       ECS Core Layer                         │
│  ┌────────────────────────────────────────────────────┐     │
│  │                 World (hecs)                       │     │
│  │  ┌──────────────────────────────────────────┐     │     │
│  │  │         Entities & Components            │     │     │
│  │  └──────────────────────────────────────────┘     │     │
│  └────────────────────────────────────────────────────┘     │
│  ┌────────────────────────────────────────────────────┐     │
│  │                   Resources                        │     │
│  │  • Maps  • Camera  • Time  • Factions  • Quests   │     │
│  └────────────────────────────────────────────────────┘     │
└───────────────────────────────┬─────────────────────────────┘
                                │
┌───────────────────────────────┴─────────────────────────────┐
│                     Data & World Layer                       │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │ Overmap  │ │  Chunks  │ │   Maps   │ │   Save   │       │
│  │  System  │ │  System  │ │  System  │ │  System  │       │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘       │
└─────────────────────────────────────────────────────────────┘
```

---

## ECS Architecture

### Core ECS Concepts

**Entity**: Unique identifier (opaque handle)
**Component**: Pure data struct
**System**: Functions that operate on components
**Resource**: Global state not tied to entities

### Component Categories

#### 1. Core Components

```rust
// Identity & Position
Position        // x, y, layer (Normal/Cosmic)
Name            // String identifier
Description     // Flavor text

// Rendering
Renderable      // glyph, fg_color, bg_color, z_order
Viewshed        // visible tiles, range, dirty flag

// Markers
Player          // Marks player entity
Monster         // Marks monster entities
NPC             // Marks NPC entities
Item            // Marks item entities
```

#### 2. Combat Components

```rust
CombatStats     // hp, max_hp, power, defense
WantsToMelee    // Intent: target entity
MeleeWeapon     // damage_bonus, hit_bonus
Armor           // defense_bonus
```

#### 3. Cosmic Horror Components

```rust
TriMeter        // insight, sanity, notice (0-100)
Corruption      // corruption level (0-100)
Mutation        // active mutations list
Lexeme          // known lexemes
MadnessEffect   // active madness effects
```

#### 4. NPC Components

```rust
NPCData         // profession, home, faction
Schedule        // daily activities
Dialogue        // dialogue tree
Inventory       // items carried
Reputation      // faction standings
Morale          // happiness, needs
```

#### 5. World Components

```rust
Settlement      // name, size, population, buildings
Building        // type, interior, owner
Faction         // name, territory, power, relationships
Quest           // objectives, rewards, stage
```

#### 6. Intent Components

Intent components represent desires/actions to be processed:

```rust
WantsToMove     // destination (x, y)
WantsToMelee    // target entity
WantsToCast     // spell, target
WantsToPickup   // item entity
WantsToDrop     // item entity
WantsToUse      // item entity
WantsToCraft    // recipe
```

### Resources (Global State)

```rust
pub struct Resources {
    // World
    pub overmap: Overmap,
    pub chunks: ChunkManager,
    pub maps: MapSet,           // Normal & Cosmic layers

    // Rendering
    pub camera: Camera,

    // Game State
    pub mode: RunMode,          // AwaitingInput, PlayerTurn, MonstersTurn
    pub player_entity: Option<Entity>,
    pub time: WorldTime,

    // Systems
    pub log: GameLog,
    pub rng: StdRng,
    pub factions: FactionManager,
    pub quests: QuestManager,
    pub events: EventManager,

    // Meta
    pub save_data: SaveMetadata,
}
```

---

## Module Structure

### Directory Layout

```
src/
├── main.rs                 # Entry point
├── lib.rs                  # Library root
│
├── game/                   # Game application
│   ├── mod.rs
│   └── app.rs             # Main game loop
│
├── ecs/                   # ECS components & resources
│   ├── mod.rs
│   ├── components.rs      # All component definitions
│   └── resources.rs       # Resources struct
│
├── systems/               # Game systems
│   ├── mod.rs
│   ├── input.rs          # Input handling
│   ├── movement.rs       # Movement & collision
│   ├── combat.rs         # Combat & death
│   ├── ai.rs             # Monster AI
│   ├── fov.rs            # Field of view
│   ├── travel.rs         # Overworld travel
│   ├── time.rs           # Time progression
│   └── transition.rs     # Layer/location transitions
│
├── world/                 # World management
│   ├── mod.rs
│   ├── overmap.rs        # Overmap structure
│   ├── chunks.rs         # Chunk streaming
│   ├── terrain.rs        # Terrain types
│   ├── generator.rs      # World generation
│   ├── settlement.rs     # Settlements
│   ├── time.rs           # Time & calendar
│   ├── weather.rs        # Weather system
│   ├── events.rs         # Dynamic events
│   ├── corruption.rs     # Corruption spreading
│   └── resources.rs      # Resource nodes
│
├── map/                   # Local maps
│   ├── mod.rs
│   ├── map.rs            # Map structure
│   ├── tile.rs           # Tile types
│   ├── fov.rs            # FOV algorithms
│   ├── generator.rs      # Dungeon generation
│   └── building_gen.rs   # Building interiors
│
├── npc/                   # NPC systems
│   ├── mod.rs
│   ├── npc.rs            # NPC data
│   ├── ai.rs             # NPC AI
│   ├── schedule.rs       # Daily schedules
│   ├── dialogue.rs       # Dialogue system
│   ├── profession.rs     # Professions
│   └── corruption.rs     # NPC corruption
│
├── faction/               # Faction systems
│   ├── mod.rs
│   ├── faction.rs        # Faction data
│   ├── territory.rs      # Territory control
│   ├── reputation.rs     # Reputation tracking
│   └── warfare.rs        # Faction conflicts
│
├── quest/                 # Quest systems
│   ├── mod.rs
│   ├── quest.rs          # Quest data
│   ├── generator.rs      # Dynamic quests
│   └── main_quest.rs     # Story quests
│
├── magic/                 # Magic systems
│   ├── mod.rs
│   ├── lexeme.rs         # Lexeme definitions
│   ├── parser.rs         # Spell parser
│   └── spells.rs         # Spell effects
│
├── mutation/              # Mutation system
│   ├── mod.rs
│   └── mutation.rs       # Mutation data & effects
│
├── items/                 # Item systems
│   ├── mod.rs
│   ├── item.rs           # Item data
│   ├── inventory.rs      # Inventory management
│   └── food.rs           # Food items
│
├── crafting/              # Crafting systems
│   ├── mod.rs
│   ├── recipe.rs         # Recipe data
│   ├── crafting.rs       # Crafting execution
│   └── cooking.rs        # Cooking system
│
├── building/              # Construction systems
│   ├── mod.rs
│   ├── construction.rs   # Building mechanics
│   └── base.rs           # Player base
│
├── survival/              # Survival systems
│   ├── mod.rs
│   ├── hunger.rs         # Hunger & thirst
│   ├── rest.rs           # Fatigue & sleep
│   └── camping.rs        # Camping mechanics
│
├── ui/                    # UI rendering
│   ├── mod.rs
│   ├── renderer.rs       # Main renderer
│   ├── overmap_renderer.rs  # Overmap view
│   ├── dialogue_ui.rs    # Dialogue interface
│   ├── crafting_ui.rs    # Crafting interface
│   ├── quest_journal.rs  # Quest log
│   ├── map_screen.rs     # Full map view
│   └── help.rs           # Help screens
│
├── save/                  # Save/load systems
│   ├── mod.rs
│   ├── world_save.rs     # World state
│   └── compression.rs    # Save compression
│
├── lore/                  # Lore system
│   ├── mod.rs
│   └── codex.rs          # Lore codex
│
├── meta/                  # Meta-progression
│   ├── mod.rs
│   ├── achievements.rs   # Achievement tracking
│   └── statistics.rs     # Statistics
│
└── tutorial/              # Tutorial system
    ├── mod.rs
    └── tutorial.rs       # Tutorial sequence
```

### Data Files Structure

```
data/
├── items/
│   ├── weapons.json
│   ├── armor.json
│   ├── consumables.json
│   └── artifacts.json
│
├── monsters/
│   ├── mundane.json
│   ├── corrupted.json
│   └── reality_entities.json
│
├── recipes/
│   ├── weapons.json
│   ├── armor.json
│   ├── alchemy.json
│   ├── food.json
│   └── rituals.json
│
├── factions.json
│
├── lexemes.json
│
├── mutations.json
│
├── quests/
│   ├── main_story.json
│   └── side_quests.json
│
├── dialogue/
│   ├── common.json
│   ├── main_quest/
│   └── npcs/
│
├── lore/
│   ├── books.json
│   ├── notes.json
│   └── inscriptions.json
│
└── locations/
    └── unique/
        ├── daelspire.json
        ├── saelcairn.json
        └── the_wound.json
```

---

## Data Flow

### Turn-Based Game Loop

```
┌─────────────────────────────────────────────────────────┐
│                    Main Loop                             │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
        ┌──────────────────┐
        │  Render Frame    │
        └──────────────────┘
                  │
                  ▼
        ┌──────────────────┐
        │  Check RunMode   │
        └──────────────────┘
                  │
          ┌───────┴───────┬────────────┬──────────────┐
          ▼               ▼            ▼              ▼
   ┌─────────────┐  ┌──────────┐  ┌─────────┐  ┌──────────┐
   │ AwaitingInput│ │PlayerTurn│  │Monsters │  │GameOver │
   └──────┬──────┘  └────┬─────┘  └────┬────┘  └──────────┘
          │              │              │
          ▼              ▼              ▼
   ┌─────────────┐  ┌──────────┐  ┌─────────┐
   │HandleInput  │  │RunPlayer │  │RunMonsters│
   │   System    │  │ Systems  │  │  Systems  │
   └─────────────┘  └──────────┘  └─────────┘
```

### Player Turn Systems Order

```
Input System
  ↓
Movement System (process WantsToMove)
  ↓
Camera Update
  ↓
Melee Combat System (process WantsToMelee)
  ↓
Death System
  ↓
FOV Update System
  ↓
Set RunMode = MonstersTurn
```

### Monster Turn Systems Order

```
Monster AI System (generate intents)
  ↓
Movement System (process WantsToMove)
  ↓
Melee Combat System (process WantsToMelee)
  ↓
Death System
  ↓
Set RunMode = AwaitingInput
```

### Intent-Based Action System

Components represent *intentions* rather than direct actions:

```
1. Input System creates WantsToMove component
2. Player entity now has both Position and WantsToMove
3. Movement System processes all WantsToMove:
   - Check collision
   - Check blocking entities
   - If blocked by enemy: convert to WantsToMelee
   - If clear: update Position
   - Remove WantsToMove
4. Combat System processes all WantsToMelee:
   - Calculate damage
   - Apply to target
   - Log message
   - Remove WantsToMelee
```

**Benefits**:
- No borrowing conflicts (read intent, then mutate)
- Easy to add new action types
- Clear separation of decision and execution
- Can be queued or cancelled

---

## World Management

### Overmap System

```
Overmap (200x200 tiles)
├── Each tile = 1 kilometer
├── Contains:
│   ├── Terrain type
│   ├── Corruption level
│   ├── Weather
│   ├── Settlement reference
│   ├── Location reference
│   └── Discovered/visited flags
└── Rendered in separate view mode
```

### Chunk System

```
World divided into Chunks (24x24 local tiles each)
├── Only load chunks in radius around player
├── Unload distant chunks (save if modified)
├── LRU cache for frequently accessed chunks
├── Async loading (optional)
└── Seamless transitions between chunks
```

### Reality Layers

```
MapSet
├── Normal Layer (default)
│   ├── Standard world
│   └── Normal monsters
└── Cosmic Layer (high Insight)
    ├── Corrupted terrain
    ├── Reality entities visible
    └── Different geometry
```

Player can switch layers when Insight > 50:
- Normal → Cosmic: Reveals hidden truths
- Cosmic → Normal: Returns to "safe" reality
- Some areas force layer (corruption 100%)

### Location Types

1. **Overworld**: Fast travel, random encounters
2. **Settlement**: Buildings, NPCs, shops, quests
3. **Building**: Interior, furniture, NPCs
4. **Dungeon**: Procedural, monsters, loot
5. **Unique Location**: Hand-crafted, story content

---

## Performance Considerations

### Entity Management

- **Entity Culling**: Only process entities in loaded chunks
- **Spatial Partitioning**: Use overmap grid for quick lookups
- **Query Caching**: Cache expensive queries when possible
- **Component Packing**: Keep components small and cache-friendly

### World Streaming

- **Chunk Loading**: Load 3x3 chunks around player (9 total)
- **Lazy Generation**: Generate content when first visited
- **Save Dirty Chunks**: Only save modified chunks
- **Compression**: Use bincode + zstd for save files

### Rendering Optimization

- **Dirty Flags**: Only re-render changed areas
- **Viewport Culling**: Only render visible tiles
- **Z-Order Sorting**: Sort renderables once per frame
- **Double Buffering**: Use Ratatui's built-in buffering

### AI Optimization

- **Budget System**: Limit AI processing per frame
- **Sleep NPCs**: Don't process NPCs in unloaded chunks
- **Simple Pathfinding**: A* with low node limit
- **Behavior Trees**: Cache decisions

### Target Performance

- **60 FPS**: Smooth terminal rendering
- **< 100ms**: Save/load times
- **< 500MB**: Memory usage
- **< 5MB**: Save file size (compressed)

---

## Design Patterns

### 1. Entity-Component-System

**Why**: Composition over inheritance, data-oriented design

```rust
// Bad: Inheritance
class Monster extends Entity { ... }

// Good: Composition
let goblin = world.spawn((
    Position { x: 10, y: 10, layer: Normal },
    Renderable { glyph: 'g', ... },
    CombatStats { hp: 10, ... },
    Monster,
    Name("Goblin".into()),
));
```

### 2. Intent-Based Actions

**Why**: Avoids borrow checker issues, clear separation

```rust
// Phase 1: Generate intents
for (entity, (pos, player)) in world.query::<(&Position, &Player)>() {
    world.insert_one(entity, WantsToMove { dest_x, dest_y })?;
}

// Phase 2: Process intents (separate mutable iteration)
for (entity, wants_move) in world.query::<&WantsToMove>() {
    // Update position
}
```

### 3. Data-Driven Content

**Why**: Easy modding, faster iteration, no recompilation

```rust
// items/weapons.json
{
  "iron_sword": {
    "name": "Iron Sword",
    "damage": 10,
    "weight": 3.5,
    "value": 100
  }
}

// Load at runtime
let items: HashMap<String, ItemData> =
    serde_json::from_str(&content)?;
```

### 4. Builder Pattern

**Why**: Clear, flexible object construction

```rust
let quest = Quest::builder()
    .title("The Missing Merchant")
    .giver(npc_entity)
    .objective(QuestObjective::FindItem { item_id: "letter" })
    .reward(QuestReward::Gold(100))
    .build()?;
```

### 5. State Machine

**Why**: Clear game state management

```rust
enum RunMode {
    AwaitingInput,
    PlayerTurn,
    MonstersTurn,
    GameOver,
}

match resources.mode {
    RunMode::AwaitingInput => handle_input(world, resources)?,
    RunMode::PlayerTurn => run_player_systems(world, resources),
    RunMode::MonstersTurn => run_monster_systems(world, resources),
    RunMode::GameOver => render_game_over(frame),
}
```

### 6. Resource Manager

**Why**: Centralized global state access

```rust
pub struct Resources {
    pub maps: MapSet,
    pub camera: Camera,
    pub time: WorldTime,
    // ... all global state
}

fn system(world: &mut World, resources: &mut Resources) {
    // Access any resource needed
}
```

### 7. Event System

**Why**: Decoupled communication between systems

```rust
enum GameEvent {
    EntityDied(Entity),
    QuestCompleted(QuestId),
    CorruptionIncreased(i32),
}

// System A: Publish
resources.events.publish(GameEvent::EntityDied(entity));

// System B: Subscribe
for event in resources.events.drain() {
    match event {
        GameEvent::EntityDied(e) => { /* handle */ }
        // ...
    }
}
```

---

## Error Handling

### Strategy

- Use `anyhow::Result` for application errors
- Use `thiserror` for library errors
- Propagate with `?` operator
- Handle at appropriate level

```rust
use anyhow::{Result, Context};

fn load_world(path: &Path) -> Result<World> {
    let content = fs::read_to_string(path)
        .context("Failed to read world file")?;

    let world: World = serde_json::from_str(&content)
        .context("Failed to parse world data")?;

    Ok(world)
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_calculation() {
        let attacker = CombatStats { power: 10, .. };
        let defender = CombatStats { defense: 5, .. };

        let damage = calculate_damage(&attacker, &defender);

        assert_eq!(damage, 5);
    }
}
```

### Integration Tests

```rust
// tests/world_generation.rs
#[test]
fn test_world_generation() {
    let mut world = World::new();
    let mut resources = Resources::new(80, 50, 12345);

    generate_overmap(&mut resources.overmap, 12345);

    assert!(resources.overmap.get(0, 0).is_some());
    // ... more assertions
}
```

---

## Conclusion

This architecture provides:

✅ **Modularity**: Independent systems, easy to modify
✅ **Performance**: Efficient ECS, chunk streaming, culling
✅ **Extensibility**: Data-driven, easy to add content
✅ **Maintainability**: Clear structure, separation of concerns
✅ **Testability**: Pure functions, dependency injection

The foundation supports building a complex, living open-world roguelike while maintaining code quality and performance.

---

*"The architecture holds reality together... for now."*
