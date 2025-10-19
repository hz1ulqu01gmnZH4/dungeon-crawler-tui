use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Floor,
    Wall,
    ClosedDoor,
    OpenDoor,
    StairsUp,
    StairsDown,
}

impl Tile {
    pub fn walkable(&self) -> bool {
        matches!(self, Tile::Floor | Tile::OpenDoor | Tile::StairsUp | Tile::StairsDown)
    }

    pub fn blocks_sight(&self) -> bool {
        matches!(self, Tile::Wall | Tile::ClosedDoor)
    }

    pub fn glyph(&self, visible: bool) -> char {
        if visible {
            match self {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::ClosedDoor => '+',
                Tile::OpenDoor => '/',
                Tile::StairsUp => '<',
                Tile::StairsDown => '>',
            }
        } else {
            match self {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::ClosedDoor => '+',
                Tile::OpenDoor => '/',
                Tile::StairsUp => '<',
                Tile::StairsDown => '>',
            }
        }
    }

    pub fn color(&self, visible: bool) -> Color {
        if visible {
            match self {
                Tile::Floor => Color::DarkGray,
                Tile::Wall => Color::Gray,
                Tile::ClosedDoor => Color::Yellow,
                Tile::OpenDoor => Color::Rgb(180, 140, 50), // Darker yellow/brown
                Tile::StairsUp => Color::Cyan,
                Tile::StairsDown => Color::Magenta,
            }
        } else {
            Color::Rgb(50, 50, 50)
        }
    }

    pub fn is_door(&self) -> bool {
        matches!(self, Tile::ClosedDoor | Tile::OpenDoor)
    }

    pub fn is_stairs(&self) -> bool {
        matches!(self, Tile::StairsUp | Tile::StairsDown)
    }
}
