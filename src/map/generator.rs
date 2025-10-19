use crate::map::{Map, Tile};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

pub fn generate_dungeon(map: &mut Map, rng: &mut impl Rng, max_rooms: i32, min_size: i32, max_size: i32) -> Vec<Rect> {
    let mut rooms = Vec::new();

    for _ in 0..max_rooms {
        let w = rng.gen_range(min_size..=max_size);
        let h = rng.gen_range(min_size..=max_size);
        let x = rng.gen_range(1..map.width - w - 1);
        let y = rng.gen_range(1..map.height - h - 1);

        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersects(other_room) {
                ok = false;
                break;
            }
        }

        if ok {
            apply_room(map, &new_room);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rng.gen_bool(0.5) {
                    apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    rooms
}

fn apply_room(map: &mut Map, room: &Rect) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            if map.in_bounds(x, y) {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in x1.min(x2)..=x1.max(x2) {
        if map.in_bounds(x, y) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = Tile::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in y1.min(y2)..=y1.max(y2) {
        if map.in_bounds(x, y) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = Tile::Floor;
        }
    }
}

/// Generate a dungeon level with stairs up and down
pub fn generate_dungeon_level(width: i32, height: i32, rng: &mut impl Rng) -> Map {
    let mut map = Map::new(width, height);

    // Generate dungeon rooms and corridors
    let rooms = generate_dungeon(&mut map, rng, 30, 6, 10);

    if rooms.is_empty() {
        return map; // No rooms generated, return empty map
    }

    // Place stairs up in first room (where player enters)
    let (stairs_up_x, stairs_up_y) = rooms[0].center();
    let stairs_up_idx = map.xy_idx(stairs_up_x, stairs_up_y);
    map.tiles[stairs_up_idx] = Tile::StairsUp;

    // Place stairs down in last room (deeper descent)
    if rooms.len() > 1 {
        let (stairs_down_x, stairs_down_y) = rooms[rooms.len() - 1].center();
        let stairs_down_idx = map.xy_idx(stairs_down_x, stairs_down_y);
        map.tiles[stairs_down_idx] = Tile::StairsDown;
    }

    map
}
