# Rust Roguelike Tech Stack

> Synthesized recommendations from GPT5 and Grok4 consultations

## Core Dependencies

### 1. Terminal UI: **Ratatui** ⭐
```toml
ratatui = "0.28"
crossterm = "0.28"
```
- **Consensus**: Both AI models strongly recommend Ratatui
- Modern successor to tui-rs with excellent widget system
- Efficient partial redraws for turn-based gameplay
- Built-in canvas widget perfect for ASCII maps
- Strong community support and documentation

### 2. Entity Component System (ECS)

**Primary Choice**: **Hecs** (for simplicity)
```toml
hecs = "0.10"
```
- Lightweight and minimal with fast queries
- Perfect for turn-based games (no forced parallelism)
- Simple API, easy to learn

**Alternative**: **Bevy ECS** (for features)
```toml
bevy_ecs = "0.14"
```
- More powerful with automatic parallelization
- Built-in change detection
- Better for complex simulations

### 3. Serialization: **Serde + Multiple Formats**
```toml
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"      # Human-readable for debugging
bincode = "1.3"   # Binary for production saves
```
- Use RON for debugging and configuration
- Use Bincode for compact save files
- Support versioning for save compatibility

### 4. Procedural Generation
```toml
rand = { version = "0.8", features = ["small_rng"] }
rand_pcg = "0.3"          # Deterministic generation
noise = "0.9"             # Perlin/Simplex noise
bracket-lib = "0.8"       # Roguelike-specific algorithms
```
- Rand for core randomness with seedable RNG
- Noise for organic terrain generation
- Bracket-lib for dungeon generation algorithms

### 5. Utilities
```toml
anyhow = "1.0"       # Error handling
thiserror = "1.0"    # Custom error types
smallvec = "1.11"    # Memory-efficient collections
```

## Architecture Patterns

### Game Loop Structure
```rust
enum GameState {
    MainMenu,
    Playing,
    Inventory,
    Paused,
    GameOver,
}

// Turn-based main loop
loop {
    render(&mut terminal, &game)?;
    let input = wait_for_input()?;
    update_game_state(&mut game, input);
    process_turn(&mut game);
}
```

### Input Handling
- Event-driven with blocking input (wait for keypress)
- Map keys to actions via enum
- Support vi-keys (hjkl) and numpad
- Modal states for menus and dialogs

### Performance Optimizations
1. **Rendering**: Dirty rectangles - only redraw changed tiles
2. **ECS**: Query only visible entities
3. **Generation**: Lazy chunk-based world generation
4. **Pathfinding**: Cache A* results for AI
5. **Memory**: Use SmallVec for small collections

## Project Structure
```
src/
├── main.rs
├── game/
│   ├── state.rs         # State machine
│   ├── world.rs         # World representation
│   └── turn_manager.rs  # Turn sequencing
├── components/          # ECS components
│   ├── position.rs
│   ├── stats.rs
│   └── inventory.rs
├── systems/            # ECS systems
│   ├── movement.rs
│   ├── combat.rs
│   └── ai.rs
├── ui/
│   ├── renderer.rs     # Ratatui rendering
│   └── input.rs        # Input handling
├── generation/
│   └── dungeon.rs      # Procedural generation
└── save/
    └── serialization.rs # Save/load system
```

## Testing Strategy
```toml
[dev-dependencies]
quickcheck = "1.0"        # Property-based testing
quickcheck_macros = "1.0"
```

- Unit tests for game logic
- Property-based tests for generation
- Deterministic seeds for reproducible tests
- Test save/load compatibility

## Implementation Priorities

1. **Phase 1**: Minimal Viable Game
   - Ratatui + Crossterm for display
   - Basic ECS with Hecs
   - Simple movement and rendering

2. **Phase 2**: Core Mechanics
   - Turn-based system
   - Basic combat
   - Procedural dungeon generation

3. **Phase 3**: Polish
   - Save/load system
   - AI behaviors
   - Inventory and items
   - Performance optimizations

## Consensus Points

Both AI models agreed on:
- **Ratatui** for UI (unanimous choice)
- **Serde** for serialization
- **Rand** for random generation
- Event-driven input handling
- State machine architecture
- Turn-based game loop
- Dirty rectangle rendering for performance

## Key Decisions

| Component | Choice | Reasoning |
|-----------|--------|-----------|
| UI | Ratatui | Best balance of features and performance |
| ECS | Hecs | Simpler for turn-based; upgrade to Bevy ECS if needed |
| Save Format | RON + Bincode | Human-readable for debug, binary for production |
| RNG | rand + PCG | Deterministic and fast |
| Architecture | State Machine | Clean separation of game states |

## Performance Targets
- Turn processing: < 100ms
- Render time: < 16ms (60 FPS capability)
- Save/load: < 1 second for large worlds
- Memory: < 100MB for typical gameplay

## Development Tips
1. Start simple with Ratatui + Hecs + rand
2. Use bracket-lib examples for inspiration
3. Test on low-end terminals
4. Profile with flamegraph for bottlenecks
5. Keep saves backward compatible with versioning