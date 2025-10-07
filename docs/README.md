# Documentation Index

**The Unraveling of Kándavael**

This directory contains all project documentation organized by purpose.

---

## 📋 Quick Navigation

| Category | Purpose | Contents |
|----------|---------|----------|
| **[planning/](planning/)** | Technical implementation plans | Architecture, systems specs, data formats, roadmap |
| **[design/](design/)** | Game design documents | Open world design, gameplay concepts |
| **[research/](research/)** | Reference material & analysis | CDDA feature analysis (research archive) |
| **[artbook/](artbook/)** | LaTeX artbook & lore | Visual design, bestiary, world lore |

---

## 📁 Directory Structure

```
docs/
├── README.md (you are here)
├── planning/                          # Technical Planning & Implementation
│   ├── ARCHITECTURE.md                # System architecture, ECS design, patterns
│   ├── SYSTEMS_SPECIFICATION.md       # Detailed specs for 17 game systems
│   ├── DATA_FORMATS.md                # JSON format specs for all content
│   ├── DEVELOPMENT_ROADMAP.md         # 11-12 month development timeline
│   └── IMPLEMENTATION_TASKS.md        # Task breakdown with 60+ tasks
│
├── design/                            # Game Design Documents
│   └── OPEN_WORLD_DESIGN.md          # Open world concept & mechanics
│
├── research/                          # Research & Reference Material
│   ├── CDDA_FEATURES_SUMMARY.md      # Cataclysm: DDA feature analysis
│   └── CDDA_FEATURES_IMPLEMENTATION_PLAN.md  # How CDDA features map to this game
│
└── artbook/                           # LaTeX Artbook
    ├── dungeon-clawler-artbook.tex   # Main LaTeX document
    ├── images/                        # Concept art
    ├── Makefile                       # Build automation
    └── README.md                      # Artbook build instructions
```

---

## 📖 Documentation Overview

### Planning Documents (Start Here)

**Essential reading for development:**

1. **[ARCHITECTURE.md](planning/ARCHITECTURE.md)** - *Read first*
   - System architecture and design patterns
   - ECS (Entity-Component-System) architecture
   - Module structure and data flow
   - Performance considerations
   - ~6,850 lines

2. **[SYSTEMS_SPECIFICATION.md](planning/SYSTEMS_SPECIFICATION.md)** - *Implementation reference*
   - Detailed specifications for 17 game systems
   - Algorithms, data structures, pseudocode
   - Movement, combat, FOV, AI, travel, time, weather
   - Corruption, tri-meter, magic, mutations, NPCs, factions, quests
   - Crafting, construction, save/load
   - ~13,420 lines

3. **[DATA_FORMATS.md](planning/DATA_FORMATS.md)** - *Content creation guide*
   - JSON format specifications for all game content
   - Items, monsters, recipes, factions, quests, dialogue
   - Lexemes (magic), mutations, lore, locations
   - Modding support guidelines
   - ~8,566 lines

4. **[DEVELOPMENT_ROADMAP.md](planning/DEVELOPMENT_ROADMAP.md)** - *Timeline & milestones*
   - 11-12 month development plan
   - 6 phases with weekly breakdowns
   - Milestones, risks, success criteria
   - Tools, practices, workflow
   - ~5,519 lines

5. **[IMPLEMENTATION_TASKS.md](planning/IMPLEMENTATION_TASKS.md)** - *Task tracking*
   - 60+ tasks across 5 development phases
   - Detailed subtasks, dependencies, estimates
   - Progress tracking templates
   - Risk register
   - ~2,172 lines

### Design Documents

**[OPEN_WORLD_DESIGN.md](design/OPEN_WORLD_DESIGN.md)** - *Game concept*
- Open world transformation concept
- Overmap system design (200×200 tiles)
- Living world simulation
- NPC schedules, faction dynamics
- Corruption spreading mechanics
- ~1,498 lines

### Research Archive

**Reference material from Cataclysm: Dark Days Ahead analysis:**

- **[CDDA_FEATURES_SUMMARY.md](research/CDDA_FEATURES_SUMMARY.md)** - Comprehensive CDDA feature breakdown (~528 lines)
- **[CDDA_FEATURES_IMPLEMENTATION_PLAN.md](research/CDDA_FEATURES_IMPLEMENTATION_PLAN.md)** - How CDDA features adapt to cosmic horror theme (~729 lines)

*Note: These are research documents, not implementation plans. They inform design but aren't direct blueprints.*

### Artbook

**[artbook/](artbook/)** - LaTeX artbook with lore, bestiary, and visual design

See [artbook/README.md](artbook/README.md) for build instructions.

---

## 🚀 Getting Started

### For Developers

1. Read **[ARCHITECTURE.md](planning/ARCHITECTURE.md)** to understand system design
2. Review **[DEVELOPMENT_ROADMAP.md](planning/DEVELOPMENT_ROADMAP.md)** for timeline
3. Check **[IMPLEMENTATION_TASKS.md](planning/IMPLEMENTATION_TASKS.md)** for current tasks
4. Reference **[SYSTEMS_SPECIFICATION.md](planning/SYSTEMS_SPECIFICATION.md)** during implementation
5. Use **[DATA_FORMATS.md](planning/DATA_FORMATS.md)** when creating content

### For Content Creators

1. Read **[DATA_FORMATS.md](planning/DATA_FORMATS.md)** for JSON specs
2. Review **[OPEN_WORLD_DESIGN.md](design/OPEN_WORLD_DESIGN.md)** for world concept
3. Check **[artbook/](artbook/)** for visual style and lore

### For Players/Modders

1. Check **[artbook/](artbook/)** for lore and world information
2. Read **[DATA_FORMATS.md](planning/DATA_FORMATS.md)** modding section
3. See game README in project root for installation

---

## 📊 Documentation Statistics

| Document | Lines | Category | Purpose |
|----------|-------|----------|---------|
| ARCHITECTURE.md | 6,850 | Planning | System design & patterns |
| SYSTEMS_SPECIFICATION.md | 13,420 | Planning | Implementation specs |
| DATA_FORMATS.md | 8,566 | Planning | JSON format specs |
| DEVELOPMENT_ROADMAP.md | 5,519 | Planning | Timeline & milestones |
| IMPLEMENTATION_TASKS.md | 2,172 | Planning | Task breakdown |
| OPEN_WORLD_DESIGN.md | 1,498 | Design | Game concept |
| CDDA_FEATURES_SUMMARY.md | 528 | Research | Feature analysis |
| CDDA_FEATURES_IMPLEMENTATION_PLAN.md | 729 | Research | Adaptation plan |
| **Total** | **39,282** | | |

---

## 🔄 Documentation Maintenance

### When to Update

- **ARCHITECTURE.md**: When system design changes
- **SYSTEMS_SPECIFICATION.md**: When system behavior changes
- **DATA_FORMATS.md**: When JSON schemas change
- **DEVELOPMENT_ROADMAP.md**: Weekly progress updates
- **IMPLEMENTATION_TASKS.md**: Daily task status updates

### Version Control

All documentation is version controlled with the code. See git history for changes.

---

## 📝 Contributing to Documentation

### Style Guide

- Use Markdown for all documentation
- Use clear headers and sections
- Include code examples where relevant
- Use tables for structured data
- Add diagrams (ASCII art) when helpful
- Keep line length reasonable (~100 chars)

### Templates

See existing docs for formatting examples. Maintain consistent structure:
1. Title and metadata
2. Table of contents
3. Overview/Introduction
4. Detailed sections
5. Examples
6. Conclusion

---

## 🔗 External Resources

- **Main README**: [../README.md](../README.md)
- **Source Code**: [../src/](../src/)
- **Game Design Doc**: [GAME_DESIGN.md](../GAME_DESIGN.md)
- **Technical Spec**: [TECHNICAL_SPEC.md](../TECHNICAL_SPEC.md)

---

## 📞 Help & Support

For questions about documentation:
1. Check relevant doc section first
2. Search git history for context
3. Review related design docs
4. Consult source code comments

---

*"Documentation is the map through which we navigate reality."*

*"Reality is documented. Reality is mutable. Documentation must adapt."*

---

**Last Updated**: 2025-10-07
**Status**: Living Documentation
