# Dungeon Clawler TUI - The Unraveling of Kándavael

A deceptive text-based roguelike that begins as heroic high-fantasy but slowly reveals cosmic horror through your own "successful" actions. Every victory corrupts the world further.

## 🗡️ Game Concept

You've been summoned to the prosperous kingdom of Kándavael as their prophesied Champion! The benevolent Dael Tríthae (Dawn Trinity) has blessed you to cleanse the land of the tyrannical Dusk Rhael's corruption. Villages celebrate your arrival with festivals!

But something feels... off. Why do prayers echo with a second voice? Why do the defeated Vardain (Wardens) plead "keep the mouth shut"? And why does the language itself seem to be changing?

By the time you realize the Dael Tríthae are actually Dæl Trith (draining throats) and every beacon you light weakens the Vârð (sutures) holding reality together, it's too late. The Dusk Rhael was the Dûšk Rhal—the last hero who tried to stitch the wound closed.

### Key Features

- **Blood Lexeme Magic**: Carve reality with words torn from dying gods' tongues
- **Death Spiral Meters**: Knowledge brings madness, madness brings attention, attention brings oblivion
- **Flesh Behind the Veil**: Peel back the world's skin to see the writhing cancer beneath
- **Earthcraft Heresy**: Gunpowder and antibiotics in a world that bleeds when you think
- **Persistent Corruption**: Each death feeds the thing that lives between runs
- **Wound the World**: Your choices leave scars that never heal

## Quick Start

```bash
# Build the project
cargo build

# Run in development mode
cargo run

# Run with optimizations
cargo run --release
```

## Controls

- **Movement**: Arrow keys or vim keys (h/j/k/l)
- **Examine**: `x` followed by direction
- **Cast Spell**: `c` then type lexeme combination
- **Switch Layer**: `tab` (costs sanity)
- **Inventory**: `i`
- **Rest**: `.` (pass turn)
- **Pause**: `p`
- **Resume**: `r`
- **Quit**: `q`

## 📚 Documentation

- [Game Design](docs/GAME_DESIGN.md) - Core mechanics and features
- [Technical Spec](docs/TECHNICAL_SPEC.md) - Implementation details  
- [Lore Bible](docs/LORE_BIBLE.md) - World building and story
- [Gameplay Examples](docs/GAMEPLAY_EXAMPLES.md) - How it plays
- [Implementation Roadmap](docs/IMPLEMENTATION_ROADMAP.md) - Development plan
- [Tech Stack](docs/TECH_STACK.md) - Technical architecture decisions

## Architecture

- **UI Framework**: Ratatui with Crossterm backend
- **ECS**: Hecs for entity management
- **Serialization**: Serde with RON/Bincode
- **Procedural Generation**: rand + noise

## Project Structure

```
src/
├── main.rs              # Entry point and game loop
├── game/                # Core game logic
├── components/          # ECS components
├── systems/            # Game systems
├── ui/                 # Terminal UI
├── generation/         # Procedural generation
└── save/               # Save/load system
```

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Check code
cargo clippy
cargo fmt
```

## License

MIT