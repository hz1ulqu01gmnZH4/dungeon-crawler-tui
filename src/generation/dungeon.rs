use rand::Rng;

pub struct Dungeon {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Empty,
}

impl Dungeon {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Empty; width]; height];
        Self { tiles, width, height }
    }

    pub fn generate_random(&mut self) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    self.tiles[y][x] = Tile::Wall;
                } else if rng.gen_bool(0.8) {
                    self.tiles[y][x] = Tile::Floor;
                } else {
                    self.tiles[y][x] = Tile::Wall;
                }
            }
        }
    }
}