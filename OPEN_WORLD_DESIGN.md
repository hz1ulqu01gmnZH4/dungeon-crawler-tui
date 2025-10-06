# The Unraveling of Kándavael - Open World Design

## Executive Summary

Transform from dungeon crawler → **Open world cosmic horror survival roguelike**

**Core Concept**: The entire kingdom of Kándavael is your playground. Watch reality unravel region by region as you "heroically" cleanse corruption - only to realize too late you're feeding the entity that's consuming the world from within.

---

## Open World Structure

### Overmap System (inspired by CDDA)

#### World Scale
```
Macro Level (Kingdom Map)
├── 200x200 overmap tiles (each tile = 24x24 game tiles)
├── ~40,000 explorable locations
├── Multiple biomes and regions
└── Procedurally generated with narrative anchors

Zoomed In (Location Map)
├── Traditional roguelike view
├── Turn-based tactical combat
├── Detailed tile-by-tile exploration
└── Dungeons, buildings, wilderness
```

#### Map Hierarchy
```
Kingdom of Kándavael (Overworld)
├── Regions (8-12 major areas)
│   ├── Cities (3-5 per region)
│   ├── Towns (10-20 per region)
│   ├── Villages (30-50 per region)
│   ├── Dungeons (Scattered)
│   ├── Ruins (Ancient sites)
│   └── Wilderness (Connecting areas)
└── Special Locations
    ├── Daelspire (Capital - reality anchor)
    ├── Saelcairn Academy (Magic university)
    ├── The Wound (Cosmic breach site)
    ├── Memory Archive (Meta-hub)
    └── Ancient Temples (Ritual sites)
```

### World Generation

#### Procedural Kingdom Generation
```rust
pub struct OvermapGenerator {
    seed: u64,
    corruption_center: Position,  // Where reality is thinning
    anchor_points: Vec<NarrativeLocation>,
    biomes: Vec<BiomeDefinition>,
}

pub struct OvermapTile {
    terrain_type: TerrainType,
    location: Option<Location>,
    corruption_level: f32,  // 0.0 = pristine, 1.0 = void
    reality_stability: f32,
    discovered: bool,
    visited: bool,
    notes: Vec<PlayerNote>,  // Player can mark map
}

pub enum TerrainType {
    // Natural
    Forest,
    Plains,
    Mountains,
    Hills,
    River,
    Lake,
    Swamp,

    // Corrupted variants (revealed later)
    BleedingWoods,      // Trees weep blood
    WrithingPlains,     // Ground pulses like flesh
    HollowMountains,    // Mountains are ribs
    WhisperSwamp,       // Voices in the mist
    VoidLake,           // Water is darkness

    // Civilization
    Road,
    City,
    Town,
    Village,
    Ruins,
    Dungeon,
    Temple,
}
```

#### Biome System
```rust
pub struct Biome {
    name: String,
    temperature: f32,
    corruption_resistance: f32,
    base_terrain: Vec<(TerrainType, f32)>,  // Weighted distribution
    monsters: Vec<MonsterSpawn>,
    resources: Vec<ResourceSpawn>,
    reality_stability: f32,
}

pub enum BiomeType {
    // Normal biomes
    CentralKingdom,      // Relatively safe
    NorthernWastes,      // Cold, harsh
    EasternForest,       // Dense woods
    SouthernMarshes,     // Dangerous swamps
    WesternMountains,    // Rocky, isolated

    // Corrupted biomes (evolve from normal)
    TheBlightland,       // Was farmland, now writhing
    CryingWoods,         // Forest gained awareness
    FleshMarsh,          // Swamp became organic
    BoneMountains,       // Stone turned to skeleton
    VoidWastes,          // Reality completely gone
}
```

---

## World Simulation & Dynamics

### Time System

#### Macro Time (Overworld)
```rust
pub struct WorldTime {
    year: i32,
    season: Season,
    day: i32,
    hour: i32,

    // Cosmic calendar
    reality_cycle: RealityCycle,  // Days until next reality event
    corruption_phase: CorruptionPhase,
    apocalypse_timer: i32,  // Days until world ends
}

pub enum Season {
    Spring,     // Normal: Growth & renewal
    Summer,     // Normal: Warm & plenty
    Autumn,     // Normal: Harvest & preparation
    Winter,     // Normal: Cold & harsh

    // Corrupted seasons (appear in late game)
    BleedingSeason,   // Red skies, blood rain
    WhisperSeason,    // Constant voices
    VoidSeason,       // Reality thin everywhere
}

pub enum RealityCycle {
    Stable,              // 10 days, reality is strong
    Tremor,              // 5 days, occasional glitches
    Storm,               // 3 days, frequent shifts
    Breach,              // 1 day, massive corruption event
}
```

#### Time Progression
- **Overworld travel**: Time passes (1 hour per tile)
- **Location exploration**: Real-time turns
- **Resting**: 8 hours (camp/inn)
- **Crafting**: Minutes to hours
- **Events**: Triggered by time thresholds

### Dynamic World Events

#### Regional Corruption Spreading
```rust
pub struct CorruptionSystem {
    epicenter: Position,  // The Wound
    spread_rate: f32,     // Accelerates with player "victories"
    infected_regions: HashMap<RegionId, CorruptionLevel>,
}

pub enum CorruptionLevel {
    Pristine,         // 0-10%: Everything normal
    Whispers,         // 10-30%: Strange occurrences
    Manifest,         // 30-60%: Visible corruption
    Consumed,         // 60-90%: Reality collapsing
    Void,             // 90-100%: Complete transformation
}
```

**Corruption Effects Per Region:**
- **10%**: NPCs have nightmares, animals flee
- **25%**: Occasional reality glitches, mutations appear
- **50%**: Buildings start transforming, NPCs go mad
- **75%**: Terrain itself corrupts, portals open
- **100%**: Region becomes part of the entity

#### Player Impact on World
```rust
pub struct PlayerLegacy {
    beacons_lit: i32,           // Each accelerates corruption
    wardens_defeated: i32,      // Removes reality stabilizers
    civilians_saved: i32,       // Unknowingly doomed
    knowledge_spread: i32,      // Enlightenment = corruption
    rituals_performed: i32,     // Direct entity feeding

    // Reputation (evolves with realization)
    early_title: "The Prophesied Champion",
    mid_title: "The Uncertain Hero",
    late_title: "The Unwitting Herald",
    final_title: "The Throat That Feeds",
}
```

**Your Actions Have Consequences:**
1. **Light a beacon** → Region illuminated → Corruption spreads 10%
2. **Defeat Warden** → "Evil" vanquished → Reality seal breaks
3. **Save village** → Grateful NPCs → More souls for entity
4. **Teach earthcraft** → Modernize region → Weaken reality
5. **Cleanse temple** → Holy site restored → Actually opened feeding ground

### NPC & Settlement Simulation

#### Settlement System
```rust
pub struct Settlement {
    name: String,
    settlement_type: SettlementType,
    population: i32,
    corruption_level: f32,
    stability: f32,
    resources: Resources,
    factions: Vec<FactionPresence>,
    special_features: Vec<Feature>,
}

pub enum SettlementType {
    // Size tiers
    Hamlet,         // 10-50 people
    Village,        // 50-200 people
    Town,          // 200-1000 people
    City,          // 1000-5000 people
    Capital,       // 5000+ people (Daelspire)

    // Special types
    MilitaryOutpost,
    TradingPost,
    MonasteryShrine,
    WizardTower,
    Ruins,
    CultCompound,
}

pub struct SettlementFeatures {
    // Services
    inn: Option<Inn>,
    market: Option<Market>,
    blacksmith: Option<Blacksmith>,
    temple: Option<Temple>,
    library: Option<Library>,
    alchemist: Option<Alchemist>,

    // Special
    ritual_site: bool,
    reality_anchor: bool,
    corruption_source: bool,
    portal: bool,
}
```

#### Living NPCs
```rust
pub struct NPC {
    id: String,
    name: String,
    profession: Profession,
    home_settlement: SettlementId,
    current_location: Position,
    schedule: Schedule,

    // Stats
    corruption: f32,
    awareness: f32,     // How much truth they know
    relationship: i32,  // Towards player
    sanity: f32,

    // State
    state: NPCState,
    inventory: Vec<Item>,
    knowledge: HashSet<KnowledgeId>,
}

pub enum NPCState {
    Normal,
    Suspicious,         // Starting to notice
    Aware,             // Knows something is wrong
    Enlightened,       // Understands truth
    Mad,               // Can't handle truth
    Corrupted,         // Physically transformed
    Possessed,         // Entity puppet
}

pub enum Profession {
    // Civilian
    Farmer,
    Merchant,
    Innkeeper,
    Guard,
    Scholar,
    Priest,

    // Special
    Warden,            // Reality stabilizer (you hunt these)
    Cultist,           // Dæl worshipper
    Stitcher,          // Resistance member
    Enlightened,       // Knows the truth
    Earthborn,         // Another isekai'd person
}
```

#### NPC Daily Schedules
```rust
pub struct Schedule {
    activities: HashMap<TimeOfDay, Activity>,
}

pub enum Activity {
    Sleep { location: Position },
    Work { location: Position, job: Job },
    Eat { location: Position },
    Socialize { location: Position },
    Trade { market: Position },
    Pray { temple: Position },

    // Special
    Flee,              // Running from corruption
    HideIndoors,       // Reality storm
    PerformRitual,     // Cultist activity
    Investigate,       // Suspicious of player
}
```

### Faction System

#### Major Factions
```rust
pub struct Faction {
    id: String,
    name: String,
    faction_type: FactionType,
    territory: Vec<RegionId>,
    power: i32,
    corruption_tolerance: f32,
    goals: Vec<FactionGoal>,
    relationship_with_player: i32,
    awareness_level: AwarenessLevel,
}

pub enum FactionType {
    // Major factions
    RoyalCrown,              // Kingdom government
    HolyChurch,              // Church of Dael (corrupted)
    MerchantsGuild,          // Trade & economy
    WardensOrder,            // Reality guardians (your targets)
    ArcaneAcademy,           // Saelcairn scholars

    // Resistance
    TheStitchers,            // Know truth, trying to seal
    OldFaith,                // Pre-Dæl religion
    Refugees,                // Fleeing corruption

    // Corrupted
    CultOfDæl,               // Worship the entity
    EnlightenedOnes,         // Accepted cosmic truth
    FleshCovenant,           // Mutation worshippers
    VoidSeekers,             // Want reality to end

    // Other
    Bandits,                 // Opportunistic raiders
    WildFolk,                // Forest dwellers
    Earthborn,               // Other isekai'd people
}

pub enum AwarenessLevel {
    Ignorant,       // Think you're a hero
    Questioning,    // Starting to doubt
    Suspicious,     // Actively investigating
    Aware,          // Know the truth
    Hostile,        // Will stop you
}
```

#### Faction Territories & Influence
```rust
pub struct RegionControl {
    region_id: RegionId,
    controlling_faction: FactionId,
    influence_level: f32,  // 0.0-1.0
    contested_by: Vec<(FactionId, f32)>,
}
```

**Faction Interactions:**
- Factions war over territories
- Player actions shift power balances
- Some factions start friendly, turn hostile
- Can play factions against each other
- Faction-specific quests and rewards

---

## Travel & Exploration

### Overworld Travel

#### Travel Modes
```rust
pub enum TravelMode {
    Walking,          // 1 tile/hour, see everything
    Horse,            // 2 tiles/hour, might miss details
    Cart,             // 1.5 tiles/hour, carry more
    FleshMount,       // 3 tiles/hour, corrupted
    FastTravel,       // Instant, but costs sanity
}

pub struct TravelEncounter {
    encounter_type: EncounterType,
    chance_modifier: f32,
    corruption_influence: f32,
}

pub enum EncounterType {
    // Normal
    Merchant,
    Travelers,
    Bandits,
    WildAnimals,
    PatrolGuards,

    // Strange
    StrangeStatue,
    AbandonedCamp,
    MysteriousShrine,
    DeadBody,
    WeirdLights,

    // Corrupted
    RealityGlitch,
    MutatedCreature,
    Cultists,
    Possession,
    PortalStorm,

    // Special
    Warden,          // Optional boss fight
    OtherEarthborn,  // Meet another isekai person
    MemoryFragment,  // Lore/flashback
    ProphecySign,    // Misleading "hero" omen
}
```

#### Road Network
```rust
pub struct Road {
    start: Position,
    end: Position,
    quality: RoadQuality,
    danger_level: i32,
    corruption: f32,
}

pub enum RoadQuality {
    KingRoad,        // Major highway, fast, patrolled
    TradeRoute,      // Well-maintained
    DirtPath,        // Common but slow
    Trail,           // Barely maintained
    Overgrown,       // Abandoned
    CorruptedPath,   // Reality unstable
}
```

### Fast Travel System

#### Memory Anchors (Fast Travel Points)
```rust
pub struct MemoryAnchor {
    location: Position,
    name: String,
    anchor_type: AnchorType,
    stability: f32,
    unlocked: bool,

    // Costs
    sanity_cost: i32,
    notice_gain: i32,
    corruption_risk: f32,
}

pub enum AnchorType {
    MajorCity,          // Safe, low cost
    PlayerBase,         // Custom anchor
    Temple,             // Risky, medium cost
    RitualSite,         // Dangerous, high cost
    MemoryArchive,      // Meta-hub
    CorruptedBeacon,    // Fast but corrupting
}
```

**Fast Travel Mechanics:**
- Unlocked by visiting location first
- Costs sanity based on distance
- Increases Notice (entities watching)
- Can fail during reality storms
- Leaves temporal "residue" (can be tracked)
- Corrupted anchors are faster but dangerous

### Map & Navigation

#### Map System
```rust
pub struct PlayerMap {
    revealed_tiles: HashSet<Position>,
    explored_locations: HashMap<LocationId, LocationData>,
    notes: HashMap<Position, PlayerNote>,
    markers: Vec<MapMarker>,
    fog_of_war: bool,
}

pub struct PlayerNote {
    position: Position,
    text: String,
    icon: char,
    color: Color,
    reminder: bool,  // Shows on overworld
}

pub enum MapMarker {
    Danger,
    Resource,
    Quest,
    Corruption,
    Safe,
    Interest,
    Memory,
}
```

**Navigation Aids:**
- Compass (points to quest objectives)
- Stars (navigation at night, unless...)
- Roads and landmarks
- NPC directions
- Ancient maps (treasure hunt items)
- Corrupted zones distort compass

---

## Location Types & Content

### Dungeons (Traditional)

#### Dungeon Categories
```rust
pub enum DungeonType {
    // Pre-cataclysm
    AncientRuins,       // Old civilization
    BurialCrypt,        // Undead
    Mine,               // Resources & monsters
    Cave,               // Natural formation
    Sewers,             // Under cities

    // Post-corruption
    FleshCatacomb,      // Living dungeon
    VoidRift,           // Dimensional tear
    CorruptedTemple,    // Fallen holy site
    RitualGrounds,      // Active cult site

    // Special
    WardensStronghold,  // Major boss dungeon
    ArcaneLibrary,      // Knowledge & lexemes
    AlchemyLab,         // Crafting & recipes
    Memory,             // Psychic/dream space
}
```

#### Dynamic Dungeons
```rust
pub struct Dungeon {
    id: String,
    world_position: Position,
    dungeon_type: DungeonType,
    depth: i32,  // Number of floors
    corruption_level: f32,

    // Content
    floors: Vec<Floor>,
    boss: Option<BossEncounter>,
    special_loot: Vec<UniqueItem>,

    // State
    cleared: bool,
    respawn_timer: i32,
    changes_with_corruption: bool,
}
```

### Cities & Towns

#### City Districts
```rust
pub struct City {
    name: String,
    population: i32,
    districts: Vec<District>,
    corruption_level: f32,
    faction_control: HashMap<FactionId, f32>,
}

pub struct District {
    name: String,
    district_type: DistrictType,
    buildings: Vec<Building>,
    npcs: Vec<NPCId>,
    corruption: f32,
}

pub enum DistrictType {
    // Normal
    Residential,
    Market,
    Crafting,
    Temple,
    Noble,
    Slums,
    Port,

    // Special
    Academy,        // Arcane learning
    Military,       // Barracks & training
    UnderCity,      // Sewers & black market

    // Corrupted
    QuarantineZone,
    CultDistrict,
    FleshQuarter,   // Buildings merged
    VoidSector,     // Reality collapsed
}
```

#### Building Types
```rust
pub enum BuildingType {
    // Services
    Inn,
    Tavern,
    Shop,
    Market,
    Blacksmith,
    Alchemist,
    Temple,
    Library,

    // Social
    TownHall,
    Barracks,
    Prison,
    Hospital,
    School,

    // Residential
    House,
    Apartment,
    Manor,
    Palace,

    // Special
    WizardTower,
    GuildHall,
    RitualSite,
    HiddenShrine,
    SafeHouse,

    // Player
    PlayerHome,
    Workshop,
    Laboratory,
    Sanctuary,
}
```

### Wilderness

#### Wilderness Encounters
```rust
pub struct WildernessZone {
    biome: Biome,
    density: WildernessDensity,
    features: Vec<WildernessFeature>,
    danger_level: i32,
}

pub enum WildernessDensity {
    Sparse,         // Mostly empty
    Normal,         // Occasional encounters
    Dense,          // Frequent content
    Overgrown,      // Very dangerous
    Corrupted,      // Reality unstable
}

pub enum WildernessFeature {
    // Resources
    HerbPatch,
    BerryBush,
    Quarry,
    WildGame,

    // Structures
    Cabin,
    Camp,
    Shrine,
    StandingStones,
    Ruins,

    // Encounters
    Travelers,
    BanditCamp,
    MonsterDen,
    CultMeeting,
    RealityAnomaly,

    // Natural
    Cave,
    River,
    Cliff,
    DeepForest,
}
```

---

## Base Building & Player Housing

### Player Base System

#### Base Types
```rust
pub struct PlayerBase {
    location: Position,
    base_type: BaseType,
    structures: Vec<Structure>,
    storage: Inventory,
    npcs: Vec<NPCId>,  // Followers/hirelings
    defenses: Vec<Defense>,
    corruption_resistance: f32,
}

pub enum BaseType {
    House,           // Small, in town
    Homestead,       // Medium, rural
    Fortress,        // Large, fortified
    Tower,           // Vertical, magic focus
    Sanctuary,       // Hidden, safe from entity
    MemoryPalace,    // Meta-hub (between runs)
}
```

#### Construction System
```rust
pub struct Structure {
    name: String,
    structure_type: StructureType,
    build_time: i32,
    requirements: CraftRequirements,
    effects: Vec<StructureEffect>,
}

pub enum StructureType {
    // Basic
    Wall,
    Door,
    Window,
    Roof,
    Floor,

    // Furniture
    Bed,             // Sleep/save point
    Storage,         // Item storage
    Workbench,       // Crafting

    // Functional
    Fireplace,       // Warmth, cooking
    Well,            // Water
    Garden,          // Food
    Workshop,        // Advanced crafting
    Laboratory,      // Alchemy
    Library,         // Research
    Forge,           // Smithing

    // Magical
    RitualCircle,    // Blood magic
    WardGlyph,       // Protection
    Altar,           // Deity/entity interaction
    Portal,          // Fast travel
    MemoryAnchor,    // Save/respawn point

    // Defensive
    Trap,
    Barricade,
    Tower,
    Moat,
}

pub enum StructureEffect {
    CraftingBonus { category: CraftCategory, bonus: f32 },
    CorruptionResistance { amount: f32 },
    HealingRate { rate: f32 },
    SanityRecovery { rate: f32 },
    StorageCapacity { slots: i32 },
    DefenseBonus { amount: i32 },
    AttractsNPCs,
    AttractsMonsters,
    RealityStabilizer,
}
```

#### Base Management
```rust
pub struct BaseManagement {
    // Resources
    food_storage: i32,
    water_storage: i32,
    materials: HashMap<MaterialType, i32>,

    // Followers
    residents: Vec<NPCId>,
    guards: Vec<NPCId>,
    workers: Vec<NPCId>,

    // Production
    farms: Vec<Farm>,
    workshops: Vec<Workshop>,

    // Defense
    defenses: Vec<Defense>,
    defense_rating: i32,

    // State
    morale: i32,
    corruption_level: f32,
}
```

---

## Survival & Resource Management

### Expanded Survival Systems

#### Vital Needs (inspired by CDDA)
```rust
pub struct SurvivalNeeds {
    // Basic
    hunger: i32,         // 0-1000, die at 1000
    thirst: i32,         // 0-500, die at 500
    fatigue: i32,        // 0-1000, pass out at 1000

    // Advanced
    health: i32,         // Overall condition
    pain: i32,           // Affects all actions
    body_temperature: i32,

    // Cosmic
    sanity: i32,         // Your main resource
    insight: i32,        // Knowledge of truth
    notice: i32,         // Entity attention
    corruption: i32,     // Physical taint
}
```

#### Food & Water System
```rust
pub struct Food {
    nutrition: i32,
    hydration: i32,
    enjoyment: i32,      // Affects morale
    spoilage_time: i32,

    // Special
    sanity_effect: i32,
    corruption_risk: f32,
    grants_buffs: Vec<StatusEffect>,
}

pub enum FoodType {
    // Normal
    Bread,
    Meat,
    Vegetables,
    Fruit,

    // Preserved
    Jerky,
    Canned,
    Pickled,

    // Special
    AlchemyPotion,
    RitualMeal,      // Prepared with magic
    CorruptedFlesh,  // Monster meat
    AmbrosiaFood,    // From entity (addictive)
}
```

#### Camping & Rest
```rust
pub struct CampSite {
    quality: CampQuality,
    fire: bool,
    shelter: bool,
    bedroll: bool,

    // Effects on rest
    healing_rate: f32,
    sanity_recovery: f32,
    random_encounter_chance: f32,
}

pub enum CampQuality {
    Rough,          // On ground, no protection
    Basic,          // Bedroll, small fire
    Comfortable,    // Tent, good fire, food
    Luxurious,      // Inn quality in wilderness
    Dangerous,      // Corrupted area
}
```

### Resource Gathering

#### Gathering System
```rust
pub enum ResourceNode {
    // Natural
    Tree,
    Rock,
    HerbPatch,
    BerryBush,
    Mushrooms,
    WildGame,
    Fish,

    // Salvage
    Ruins,
    BattleSite,
    AbandonedCamp,
    Corpse,

    // Mystical
    MagicCrystal,
    RealityFragment,
    EntityResidue,
    AncientArtifact,
}

pub struct Gathering {
    skill_required: Skill,
    time_required: i32,
    tool_required: Option<ToolType>,
    yields: Vec<(ItemId, f32)>,  // Item + chance

    // Risks
    injury_chance: f32,
    corruption_chance: f32,
    encounter_chance: f32,
}
```

---

## Advanced World Features

### Weather System

#### Weather Types (Evolved)
```rust
pub struct Weather {
    weather_type: WeatherType,
    intensity: f32,
    duration: i32,
    region_effect: RegionId,
}

pub enum WeatherType {
    // Normal
    Clear,
    Cloudy,
    Rain,
    Snow,
    Storm,
    Fog,

    // Corrupted (appear in late game)
    BloodRain,           // Red rain, increases corruption
    RealityStorm,        // Glitches, layer switches
    WhisperFog,          // Voices, madness
    VoidDarkness,        // Supernatural darkness
    FleshSnow,           // Snow is organic
    SoulWind,            // Wind steals memories

    // Apocalyptic
    TheUnraveling,       // Reality collapsing everywhere
}

pub struct WeatherEffect {
    visibility_modifier: i32,
    travel_speed: f32,
    combat_modifier: f32,
    sanity_drain: f32,
    corruption_rate: f32,

    // Special effects
    layer_instability: bool,
    monster_behavior_change: bool,
    prevents_fast_travel: bool,
}
```

### Day/Night Cycle

#### Time of Day Effects
```rust
pub enum TimeOfDay {
    Dawn,          // 5-7am: Reality stable
    Morning,       // 7-12pm: Normal
    Afternoon,     // 12-5pm: Normal
    Dusk,          // 5-7pm: Reality weakens
    Evening,       // 7-10pm: Dangerous
    Night,         // 10pm-2am: Very dangerous
    DeadNight,     // 2-5am: Reality thinnest
}

pub struct TimeEffects {
    visibility: i32,
    monster_spawns: f32,
    npc_activity: NPCActivity,
    reality_stability: f32,
    sanity_drain_rate: f32,
}
```

**Night Mechanics:**
- Most NPCs sleep
- Monsters more active
- Corruption spreads faster
- Can see "true" forms easier
- Reality glitches more common
- Cultist activity peaks at midnight

### Seasonal Effects

#### Seasons & Corruption
```rust
pub enum GameSeason {
    // Year 1 (Normal)
    Spring,
    Summer,
    Autumn,
    Winter,

    // Year 2 (Reality Weakening)
    TaintedSpring,   // Growth is wrong
    HollowSummer,    // Heat feels empty
    BloodAutumn,     // Harvests bleed
    DeadWinter,      // Cold is hungry

    // Year 3+ (Collapse)
    TheFifthSeason,  // Time loses meaning
}

pub struct SeasonEffects {
    temperature_range: (i32, i32),
    crop_growth: f32,
    monster_types: Vec<MonsterType>,
    corruption_modifier: f32,
    special_events: Vec<SeasonalEvent>,
}
```

---

## Quest & Narrative System

### Dynamic Quest Generation

#### Quest Types
```rust
pub enum QuestType {
    // Main Story
    MainQuest {
        stage: i32,
        revelation_level: i32,  // How much truth revealed
    },

    // Faction Quests
    FactionQuest {
        faction: FactionId,
        quest_line: String,
        affects_relationship: bool,
    },

    // Dynamic Quests
    BountyHunt,          // Kill target
    Escort,              // Protect NPC
    Delivery,            // Transport item
    Investigation,       // Find clues
    Rescue,              // Save someone
    Cleansing,           // "Purify" area (corrupt it)
    Ritual,              // Perform ceremony

    // Player-Driven
    Exploration,         // Discover location
    Collection,          // Gather resources
    Crafting,            // Create specific item
    BuildingProject,     // Construct base feature

    // Special
    MemoryQuest,         // Flashback sequence
    DreamQuest,          // Psychic realm
    CosmicRevelation,    // Truth bomb moment
}
```

#### Quest Stages & Morality
```rust
pub struct Quest {
    id: String,
    title: String,
    description: String,
    giver: Option<NPCId>,

    // Dual nature (early vs late game understanding)
    surface_goal: String,      // "Defeat the Warden"
    true_goal: String,          // "Break Reality Seal"

    // Progress
    stage: QuestStage,
    objectives: Vec<Objective>,

    // Consequences
    rewards: Vec<Reward>,
    corruption_increase: f32,
    region_effects: Vec<RegionEffect>,
    faction_changes: HashMap<FactionId, i32>,

    // Narrative
    revelation: Option<String>,  // Truth revealed on completion
}

pub enum QuestStage {
    Available,
    Active,
    Completed,
    Failed,
    Abandoned,
    Revealed,        // True nature exposed
}
```

### Progressive Revelation System

#### Truth Stages
```rust
pub struct RevelationSystem {
    current_stage: RevelationStage,
    clues_discovered: HashSet<ClueId>,
    enlightenment_level: i32,
}

pub enum RevelationStage {
    // Act 1: Hero (Hours 0-5)
    Innocent,            // Everything seems normal
    FirstDoubt,          // Something feels off

    // Act 2: Questioning (Hours 5-15)
    Suspicious,          // Noticing inconsistencies
    Investigating,       // Actively seeking truth

    // Act 3: Horror (Hours 15-30)
    Realization,         // Understand what you've done
    Acceptance,          // Can't undo it

    // Act 4: Desperation (Hours 30-50)
    FightingBack,        // Try to stop it
    LastHope,            // Final desperate measures

    // Act 5: Ending
    Triumph,             // Found a way (rare)
    Transformation,      // Become part of entity
    Sacrifice,           // Stop it by dying
    Apocalypse,          // Failed completely
}
```

---

## Meta-Progression & Permadeath

### Memory Archive (Hub Between Runs)

#### The Archive
```rust
pub struct MemoryArchive {
    // Progress
    runs_completed: i32,
    total_corruption_spread: f32,
    knowledge_fragments: Vec<KnowledgeFragment>,

    // Unlocks
    unlocked_classes: Vec<MaskType>,
    unlocked_abilities: Vec<AbilityId>,
    unlocked_starting_locations: Vec<Position>,

    // Permanent upgrades
    max_sanity_bonus: i32,
    starting_lexemes: Vec<String>,
    special_items: Vec<ItemId>,

    // Corruption
    archive_corruption: f32,  // Meta-progression itself corrupts
}

pub struct KnowledgeFragment {
    id: String,
    category: KnowledgeCategory,
    content: String,
    sanity_cost_to_use: i32,
    corrupts_next_run: bool,
}

pub enum KnowledgeCategory {
    Lexeme,              // New spell word
    Recipe,              // Crafting knowledge
    Location,            // Map knowledge
    LoreFragment,        // Story piece
    NPCSecret,           // Character info
    FactionIntel,        // Politics
    CosmicTruth,         // Reality secrets
    DangerousKnowledge,  // Shouldn't know this
}
```

#### Permadeath with Meaning
```rust
pub struct DeathSystem {
    cause_of_death: DeathCause,

    // What carries over
    knowledge_retained: Vec<KnowledgeFragment>,
    corruption_spread: f32,  // World state persists!
    faction_reputation: HashMap<FactionId, i32>,

    // Consequences
    world_changes: Vec<WorldChange>,
    npcs_affected: Vec<NPCId>,
    regions_corrupted: Vec<RegionId>,
}

pub enum DeathCause {
    Combat,
    Starvation,
    Corruption,
    Madness,
    Sacrifice,           // Intentional death for ritual
    Consumption,         // Entity ate you
    RealityCollapse,     // Too corrupted
}
```

**Death Effects:**
- Your body becomes a landmark (other players can find it)
- NPCs remember you ("The Hero Who Failed")
- Corruption you spread STAYS in world
- Some factions hunt your next incarnation
- Can meet your ghost/memory
- Archive slowly corrupts from overuse

---

## Technical Implementation

### World Generation Pipeline

```rust
pub struct WorldGenerator {
    seed: u64,
}

impl WorldGenerator {
    pub fn generate_kingdom(&mut self) -> Kingdom {
        // Stage 1: Terrain
        let terrain = self.generate_terrain();

        // Stage 2: Rivers & Lakes
        let hydrology = self.generate_water_systems(&terrain);

        // Stage 3: Biomes
        let biomes = self.assign_biomes(&terrain, &hydrology);

        // Stage 4: Roads
        let roads = self.generate_road_network(&terrain);

        // Stage 5: Settlements
        let settlements = self.place_settlements(&terrain, &roads);

        // Stage 6: Dungeons
        let dungeons = self.scatter_dungeons(&terrain, &settlements);

        // Stage 7: Narrative Anchors
        self.place_story_locations(&terrain, &settlements);

        // Stage 8: Corruption
        self.initialize_corruption_system();

        Kingdom {
            terrain,
            hydrology,
            biomes,
            roads,
            settlements,
            dungeons,
        }
    }
}
```

### Streaming World System

```rust
pub struct WorldStreaming {
    loaded_chunks: HashMap<ChunkId, Chunk>,
    player_position: Position,
    load_radius: i32,

    // Optimization
    chunk_cache: LRUCache<ChunkId, Chunk>,
    background_loader: AsyncLoader,
}

impl WorldStreaming {
    pub fn update(&mut self, player_pos: Position) {
        // Unload far chunks
        self.unload_distant_chunks(player_pos);

        // Load nearby chunks
        self.load_nearby_chunks(player_pos);

        // Save modified chunks
        self.save_dirty_chunks();
    }

    fn load_nearby_chunks(&mut self, pos: Position) {
        let chunks_to_load = self.get_chunks_in_radius(pos, self.load_radius);

        for chunk_id in chunks_to_load {
            if !self.loaded_chunks.contains_key(&chunk_id) {
                self.background_loader.load_chunk(chunk_id);
            }
        }
    }
}
```

### Save System

```rust
pub struct SaveData {
    version: String,

    // World State
    world_state: WorldState,
    corruption_map: CorruptionMap,
    time: WorldTime,
    weather: Weather,

    // Player
    player: Player,
    player_position: Position,

    // NPCs
    npcs: HashMap<NPCId, NPC>,
    npc_relationships: HashMap<NPCId, i32>,

    // Factions
    factions: HashMap<FactionId, Faction>,
    faction_territories: HashMap<RegionId, FactionId>,

    // Quests
    active_quests: Vec<Quest>,
    completed_quests: Vec<QuestId>,

    // World changes
    destroyed_settlements: Vec<SettlementId>,
    corrupted_regions: Vec<RegionId>,
    player_actions: Vec<PlayerAction>,

    // Meta
    meta_progression: MemoryArchive,
}
```

---

## Performance Considerations

### Optimization Strategies

1. **Chunk-Based Loading**
   - Only load 3x3 chunks around player
   - Background loading for adjacent chunks
   - Aggressive unloading of distant chunks

2. **Entity Culling**
   - Simulate only entities in loaded chunks
   - Simple state machines for distant NPCs
   - Freeze distant settlements

3. **Procedural Detail**
   - Generate detail on-demand
   - Cache frequently accessed areas
   - Simple representation for distant regions

4. **Save Optimization**
   - Delta compression for world changes
   - Only save modified regions
   - Separate save files for different systems

---

## Estimated Scope & Timeline

### Development Phases

**Phase 1: Open World Foundation (8-12 weeks)**
- Overmap system
- Chunk loading/streaming
- Basic travel
- Settlement generation
- World time system

**Phase 2: Content Systems (6-8 weeks)**
- NPCs and schedules
- Faction system
- Quest system
- Dynamic events

**Phase 3: Survival & Base Building (4-6 weeks)**
- Expanded survival needs
- Resource gathering
- Construction system
- Base management

**Phase 4: World Simulation (6-8 weeks)**
- Weather system
- Corruption spreading
- Regional events
- Faction warfare

**Phase 5: Polish & Content (8-12 weeks)**
- Narrative integration
- Unique locations
- Special encounters
- Balance and testing

**Total**: 32-46 weeks (8-11 months)

---

## Conclusion

This transforms your roguelike into a **persistent open world cosmic horror survival game** where:

1. **The entire kingdom is your canvas** - 40,000+ tiles to explore
2. **Your actions shape the world** - Watch regions transform
3. **NPCs have lives** - They sleep, work, notice truth
4. **Time matters** - World evolves with or without you
5. **Death is meaningful** - Corruption you spread persists
6. **Multiple playstyles** - Explore, build, quest, or speedrun apocalypse

**Unique Selling Points:**
- Open world + cosmic horror (rare combination)
- Your "heroic" actions cause the apocalypse
- Watch the world progressively transform
- Persistent consequences across deaths
- True sandbox with narrative structure

This is ambitious but achievable with modular development. Start with Phase 1 (overmap + travel), then iteratively add systems. Each phase adds standalone value while building toward the complete vision.

The CDDA comparison: Where CDDA is "survive the zombie apocalypse," yours is **"heroically cause the cosmic apocalypse while thinking you're the chosen one."**
