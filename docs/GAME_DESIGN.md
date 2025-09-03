# Dungeon Clawler TUI - Game Design Document

## Core Concept
**The Unraveling of Kándavael** - A text-based roguelike that begins as heroic fantasy and slowly reveals itself as cosmic horror through your own "successful" actions.

## Narrative Hook
You've been summoned to the prosperous kingdom of Kándavael as their prophesied Champion! The benevolent Dael Tríthae (Dawn Triumvirate) has blessed you with power to cleanse the land of the tyrannical Dusk Rhael's lingering corruption. Villages celebrate your arrival, children throw flowers, and hope returns to the realm.

But something feels off. Prayers are answered too quickly. Blessed items won't come off. The monsters you're "saving" people from seem to be running FROM your light, not attacking. And why do the defeated Wardens keep begging you to "keep the mouth shut"?

By the time you realize the Dael Tríthae aren't gods but Dæl Trith (draining throats), and your every victory weakens the Vârð (sutures) keeping them out, you've already done too much damage. The Dusk Rhael wasn't a tyrant—he was the Dûšk Rhal (Twilight Stitcher) who tried to seal the wound in reality.

## Core Mechanics

### 1. Progressive Meter System
```
Early Game Display:
Valor    ████████░░ [80%] - Your heroic might
Faith    ██████░░░░ [60%] - Devotion to the Dawn
Renown   ███░░░░░░░ [30%] - Fame across the realm

Late Game Reality (same meters, revealed names):
Radiance ████████░░ [80%] - How bright you burn for Them
Sanity   ██████░░░░ [60%] - What's left of your mind
Notice   ███░░░░░░░ [30%] - Their attention on you
```
The meters never change mechanically—only your understanding of what they represent.

### 2. Evolving Magic System

**Early Game: "Divine Prayers"**
- `LIGHT + WARD = Protective Aura`
- `HEAL + SELF = Restoration`
- `BLESS + BLADE = Sacred Weapon`
*"The Dawn Triumvirate grants you holy words!"*

**Late Game: Same Spells, True Nature Revealed**
- `LIGHT + WARD = Beacon for Them`
- `HEAL + SELF = Burn Tomorrow's Health`
- `BLESS + BLADE = Blade Can't Be Dropped`
*"You're speaking the parasites' feeding language"*

### 3. Dual-Layer Reality
Peel back the skin of the world to see the cancer beneath:
- **Veil**: The rotting fairy tale the kingdom tells itself while it dies
- **True**: The writhing truth where walls are meat and prayers are parasites

### 4. Blood Magic vs Earthcraft
Two paths to power, both stained:
- **Earthcraft**: Gunpowder and penicillin in a world that bleeds magic
- **Blood Sorcery**: Feed the old words with sacrifice - yours or others'

## Key Features

### Death & Rebirth
- Permadeath with meta-progression
- Memory Archive between runs stores discovered knowledge
- Overusing meta-knowledge corrupts future runs

### Mask System (Classes)
- **Oathbreaker Knight**: Armor fused with flesh, honor turned to rust
- **Mad Scholar**: Eyes that bleed truth, knowledge that eats memories  
- **Bone Witch**: Pacts written in marrow, magic that costs fingers
- **Flagellant**: Faith through suffering, miracles born from pain
- **Plague Engineer**: Modern knowledge twisted, chemistry meets necromancy

### Isekai Elements
- Language learning as progression
- Modern knowledge as special abilities
- Cultural clash affecting NPC relations
- Memory Palace hub between deaths

## Setting Highlights

### Daelspire (Dælspîr)
A fortress-tomb where they burn their dead for light because the sun died screaming three years ago. Children born here have no word for "dael" (dawn), only "dæl" (drain). The bells are made from skulls that ring warnings when something that used to be human gets too close.

### The Saelcairn Academy (Sælkhairn)
A Gothic cancer where students dissect angels to understand why heaven went silent. Professors teach with their tongues cut out - knowledge here is contagious. The library's "sael" (holy) texts are actually "sæl" (skin) texts, each one still warm.

### The Vaelmark Road (Vêlmark)
A trade route paved with the fossilized spine of the world-serpent that the gods killed to make the earth. What merchants call the "Vaelmark" (Vale Mark) is actually the "Vêlmark" (Throat Scar), and every inn is a shrine to forgetting what travels through. Every inn is a shrine to forgetting what you've seen between the bones.

## Technical Overview

### Interface
- Pure text-based with rich formatting
- ASCII art for maps
- Context-sensitive command system
- Sanity-affected text distortion

### Core Loop
1. Explore procedurally generated dungeons
2. Discover lexeme fragments and lore
3. Manage resources (HP, Sanity, Insight, Notice)
4. Make choices that affect future runs
5. Die and carry knowledge forward

## Unique Selling Points

1. **Words as Weapons** - Spell creation through language
2. **Unreliable Interface** - Text distortions carry meaning
3. **Madness as Progress** - Low sanity unlocks unique paths
4. **Living World** - Your Earth knowledge permanently changes the realm
5. **No Perfect Build** - Every power gain increases cosmic attention

## MVP Scope

### Phase 1: Foundation
- [ ] Basic movement and room generation
- [ ] Simple combat system
- [ ] Tri-meter implementation
- [ ] Basic lexeme combinations (5-10 spells)

### Phase 2: Core Features
- [ ] Mask system with 3 initial classes
- [ ] Dual-layer reality switching
- [ ] Save/load with permadeath
- [ ] 20+ lexeme combinations

### Phase 3: Depth
- [ ] Memory Archive meta-hub
- [ ] Earthcraft system
- [ ] Oath/pact mechanics
- [ ] 50+ rooms/events

### Phase 4: Polish
- [ ] Full narrative integration
- [ ] 5 different endings
- [ ] Achievement system
- [ ] Accessibility features