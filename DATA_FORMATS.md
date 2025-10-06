# Data Formats Specification

**Version**: 1.0
**Last Updated**: 2025-10-07

---

## Table of Contents

1. [Overview](#overview)
2. [General Conventions](#general-conventions)
3. [Item Data](#item-data)
4. [Monster Data](#monster-data)
5. [Recipe Data](#recipe-data)
6. [Faction Data](#faction-data)
7. [Quest Data](#quest-data)
8. [Dialogue Data](#dialogue-data)
9. [Lexeme Data](#lexeme-data)
10. [Mutation Data](#mutation-data)
11. [Lore Data](#lore-data)
12. [Location Data](#location-data)

---

## Overview

All game content is defined in JSON files for easy modding and iteration. This document specifies the exact format for each data type.

### Benefits

- **No Recompilation**: Modify content without rebuilding
- **Modding Support**: Easy for community to add content
- **Version Control**: Track content changes separately from code
- **Validation**: Can validate JSON against schemas
- **Hot Reload**: Can reload data during development

---

## General Conventions

### File Naming

- Use `snake_case` for file names
- Group related content in subdirectories
- Use `.json` extension

### JSON Style

```json
{
  "id": "use_snake_case",
  "name": "Use Title Case",
  "description": "Use full sentences with proper punctuation.",
  "numbers": 100,
  "booleans": true,
  "arrays": ["one", "two", "three"],
  "nested": {
    "key": "value"
  }
}
```

### IDs

- Unique string identifiers
- Use `category_name` format (e.g., `weapon_iron_sword`)
- Never change IDs (breaks save compatibility)

### Colors

Format: `"#RRGGBB"` or named colors: `"red"`, `"blue"`, etc.

### Common Fields

Most data types share these fields:

```json
{
  "id": "unique_identifier",
  "name": "Display Name",
  "description": "Player-visible description.",
  "tags": ["tag1", "tag2"]
}
```

---

## Item Data

### Location

`data/items/`
- `weapons.json`
- `armor.json`
- `consumables.json`
- `tools.json`
- `artifacts.json`

### Schema

```json
{
  "items": [
    {
      "id": "weapon_iron_sword",
      "name": "Iron Sword",
      "description": "A well-balanced blade of tempered iron.",

      "item_type": "Weapon",
      "category": "MeleeWeapon",

      "glyph": "/",
      "color": "#C0C0C0",

      "weight": 3.5,
      "value": 100,

      "combat": {
        "damage": "1d8+2",
        "damage_type": "Slashing",
        "hit_bonus": 0,
        "crit_chance": 0.05
      },

      "requirements": {
        "strength": 10
      },

      "tags": ["metal", "weapon", "common"],

      "corruption_cost": 0
    },

    {
      "id": "consumable_health_potion",
      "name": "Health Potion",
      "description": "A crimson liquid that mends wounds.",

      "item_type": "Consumable",
      "category": "Potion",

      "glyph": "!",
      "color": "#FF0000",

      "weight": 0.5,
      "value": 50,

      "effects": [
        {
          "type": "Heal",
          "amount": "2d8+4"
        }
      ],

      "use_time": 1,

      "tags": ["potion", "healing", "common"]
    },

    {
      "id": "artifact_eye_of_dael",
      "name": "Eye of Dæl",
      "description": "A pulsating crystalline eye that whispers forbidden truths.",

      "item_type": "Artifact",
      "category": "Amulet",

      "glyph": "*",
      "color": "#9B30FF",

      "weight": 0.1,
      "value": 10000,

      "effects": [
        {
          "type": "StatModifier",
          "stat": "insight",
          "amount": 25
        },
        {
          "type": "PeriodicEffect",
          "effect": "SanityDrain",
          "amount": 1,
          "interval_hours": 1
        }
      ],

      "equip_slot": "Amulet",

      "tags": ["artifact", "cursed", "unique"],

      "corruption_cost": 5,

      "lore_entry": "lore_eye_of_dael"
    }
  ]
}
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier |
| `name` | string | Yes | Display name |
| `description` | string | Yes | Player-visible text |
| `item_type` | enum | Yes | Weapon, Armor, Consumable, Tool, Artifact, Junk |
| `category` | string | Yes | Subcategory (e.g., MeleeWeapon, LightArmor) |
| `glyph` | char | Yes | ASCII character |
| `color` | string | Yes | Hex color or named |
| `weight` | float | Yes | Weight in pounds |
| `value` | int | Yes | Gold value |
| `combat` | object | No | Combat stats (weapons) |
| `effects` | array | No | Item effects |
| `requirements` | object | No | Stat requirements |
| `equip_slot` | string | No | Equipment slot |
| `tags` | array | Yes | String tags |
| `corruption_cost` | int | No | Corruption on use/equip (default: 0) |
| `lore_entry` | string | No | Associated lore ID |

### Damage Format

Dice notation: `XdY+Z` where:
- X = number of dice
- Y = die size
- Z = modifier

Examples: `1d6`, `2d8+4`, `3d10-2`

---

## Monster Data

### Location

`data/monsters/`
- `mundane.json` - Normal animals and humanoids
- `corrupted.json` - Corrupted variants
- `reality_entities.json` - Cosmic horrors

### Schema

```json
{
  "monsters": [
    {
      "id": "goblin",
      "name": "Goblin",
      "description": "A small, green-skinned humanoid with sharp teeth.",

      "glyph": "g",
      "color": "#00FF00",
      "z_order": 10,

      "stats": {
        "hp": 12,
        "power": 4,
        "defense": 2,
        "speed": 100
      },

      "ai_type": "Melee",
      "vision_range": 8,
      "faction": "hostile",

      "loot_table": "goblin_loot",
      "xp_value": 10,

      "tags": ["humanoid", "common"],

      "special_abilities": []
    },

    {
      "id": "void_spawn",
      "name": "Void Spawn",
      "description": "A writhing mass of impossible geometry and grasping tendrils.",

      "glyph": "V",
      "color": "#1A1A2E",
      "z_order": 15,

      "stats": {
        "hp": 45,
        "power": 15,
        "defense": 8,
        "speed": 80
      },

      "ai_type": "RealityEntity",
      "vision_range": 12,
      "faction": "void",

      "requires_layer": "Cosmic",
      "corrupts_nearby": true,

      "loot_table": "void_loot",
      "xp_value": 100,

      "tags": ["reality_entity", "void", "rare"],

      "special_abilities": [
        {
          "type": "SanityDamage",
          "amount": 5,
          "radius": 3,
          "trigger": "OnSight"
        },
        {
          "type": "Spell",
          "lexeme": "VOL GRASP FOE",
          "chance": 0.3
        }
      ],

      "resistances": {
        "physical": 0.5,
        "reality": 0.0
      },

      "vulnerabilities": {
        "holy": 2.0
      }
    }
  ]
}
```

### AI Types

- `Passive` - Does not attack
- `Melee` - Chases and attacks in melee
- `Ranged` - Keeps distance and shoots
- `Caster` - Uses spells
- `RealityEntity` - Special cosmic horror AI
- `Boss` - Scripted boss behavior

---

## Recipe Data

### Location

`data/recipes/`
- `weapons.json`
- `armor.json`
- `alchemy.json`
- `food.json`
- `rituals.json`

### Schema

```json
{
  "recipes": [
    {
      "id": "recipe_iron_sword",
      "name": "Iron Sword",
      "description": "Forge a simple iron sword.",

      "result": {
        "item_id": "weapon_iron_sword",
        "quantity": 1
      },

      "requirements": {
        "ingredients": [
          { "item_id": "iron_ingot", "quantity": 3 },
          { "item_id": "leather_strips", "quantity": 1 }
        ],

        "tools": ["forge", "hammer"],

        "location_type": "Forge",

        "skill": {
          "name": "Blacksmithing",
          "level": 2
        },

        "time_minutes": 60
      },

      "category": "Blacksmithing",
      "difficulty": "Easy",

      "tags": ["weapon", "metal"],

      "corruption_cost": 0
    },

    {
      "id": "recipe_summon_familiar",
      "name": "Summon Familiar",
      "description": "A blood ritual to summon a servant from beyond.",

      "result": {
        "effect": "SummonEntity",
        "entity_id": "familiar_void_imp",
        "duration_hours": 24
      },

      "requirements": {
        "ingredients": [
          { "item_id": "fresh_blood", "quantity": 1 },
          { "item_id": "chalk", "quantity": 1 },
          { "item_id": "candles", "quantity": 5 }
        ],

        "lexemes": ["VOL", "SUMMON", "GRASP"],

        "location_type": "RitualCircle",

        "sanity_cost": 15,
        "insight_required": 40,

        "time_minutes": 30
      },

      "category": "BloodMagic",
      "difficulty": "Hard",

      "tags": ["ritual", "summoning", "dark"],

      "corruption_cost": 10,

      "lore_entry": "lore_summoning_rituals"
    }
  ]
}
```

### Result Types

- `Item` - Creates item(s)
- `Effect` - Applies effect (buff, summon, etc.)
- `Both` - Creates item with special effect

---

## Faction Data

### Location

`data/factions.json`

### Schema

```json
{
  "factions": [
    {
      "id": "royal_crown",
      "name": "The Royal Crown",
      "description": "The ruling monarchy of Kándavael, growing increasingly paranoid.",

      "type": "Government",
      "alignment": "LawfulNeutral",

      "color": "#FFD700",
      "symbol": "♔",

      "starting_reputation": 50,

      "goals": [
        "Maintain order",
        "Suppress information about corruption",
        "Protect trade routes"
      ],

      "territory": {
        "capital": "Daelspire",
        "settlements": ["Daelspire", "Thornhaven", "Eastwatch"]
      },

      "relationships": {
        "holy_church": 25,
        "wardens_order": 50,
        "merchants_guild": 75,
        "stitchers": -25,
        "cult_of_dael": -100
      },

      "ranks": [
        { "threshold": -100, "name": "Enemy of the State" },
        { "threshold": -50, "name": "Wanted Criminal" },
        { "threshold": 0, "name": "Citizen" },
        { "threshold": 50, "name": "Respected" },
        { "threshold": 75, "name": "Knight" },
        { "threshold": 100, "name": "Lord/Lady" }
      ],

      "perks": {
        "50": "Access to royal guards as companions",
        "75": "10% discount at crown-affiliated shops",
        "100": "Can request military support"
      }
    }
  ]
}
```

---

## Quest Data

### Location

`data/quests/`
- `main_story.json`
- `side_quests.json`
- `dynamic_templates.json`

### Schema

```json
{
  "quests": [
    {
      "id": "main_001_heros_welcome",
      "name": "Hero's Welcome",
      "description": "The people of Daelspire celebrate your victory over the bandits.",

      "type": "MainQuest",
      "act": 1,
      "order": 1,

      "giver": {
        "type": "NPC",
        "npc_id": "mayor_aldric"
      },

      "location": "settlement_daelspire",

      "stages": [
        {
          "id": "stage_1",
          "description": "Attend the celebration feast.",

          "objectives": [
            {
              "type": "GoToLocation",
              "location": "building_daelspire_town_hall",
              "marker": true
            }
          ]
        },
        {
          "id": "stage_2",
          "description": "Speak with the mayor.",

          "objectives": [
            {
              "type": "TalkToNPC",
              "npc_id": "mayor_aldric",
              "dialogue_tree": "main_001_stage_2"
            }
          ]
        },
        {
          "id": "stage_3",
          "description": "Investigate the strange occurrence at the beacon.",

          "objectives": [
            {
              "type": "GoToLocation",
              "location": "unique_the_beacon",
              "marker": true
            },
            {
              "type": "KillTarget",
              "target_id": "corrupted_warden",
              "quantity": 1
            }
          ]
        }
      ],

      "rewards": {
        "gold": 500,
        "xp": 1000,
        "items": ["artifact_beacon_shard"],
        "reputation": {
          "royal_crown": 10
        },
        "unlocks": ["main_002_growing_doubts"]
      },

      "failure_conditions": [
        {
          "type": "NPCDied",
          "npc_id": "mayor_aldric"
        }
      ],

      "time_limit": null,

      "tags": ["main", "act1", "story"]
    },

    {
      "id": "template_bounty",
      "name": "{Monster} Bounty",
      "description": "A {monster} has been terrorizing {location}. Hunt it down.",

      "type": "DynamicTemplate",
      "category": "Bounty",

      "variables": {
        "monster": "monster_id",
        "location": "settlement_id",
        "reward_gold": "50-200"
      },

      "objectives": [
        {
          "type": "KillTarget",
          "target_id": "{monster}",
          "quantity": 1
        },
        {
          "type": "ReturnToQuestGiver"
        }
      ],

      "rewards": {
        "gold": "{reward_gold}",
        "xp": "{reward_gold * 2}",
        "reputation": {
          "giver_faction": 5
        }
      },

      "generation_rules": {
        "monster_must_be_threat": true,
        "location_near_giver": true,
        "cooldown_days": 3
      }
    }
  ]
}
```

### Objective Types

- `GoToLocation` - Travel to location
- `TalkToNPC` - Dialogue with NPC
- `KillTarget` - Defeat entity/entities
- `FetchItem` - Obtain item(s)
- `DeliverItem` - Give item to NPC
- `Escort` - Protect NPC during travel
- `Investigate` - Discover clue/information
- `Wait` - Time passes
- `Custom` - Scripted objective

---

## Dialogue Data

### Location

`data/dialogue/`
- `common.json` - Generic NPC responses
- `main_quest/*.json` - Main quest dialogues
- `npcs/*.json` - Specific NPC dialogues

### Schema

```json
{
  "dialogues": {
    "greeting_commoner": {
      "id": "greeting_commoner",
      "speaker": "Commoner",

      "root": "node_greeting",

      "nodes": {
        "node_greeting": {
          "text": "Good day, traveler. What brings you to {settlement_name}?",

          "conditions": [],

          "choices": [
            {
              "text": "Just passing through.",
              "next": "node_passing_through"
            },
            {
              "text": "I'm looking for work.",
              "next": "node_looking_for_work",
              "conditions": [
                { "type": "QuestAvailable", "any": true }
              ]
            },
            {
              "text": "Tell me about the corruption.",
              "next": "node_corruption",
              "conditions": [
                { "type": "InsightGreaterThan", "value": 30 }
              ]
            },
            {
              "text": "Goodbye.",
              "next": "node_exit"
            }
          ]
        },

        "node_passing_through": {
          "text": "Safe travels! Watch out for bandits on the roads.",

          "effects": [],

          "choices": [
            {
              "text": "Thanks for the warning.",
              "next": "node_exit"
            }
          ]
        },

        "node_looking_for_work": {
          "text": "You should speak with the mayor. There's always something needs doing.",

          "effects": [
            {
              "type": "RevealQuestGiver",
              "npc_id": "mayor"
            }
          ],

          "choices": [
            {
              "text": "Where can I find the mayor?",
              "next": "node_mayor_location"
            },
            {
              "text": "I'll do that.",
              "next": "node_exit"
            }
          ]
        },

        "node_corruption": {
          "text": "[The commoner glances around nervously] I... I don't know what you mean. Strange things? No, no. Nothing strange here...",

          "effects": [
            {
              "type": "ModifyReputation",
              "faction": "commoners",
              "amount": -5
            }
          ],

          "choices": [
            {
              "text": "I see. Never mind then.",
              "next": "node_exit"
            },
            {
              "text": "[Intimidate] Tell me what you know.",
              "next": "node_intimidate",
              "conditions": [
                { "type": "StatCheck", "stat": "power", "threshold": 15 }
              ],
              "stat_check": {
                "stat": "power",
                "threshold": 15,
                "success": "node_intimidate_success",
                "failure": "node_intimidate_failure"
              }
            }
          ]
        },

        "node_exit": {
          "text": "",
          "exit": true
        }
      }
    }
  }
}
```

### Condition Types

- `QuestStage` - Check quest progress
- `QuestAvailable` - Has available quests
- `ItemInInventory` - Has specific item
- `InsightGreaterThan` - Insight level check
- `ReputationGreaterThan` - Faction standing check
- `StatCheck` - Attribute check
- `FlagSet` - Global flag check

### Effect Types

- `GiveQuest` - Add quest to journal
- `CompleteQuestStage` - Progress quest
- `GiveItem` - Add item to inventory
- `TakeItem` - Remove item from inventory
- `ModifyReputation` - Change faction standing
- `SetFlag` - Set global flag
- `RevealQuestGiver` - Mark NPC on map

---

## Lexeme Data

### Location

`data/lexemes.json`

### Schema

```json
{
  "lexemes": [
    {
      "id": "lex_fire",
      "word": "IGNIS",
      "category": "Element",

      "description": "The primordial flame, destroyer and purifier.",

      "base_effect": {
        "type": "Damage",
        "element": "Fire",
        "base_damage": "2d6"
      },

      "discovery": {
        "method": "ItemRead",
        "item_id": "book_basic_lexemes",
        "difficulty": "Easy"
      },

      "sanity_cost": 2,
      "notice_increase": 1,

      "tags": ["element", "offensive", "basic"]
    },

    {
      "id": "lex_great",
      "word": "MAG",
      "category": "Modifier",

      "description": "Amplifies the power of the following lexeme.",

      "modifier": {
        "type": "PowerMultiplier",
        "multiplier": 1.5
      },

      "must_precede": ["Element", "Action"],

      "discovery": {
        "method": "Experiment",
        "requires_lexemes": ["IGNIS", "AQUA", "TERRA"],
        "insight_required": 40
      },

      "sanity_cost": 5,
      "notice_increase": 2,

      "tags": ["modifier", "power", "advanced"]
    },

    {
      "id": "lex_summon",
      "word": "EVOCA",
      "category": "Action",

      "description": "Calls forth entities from beyond the veil.",

      "base_effect": {
        "type": "Summon",
        "entity_pool": "summons",
        "duration_turns": 10
      },

      "requires_target_type": "Entity",

      "discovery": {
        "method": "Ritual",
        "ritual_id": "ritual_first_summoning",
        "corruption_required": 25
      },

      "sanity_cost": 15,
      "notice_increase": 10,

      "tags": ["action", "summoning", "dangerous"]
    }
  ],

  "grammar": {
    "valid_orders": [
      ["Modifier", "Element", "Target"],
      ["Element", "Target"],
      ["Action", "Entity", "Target"],
      ["Modifier", "Action", "Target"]
    ],

    "max_length": 5
  },

  "combinations": [
    {
      "lexemes": ["IGNIS", "FOE"],
      "name": "Fireball",
      "description": "Hurls a ball of flame at an enemy."
    },
    {
      "lexemes": ["MAG", "IGNIS", "OMNI"],
      "name": "Inferno",
      "description": "Unleashes a massive firestorm around you."
    }
  ]
}
```

---

## Mutation Data

### Location

`data/mutations.json`

### Schema

```json
{
  "mutations": [
    {
      "id": "mut_third_eye",
      "name": "Third Eye",
      "description": "A pulsating eye has opened on your forehead, granting terrible insight.",

      "category": "CosmicInsight",
      "tier": 1,

      "effects": [
        {
          "type": "StatModifier",
          "stat": "insight",
          "amount": 10
        },
        {
          "type": "VisionBonus",
          "amount": 2
        },
        {
          "type": "PeriodicSanityDrain",
          "amount": 1,
          "interval_hours": 6
        }
      ],

      "visual_changes": {
        "glyph_prefix": "Ꚛ",
        "description_append": "A third eye stares from their forehead."
      },

      "acquisition": {
        "methods": ["CorruptionThreshold", "Ritual", "Item"],
        "corruption_threshold": 30,
        "insight_required": 50
      },

      "prerequisites": [],
      "conflicts": [],

      "tags": ["insight", "vision", "tier1"],

      "npc_reaction_modifier": -10
    },

    {
      "id": "mut_void_touched",
      "name": "Void-Touched",
      "description": "Your flesh is partially phased out of reality, making you harder to hit but difficult to heal.",

      "category": "DimensionalTaint",
      "tier": 3,

      "effects": [
        {
          "type": "DefenseModifier",
          "amount": 5
        },
        {
          "type": "HealingResistance",
          "multiplier": 0.5
        },
        {
          "type": "PhaseChance",
          "chance": 0.15,
          "description": "15% chance to phase through attacks"
        }
      ],

      "visual_changes": {
        "color_tint": "#1A1A2E",
        "description_append": "Parts of their body flicker in and out of existence."
      },

      "acquisition": {
        "methods": ["Ritual", "Event"],
        "corruption_threshold": 75
      },

      "prerequisites": ["mut_third_eye", "mut_reality_sense"],
      "conflicts": ["mut_flesh_titan"],

      "tags": ["void", "defensive", "tier3"],

      "npc_reaction_modifier": -30,

      "threshold_mutation": {
        "category": "DimensionalTaint",
        "required_count": 5,
        "description": "Obtaining 5 Dimensional Taint mutations transforms you"
      }
    }
  ],

  "categories": [
    {
      "id": "CosmicInsight",
      "name": "Cosmic Insight",
      "description": "Mutations that enhance perception and understanding at the cost of sanity.",
      "color": "#9B30FF"
    },
    {
      "id": "FleshWarping",
      "name": "Flesh Warping",
      "description": "Physical transformations that enhance body at the cost of humanity.",
      "color": "#8B0000"
    },
    {
      "id": "DimensionalTaint",
      "name": "Dimensional Taint",
      "description": "Existence partially shifted between realities.",
      "color": "#1A1A2E"
    },
    {
      "id": "ParasiticGrowth",
      "name": "Parasitic Growth",
      "description": "Living entities bonded to your flesh.",
      "color": "#2E8B57"
    },
    {
      "id": "VoidTouched",
      "name": "Void Touched",
      "description": "Touched by the void between realities.",
      "color": "#0F0F1E"
    }
  ]
}
```

---

## Lore Data

### Location

`data/lore/`
- `books.json`
- `notes.json`
- `inscriptions.json`

### Schema

```json
{
  "lore_entries": [
    {
      "id": "lore_the_betrayal",
      "title": "The Betrayal of Thælavîm",

      "type": "Book",
      "category": "History",

      "content": [
        "In the year 742, the hero Kándavael returned victorious from the Void Wars.",
        "The people celebrated, unaware of the terrible price he had paid.",
        "For Kándavael had not defeated the Void—he had made a pact with it.",
        "",
        "The entity known as Dæl whispered promises of eternal peace.",
        "All that was required was five beacons, lit across the land.",
        "Five beacons to 'seal' the rift. Five beacons to hold back the darkness.",
        "",
        "But the beacons were not seals—they were invitations.",
        "And now, in the year 762, the truth can no longer be hidden.",
        "The beacons pulse with eldritch light.",
        "Reality itself begins to fray.",
        "",
        "[The rest of the page is covered in frantic scrawlings]",
        "[THEY ARE COMING THROUGH]",
        "[HE LIED HE LIED HE LIED]"
      ],

      "author": "Warden Historian Elric the Mad",
      "date": "Year 762, 3rd of Duskmoon",

      "insight_gain": 5,
      "sanity_cost": 2,

      "unlocks": {
        "quest": "main_010_the_truth",
        "codex_entries": ["codex_the_beacons", "codex_dael"],
        "dialogue_options": ["dialogue_confrontation"]
      },

      "locations": [
        "building_saelcairn_library",
        "dungeon_abandoned_archive"
      ],

      "tags": ["history", "main_story", "dangerous_knowledge"]
    },

    {
      "id": "lore_note_warning",
      "title": "Hastily Scrawled Note",

      "type": "Note",
      "category": "Warning",

      "content": [
        "Don't go to the village.",
        "They're all dead, but they won't stop moving.",
        "Their eyes... wrong. All wrong.",
        "Heading east. Maybe the church can help.",
        "Maybe.",
        "",
        "— T."
      ],

      "author": "Unknown",
      "date": null,

      "insight_gain": 1,
      "sanity_cost": 0,

      "unlocks": {
        "quest": "dynamic_investigate_village"
      },

      "tags": ["warning", "corruption", "common"]
    }
  ]
}
```

---

## Location Data

### Location

`data/locations/unique/`
- `daelspire.json`
- `saelcairn.json`
- `the_wound.json`
- etc.

### Schema

```json
{
  "id": "unique_daelspire",
  "name": "Daelspire",
  "type": "City",

  "description": "The gleaming capital of Kándavael, built around the central Beacon Tower.",

  "overmap_position": { "x": 100, "y": 100 },
  "overmap_size": { "width": 3, "height": 3 },

  "symbol": "◆",
  "color": "#FFD700",

  "population": 15000,
  "faction": "royal_crown",

  "buildings": [
    {
      "id": "building_palace",
      "name": "Royal Palace",
      "type": "Palace",
      "position": { "x": 50, "y": 50 },
      "size": { "width": 20, "height": 20 },
      "entrance": { "x": 50, "y": 60 },
      "npcs": ["king_alduin", "queen_elara", "court_wizard"],
      "guards": 20,
      "locked": true,
      "key_required": "palace_key"
    },
    {
      "id": "building_beacon_tower",
      "name": "The Beacon Tower",
      "type": "Unique",
      "position": { "x": 50, "y": 30 },
      "size": { "width": 10, "height": 10 },
      "entrance": { "x": 50, "y": 40 },
      "interior_map": "maps/beacon_tower.json",
      "corruption_source": true,
      "corruption_radius": 10,
      "guarded": true
    }
  ],

  "npcs": {
    "unique": ["king_alduin", "mayor_aldric", "sage_elara"],
    "generated": {
      "shopkeepers": 15,
      "guards": 50,
      "commoners": 200
    }
  },

  "shops": [
    {
      "id": "shop_general",
      "name": "Daelspire General Store",
      "type": "General",
      "position": { "x": 40, "y": 50 },
      "inventory_categories": ["tools", "consumables", "basic_gear"],
      "gold": 5000
    }
  ],

  "quests": [
    "main_001_heros_welcome",
    "main_002_growing_doubts",
    "side_noble_intrigue"
  ],

  "special_events": [
    {
      "id": "event_festival",
      "trigger": { "type": "Time", "month": 6, "day": 15 },
      "description": "The Summer Festival fills the streets with celebration."
    },
    {
      "id": "event_beacon_pulse",
      "trigger": { "type": "Corruption", "threshold": 30 },
      "description": "The Beacon Tower pulses with unnatural light."
    }
  ],

  "corruption_stages": [
    {
      "threshold": 0,
      "description": "The city gleams in the sunlight, a beacon of civilization."
    },
    {
      "threshold": 30,
      "description": "Whispers echo through the streets at night. People go missing.",
      "effects": ["curfew", "increased_guards", "paranoia"]
    },
    {
      "threshold": 60,
      "description": "Reality cracks. Buildings shift when unobserved. NPCs corrupted.",
      "effects": ["martial_law", "mass_madness", "reality_rifts"]
    },
    {
      "threshold": 100,
      "description": "The city has fallen. Void entities rule the twisted ruins.",
      "effects": ["total_collapse", "no_npcs", "cosmic_layer_only"]
    }
  ],

  "tags": ["capital", "main_story", "unique"]
}
```

---

## Validation

### JSON Schema Validation

Use JSON Schema to validate data files:

```bash
# Install validator
npm install -g ajv-cli

# Validate
ajv validate -s schemas/item.schema.json -d data/items/weapons.json
```

### In-Game Validation

```rust
// Load and validate on startup
let items = load_items("data/items/")?;
validate_items(&items)?;  // Check references, ranges, etc.
```

### Common Validation Rules

1. **ID Uniqueness**: No duplicate IDs across all data
2. **Reference Validity**: All referenced IDs must exist
3. **Range Checks**: Numeric values within valid ranges
4. **Required Fields**: All required fields present
5. **Enum Values**: String values match defined enums

---

## Modding Support

### Adding New Content

1. Create JSON file in appropriate `data/` directory
2. Follow schema exactly
3. Use unique ID with mod prefix: `modname_item_id`
4. Test in-game
5. Share with community

### Example Mod Structure

```
my_cosmic_horror_mod/
├── data/
│   ├── items/
│   │   └── my_items.json
│   ├── monsters/
│   │   └── my_monsters.json
│   └── mod.json
└── README.md
```

### Mod Metadata

```json
{
  "id": "my_cosmic_horror_mod",
  "name": "My Cosmic Horror Mod",
  "version": "1.0.0",
  "author": "Modder Name",
  "description": "Adds new eldritch horrors to encounter.",
  "game_version": "1.0.0",
  "dependencies": [],
  "content": {
    "items": 10,
    "monsters": 5,
    "quests": 3
  }
}
```

---

## Conclusion

This specification defines all data formats for the game. All content should follow these schemas exactly for consistency and compatibility.

For validation schemas and examples, see the `/schemas` directory.

---

*"Data defines reality. Define carefully."*
