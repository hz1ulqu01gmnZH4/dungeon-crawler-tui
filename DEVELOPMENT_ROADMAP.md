# Development Roadmap

**Project**: The Unraveling of Kándavael
**Target Release**: 11-12 months from start
**Current Status**: Foundation Complete

---

## Timeline Overview

```
Month 1-3    Month 4-5    Month 6-7    Month 8-9    Month 10-11  Month 11-12
═══════════╦═══════════╦═══════════╦═══════════╦═══════════╦═══════════
Phase 1    ║ Phase 2   ║ Phase 3   ║ Phase 4   ║ Phase 5   ║ Release
Open World ║ Content   ║ Survival  ║ Living    ║ Polish &  ║ Prep &
Foundation ║ Systems   ║ & Base    ║ World     ║ Content   ║ Launch
           ║           ║ Building  ║ Sim       ║ Complete  ║
═══════════╩═══════════╩═══════════╩═══════════╩═══════════╩═══════════
```

---

## Phase 0: Current State ✅ COMPLETE

**Duration**: 2 weeks (completed)
**Status**: ✅ Done

### What We Have

✅ **ECS Architecture**
- hecs Entity-Component-System
- 11 components defined
- 6 systems implemented
- Intent-based action system

✅ **Basic Game Loop**
- Turn-based execution
- Player/Monster turn alternation
- Input handling (crossterm)
- Rendering (ratatui)

✅ **Local Maps**
- Procedural dungeon generation
- Rooms and corridors
- Wall/floor collision

✅ **Combat**
- Melee combat with damage calculation
- HP tracking and death
- Combat log messages

✅ **Field of View**
- Ray-casting FOV algorithm
- Fog of war
- Explored tile tracking
- Camera following player

✅ **Basic Monsters**
- Simple chase AI
- 3 monster types
- Vision-based detection

✅ **UI**
- Map view
- Status bar
- Message log
- Game over screen

### Technical Baseline

- **Lines of Code**: ~2,500
- **Build Time**: < 5 seconds
- **Performance**: 60 FPS in terminal
- **Memory**: ~20 MB

---

## Phase 1: Open World Foundation

**Duration**: 8-12 weeks (Months 1-3)
**Status**: 🔴 Not Started

### Goals

Transform from single-dungeon crawler to explorable open world with persistent state.

### Deliverables

#### Month 1: Overmap & Terrain

**Week 1-2: Overmap System**
- [ ] OvermapTile and Overmap data structures
- [ ] 50x50 tile overmap (MVP)
- [ ] Terrain type enums and data
- [ ] Coordinate conversion utilities
- [ ] Basic overmap rendering

**Week 3-4: Terrain Generation**
- [ ] Perlin noise terrain generation
- [ ] Biome assignment (plains, forest, mountains, rivers)
- [ ] Road network generation
- [ ] Settlement placement algorithm
- [ ] Dungeon scattering

#### Month 2: Travel & Settlements

**Week 5-6: Travel System**
- [ ] Overworld movement (arrow keys)
- [ ] Time progression (1 hour per tile)
- [ ] Terrain movement costs
- [ ] Random encounters
- [ ] Mini-battle maps for encounters

**Week 7-8: Settlements**
- [ ] Building generation (5+ types)
- [ ] Building interiors (rooms, furniture)
- [ ] Transition system (overmap ↔ local)
- [ ] Settlement data structure
- [ ] Name generation for settlements

#### Month 3: Time & World State

**Week 9-10: Time System**
- [ ] WorldTime structure (year, season, day, hour)
- [ ] Time display in UI
- [ ] Day/night cycle
- [ ] Lighting system
- [ ] NPC activity changes by time

**Week 11-12: Chunk Loading & Save**
- [ ] Chunk system (24x24 tiles)
- [ ] Load/unload chunks dynamically
- [ ] LRU cache for chunks
- [ ] Enhanced save system (entire world)
- [ ] Multiple save slots

### Success Metrics

✅ Can travel on overmap
✅ 50x50 tile world with varied terrain
✅ 5+ settlement types placed logically
✅ Can enter/exit buildings and dungeons
✅ Time passes and affects world
✅ Can save/load entire game state
✅ Performance remains good (60 FPS)

### Risks

⚠️ **Scope creep**: World generation is complex
- Mitigation: Start with simple algorithm, iterate

⚠️ **Performance**: Large world could slow down
- Mitigation: Chunk streaming, entity culling

⚠️ **Save file size**: Could be too large
- Mitigation: Compression, only save modified chunks

---

## Phase 2: Content Systems

**Duration**: 6-8 weeks (Months 4-5)
**Status**: 🔴 Not Started

### Goals

Populate world with living NPCs, factions, quests, and dynamic events.

### Deliverables

#### Month 4: NPCs & Dialogue

**Week 13-14: NPC Foundation**
- [ ] NPC data structure (name, profession, home)
- [ ] NPC spawning in settlements
- [ ] Daily schedule system (sleep, work, socialize)
- [ ] Basic pathfinding for NPCs

**Week 15-16: Dialogue System**
- [ ] Dialogue tree structure
- [ ] Dialogue UI (text display, choices)
- [ ] Context-aware responses
- [ ] Dialogue data files (JSON)

#### Month 5: Factions & Quests

**Week 17-18: Faction System**
- [ ] 6 major factions defined
- [ ] Faction territory visualization
- [ ] Reputation system
- [ ] Reputation effects (shop prices, access)

**Week 19-20: Quest System**
- [ ] Quest data structure
- [ ] Quest journal UI
- [ ] Dynamic quest generation (bounties, deliveries)
- [ ] Main quest line (Act 1: 5 quests)

#### Bonus Week: Dynamic World

**Week 21: Events & Corruption**
- [ ] World event system (raids, festivals, disasters)
- [ ] Corruption spreading algorithm
- [ ] Regional transformation based on corruption
- [ ] Corruption visualization on overmap

### Success Metrics

✅ 100+ NPCs with daily schedules
✅ Can talk to any NPC
✅ 6 factions with territories
✅ Reputation affects gameplay
✅ 5+ main quests, unlimited dynamic quests
✅ Corruption spreads visibly
✅ World feels alive

### Risks

⚠️ **NPC performance**: Too many NPCs could lag
- Mitigation: Only simulate loaded chunks

⚠️ **Quest bugs**: Complex quest chains could break
- Mitigation: Extensive testing, fallback options

---

## Phase 3: Survival & Base Building

**Duration**: 4-6 weeks (Months 6-7)
**Status**: 🔴 Not Started

### Goals

Add survival mechanics and player agency to shape the world.

### Deliverables

#### Month 6: Survival Systems

**Week 22-23: Hunger, Thirst, Fatigue**
- [ ] Hunger/thirst meters
- [ ] Food and water items
- [ ] Cooking system (recipes)
- [ ] Fatigue system
- [ ] Camping mechanics (bedroll, tent, campfire)

**Week 24-25: Resource Gathering**
- [ ] Resource nodes (trees, rocks, herbs, animals)
- [ ] Gathering mechanics with tools
- [ ] Resource respawning
- [ ] Inventory management improvements

#### Month 7: Crafting & Building

**Week 26-27: Crafting Framework**
- [ ] Recipe data structure
- [ ] Crafting UI
- [ ] 50+ craftable items across categories
- [ ] Recipe discovery system
- [ ] Blood magic (ritual crafting)

**Week 28: Construction System**
- [ ] Structure placement system
- [ ] 10+ buildable structures
- [ ] Player base system
- [ ] Base storage and workbenches

### Success Metrics

✅ Must eat, drink, and rest to survive
✅ Can gather materials from environment
✅ Can craft weapons, armor, tools, potions
✅ Can build player base
✅ Survival not tedious, adds depth

### Risks

⚠️ **Tedium**: Survival could feel like chore
- Mitigation: Balance decay rates, make interesting

⚠️ **Crafting complexity**: Too many recipes
- Mitigation: Clear categorization, recipe book

---

## Phase 4: World Simulation

**Duration**: 6-8 weeks (Months 8-9)
**Status**: 🔴 Not Started

### Goals

Make the world truly dynamic and reactive to player actions.

### Deliverables

#### Month 8: Environmental Depth

**Week 29-30: Advanced Weather & Seasons**
- [ ] Corrupted weather types
- [ ] Seasonal system (4 seasons)
- [ ] Environmental hazards
- [ ] Weather affects gameplay significantly

**Week 31-32: Faction Dynamics**
- [ ] Faction warfare system
- [ ] Settlement sieges
- [ ] Faction response to corruption
- [ ] Dynamic alliances and betrayals

#### Month 9: NPC Depth & Events

**Week 33-34: NPC Simulation**
- [ ] NPC needs and morale
- [ ] NPC corruption progression
- [ ] NPC death (permanent)
- [ ] Population dynamics

**Week 35-36: Large-Scale Events**
- [ ] Portal storms (reality tears)
- [ ] Mass corruption events
- [ ] Refugee crises
- [ ] Cult uprisings
- [ ] Apocalypse timer (doomsday clock)

### Success Metrics

✅ Seasons visibly change world
✅ Factions fight over territory
✅ NPCs react to corruption
✅ World transforms as corruption spreads
✅ Epic-scale events occur
✅ Player actions have lasting consequences

### Risks

⚠️ **Simulation complexity**: Too much happening
- Mitigation: Limit events per area, budget CPU

⚠️ **Player agency**: Events outside player control
- Mitigation: Player can always influence outcomes

---

## Phase 5: Polish & Content

**Duration**: 8-12 weeks (Months 10-11)
**Status**: 🔴 Not Started

### Goals

Complete narrative, add unique content, balance and polish everything.

### Deliverables

#### Month 10: Core Systems

**Week 37-38: Lexeme Magic System**
- [ ] Lexeme parser (grammar rules)
- [ ] 30+ lexemes defined
- [ ] Spell effects for all combinations
- [ ] Discovery system
- [ ] Balance sanity/notice costs

**Week 39-40: Mutation System**
- [ ] 50+ mutations across 5 categories
- [ ] Mutation acquisition triggers
- [ ] Visual changes for mutations
- [ ] Threshold transformations
- [ ] NPC reactions to mutations

**Week 41-42: Combat & AI**
- [ ] Enhanced combat (status effects, positioning)
- [ ] Spell casting in combat
- [ ] Better monster AI (flee, call help, use environment)
- [ ] Boss AI (phases, special attacks)
- [ ] Corrupted monster variants

#### Month 11: Content & Narrative

**Week 43-44: Main Story**
- [ ] Complete all 25+ main quests
- [ ] 3-5 different endings
- [ ] Cutscenes/special moments
- [ ] Final boss encounters
- [ ] Epilogue system

**Week 45-46: Unique Locations**
- [ ] 10+ hand-crafted locations
- [ ] Daelspire (capital city)
- [ ] Saelcairn Academy (magic school)
- [ ] The Wound (corruption epicenter)
- [ ] Memory Archive (meta-hub)
- [ ] Special loot and secrets

**Week 47-48: Lore & World**
- [ ] Lore system (books, notes, inscriptions)
- [ ] Lore codex UI
- [ ] 100+ lore entries
- [ ] Hidden lore and secrets
- [ ] Contradictory accounts (unreliable narrators)

#### Month 11-12: Polish

**Week 49-50: Balance & Testing**
- [ ] Combat balance (damage, HP, progression)
- [ ] Economy balance (prices, loot, costs)
- [ ] Progression curve testing
- [ ] Tri-meter balance (sanity/insight/notice)
- [ ] Corruption pacing

**Week 51-52: Extensive Playtesting**
- [ ] Multiple full playthroughs
- [ ] All playstyles (combat, magic, stealth, social)
- [ ] Bug hunting and fixing
- [ ] Quality of life improvements
- [ ] Performance optimization

**Week 53: Meta & Tutorial**
- [ ] Achievement system (30+ achievements)
- [ ] Statistics tracking
- [ ] In-game tutorial sequence
- [ ] Help screens and tooltips

**Week 54: Final Polish**
- [ ] UI polish (colors, animations, transitions)
- [ ] Accessibility features
- [ ] Code cleanup
- [ ] Final bug fixes

### Success Metrics

✅ 30+ usable lexeme combinations
✅ 50+ mutations available
✅ Complete story with multiple endings
✅ 10+ memorable unique locations
✅ Rich lore throughout world
✅ Balanced and fair gameplay
✅ No critical bugs
✅ Professional polish

### Risks

⚠️ **Burnout**: Long polish phase
- Mitigation: Take breaks, iterate in small chunks

⚠️ **Balance issues**: Hard to balance all systems
- Mitigation: Continuous playtesting, community feedback

---

## Phase 6: Release Preparation

**Duration**: 2 weeks (Month 12)
**Status**: 🔴 Not Started

### Goals

Prepare for public release.

### Deliverables

**Week 55: Documentation**
- [ ] README update (installation, features)
- [ ] Controls reference
- [ ] Game mechanics guide
- [ ] FAQ
- [ ] Troubleshooting guide
- [ ] Credits

**Week 56: Release Build**
- [ ] Version 1.0 tag
- [ ] Release builds (Linux, Windows, macOS)
- [ ] Packaging and installers
- [ ] Distribution (GitHub, itch.io, Steam)
- [ ] Launch announcement

### Success Metrics

✅ Builds work on all platforms
✅ Clear documentation
✅ Easy to install
✅ Publicly available
✅ Launch successful

---

## Milestones

### Milestone 1: Playable Overworld
**Target**: End of Month 3
**Criteria**:
- Can explore 50x50 overmap
- Can enter settlements and dungeons
- Time system working
- Can save/load game

### Milestone 2: Living World
**Target**: End of Month 5
**Criteria**:
- NPCs with schedules
- Dialogue system
- 6 factions active
- Quest system working
- Corruption spreading

### Milestone 3: Survival Systems
**Target**: End of Month 7
**Criteria**:
- Must manage hunger/thirst/fatigue
- Crafting system complete
- Can build structures
- 50+ recipes available

### Milestone 4: Dynamic World
**Target**: End of Month 9
**Criteria**:
- Seasons and weather
- Faction warfare
- NPC corruption
- Large-scale events
- World simulation running

### Milestone 5: Content Complete
**Target**: End of Month 11
**Criteria**:
- Magic system complete
- Mutation system complete
- Main story complete
- 10+ unique locations
- All systems implemented

### Milestone 6: Release Ready
**Target**: End of Month 12
**Criteria**:
- Balanced gameplay
- No critical bugs
- Documentation complete
- Release builds created
- Ready for public

---

## Weekly Development Cycle

### Rhythm

```
Monday:
  - Plan week tasks
  - Set goals
  - Review previous week

Tuesday-Friday:
  - Implementation
  - Testing
  - Iteration

Saturday:
  - Playtesting
  - Bug fixing
  - Code review

Sunday:
  - Rest
  - Design thinking
  - Documentation
```

### Daily Workflow

```
1. Morning:
   - Review tasks
   - Run: cargo clippy, cargo test
   - Start implementation

2. Afternoon:
   - Continue implementation
   - Test changes
   - Commit progress

3. Evening:
   - Playtest changes
   - Update task list
   - Plan next day
```

---

## Tools & Practices

### Version Control

```bash
# Feature branches
git checkout -b feature/task-1.5

# Frequent commits
git commit -m "Task 1.5: Add settlement placement"

# Tag milestones
git tag v0.1.0-milestone1
```

### Testing Strategy

```bash
# Run tests frequently
cargo test

# Test release build
cargo build --release
cargo run --release

# Profile performance
cargo install cargo-flamegraph
cargo flamegraph
```

### Quality Checks

```bash
# Before every commit
cargo fmt              # Format code
cargo clippy -- -D warnings  # Lint
cargo test            # Run tests
cargo build           # Ensure builds
```

---

## Risk Management

### High-Risk Areas

1. **World Generation**: Complex, easy to mess up
   - **Mitigation**: Start simple, iterate, test extensively

2. **Performance**: Large world could lag
   - **Mitigation**: Profile early, optimize hot paths, chunk streaming

3. **Save Corruption**: Could lose player progress
   - **Mitigation**: Extensive testing, backup system, validation

4. **Scope Creep**: Feature bloat
   - **Mitigation**: Stick to roadmap, defer nice-to-haves

5. **Burnout**: Long project
   - **Mitigation**: Take breaks, celebrate milestones, stay motivated

### Contingency Plans

**If behind schedule**:
- Cut nice-to-have features
- Reduce scope (e.g., fewer unique locations)
- Extend timeline by 1-2 months

**If performance issues**:
- Reduce world size (200x200 → 100x100)
- More aggressive chunking
- Simplify AI

**If critical bug found late**:
- Delay release if necessary
- Quality over timeline

---

## Success Criteria

### Technical Success

✅ Runs at 60 FPS on modest hardware
✅ Save/load works reliably
✅ No game-breaking bugs
✅ Cross-platform compatibility
✅ Memory usage < 500 MB

### Gameplay Success

✅ Engaging core loop (explore, survive, discover)
✅ Meaningful choices (factions, mutations, magic)
✅ Emergent stories from world simulation
✅ Multiple playstyles viable
✅ Replayability (different endings, builds)

### Content Success

✅ 10+ hours of content for first playthrough
✅ 30+ hours for completionist
✅ Complete narrative arc with satisfying endings
✅ Rich lore to discover
✅ Enough variety to stay interesting

### Community Success

✅ Positive reception
✅ Active player base
✅ Modding community emerges
✅ Constructive feedback for future updates

---

## Post-Release Plans

### Version 1.1 (1-2 months after release)
- Bug fixes from community feedback
- Balance adjustments
- Quality of life improvements
- Minor content additions

### Version 1.5 (3-6 months after release)
- New faction
- New unique locations
- Extended story content
- New mutation categories

### Version 2.0 (6-12 months after release)
- Expanded world (200x200 → 400x400)
- New reality layer
- Multiplayer (optional)
- Modding tools

---

## Resource Requirements

### Hardware

- Development machine: Modern laptop/desktop
- RAM: 8+ GB
- Storage: 50+ GB

### Software

- Rust toolchain
- IDE (VS Code + rust-analyzer)
- Git
- Terminal emulator

### Time Commitment

- **Phase 1-2**: 20-30 hours/week
- **Phase 3-4**: 15-25 hours/week
- **Phase 5**: 30-40 hours/week (crunch)
- **Total**: ~800-1000 hours

---

## Conclusion

This roadmap provides a clear path from current prototype to complete game over 11-12 months.

The plan is ambitious but achievable with consistent effort and scope management.

Each phase builds on the previous, allowing for playable game at each milestone.

---

**Next Steps**:
1. Review and approve roadmap
2. Set up task tracking (GitHub Projects, Trello, etc.)
3. Begin Phase 1, Task 1.1
4. Commit to weekly progress updates

---

*"The road is long, but every journey begins with a single step."*

*"And every step takes us closer to The Unraveling..."*
