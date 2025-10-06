# Systems Specification

**Version**: 1.0
**Last Updated**: 2025-10-07

---

## Table of Contents

1. [Movement System](#movement-system)
2. [Combat System](#combat-system)
3. [Field of View System](#field-of-view-system)
4. [AI System](#ai-system)
5. [Travel System](#travel-system)
6. [Time System](#time-system)
7. [Weather System](#weather-system)
8. [Corruption System](#corruption-system)
9. [Tri-Meter System](#tri-meter-system)
10. [Lexeme Magic System](#lexeme-magic-system)
11. [Mutation System](#mutation-system)
12. [NPC System](#npc-system)
13. [Faction System](#faction-system)
14. [Quest System](#quest-system)
15. [Crafting System](#crafting-system)
16. [Construction System](#construction-system)
17. [Save/Load System](#saveload-system)

---

## Movement System

### Purpose

Handle entity movement with collision detection, blocking, and intent-based actions.

### Components Used

- `Position` - Current location
- `WantsToMove` - Movement intent
- `Player` - Player marker
- `Monster` - Monster marker

### Algorithm

```
For each entity with WantsToMove:
  1. Get destination (dest_x, dest_y)
  2. Check if in bounds
  3. Check if tile is walkable (not wall)
  4. Check if entity blocks destination
     - If blocked by entity:
       * Check if hostile
       * If hostile: convert to WantsToMelee
       * If friendly: cancel move
     - If not blocked:
       * Update Position
       * Set Viewshed dirty flag
  5. Remove WantsToMove component
```

### Collision Layers

- **Terrain**: Walls block all movement
- **Entities**: Monsters/NPCs block space
- **Player**: Occupies one tile

### Edge Cases

- **Out of Bounds**: Cancel move
- **Diagonal Movement**: Not supported (grid-only)
- **Swapping**: Not allowed (one entity per tile)
- **Layer Mismatch**: Can't interact across layers

### Performance

- O(n) where n = entities with WantsToMove
- Spatial hash for entity lookup (future optimization)

---

## Combat System

### Purpose

Process melee combat intents, calculate damage, apply to targets.

### Components Used

- `WantsToMelee` - Combat intent (attacker → target)
- `CombatStats` - HP, power, defense
- `Name` - For combat log

### Damage Formula

```
base_damage = attacker.power
defense = defender.defense
damage = max(1, base_damage - defense)
```

### Combat Flow

```
For each entity with WantsToMelee:
  1. Get attacker stats
  2. Get target entity
  3. Get defender stats
  4. Calculate damage
  5. Apply damage to defender.hp
  6. Log message: "X hits Y for Z damage"
  7. Remove WantsToMelee component
```

### Death Handling

Separate death system runs after combat:

```
For each entity with CombatStats where hp <= 0:
  1. Check if player
     - If player: Set RunMode = GameOver
     - If not player: Despawn entity
  2. Log death message
```

### Future Enhancements

- **Weapons**: Add weapon damage bonus
- **Armor**: Add armor defense bonus
- **Critical Hits**: Chance for extra damage
- **Status Effects**: Apply burn, poison, etc.
- **Dodge/Block**: Chance to avoid damage
- **Damage Types**: Physical, fire, reality, etc.

---

## Field of View System

### Purpose

Calculate visible tiles from entity positions using ray-casting.

### Components Used

- `Position` - Observer location
- `Viewshed` - Visible tiles list, range, dirty flag

### Algorithm

Ray-casting FOV:

```
For each entity with Viewshed where dirty == true:
  1. Get position
  2. Clear current visible list
  3. For each angle 0-360 degrees:
     a. Cast ray from position
     b. For each tile in ray up to range:
        - If tile blocks vision: stop ray
        - Add tile to visible list
  4. Set dirty = false
  5. If entity is player:
     - Update map.visible
     - Update map.revealed
```

### Visibility Rules

- **Walls Block**: Can't see through walls
- **Entities Don't Block**: Can see past creatures
- **Range Limit**: Typically 8-12 tiles
- **Layer-Specific**: Only see current layer

### Map Visibility Tracking

- `visible`: Currently in FOV (this turn)
- `revealed`: Ever seen (persists)
- Rendering: Show visible tiles full color, revealed tiles dimmed

### Performance

- Ray-casting: O(range × 360) per entity
- Optimization: Symmetric shadowcasting (future)
- Only update when dirty flag set

---

## AI System

### Purpose

Control monster behavior, movement, and actions.

### AI Types

#### 1. Passive
- Does not attack
- Flees when hurt
- Examples: Deer, rabbit

#### 2. Melee Chaser
- Chases player when visible
- Attacks in melee range
- Examples: Goblin, wolf

#### 3. Ranged
- Maintains distance
- Shoots when in range
- Retreats if player too close

#### 4. Caster
- Uses spells/abilities
- Tactical positioning
- Examples: Cultist, void mage

#### 5. Reality Entity
- Special cosmic horror behavior
- Phases between layers
- Sanity damage aura

### Simple Chase AI

```
For each monster with Position, Viewshed:
  1. Check if player visible in Viewshed
  2. If not visible: wander randomly
  3. If visible:
     a. Calculate direction to player
     b. If adjacent: already in range (do nothing)
     c. If not adjacent: move closer
        - Create WantsToMove toward player
```

### Pathfinding

Basic: Manhattan distance, move toward player
Advanced: A* with node limit (future)

### AI Budget

To maintain performance:
- Limit AI calculations per frame
- Process closest monsters first
- Skip monsters in unloaded chunks

---

## Travel System

### Purpose

Handle overworld travel, time progression, random encounters.

### Overworld Movement

```
On arrow key press in overworld mode:
  1. Calculate new overmap position
  2. Check terrain walkability
  3. If walkable:
     a. Update player overmap position
     b. Advance time (1 hour per tile)
     c. Update stamina/fatigue
     d. Roll for random encounter
     e. Update weather
     f. Check corruption spread
  4. If not walkable: cancel move
```

### Travel Costs

| Terrain | Time | Stamina |
|---------|------|---------|
| Road | 30 min | 1 |
| Plains | 1 hour | 2 |
| Forest | 1.5 hours | 3 |
| Hills | 2 hours | 4 |
| Mountains | 3 hours | 6 |
| Swamp | 2 hours | 5 |

### Random Encounters

```
Roll per tile:
  Base chance: 10%
  Modified by:
    - Terrain type (+/- 5%)
    - Time of day (night +10%)
    - Corruption level (+corruption/10 %)

If encounter:
  1. Determine type (travelers, bandits, monsters)
  2. Generate encounter map
  3. Spawn entities
  4. Enter encounter mode
```

### Transition System

```
Enter Location:
  1. Save overmap state
  2. Load/generate local map
  3. Spawn player at entrance
  4. Load NPCs/monsters
  5. Switch to local mode

Exit Location:
  1. Save local map state
  2. Return to overmap
  3. Restore overmap position
  4. Continue travel
```

---

## Time System

### Purpose

Track world time, day/night cycle, seasons, scheduling.

### Time Structure

```rust
pub struct WorldTime {
    pub year: i32,      // Starting year: 762
    pub season: Season, // Spring, Summer, Autumn, Winter
    pub day: i32,       // 1-90 per season
    pub hour: i32,      // 0-23
    pub minute: i32,    // 0-59
}
```

### Time Progression

| Action | Time Cost |
|--------|-----------|
| Move (local) | 6 seconds |
| Move (overmap) | 1 hour |
| Melee attack | 6 seconds |
| Cast spell | 1 minute |
| Rest | 8 hours |
| Craft item | Varies (10 min - 8 hours) |
| Eat/drink | 5 minutes |
| Read book | 1 hour |

### Day/Night Cycle

```
Hours:
  0-5: Deep Night (darkest)
  6-7: Dawn (lighting)
  8-17: Day (full light)
  18-19: Dusk (dimming)
  20-23: Night (dark)
```

### Effects by Time

**Night**:
- Reduced vision range (-2 tiles)
- NPCs sleep (homes)
- Nocturnal monsters more common
- Some shops closed

**Day**:
- Full vision
- NPCs active (schedules)
- Safer travel
- Shops open

### Seasons

**Spring** (Days 1-90):
- Moderate weather
- Crops growing
- Travel easy

**Summer** (Days 91-180):
- Hot, dry
- Festivals
- Best travel conditions

**Autumn** (Days 181-270):
- Cool, rainy
- Harvest time
- Increased monster activity

**Winter** (Days 271-360):
- Cold, snow
- Harsh travel
- Food scarce
- NPCs stay indoors

### Calendar Events

```json
{
  "events": [
    { "season": "Summer", "day": 45, "name": "Midsummer Festival" },
    { "season": "Winter", "day": 1, "name": "The Long Night" }
  ]
}
```

---

## Weather System

### Purpose

Dynamic weather affecting visibility, travel, and atmosphere.

### Weather Types

#### Normal Weather
- **Clear**: No effects, full visibility
- **Cloudy**: Slightly dimmed
- **Rain**: Reduced vision (-1), slower travel
- **Storm**: Heavy rain + lightning, -2 vision, much slower
- **Fog**: -3 vision, eerie
- **Snow**: -1 vision, cold damage

#### Corrupted Weather (High Corruption)
- **Blood Rain**: Red rain, sanity drain
- **Reality Storm**: Layers flicker, forced shifts
- **Whisper Fog**: Dense fog, hear voices, sanity loss
- **Void Darkness**: Unnatural darkness, can't see past 3 tiles
- **Flesh Snow**: Organic snow that writhes

### Weather Generation

```
Every 4 hours:
  1. Check current weather
  2. Roll for weather change (30% chance)
  3. If change:
     a. Consider season
     b. Consider region
     c. Consider corruption level
     d. Select new weather
  4. Interpolate transition
```

### Weather Effects

```rust
match weather {
    Weather::Rain => {
        vision_modifier = -1;
        travel_speed *= 0.8;
        mood_modifier = -5;
    }
    Weather::BloodRain => {
        vision_modifier = -2;
        sanity_drain = 1 per hour;
        corruption_spread_rate *= 1.5;
        npc_panic = true;
    }
}
```

---

## Corruption System

### Purpose

Track and spread reality corruption across the world.

### Corruption Tracking

Each overmap tile has corruption level (0-100):

```rust
pub struct OvermapTile {
    pub terrain: TerrainType,
    pub corruption: u8,  // 0-100
    pub corruption_source: bool,
    // ...
}
```

### Corruption Sources

1. **Beacons**: The five beacon towers (constant radiation)
2. **Player Actions**: Lighting beacons, defeating wardens
3. **Rituals**: Performing blood magic
4. **Lexeme Usage**: Casting spells increases local corruption
5. **Reality Tears**: Dungeons, special locations

### Spreading Algorithm

```
Every game day:
  For each tile with corruption > 0:
    For each adjacent tile:
      If random() < (source_corruption / 100):
        Increase adjacent corruption by 1
        Clamp to 100

  For each beacon (if active):
    Increase corruption in radius by 2
    Radius = 10 tiles
```

### Corruption Effects by Level

| Level | Effects |
|-------|---------|
| 0-20 | Normal - Whispers, nightmares |
| 21-40 | Unsettling - Strange creatures, NPCs nervous |
| 41-60 | Dangerous - Reality warping, corrupted monsters |
| 61-80 | Collapsing - Mass NPC corruption, rifts |
| 81-100 | Fallen - Void consumed, cosmic layer only |

### Corruption Effects on Gameplay

```rust
match tile_corruption {
    0..=20 => {
        // Subtle hints
        occasional_whispers();
        nightmare_chance += 0.1;
    }
    21..=40 => {
        // Visible changes
        terrain_discoloration();
        spawn_corrupted_variants();
        npc_morale -= 10;
    }
    41..=60 => {
        // Reality breaks
        forced_layer_switches();
        sanity_drain_constant();
        buildings_shift();
    }
    61..=80 => {
        // Mass chaos
        npcs_go_mad();
        factions_collapse();
        rifts_spawn();
    }
    81..=100 => {
        // Total collapse
        force_cosmic_layer();
        only_void_entities();
        player_can_only_flee();
    }
}
```

---

## Tri-Meter System

### Purpose

Track player's Insight, Sanity, and Notice—the core cosmic horror mechanics.

### The Three Meters

#### 1. Insight (0-100)

**What it represents**: Knowledge of the true nature of reality.

**Increases from**:
- Reading forbidden texts (+5-20)
- Witnessing cosmic events (+10-30)
- Performing rituals (+5-15)
- Encountering reality entities (+2-10)
- Using lexeme magic (+1-5 per spell)

**Effects**:
- **< 20**: Cannot see reality layer, limited lexeme access
- **20-40**: Can glimpse reality layer briefly
- **40-60**: Can switch layers voluntarily, more lexemes
- **60-80**: Reality layer always visible (overlay), all lexemes
- **80-100**: Can manipulate reality, but...

**Drawbacks**:
- Higher insight = faster sanity drain
- Can't "unlearn" what you know
- NPCs fear/avoid high-insight characters

#### 2. Sanity (0-100)

**What it represents**: Mental stability and grip on reality.

**Decreases from**:
- Seeing horrific things (-5-20)
- Using magic (-1-10 per spell)
- High insight passive drain (-1 per hour at 80+)
- Corrupted weather (-1 per hour)
- Witnessing NPC death (-2-5)
- Reading dangerous lore (-5-15)

**Increases from**:
- Resting in safe locations (+10-20)
- Interacting with friendly NPCs (+1-5)
- Completing goals (+5-10)
- Using sanity-restoring items (+10-30)

**Effects**:
- **< 20**: Madness effects, hallucinations
- **20-40**: Debuffs, anxiety
- **40-60**: Normal function
- **60-80**: Peak performance
- **80-100**: Healthy mind

**Madness Effects** (< 20 sanity):
```
Random effects:
- Visual hallucinations (fake enemies)
- Auditory hallucinations (whispers)
- Paranoia (NPCs seem hostile)
- Compulsions (must do specific actions)
- Blackouts (lose time)
- Split personality (different dialogue options)
```

#### 3. Notice (0-100)

**What it represents**: How much "They" are aware of you.

**Increases from**:
- Using lexeme magic (+1-10 per spell)
- Performing rituals (+10-30)
- Defeating reality entities (+5-15)
- Interfering with corruption (+5)
- High insight for extended time (+1 per day at 80+)

**Decreases from**:
- Time passage (-1 per week)
- Hiding in mundane life (-5 per week)
- Using concealment rituals (-10-30)

**Effects**:
- **< 20**: Unnoticed, safe
- **20-40**: Occasional attention
- **40-60**: Active interest—more encounters
- **60-80**: Hunted—Wardens and entities seek you
- **80-100**: Target—Boss entities spawn to eliminate you

**High Notice Consequences**:
```rust
match notice {
    0..=20 => {
        // Safe
    }
    21..=40 => {
        encounter_rate *= 1.2;
        reality_entity_chance += 0.05;
    }
    41..=60 => {
        warden_patrols_track_player();
        nightmares_every_sleep();
        cultists_seek_you();
    }
    61..=80 => {
        boss_entities_spawn();
        cant_hide_anywhere();
        factions_know_your_power();
    }
    81..=100 => {
        dael_sends_avatar();
        reality_warps_around_you();
        final_confrontation_imminent();
    }
}
```

### Balancing Act

The player must balance:
- **High Insight**: Needed for power, but drains sanity and raises notice
- **High Sanity**: Needed to function, but limits power
- **Low Notice**: Needed to survive, but gained by using power

**Optimal Strategy**: Rise insight for power, use sparingly, rest often, manage notice carefully.

---

## Lexeme Magic System

### Purpose

Word-based magic system where players combine words to create spells.

### Grammar Structure

```
[Modifier] [Element/Action] [Target]

Examples:
- IGNIS FOE = Fireball at enemy
- MAG IGNIS OMNI = Great firestorm all around
- VITA SELF = Heal self
- VOL SUMMON GRASP = Summon void servant
```

### Lexeme Categories

1. **Elements**: IGNIS (fire), AQUA (water), TERRA (earth), AER (air)
2. **Modifiers**: MAG (great), PARVUS (lesser), MULTI (multiple)
3. **Actions**: SUMMON, BIND, BREAK, SEAL, OPEN
4. **Targets**: SELF, FOE, OMNI (all), AREA
5. **Cosmic**: VOL (void), LUX (light), UMBRA (shadow), CARN (flesh)

### Spell Parsing

```rust
fn parse_spell(lexemes: &[String]) -> Result<Spell> {
    let mut power_mult = 1.0;
    let mut element = None;
    let mut target = None;

    for (i, lex) in lexemes.iter().enumerate() {
        match lexeme_type(lex) {
            LexemeType::Modifier => {
                power_mult *= modifier_value(lex);
            }
            LexemeType::Element => {
                if element.is_some() {
                    return Err("Multiple elements");
                }
                element = Some(lex);
            }
            LexemeType::Target => {
                target = Some(lex);
            }
            // ... more cases
        }
    }

    validate_combination(power_mult, element, target)?;

    Ok(Spell {
        lexemes: lexemes.to_vec(),
        power: calculate_power(element, power_mult),
        sanity_cost: calculate_sanity_cost(lexemes),
        notice_increase: calculate_notice(lexemes),
        effect: generate_effect(element, target, power_mult),
    })
}
```

### Discovery System

Lexemes are discovered through:
- Reading books
- Finding inscriptions
- NPC teaching
- Experimentation (random combinations)
- Story progression

### Spell Effects

Each spell has:
- **Damage/Healing**: Based on element and power
- **Sanity Cost**: Higher power = higher cost
- **Notice Increase**: Using magic attracts attention
- **Special Effects**: Summons, buffs, debuffs

### Example Spells

```
IGNIS FOE:
  Effect: 2d6 fire damage to target
  Sanity: -2
  Notice: +1

MAG IGNIS OMNI:
  Effect: 4d6 fire damage in 5-tile radius
  Sanity: -10
  Notice: +5

VOL SUMMON GRASP:
  Effect: Summon void imp for 10 turns
  Sanity: -15
  Notice: +10

VITA SELF:
  Effect: Heal 3d8+6 HP
  Sanity: -5
  Notice: +2
```

---

## Mutation System

### Purpose

Physical transformations from corruption, enhancing abilities at cost of humanity.

### Mutation Categories

1. **Cosmic Insight**: Enhanced perception, sanity drain
2. **Flesh Warping**: Physical enhancements, appearance changes
3. **Dimensional Taint**: Reality phasing, healing resistance
4. **Parasitic Growth**: Living organisms on body
5. **Void Touched**: Void corruption, NPC fear

### Acquiring Mutations

```
Mutations gained through:
1. Corruption thresholds (automatic at certain levels)
2. Rituals (intentional transformation)
3. Items (cursed artifacts)
4. Events (exposure to reality entities)
5. Random chance (corruption zones)
```

### Mutation Tiers

- **Tier 1**: Minor changes, small bonuses
- **Tier 2**: Noticeable changes, significant bonuses
- **Tier 3**: Major changes, powerful abilities

### Threshold Transformations

Gaining 5+ mutations in one category triggers threshold:

```
Cosmic Insight Threshold:
  - Third, fourth, fifth eyes
  - See all realities at once
  - Constant sanity drain
  - Can't close eyes
  - NPCs flee on sight

Flesh Warping Threshold:
  - Body becomes monstrous
  - Extreme strength/toughness
  - Can't wear normal equipment
  - Guards attack on sight
  - Can't enter cities
```

### Mutation Conflicts

Some mutations are incompatible:
- Void Touched ↔ Flesh Titan (different forms)
- Multiple Eye ↔ Blind Sight (contradictory)

### NPC Reactions

```rust
fn npc_reaction_modifier(player_mutations: &[Mutation]) -> i32 {
    let mut modifier = 0;
    for mutation in player_mutations {
        modifier += mutation.npc_reaction_penalty;
    }
    // Visible mutations make NPCs hostile
    if modifier < -50 {
        return -100;  // Hostile on sight
    }
    modifier
}
```

---

## NPC System

### Purpose

Populate world with living, scheduled NPCs with needs and personalities.

### NPC Data

```rust
pub struct NPC {
    pub name: String,
    pub profession: Profession,
    pub home: BuildingId,
    pub schedule: DailySchedule,
    pub needs: Needs,
    pub morale: i32,
    pub corruption_level: u8,
    pub faction: FactionId,
    pub reputation: i32,
}
```

### Daily Schedules

```
Example: Blacksmith
  6:00 - Wake, eat breakfast at home
  8:00 - Walk to smithy
  8:30-12:00 - Work (crafting, customers)
  12:00 - Lunch break
  13:00-18:00 - Work
  18:00 - Walk to tavern
  18:30-21:00 - Socialize, drink
  21:00 - Walk home
  21:30 - Sleep
```

### NPC Needs

```rust
pub struct Needs {
    pub hunger: u8,      // 0-100
    pub thirst: u8,
    pub sleep: u8,
    pub safety: u8,
    pub social: u8,
}
```

NPCs will interrupt schedule to meet critical needs.

### Morale System

```rust
morale = (100 - hunger/2 - thirst/2 + safety + social/2)

if morale < 20 {
    npc.behavior = NPCBehavior::Fleeing;
} else if morale < 40 {
    npc.behavior = NPCBehavior::Hiding;
} else {
    npc.behavior = NPCBehavior::Normal;
}
```

### NPC Corruption

As regional corruption increases, NPCs corrupt:

```
0-20: Normal
21-40: Nervous, suspicious
41-60: Paranoid, some flee
61-80: Many corrupted, turn hostile
81-100: All corrupted or dead
```

### NPC Death

NPCs can die from:
- Combat
- Starvation
- Corruption
- Events (raids, disasters)

Death is permanent, affects quests and world state.

---

## Faction System

### Purpose

Track faction territories, relationships, and dynamics.

### Faction Data

```rust
pub struct Faction {
    pub id: String,
    pub name: String,
    pub faction_type: FactionType,
    pub territory: Vec<SettlementId>,
    pub power: i32,
    pub relationships: HashMap<FactionId, i32>,
    pub player_reputation: i32,
}
```

### Reputation System

Player reputation with each faction:

```
-100 to -51: Hated (attacked on sight)
-50 to -1: Hostile (no trade, aggressive)
0 to 24: Neutral (basic interaction)
25 to 49: Friendly (discounts, minor help)
50 to 74: Ally (special quests, backup)
75 to 100: Champion (major benefits, command NPCs)
```

### Reputation Changes

```rust
// Quest completion
+10 to +25

// Kill faction member
-50

// Trade
+1 per transaction

// Faction-specific actions
Varies
```

### Faction Warfare

Factions can go to war:

```
Every week:
  For each pair of factions:
    If relationship < -50 and both have power:
      Chance of war declaration
      If war:
        - Battle calculations
        - Territory changes
        - NPC casualties
        - Player can choose side
```

### Faction Corruption

As corruption spreads, factions react:

**Royal Crown**: Denies problem, martial law
**Holy Church**: Hunts corrupted, some become cult
**Wardens**: Fight corruption, but revealed as complicit
**Cult**: Gains power, recruits
**Merchants**: Flee or profit from chaos

---

## Quest System

### Purpose

Drive narrative, provide goals, reward progression.

### Quest Structure

```rust
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quest_type: QuestType,
    pub stages: Vec<QuestStage>,
    pub current_stage: usize,
    pub rewards: QuestRewards,
    pub failure_conditions: Vec<FailureCondition>,
    pub time_limit: Option<WorldTime>,
}
```

### Quest Types

1. **Main Quest**: Story progression
2. **Side Quest**: Optional content
3. **Dynamic Quest**: Procedurally generated
4. **Faction Quest**: Faction-specific
5. **Personal Quest**: Character backstory

### Quest Stages

```rust
pub struct QuestStage {
    pub id: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub all_complete: bool,
}

pub enum Objective {
    GoToLocation(LocationId),
    TalkToNPC(NPCId),
    KillTarget(EntityId, u32),
    FetchItem(ItemId, u32),
    DeliverItem(ItemId, NPCId),
    Escort(NPCId, LocationId),
    Investigate(ClueId),
    Wait(Duration),
}
```

### Quest Progression

```
Player completes objective:
  1. Check if objective matches any active quests
  2. Mark objective complete
  3. Check if all objectives in stage complete
  4. If stage complete:
     a. Advance to next stage
     b. Update quest markers
     c. Trigger stage events
  5. If final stage complete:
     a. Grant rewards
     b. Update world state
     c. Unlock follow-up quests
     d. Log completion
```

### Dynamic Quest Generation

```rust
fn generate_bounty_quest(settlement: &Settlement) -> Quest {
    let monster = select_nearby_threat();
    let reward = calculate_reward(monster.challenge_rating);

    Quest {
        name: format!("{} Bounty", monster.name),
        description: format!("Hunt the {} terrorizing nearby.", monster.name),
        stages: vec![
            QuestStage {
                objectives: vec![
                    Objective::KillTarget(monster.id, 1),
                    Objective::ReturnToQuestGiver,
                ],
            },
        ],
        rewards: QuestRewards {
            gold: reward,
            reputation: faction_reputation(settlement.faction, 5),
        },
    }
}
```

---

## Crafting System

### Purpose

Allow players to create items from resources.

### Recipe Structure

```rust
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub result: CraftingResult,
    pub requirements: CraftingRequirements,
    pub time_minutes: u32,
    pub corruption_cost: u8,
}

pub struct CraftingRequirements {
    pub ingredients: Vec<(ItemId, u32)>,
    pub tools: Vec<ItemId>,
    pub location_type: LocationType,
    pub skill: Option<(Skill, u32)>,
}
```

### Crafting Process

```
Player initiates craft:
  1. Check ingredients in inventory
  2. Check tools available
  3. Check at correct location (forge, lab, etc.)
  4. Check skill level
  5. If all pass:
     a. Remove ingredients
     b. Start crafting (time passes)
     c. Roll for success (based on skill)
     d. If success: create item
     e. If failure: some ingredients lost
     f. Gain skill XP
```

### Crafting Categories

1. **Blacksmithing**: Weapons, armor, tools
2. **Alchemy**: Potions, poisons, elixirs
3. **Cooking**: Food, meals
4. **Tailoring**: Cloth armor, bags
5. **Tinkering**: Gadgets, traps
6. **Blood Magic**: Ritual items, cursed artifacts

### Special: Ritual Crafting

```
Blood magic rituals:
  Requirements:
    - Specific ingredients (often grim)
    - Lexeme knowledge
    - Ritual circle
    - Sanity cost
    - Corruption increase

  Process:
    - Draw circle (time + chalk)
    - Place ingredients
    - Chant lexemes
    - Channel power (sanity drain)
    - Result created (if successful)

  Risks:
    - Failure can summon entities
    - Always increases corruption
    - Raises Notice significantly
    - Can mutate player
```

---

## Construction System

### Purpose

Allow players to build structures and bases.

### Building Process

```
Player selects structure to build:
  1. Enter build mode (show blueprint)
  2. Position structure
  3. Check space available
  4. Check resources in inventory
  5. If valid:
     a. Remove resources
     b. Place construction site
     c. Time passes
     d. Structure completed
```

### Structure Types

| Type | Resources | Time | Features |
|------|-----------|------|----------|
| Campfire | Wood x5 | 10 min | Warmth, cooking |
| Tent | Cloth x3, Rope x2 | 20 min | Rest, storage |
| Wall | Wood x10 or Stone x5 | 1 hour | Blocks movement |
| Door | Wood x5, Metal x2 | 30 min | Controllable passage |
| Workbench | Wood x15 | 2 hours | Crafting station |
| Storage Chest | Wood x10 | 1 hour | Inventory expansion |

### Player Base

```rust
pub struct PlayerBase {
    pub location: LocationId,
    pub structures: Vec<Structure>,
    pub storage: Inventory,
    pub defense_rating: i32,
    pub residents: Vec<NPCId>,
}
```

Benefits:
- Safe place to rest
- Storage for items
- Crafting stations
- NPC companions can live here
- Defense against raids

---

## Save/Load System

### Purpose

Persist entire game state to disk.

### Save Data Structure

```rust
pub struct SaveGame {
    pub version: String,
    pub timestamp: SystemTime,
    pub player_name: String,

    // World
    pub overmap: Overmap,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub world_time: WorldTime,
    pub weather: Weather,

    // Entities
    pub world: World,  // ECS world (serialized)
    pub player_entity: Entity,

    // Systems
    pub factions: FactionManager,
    pub quests: QuestManager,
    pub events: Vec<WorldEvent>,

    // Flags
    pub flags: HashMap<String, bool>,
    pub discovered_locations: HashSet<LocationId>,
}
```

### Serialization

```rust
use serde::{Serialize, Deserialize};
use bincode;
use zstd;

fn save_game(save: &SaveGame, path: &Path) -> Result<()> {
    // Serialize to binary
    let data = bincode::serialize(save)?;

    // Compress
    let compressed = zstd::encode_all(&data[..], 3)?;

    // Write to file
    fs::write(path, compressed)?;

    Ok(())
}

fn load_game(path: &Path) -> Result<SaveGame> {
    // Read file
    let compressed = fs::read(path)?;

    // Decompress
    let data = zstd::decode_all(&compressed[..])?;

    // Deserialize
    let save: SaveGame = bincode::deserialize(&data)?;

    Ok(save)
}
```

### Auto-Save

```
Auto-save triggers:
  - Every 10 minutes
  - Before dangerous actions
  - When entering/exiting locations
  - On sleep/rest
  - Manual save (S key)

Save slots:
  - Auto-save (1 slot, rotating)
  - Manual saves (10 slots)
  - Each save shows: name, time, location
```

### Save Compatibility

```
Version check on load:
  If save_version != current_version:
    If compatible:
      Migrate data
      Load successfully
    Else:
      Error: "Save incompatible with this version"
```

---

## Conclusion

These systems work together to create a living, reactive world where player choices matter and reality slowly unravels.

Each system is modular and can be implemented/tested independently while interfacing cleanly through the ECS architecture.

---

*"Systems define reality. Reality is fragile."*
