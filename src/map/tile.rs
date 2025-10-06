use ratatui::style::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Door,
}

impl Tile {
    pub fn walkable(&self) -> bool {
        matches!(self, Tile::Floor | Tile::Door)
    }

    pub fn blocks_sight(&self) -> bool {
        matches!(self, Tile::Wall)
    }

    pub fn glyph(&self, visible: bool) -> char {
        if visible {
            match self {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::Door => '+',
            }
        } else {
            match self {
                Tile::Floor => '.',
                Tile::Wall => '#',
                Tile::Door => '+',
            }
        }
    }

    pub fn color(&self, visible: bool) -> Color {
        if visible {
            match self {
                Tile::Floor => Color::DarkGray,
                Tile::Wall => Color::Gray,
                Tile::Door => Color::Yellow,
            }
        } else {
            Color::Rgb(50, 50, 50)
        }
    }
}
