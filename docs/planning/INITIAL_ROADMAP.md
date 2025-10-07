# Implementation Roadmap - Dungeon Clawler TUI

## Project Overview
Building "The Unraveling of Candlevale" - a text-based roguelike with high-fantasy, cosmic horror, and isekai elements.

## Tech Stack Decision
**Language**: Rust
**UI Framework**: Ratatui
**Target Platforms**: Linux, macOS, Windows Terminal

## Milestone 1: Foundation (Week 1-2)
### Goal: Playable prototype with basic movement and display

- [ ] Project setup with Cargo
- [ ] Basic game loop with Ratatui
- [ ] Simple room structure and movement
- [ ] Text rendering with word wrap
- [ ] Command parser (move, look, examine)
- [ ] Basic save/load system

**Deliverable**: Walk around 5 connected rooms with descriptions

## Milestone 2: Core Mechanics (Week 3-4)
### Goal: Tri-meter system and basic combat

- [ ] Implement Insight/Sanity/Notice meters
- [ ] Create status effects system
- [ ] Basic combat engine
- [ ] Simple enemy AI (move toward player, attack)
- [ ] HP and death/restart mechanics
- [ ] Item system (pick up, drop, use)

**Deliverable**: Fight enemies while managing three resources

## Milestone 3: Lexeme Magic System (Week 5-6)
### Goal: Word-based spell creation

- [ ] Lexeme parser and grammar rules
- [ ] Spell effect system
- [ ] 10 initial lexemes with combinations
- [ ] Spell failure/backfire mechanics
- [ ] Visual feedback for spell casting
- [ ] Lexeme discovery through exploration

**Deliverable**: Combine words to create and cast spells

## Milestone 4: Dual-Layer Reality (Week 7-8)
### Goal: Two-layer world system

- [ ] Implement Veil and True layer maps
- [ ] Layer switching mechanics
- [ ] Different descriptions per layer
- [ ] Layer-specific entities and items
- [ ] Sanity cost for True layer viewing
- [ ] Layer-based puzzle design

**Deliverable**: Switch between realities to solve problems

## Milestone 5: Procedural Generation (Week 9-10)
### Goal: Infinite replayability

- [ ] Room template system
- [ ] Wave function collapse implementation
- [ ] Procedural room descriptions
- [ ] Enemy placement algorithm
- [ ] Loot distribution system
- [ ] Difficulty scaling by depth

**Deliverable**: New dungeon layout each run

## Milestone 6: Meta-Progression (Week 11-12)
### Goal: Death isn't the end

- [ ] Memory Archive hub
- [ ] Persistent unlocks between runs
- [ ] Meta-knowledge system
- [ ] Corruption mechanics
- [ ] Character customization
- [ ] Achievement system

**Deliverable**: Progress carries between deaths

## Milestone 7: Narrative Engine (Week 13-14)
### Goal: Rich storytelling

- [ ] NPC dialogue system
- [ ] Quest/objective tracking
- [ ] Lore discovery mechanics
- [ ] Multiple ending paths
- [ ] Faction reputation system
- [ ] Dynamic event generation

**Deliverable**: Engaging story that reacts to player choices

## Milestone 8: Polish Phase (Week 15-16)
### Goal: Game feel and accessibility

- [ ] Text effects (glitch, fade, shake)
- [ ] Color coding for important elements
- [ ] Sound effects (if terminal supports)
- [ ] Settings menu (difficulty, accessibility)
- [ ] Tutorial/onboarding
- [ ] Performance optimization

**Deliverable**: Polished, accessible experience

## Stretch Goals

### Advanced Features (If time permits)
- [ ] Mod support via Lua scripting
- [ ] Online leaderboards
- [ ] Daily challenge runs
- [ ] Twitch integration for stream voting
- [ ] Mobile terminal support
- [ ] Multiplayer notes/messages system

## File Structure
```
dungeon-clawler-tui/
├── Cargo.toml
├── README.md
├── docs/
│   ├── GAME_DESIGN.md
│   ├── TECHNICAL_SPEC.md
│   ├── LORE_BIBLE.md
│   ├── GAMEPLAY_EXAMPLES.md
│   └── IMPLEMENTATION_ROADMAP.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── game/
│   │   ├── mod.rs
│   │   ├── state.rs
│   │   ├── player.rs
│   │   └── world.rs
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── combat.rs
│   │   ├── magic.rs
│   │   ├── sanity.rs
│   │   └── generation.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── renderer.rs
│   │   ├── input.rs
│   │   └── effects.rs
│   ├── data/
│   │   ├── mod.rs
│   │   ├── items.rs
│   │   ├── enemies.rs
│   │   ├── lexemes.rs
│   │   └── rooms.rs
│   └── utils/
│       ├── mod.rs
│       ├── save.rs
│       └── random.rs
├── assets/
│   ├── text/
│   │   ├── descriptions.json
│   │   ├── dialogue.json
│   │   └── lore.json
│   └── data/
│       ├── items.json
│       ├── enemies.json
│       └── lexemes.json
└── tests/
    ├── integration/
    └── unit/
```

## Development Workflow

### Daily Tasks
1. Morning: Review previous day's work
2. Code: 2-3 hour focused session
3. Test: Manual playtesting
4. Document: Update relevant docs
5. Commit: Atomic commits with clear messages

### Weekly Reviews
- Monday: Plan week's goals
- Wednesday: Mid-week check-in
- Friday: Review and demo progress

### Git Branch Strategy
- `main`: Stable releases
- `develop`: Active development
- `feature/*`: Individual features
- `bugfix/*`: Bug fixes
- `experimental/*`: Trying new ideas

## Testing Strategy

### Unit Tests
- Game logic (combat calculations)
- Parser functionality
- Spell combinations
- Save/load integrity

### Integration Tests
- Full game loop
- Multi-system interactions
- Performance benchmarks

### Playtesting Focus
- Early game (First 10 minutes)
- Death and restart flow
- Spell discovery experience
- Endgame scenarios

## Performance Targets
- 60 FPS rendering
- <100ms input response
- <50MB RAM usage
- <500ms save/load time
- Support 1000+ room dungeons

## Release Plan

### Alpha Release (Week 8)
- Core mechanics complete
- 3 floors of content
- Basic meta-progression
- Friends & family testing

### Beta Release (Week 12)
- All systems implemented
- 10+ floors of content
- Multiple endings available
- Public testing

### 1.0 Release (Week 16)
- Fully polished
- 20+ floors
- Achievement system
- Documentation complete
- Marketing materials ready

## Marketing Hooks
- "Every word is a spell"
- "Reality has layers"
- "Death is just the beginning"
- "Your Earth knowledge is your weapon"
- "Madness is a valid strategy"

## Success Metrics
- 100+ daily active players
- 4.5+ star rating
- 50+ user-generated guides
- Active speedrun community
- Fan art and fiction

## Risk Mitigation

### Technical Risks
- **Scope creep**: Strict milestone adherence
- **Performance issues**: Profile early and often
- **Platform bugs**: Test on all targets weekly

### Design Risks
- **Complexity overload**: Gradual tutorial
- **Balance issues**: Automated playtesting
- **Narrative confusion**: Clear lore delivery

### Timeline Risks
- **Delays**: Buffer time built into schedule
- **Burnout**: Regular breaks planned
- **Feature cuts**: Priority system in place

## Post-Launch Support
- Week 1: Hotfixes for critical bugs
- Month 1: Balance patch based on player data
- Month 2: First content update (New Game+)
- Month 3: Community mod support
- Ongoing: Monthly events and challenges