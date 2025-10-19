/// Item spawning and generation system
///
/// Creates items from templates and places them in the world

use crate::ecs::{
    Consumable, Equipable, EquipSlot, Item, ItemCategory, ItemData, OnGround, Position,
    RealityLayer, Renderable, Stackable,
};
use hecs::World;
use rand::Rng;

/// Item rarity affects drop chance and stats
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl ItemRarity {
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            ItemRarity::Common => ratatui::style::Color::Gray,
            ItemRarity::Uncommon => ratatui::style::Color::Green,
            ItemRarity::Rare => ratatui::style::Color::Blue,
            ItemRarity::Epic => ratatui::style::Color::Magenta,
        }
    }
}

/// Spawn a healing potion at the given position
pub fn spawn_healing_potion(world: &mut World, x: i32, y: i32) -> hecs::Entity {
    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph: '!',
            fg: ratatui::style::Color::Red,
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            "Healing Potion",
            "Restores 20 HP when consumed.",
            "A crimson liquid swirls within this small glass vial. The potion glows \
             faintly with restorative magic. Common among adventurers, these potions \
             are brewed by alchemists using medicinal herbs and minor healing enchantments.\n\n\
             Effect: Restores 20 HP instantly.\n\
             Uses: Single use, vial shatters after consumption.",
            1,
            25,
            ItemCategory::Consumable,
            10, // Max stack of 10 potions
        ),
        Consumable::healing_potion(),
        Stackable::single(),
    ))
}

/// Spawn a rusty sword at the given position
pub fn spawn_rusty_sword(world: &mut World, x: i32, y: i32) -> hecs::Entity {
    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph: '/',
            fg: ratatui::style::Color::Gray,
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            "Rusty Sword",
            "A worn blade that has seen better days.",
            "This iron sword has seen extensive use and poor maintenance. Orange rust \
             spots the blade, and nicks mar its edge. Despite its condition, the blade \
             remains serviceable for basic combat. Better than nothing, but barely.\n\n\
             Damage: +2 Power\n\
             Condition: Poor\n\
             Type: One-handed slashing weapon",
            3,
            10,
            ItemCategory::Weapon,
            1, // Non-stackable
        ),
        Equipable::with_bonuses(EquipSlot::MainHand, 2, 0),
    ))
}

/// Spawn an iron sword at the given position
pub fn spawn_iron_sword(world: &mut World, x: i32, y: i32) -> hecs::Entity {
    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph: '/',
            fg: ratatui::style::Color::White,
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            "Iron Sword",
            "A well-crafted iron blade.",
            "Forged by a skilled blacksmith, this iron sword features a straight double-edged \
             blade with a simple crossguard. The metal has been properly tempered and shows \
             professional craftsmanship. A reliable weapon for any warrior.\n\n\
             Damage: +5 Power\n\
             Condition: Excellent\n\
             Type: One-handed slashing weapon",
            5,
            50,
            ItemCategory::Weapon,
            1,
        ),
        Equipable::with_bonuses(EquipSlot::MainHand, 5, 0),
    ))
}

/// Spawn a wooden shield at the given position
pub fn spawn_wooden_shield(world: &mut World, x: i32, y: i32) -> hecs::Entity {
    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph: '[',
            fg: ratatui::style::Color::Yellow,
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            "Wooden Shield",
            "A simple wooden shield.",
            "Constructed from sturdy oak planks reinforced with iron bands, this round shield \
             provides basic protection. While not as durable as metal shields, it's lightweight \
             and effective against common weapons.\n\n\
             Defense: +2 Defense\n\
             Type: Small shield\n\
             Material: Oak wood with iron bands",
            4,
            15,
            ItemCategory::Armor,
            1,
        ),
        Equipable::with_bonuses(EquipSlot::OffHand, 0, 2),
    ))
}

/// Spawn leather armor at the given position
pub fn spawn_leather_armor(world: &mut World, x: i32, y: i32) -> hecs::Entity {
    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph: ']',
            fg: ratatui::style::Color::Yellow,
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            "Leather Armor",
            "Sturdy leather protection.",
            "Crafted from thick, cured leather panels stitched together with reinforced seams. \
             The armor is flexible enough to allow free movement while providing solid protection \
             against slashing attacks. Worn by scouts and light infantry who value mobility.\\n\\n\
             Defense: +3 Defense\\n\
             Type: Light armor\\n\
             Material: Hardened leather with cloth backing",
            8,
            30,
            ItemCategory::Armor,
            1,
        ),
        Equipable::with_bonuses(EquipSlot::Body, 0, 3),
    ))
}

/// Spawn a random item at the given position
pub fn spawn_random_item<R: Rng>(world: &mut World, x: i32, y: i32, rng: &mut R) -> hecs::Entity {
    let roll = rng.gen_range(0..100);

    match roll {
        0..=30 => spawn_healing_potion(world, x, y),      // 30% - Healing potion
        31..=50 => spawn_rusty_sword(world, x, y),        // 20% - Rusty sword
        51..=65 => spawn_wooden_shield(world, x, y),      // 15% - Wooden shield
        66..=80 => spawn_leather_armor(world, x, y),      // 15% - Leather armor
        81..=100 => spawn_iron_sword(world, x, y),        // 20% - Iron sword
        _ => spawn_healing_potion(world, x, y),
    }
}

/// Spawn items in a room (called during dungeon generation)
pub fn spawn_items_in_room<R: Rng>(
    world: &mut World,
    room_tiles: &[(i32, i32)],
    rng: &mut R,
) {
    // 50% chance to spawn 1-2 items in the room
    if rng.gen_bool(0.5) {
        let item_count = rng.gen_range(1..=2);

        for _ in 0..item_count {
            // Pick a random tile in the room
            if let Some(&(x, y)) = room_tiles.get(rng.gen_range(0..room_tiles.len())) {
                spawn_random_item(world, x, y, rng);
            }
        }
    }
}

/// Generate a weapon with random bonuses
pub fn spawn_random_weapon<R: Rng>(
    world: &mut World,
    x: i32,
    y: i32,
    rarity: ItemRarity,
    rng: &mut R,
) -> hecs::Entity {
    let (name, base_power, value, glyph) = match rarity {
        ItemRarity::Common => {
            let weapons = [
                ("Club", 1, 5, '|'),
                ("Dagger", 2, 8, '-'),
                ("Short Sword", 3, 12, '/'),
            ];
            weapons[rng.gen_range(0..weapons.len())]
        }
        ItemRarity::Uncommon => {
            let weapons = [
                ("Longsword", 4, 25, '/'),
                ("Mace", 4, 22, '|'),
                ("Axe", 5, 28, '~'),
            ];
            weapons[rng.gen_range(0..weapons.len())]
        }
        ItemRarity::Rare => {
            let weapons = [
                ("Greatsword", 7, 60, '/'),
                ("War Hammer", 7, 55, '|'),
                ("Battle Axe", 8, 65, '~'),
            ];
            weapons[rng.gen_range(0..weapons.len())]
        }
        ItemRarity::Epic => {
            let weapons = [
                ("Legendary Blade", 12, 200, '/'),
                ("Dragon Slayer", 15, 300, '/'),
                ("Mjolnir", 13, 250, '|'),
            ];
            weapons[rng.gen_range(0..weapons.len())]
        }
    };

    let bonus_power = rng.gen_range(0..=2);
    let total_power = base_power + bonus_power;

    let quality = match rarity {
        ItemRarity::Common => "common",
        ItemRarity::Uncommon => "well-made",
        ItemRarity::Rare => "rare",
        ItemRarity::Epic => "legendary",
    };

    let detailed_desc = format!(
        "A {} {} forged with varying degrees of craftsmanship.\\n\\n\
         Damage: +{} Power\\n\
         Quality: {}\\n\
         Type: Melee weapon",
        quality, name.to_lowercase(), total_power,
        match rarity {
            ItemRarity::Common => "Standard",
            ItemRarity::Uncommon => "Fine",
            ItemRarity::Rare => "Exceptional",
            ItemRarity::Epic => "Masterwork",
        }
    );

    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph,
            fg: rarity.color(),
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            name,
            format!("A {} weapon.", quality),
            detailed_desc,
            3 + base_power / 2,
            value,
            ItemCategory::Weapon,
            1,
        ),
        Equipable::with_bonuses(EquipSlot::MainHand, total_power, 0),
    ))
}

/// Generate armor with random bonuses
pub fn spawn_random_armor<R: Rng>(
    world: &mut World,
    x: i32,
    y: i32,
    rarity: ItemRarity,
    rng: &mut R,
) -> hecs::Entity {
    let (name, base_defense, value) = match rarity {
        ItemRarity::Common => {
            let armors = [
                ("Cloth Robe", 1, 8),
                ("Padded Armor", 2, 12),
                ("Leather Vest", 2, 15),
            ];
            armors[rng.gen_range(0..armors.len())]
        }
        ItemRarity::Uncommon => {
            let armors = [
                ("Leather Armor", 3, 30),
                ("Studded Leather", 4, 35),
                ("Chainmail Shirt", 4, 40),
            ];
            armors[rng.gen_range(0..armors.len())]
        }
        ItemRarity::Rare => {
            let armors = [
                ("Scale Mail", 6, 80),
                ("Plate Armor", 7, 100),
                ("Enchanted Robe", 5, 90),
            ];
            armors[rng.gen_range(0..armors.len())]
        }
        ItemRarity::Epic => {
            let armors = [
                ("Dragon Scale Armor", 10, 300),
                ("Mithril Plate", 12, 400),
                ("Archmage Robes", 8, 350),
            ];
            armors[rng.gen_range(0..armors.len())]
        }
    };

    let bonus_defense = rng.gen_range(0..=2);
    let total_defense = base_defense + bonus_defense;

    let quality = match rarity {
        ItemRarity::Common => "basic",
        ItemRarity::Uncommon => "sturdy",
        ItemRarity::Rare => "exceptional",
        ItemRarity::Epic => "legendary",
    };

    let detailed_desc = format!(
        "A {} piece of protective equipment crafted with varying skill.\\n\\n\
         Defense: +{} Defense\\n\
         Quality: {}\\n\
         Type: Body armor",
        quality, total_defense,
        match rarity {
            ItemRarity::Common => "Standard",
            ItemRarity::Uncommon => "Fine",
            ItemRarity::Rare => "Exceptional",
            ItemRarity::Epic => "Masterwork",
        }
    );

    world.spawn((
        Position::new(x, y, RealityLayer::Normal),
        Renderable {
            glyph: ']',
            fg: rarity.color(),
            bg: ratatui::style::Color::Reset,
            z: 1,
        },
        Item,
        OnGround,
        ItemData::detailed(
            name,
            format!("A {} piece of armor.", quality),
            detailed_desc,
            5 + base_defense / 2,
            value,
            ItemCategory::Armor,
            1,
        ),
        Equipable::with_bonuses(EquipSlot::Body, 0, total_defense),
    ))
}
