# Phase 2 Roadmap - Dungeon Crawler TUI

**Status**: Planning → Implementation
**Started**: 2025-10-14
**Phase 1 Completion**: ✅ All systems working, 175/181 tests passing

---

## Phase 2 Overview

Phase 2 focuses on **deepening gameplay mechanics** by adding:
- Inventory and item systems
- Enhanced combat mechanics
- NPC interactions
- Quest system
- Building interiors with content
- Experience and character progression
- Enhanced world content

---

## Priority Breakdown

### 🔴 High Priority (Core Gameplay)
These features are essential for a complete gameplay loop:

1. **Inventory System** (2-3 days)
2. **Item System** (2-3 days)
3. **Enhanced Combat** (2-3 days)
4. **Building Interiors** (2-3 days)

### 🟡 Medium Priority (Content & Depth)
These features add depth and replayability:

5. **NPC System** (3-4 days)
6. **Quest System** (3-4 days)
7. **Experience & Leveling** (2-3 days)
8. **Terrain-Based Local Maps** (3-4 days)

### 🟢 Low Priority (Polish & Expansion)
These features add polish and variety:

9. **Multi-Level Dungeons** (2-3 days)
10. **Advanced AI** (2-3 days)
11. **Magic System** (3-4 days)
12. **Crafting System** (2-3 days)

**Estimated Total**: 6-8 weeks for complete Phase 2

---

## Detailed Feature Plans

### Task 2.1: Inventory System ✨
**Priority**: 🔴 High
**Estimated Time**: 2-3 days
**Dependencies**: None

#### Components
```rust
// src/ecs/components.rs
pub struct Inventory {
    pub items: Vec<Entity>,      // Item entities
    pub capacity: usize,          // Max items (default: 26)
    pub equipped: HashMap<EquipSlot, Entity>,
}

pub enum EquipSlot {
    MainHand,
    OffHand,
    Head,
    Body,
    Legs,
    Feet,
    Ring1,
    Ring2,
    Amulet,
}
```

#### UI Implementation
- New inventory panel (press 'i')
- Show items in alphabetical list (a-z)
- Display item stats and description
- Equipment slots visualization
- Weight/capacity indicator

#### Systems
- Inventory management system
- Equipment system
- Item pickup system
- Item drop system

#### Tests
- Inventory creation and capacity
- Item add/remove
- Equipment slot management
- Full inventory handling

---

### Task 2.2: Item System ✨
**Priority**: 🔴 High
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.1

#### Item Types
```rust
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub weight: i32,
    pub value: i32,
}

pub enum ItemType {
    Weapon(WeaponData),
    Armor(ArmorData),
    Consumable(ConsumableData),
    Quest(QuestData),
    Misc,
}

pub struct WeaponData {
    pub damage: DiceRoll,
    pub weapon_type: WeaponType,  // Sword, Axe, Bow, etc.
}

pub struct ArmorData {
    pub defense: i32,
    pub slot: EquipSlot,
}

pub struct ConsumableData {
    pub effect: Effect,
    pub uses: i32,
}
```

#### Item Generation
- Loot tables by location type
- Rarity system (Common, Uncommon, Rare, Epic, Legendary)
- Random stat modifiers
- Named/unique items

#### Interactions
- 'g' key to pick up items
- 'd' key to drop items
- 'u' key to use/consume items
- 'e' key to equip/unequip

#### Tests
- Item creation
- Item properties
- Loot table generation
- Equipment effects

---

### Task 2.3: Enhanced Combat System ⚔️
**Priority**: 🔴 High
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.2

#### Combat Options
```rust
pub enum CombatAction {
    Attack(Entity),              // Basic attack
    Defend,                      // +defense this turn
    UseItem(Entity),             // Use consumable
    SpecialAttack(SpecialType),  // Class abilities
    Flee,                        // Attempt escape
}
```

#### Combat UI
- Combat log with detailed messages
- Show enemy HP bar
- Display hit chances
- Show damage rolls
- Action menu during combat

#### Features
- **Turn-based**: Player chooses action each turn
- **Critical hits**: 5% chance for 2x damage
- **Weapon types**: Different damage types and speeds
- **Status effects**: Poison, burn, stun, etc.
- **Armor effectiveness**: Reduces damage taken

#### Tests
- Combat action selection
- Damage calculation with equipment
- Critical hit mechanics
- Status effect application
- Flee success rate

---

### Task 2.4: Building Interior System 🏠
**Priority**: 🔴 High
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.2

#### Implementation
Replace placeholder settlement maps with detailed building interiors.

#### Building Types (Enhanced)
```rust
pub enum BuildingType {
    House {
        occupants: Vec<NPC>,
        furniture: Vec<Furniture>,
    },
    Inn {
        innkeeper: NPC,
        guests: Vec<NPC>,
        rooms: Vec<Room>,
    },
    Shop {
        shopkeeper: NPC,
        inventory: Vec<Item>,
        specialty: ShopType,
    },
    Temple {
        priest: NPC,
        deity: Deity,
        services: Vec<Service>,
    },
    Blacksmith {
        smith: NPC,
        forge: Forge,
        wares: Vec<Item>,
    },
    Library {
        librarian: NPC,
        books: Vec<Book>,
        skill_trainers: Vec<SkillTrainer>,
    },
    Warehouse {
        guard: NPC,
        storage: Vec<Container>,
    },
}
```

#### Features
- Proper interior layouts with furniture
- NPCs placed realistically
- Items on shelves/tables
- Doors between rooms
- Interactive objects

#### Building Entry
- Press Enter on building door to enter
- Separate map for each building
- Tab to exit back to settlement
- Save building state

#### Tests
- Building generation with content
- NPC placement
- Item placement
- Door functionality
- Navigation between buildings

---

### Task 2.5: NPC System 👥
**Priority**: 🟡 Medium
**Estimated Time**: 3-4 days
**Dependencies**: Task 2.4

#### Components
```rust
pub struct NPC {
    pub name: String,
    pub role: NPCRole,
    pub dialogue: DialogueTree,
    pub inventory: Inventory,
    pub disposition: i32,  // -100 (hostile) to 100 (friendly)
}

pub enum NPCRole {
    Shopkeeper(ShopType),
    Innkeeper,
    Guard,
    Priest,
    Blacksmith,
    Trainer(Skill),
    QuestGiver,
    Civilian,
}
```

#### Dialogue System
- Press 't' to talk to NPCs
- Branching dialogue trees
- Disposition affects responses
- Trade interface
- Quest acceptance

#### Services
- **Shops**: Buy/sell items
- **Inns**: Rest and heal (pay for room)
- **Temples**: Healing, blessings, remove curses
- **Trainers**: Learn skills, increase stats
- **Guards**: Enforce laws, provide protection

#### Tests
- NPC creation and placement
- Dialogue tree navigation
- Trading system
- Service interactions
- Disposition changes

---

### Task 2.6: Quest System 📜
**Priority**: 🟡 Medium
**Estimated Time**: 3-4 days
**Dependencies**: Task 2.5

#### Quest Types
```rust
pub enum QuestType {
    Kill { target: MonsterType, count: i32 },
    Fetch { item: ItemType, count: i32 },
    Deliver { item: Entity, destination: NPC },
    Explore { location: (i32, i32) },
    Talk { target: NPC },
    Escort { npc: Entity, destination: (i32, i32) },
}

pub struct Quest {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub quest_type: QuestType,
    pub reward: QuestReward,
    pub status: QuestStatus,
}
```

#### Quest Management
- Quest log (press 'Q')
- Active quests tracking
- Completed quests history
- Quest markers on map
- Multiple active quests

#### Quest Rewards
- Experience points
- Gold
- Items
- Reputation
- Unlock new areas

#### Tests
- Quest creation and tracking
- Quest completion detection
- Reward distribution
- Quest log management

---

### Task 2.7: Experience & Leveling ⭐
**Priority**: 🟡 Medium
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.3

#### Components
```rust
pub struct Experience {
    pub level: i32,
    pub xp: i32,
    pub xp_to_next: i32,
}

pub struct PlayerStats {
    pub strength: i32,     // Melee damage
    pub dexterity: i32,    // Hit chance, dodge
    pub constitution: i32, // HP
    pub intelligence: i32, // Magic damage
    pub wisdom: i32,       // Mana, magic resistance
    pub charisma: i32,     // NPC interactions
}
```

#### Progression
- Gain XP from killing monsters
- Gain XP from completing quests
- Level up increases stats
- Unlock new abilities
- Choose stat points on level up

#### Level Up UI
- Show level up notification
- Display stat increase options
- Show new abilities unlocked
- Celebrate milestone levels

#### Tests
- XP calculation
- Level up mechanics
- Stat point allocation
- Ability unlocks

---

### Task 2.8: Terrain-Based Local Maps 🗺️
**Priority**: 🟡 Medium
**Estimated Time**: 3-4 days
**Dependencies**: None (can be parallel)

**See**: `TERRAIN_MAPS_PLAN.md` for full implementation details

#### Overview
Generate appropriate local maps based on overmap terrain when entering wilderness areas.

#### Terrain Generators
- **Plains**: Open grassland with occasional trees
- **Forest**: Dense trees, clearings, undergrowth
- **Mountains**: Rocky terrain, caves, cliffs
- **Swamp**: Mud, shallow water, dead trees
- **Desert**: Sand, cacti, rock formations
- **Tundra**: Snow, ice, sparse vegetation
- **River**: Flowing water, banks, bridges

#### Implementation
```rust
pub fn generate_local_map(terrain: TerrainType, seed: u64) -> Map {
    match terrain {
        TerrainType::Plains => generate_plains_map(seed),
        TerrainType::Forest => generate_forest_map(seed),
        // ... etc
    }
}
```

#### Tests
- Each terrain type generates correctly
- Transitions between terrain types
- Resource placement by terrain
- Monster spawns by terrain

---

### Task 2.9: Multi-Level Dungeons 🏰
**Priority**: 🟢 Low
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.1, Task 2.2

#### Features
- Stairs down/up between levels
- Difficulty increases per level
- Persistent level state
- Boss encounters
- Treasure rooms

#### Components
```rust
pub struct Dungeon {
    pub floors: HashMap<i32, Map>,
    pub current_floor: i32,
    pub max_depth: i32,
}
```

#### Tests
- Level generation
- Stair placement
- Level transitions
- State persistence

---

### Task 2.10: Advanced AI 🤖
**Priority**: 🟢 Low
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.3

#### Features
- Pathfinding to player
- Group tactics
- Retreat when low HP
- Call for help
- Use items/abilities
- Different behavior patterns

#### Tests
- Pathfinding correctness
- Tactical decisions
- Group coordination

---

### Task 2.11: Magic System ✨
**Priority**: 🟢 Low
**Estimated Time**: 3-4 days
**Dependencies**: Task 2.7

#### Components
```rust
pub struct Mana {
    pub current: i32,
    pub max: i32,
    pub regen_rate: i32,
}

pub enum SpellType {
    Damage(DamageSpell),
    Heal(HealSpell),
    Buff(BuffSpell),
    Debuff(DebuffSpell),
    Utility(UtilitySpell),
}
```

#### Features
- Mana system
- Spell learning
- Spell casting
- Spell effects
- Magic items

---

### Task 2.12: Crafting System 🔨
**Priority**: 🟢 Low
**Estimated Time**: 2-3 days
**Dependencies**: Task 2.2

#### Features
- Gather resources
- Learn recipes
- Craft items
- Improve equipment
- Special materials

---

## Implementation Order

### Phase 2A: Core Systems (Week 1-2)
1. ✅ Fix camping bug (from Phase 1)
2. Task 2.1: Inventory System
3. Task 2.2: Item System
4. Task 2.3: Enhanced Combat

### Phase 2B: World Content (Week 3-4)
5. Task 2.4: Building Interiors
6. Task 2.5: NPC System
7. Task 2.8: Terrain-Based Maps

### Phase 2C: Progression (Week 5-6)
8. Task 2.6: Quest System
9. Task 2.7: Experience & Leveling
10. Task 2.9: Multi-Level Dungeons

### Phase 2D: Polish (Week 7-8)
11. Task 2.10: Advanced AI
12. Task 2.11: Magic System (optional)
13. Task 2.12: Crafting System (optional)

---

## Testing Strategy

### Integration Tests
- Inventory management workflow
- Combat with items/equipment
- NPC interaction and trading
- Quest completion flows
- Full gameplay loops

### Performance Tests
- Item system with 1000+ items
- NPC pathfinding with 50+ NPCs
- Large dungeon generation
- Save/load with complex state

---

## Success Criteria

Phase 2 is complete when:

- ✅ Player can manage inventory (add/remove/equip items)
- ✅ Combat has tactical depth (actions, equipment matters)
- ✅ Buildings have detailed interiors with NPCs
- ✅ NPCs can be talked to and traded with
- ✅ Quests can be accepted and completed
- ✅ Character progression through XP/levels works
- ✅ All systems have integration tests
- ✅ Game is playable for 2-3 hour sessions

---

## Next Steps

**Starting with Task 2.1: Inventory System**

This is the foundation for items, equipment, and most other Phase 2 features. Once inventory is working, everything else builds on top of it.

Ready to begin? 🚀
