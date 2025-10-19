use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use crate::domain_types::{Defense, HitPoints, MaxHitPoints, Money, Power};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealityLayer {
    Normal,
    Cosmic,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub layer: RealityLayer,
}

impl Position {
    pub fn new(x: i32, y: i32, layer: RealityLayer) -> Self {
        Self { x, y, layer }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Renderable {
    pub glyph: char,
    #[serde(with = "ColorDef")]
    pub fg: Color,
    #[serde(with = "ColorDef")]
    pub bg: Color,
    pub z: i32,
}

impl Renderable {
    pub fn new(glyph: char, fg: Color) -> Self {
        Self {
            glyph,
            fg,
            bg: Color::Reset,
            z: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Viewshed {
    pub range: i32,
    pub visible: Vec<(i32, i32)>,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            range,
            visible: Vec::new(),
            dirty: true,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CombatStats {
    pub hp: HitPoints,
    pub max_hp: MaxHitPoints,
    pub power: Power,
    pub defense: Defense,
}

impl CombatStats {
    pub fn new(hp: i32, power: i32, defense: i32) -> Self {
        let hp_val = HitPoints::new(hp);
        Self {
            hp: hp_val,
            max_hp: MaxHitPoints::new(hp),
            power: Power::new(power),
            defense: Defense::new(defense),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TriMeter {
    pub insight: i32,
    pub sanity: i32,
    pub notice: i32,
}

impl TriMeter {
    pub fn new() -> Self {
        Self {
            insight: 0,
            sanity: 100,
            notice: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Name(pub String);

// Marker components
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Monster;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BlocksMovement;

// ============================================================================
// Inventory & Items (Phase 2)
// ============================================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl EquipSlot {
    pub fn name(&self) -> &'static str {
        match self {
            EquipSlot::MainHand => "Main Hand",
            EquipSlot::OffHand => "Off Hand",
            EquipSlot::Head => "Head",
            EquipSlot::Body => "Body",
            EquipSlot::Legs => "Legs",
            EquipSlot::Feet => "Feet",
            EquipSlot::Ring1 => "Ring 1",
            EquipSlot::Ring2 => "Ring 2",
            EquipSlot::Amulet => "Amulet",
        }
    }

    pub fn all() -> [EquipSlot; 9] {
        [
            EquipSlot::MainHand,
            EquipSlot::OffHand,
            EquipSlot::Head,
            EquipSlot::Body,
            EquipSlot::Legs,
            EquipSlot::Feet,
            EquipSlot::Ring1,
            EquipSlot::Ring2,
            EquipSlot::Amulet,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Inventory {
    pub items: Vec<hecs::Entity>,
    pub capacity: usize,
    pub equipped: std::collections::HashMap<EquipSlot, hecs::Entity>,
}

// Custom serialization for Inventory (entity IDs as u64)
impl serde::Serialize for Inventory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Inventory", 3)?;

        // Serialize items as Vec<u32>
        let item_ids: Vec<u32> = self.items.iter().map(|e| e.id()).collect();
        state.serialize_field("items", &item_ids)?;
        state.serialize_field("capacity", &self.capacity)?;

        // Serialize equipped as Vec<(EquipSlot, u32)>
        let equipped_pairs: Vec<(EquipSlot, u32)> = self.equipped
            .iter()
            .map(|(slot, entity)| (*slot, entity.id()))
            .collect();
        state.serialize_field("equipped", &equipped_pairs)?;

        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for Inventory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Items, Capacity, Equipped }

        struct InventoryVisitor;

        impl<'de> Visitor<'de> for InventoryVisitor {
            type Value = Inventory;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Inventory")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Inventory, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut items: Option<Vec<u32>> = None;
                let mut capacity = None;
                let mut equipped: Option<Vec<(EquipSlot, u32)>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Items => {
                            if items.is_some() {
                                return Err(de::Error::duplicate_field("items"));
                            }
                            items = Some(map.next_value()?);
                        }
                        Field::Capacity => {
                            if capacity.is_some() {
                                return Err(de::Error::duplicate_field("capacity"));
                            }
                            capacity = Some(map.next_value()?);
                        }
                        Field::Equipped => {
                            if equipped.is_some() {
                                return Err(de::Error::duplicate_field("equipped"));
                            }
                            equipped = Some(map.next_value()?);
                        }
                    }
                }

                let items = items.ok_or_else(|| de::Error::missing_field("items"))?;
                let capacity = capacity.ok_or_else(|| de::Error::missing_field("capacity"))?;
                let equipped = equipped.ok_or_else(|| de::Error::missing_field("equipped"))?;

                // Convert u32 IDs back to Entity
                let item_entities: Vec<hecs::Entity> = items
                    .into_iter()
                    .map(|id| hecs::Entity::from_bits(id as u64).expect("Invalid entity ID"))
                    .collect();

                let equipped_map: std::collections::HashMap<EquipSlot, hecs::Entity> = equipped
                    .into_iter()
                    .map(|(slot, id)| (slot, hecs::Entity::from_bits(id as u64).expect("Invalid entity ID")))
                    .collect();

                Ok(Inventory {
                    items: item_entities,
                    capacity,
                    equipped: equipped_map,
                })
            }
        }

        const FIELDS: &[&str] = &["items", "capacity", "equipped"];
        deserializer.deserialize_struct("Inventory", FIELDS, InventoryVisitor)
    }
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
            equipped: std::collections::HashMap::new(),
        }
    }

    pub fn default_player() -> Self {
        Self::new(26) // a-z
    }

    pub fn add_item(&mut self, item: hecs::Entity) -> Result<(), &'static str> {
        if self.items.len() >= self.capacity {
            return Err("Inventory is full!");
        }
        self.items.push(item);
        Ok(())
    }

    pub fn remove_item(&mut self, item: hecs::Entity) -> bool {
        if let Some(pos) = self.items.iter().position(|&e| e == item) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn has_item(&self, item: hecs::Entity) -> bool {
        self.items.contains(&item)
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    pub fn equip(&mut self, slot: EquipSlot, item: hecs::Entity) -> Option<hecs::Entity> {
        self.equipped.insert(slot, item)
    }

    pub fn unequip(&mut self, slot: EquipSlot) -> Option<hecs::Entity> {
        self.equipped.remove(&slot)
    }

    pub fn get_equipped(&self, slot: EquipSlot) -> Option<hecs::Entity> {
        self.equipped.get(&slot).copied()
    }
}

// Marker for items that can be picked up
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Item;

// Marker for items on the ground (not in inventory)
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OnGround;

// Item categories for sorting and organization
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Consumable,
    Material,
    Tool,
    Misc,
}

impl ItemCategory {
    pub fn name(&self) -> &'static str {
        match self {
            ItemCategory::Weapon => "Weapon",
            ItemCategory::Armor => "Armor",
            ItemCategory::Consumable => "Consumable",
            ItemCategory::Material => "Material",
            ItemCategory::Tool => "Tool",
            ItemCategory::Misc => "Misc",
        }
    }
}

// Item data components with enhanced descriptions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub description: String,        // Short one-line description
    pub detailed_description: String, // Multi-line detailed lore/mechanics
    pub weight: i32,
    pub value: Money,
    pub category: ItemCategory,
    pub max_stack: usize,            // Max stack size (1 = non-stackable)
}

impl ItemData {
    pub fn new(name: impl Into<String>, description: impl Into<String>, weight: i32, value: i32) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            detailed_description: String::new(), // Legacy: empty detailed description
            weight,
            value: Money::new(value as u32),
            category: ItemCategory::Misc,
            max_stack: 1,
        }
    }

    pub fn with_category(mut self, category: ItemCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_max_stack(mut self, max_stack: usize) -> Self {
        self.max_stack = max_stack;
        self
    }

    pub fn with_detailed_description(mut self, detailed: impl Into<String>) -> Self {
        self.detailed_description = detailed.into();
        self
    }

    // Builder for new items with full details
    pub fn detailed(
        name: impl Into<String>,
        description: impl Into<String>,
        detailed_description: impl Into<String>,
        weight: i32,
        value: i32,
        category: ItemCategory,
        max_stack: usize,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            detailed_description: detailed_description.into(),
            weight,
            value: Money::new(value as u32),
            category,
            max_stack,
        }
    }
}

// Stackable component for items that can stack
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Stackable {
    pub quantity: usize,
}

impl Stackable {
    pub fn new(quantity: usize) -> Self {
        Self { quantity }
    }

    pub fn single() -> Self {
        Self { quantity: 1 }
    }
}

// Equipment data - items that can be equipped
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Equipable {
    pub slot: EquipSlot,
    pub power_bonus: Power,
    pub defense_bonus: Defense,
}

impl Equipable {
    pub fn new(slot: EquipSlot) -> Self {
        Self {
            slot,
            power_bonus: Power::new(0),
            defense_bonus: Defense::new(0),
        }
    }

    pub fn with_bonuses(slot: EquipSlot, power: i32, defense: i32) -> Self {
        Self {
            slot,
            power_bonus: Power::new(power),
            defense_bonus: Defense::new(defense),
        }
    }
}

// Consumable items - items that can be used
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Consumable {
    pub hp_restore: i32,
    pub uses: i32,
}

impl Consumable {
    pub fn new(hp_restore: i32, uses: i32) -> Self {
        Self { hp_restore, uses }
    }

    pub fn healing_potion() -> Self {
        Self::new(20, 1)
    }
}
