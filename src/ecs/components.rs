use ratatui::style::Color;
use serde::{Deserialize, Serialize};

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
    pub hp: i32,
    pub max_hp: i32,
    pub power: i32,
    pub defense: i32,
}

impl CombatStats {
    pub fn new(hp: i32, power: i32, defense: i32) -> Self {
        Self {
            hp,
            max_hp: hp,
            power,
            defense,
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

// Intent components
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WantsToMove {
    pub dest_x: i32,
    pub dest_y: i32,
}

impl WantsToMove {
    pub fn new(dest_x: i32, dest_y: i32) -> Self {
        Self { dest_x, dest_y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WantsToMelee {
    pub target: hecs::Entity,
}

impl WantsToMelee {
    pub fn new(target: hecs::Entity) -> Self {
        Self { target }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WantsToWait;
