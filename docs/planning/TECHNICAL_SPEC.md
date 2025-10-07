# Technical Specification - Dungeon Clawler TUI

## Technology Stack

### Core Language: Rust
- Performance-critical for text processing
- Memory safety for complex state management
- Excellent terminal UI libraries (ratatui, crossterm)

### Key Dependencies
```toml
[dependencies]
ratatui = "0.28"          # Terminal UI framework
crossterm = "0.28"        # Cross-platform terminal control
serde = "1.0"             # Serialization for saves
serde_json = "1.0"        # JSON save files
rand = "0.8"              # RNG for procedural generation
petgraph = "0.6"          # Graph algorithms for dungeon generation
regex = "1.10"            # Lexeme parsing
anyhow = "1.0"            # Error handling
tokio = "1.40"            # Async runtime for input handling
```

## Architecture Overview

```
┌─────────────────────────────────────┐
│          Main Game Loop             │
├─────────────────────────────────────┤
│         Input Handler               │
├─────────────────────────────────────┤
│         Game State Manager          │
├──────────┬──────────┬───────────────┤
│  World   │ Entity   │   Narrative   │
│  System  │ System   │   Engine      │
├──────────┼──────────┼───────────────┤
│ Renderer │ Audio    │ Save System   │
└──────────┴──────────┴───────────────┘
```

## Core Systems

### 1. Game State Management
```rust
pub struct GameState {
    player: Player,
    world: World,
    narrative: NarrativeState,
    meta_progression: MetaData,
    rng: StdRng,
}

pub struct Player {
    position: Position,
    stats: PlayerStats,
    inventory: Vec<Item>,
    lexemes: HashSet<Lexeme>,
    mask: Option<Mask>,
    oaths: Vec<Oath>,
}

pub struct PlayerStats {
    hp: i32,
    max_hp: i32,
    insight: f32,  // 0.0 - 100.0
    sanity: f32,   // 0.0 - 100.0
    notice: f32,   // 0.0 - 100.0
}
```

### 2. Dual-Layer World System
```rust
pub struct World {
    veil_layer: Layer,
    true_layer: Layer,
    active_layer: LayerType,
}

pub struct Layer {
    rooms: HashMap<Position, Room>,
    entities: Vec<Entity>,
}

pub struct Room {
    description: String,
    exits: HashMap<Direction, Position>,
    contents: Vec<EntityId>,
    properties: RoomProperties,
}
```

### 3. Lexeme Magic System
```rust
pub struct LexemeEngine {
    known_lexemes: HashSet<String>,
    grammar_rules: GrammarRules,
}

impl LexemeEngine {
    pub fn parse_spell(&self, input: &str) -> Result<Spell> {
        // Parse input into lexeme components
        // Validate grammar
        // Return constructed spell or error
    }
}

pub struct Spell {
    components: Vec<Lexeme>,
    effect: SpellEffect,
    sanity_cost: f32,
    notice_increase: f32,
}
```

### 4. Narrative Engine
```rust
pub struct NarrativeEngine {
    events: HashMap<EventId, Event>,
    active_quests: Vec<Quest>,
    dialogue_state: DialogueState,
    text_processor: TextProcessor,
}

pub struct TextProcessor {
    sanity_level: f32,
    distortion_rules: Vec<DistortionRule>,
}

impl TextProcessor {
    pub fn process(&self, text: &str) -> String {
        // Apply sanity-based text distortions
        // Handle redactions, glitches, whispers
    }
}
```

### 5. Procedural Generation
```rust
pub struct DungeonGenerator {
    seed: u64,
    templates: Vec<RoomTemplate>,
    wave_function: WaveFunctionCollapse,
}

impl DungeonGenerator {
    pub fn generate_floor(&mut self, depth: i32) -> Floor {
        // Use wave function collapse for coherent layouts
        // Apply depth-based difficulty scaling
        // Integrate narrative elements
    }
}
```

## Data Structures

### Save File Format
```json
{
  "version": "1.0.0",
  "run_data": {
    "seed": 12345,
    "player": { /* player state */ },
    "current_floor": 3,
    "world_state": { /* serialized world */ }
  },
  "meta_progression": {
    "unlocked_lexemes": ["FIRE", "SHIELD", "UN"],
    "discovered_lore": ["entry_1", "entry_2"],
    "memory_palace": { /* persistent upgrades */ },
    "corruption_level": 0.15
  }
}
```

### Command Parser
```rust
pub enum Command {
    Move(Direction),
    Examine(String),
    Cast(String),       // Lexeme combination
    Use(ItemId),
    SwitchLayer,
    Craft(Recipe),
    Talk(EntityId),
    Rest,
}

impl CommandParser {
    pub fn parse(&self, input: &str) -> Result<Command> {
        // Natural language parsing
        // Shortcut recognition (n, s, e, w)
        // Context-sensitive suggestions
    }
}
```

## Rendering System

### Terminal UI Layout
```
┌──────────────────────────────────────────────────┐
│ The Unraveling of Candlevale        Floor 3      │
├──────────────────────────────────────────────────┤
│                                                   │
│  [Main viewport - room description and actions]  │
│                                                   │
│  You stand in a candlelit chamber. Ancient       │
│  runes pulse with eldritch light. A door leads   │
│  north, stairs descend to the east.              │
│                                                   │
├──────────────────────────────────────────────────┤
│ HP: 45/50  │ Insight: ████░░ │ Sanity: ███░░░   │
│            │ Notice:  ██░░░░ │                   │
├──────────────────────────────────────────────────┤
│ > examine runes                                  │
└──────────────────────────────────────────────────┘
```

### Text Effects
```rust
pub enum TextEffect {
    Normal,
    Italic,          // Whispers, thoughts
    Bold,            // Emphasis, important
    Strikethrough,   // Redacted, forbidden
    Glitch,          // Random character replacement
    Fade,            // Gradually disappearing
    Shake,           // Trembling text
}
```

## Performance Considerations

### Memory Management
- Use arena allocators for per-floor data
- String interning for repeated descriptions
- Lazy loading of narrative content

### Optimization Targets
- 60 FPS terminal refresh rate
- < 100ms command response time
- < 50MB RAM usage
- Instant save/load (< 500ms)

## Platform Support

### Primary Targets
- Linux (native)
- macOS (native)
- Windows (via Windows Terminal)

### Terminal Requirements
- 80x24 minimum resolution
- 256 color support preferred
- UTF-8 encoding for special characters

## Testing Strategy

### Unit Tests
- Lexeme parser validation
- Spell effect calculations
- Procedural generation determinism

### Integration Tests
- Save/load cycle integrity
- Command parsing coverage
- Game state transitions

### Playtesting Metrics
- Average run duration
- Death frequency by cause
- Lexeme discovery rate
- Ending achievement distribution

## Development Phases

### Phase 1: Core Engine (Weeks 1-4)
- Basic game loop
- Terminal rendering
- Command parsing
- Simple combat

### Phase 2: Magic System (Weeks 5-8)
- Lexeme parser
- Spell effects
- Tri-meter implementation

### Phase 3: World Building (Weeks 9-12)
- Procedural generation
- Dual-layer system
- NPC interactions

### Phase 4: Meta Systems (Weeks 13-16)
- Save/load
- Memory Palace
- Multiple endings
- Achievements

### Phase 5: Polish (Weeks 17-20)
- Narrative content
- Balance tuning
- Accessibility features
- Performance optimization