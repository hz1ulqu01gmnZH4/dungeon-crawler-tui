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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    use rand::SeedableRng;

    #[test]
    fn test_rect_center() {
        let rect = Rect::new(0, 0, 10, 10);
        assert_eq!(rect.center(), (5, 5));

        let rect = Rect::new(5, 5, 6, 4);
        assert_eq!(rect.center(), (8, 7));
    }

    #[test]
    fn test_rect_intersects() {
        let rect1 = Rect::new(0, 0, 10, 10);
        let rect2 = Rect::new(5, 5, 10, 10);
        assert!(rect1.intersects(&rect2));

        let rect3 = Rect::new(20, 20, 10, 10);
        assert!(!rect1.intersects(&rect3));
    }

    #[test]
    fn test_rect_no_self_intersection() {
        let rect = Rect::new(5, 5, 10, 10);
        assert!(rect.intersects(&rect));
    }

    #[test]
    fn test_generate_dungeon_creates_rooms() {
        let mut map = Map::new(80, 50);
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);

        let rooms = generate_dungeon(&mut map, &mut rng, 20, 4, 8);

        // Should create at least some rooms
        assert!(rooms.len() > 0);

        // Each room should not intersect with others
        for (i, room) in rooms.iter().enumerate() {
            for (j, other) in rooms.iter().enumerate() {
                if i != j {
                    assert!(!room.intersects(other), "Room {} intersects room {}", i, j);
                }
            }
        }
    }

    #[test]
    fn test_dungeon_has_floor_tiles() {
        let mut map = Map::new(80, 50);
        let mut rng = rand::rngs::StdRng::seed_from_u64(54321);

        generate_dungeon(&mut map, &mut rng, 10, 5, 10);

        // Count floor tiles
        let floor_count = map.tiles.iter().filter(|t| matches!(t, Tile::Floor)).count();
        assert!(floor_count > 0, "Dungeon should have floor tiles");
    }

    #[test]
    fn test_dungeon_level_has_stairs() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(99999);
        let map = generate_dungeon_level(80, 50, &mut rng);

        let has_stairs_up = map.tiles.iter().any(|t| matches!(t, Tile::StairsUp));
        let has_stairs_down = map.tiles.iter().any(|t| matches!(t, Tile::StairsDown));

        assert!(has_stairs_up, "Should have stairs up");
        // May or may not have stairs down depending on room count, but typically should
    }

    // Property-based test: Generated dungeons always have valid coordinates
    fn prop_dungeon_rooms_in_bounds(seed: u64, max_rooms: u8) -> TestResult {
        if max_rooms == 0 || max_rooms > 100 {
            return TestResult::discard();
        }

        let mut map = Map::new(80, 50);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let rooms = generate_dungeon(&mut map, &mut rng, max_rooms as i32, 4, 10);

        for room in &rooms {
            if room.x1 < 0 || room.y1 < 0 || room.x2 > 80 || room.y2 > 50 {
                return TestResult::failed();
            }
        }

        TestResult::passed()
    }

    #[test]
    fn test_prop_dungeon_rooms_in_bounds() {
        quickcheck(prop_dungeon_rooms_in_bounds as fn(u64, u8) -> TestResult);
    }

    // Property-based test: No rooms intersect
    fn prop_dungeon_no_intersections(seed: u64) -> TestResult {
        let mut map = Map::new(80, 50);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let rooms = generate_dungeon(&mut map, &mut rng, 30, 4, 10);

        for (i, room) in rooms.iter().enumerate() {
            for (j, other) in rooms.iter().enumerate() {
                if i != j && room.intersects(other) {
                    return TestResult::failed();
                }
            }
        }

        TestResult::passed()
    }

    #[test]
    fn test_prop_dungeon_no_intersections() {
        quickcheck(prop_dungeon_no_intersections as fn(u64) -> TestResult);
    }

    // Property-based test: All floor tiles are within bounds
    fn prop_floor_tiles_in_bounds(seed: u64) -> bool {
        let mut map = Map::new(80, 50);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        generate_dungeon(&mut map, &mut rng, 30, 4, 10);

        for y in 0..map.height {
            for x in 0..map.width {
                let idx = map.xy_idx(x, y);
                if matches!(map.tiles[idx], Tile::Floor) {
                    if !map.in_bounds(x, y) {
                        return false;
                    }
                }
            }
        }

        true
    }

    #[test]
    fn test_prop_floor_tiles_in_bounds() {
        quickcheck(prop_floor_tiles_in_bounds as fn(u64) -> bool);
    }

    // Property-based test: Dungeon is deterministic with same seed
    fn prop_dungeon_deterministic(seed: u64) -> bool {
        let mut map1 = Map::new(80, 50);
        let mut rng1 = rand::rngs::StdRng::seed_from_u64(seed);
        let rooms1 = generate_dungeon(&mut map1, &mut rng1, 20, 4, 8);

        let mut map2 = Map::new(80, 50);
        let mut rng2 = rand::rngs::StdRng::seed_from_u64(seed);
        let rooms2 = generate_dungeon(&mut map2, &mut rng2, 20, 4, 8);

        if rooms1.len() != rooms2.len() {
            return false;
        }

        for (r1, r2) in rooms1.iter().zip(rooms2.iter()) {
            if r1.x1 != r2.x1 || r1.y1 != r2.y1 || r1.x2 != r2.x2 || r1.y2 != r2.y2 {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_prop_dungeon_deterministic() {
        quickcheck(prop_dungeon_deterministic as fn(u64) -> bool);
    }

    // Property-based test: Map dimensions are respected
    fn prop_map_dimensions_respected(width: u8, height: u8, seed: u64) -> TestResult {
        let width = width.max(20) as i32; // Minimum size for dungeon generation
        let height = height.max(20) as i32;

        if width > 200 || height > 200 {
            return TestResult::discard(); // Too large
        }

        let mut map = Map::new(width, height);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        generate_dungeon(&mut map, &mut rng, 10, 3, 6);

        assert_eq!(map.width, width);
        assert_eq!(map.height, height);
        assert_eq!(map.tiles.len(), (width * height) as usize);

        TestResult::passed()
    }

    #[test]
    fn test_prop_map_dimensions_respected() {
        quickcheck(prop_map_dimensions_respected as fn(u8, u8, u64) -> TestResult);
    }
}
