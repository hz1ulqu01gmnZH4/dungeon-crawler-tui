use crate::map::{Map, Tile};
use crate::world::{Settlement, SettlementType};
use rand::prelude::*;

/// Generate a local map for a settlement
pub fn generate_settlement_map(settlement: &Settlement, seed: u64) -> Map {
    let mut rng = StdRng::seed_from_u64(seed.wrapping_add(settlement.id as u64));

    match settlement.settlement_type {
        SettlementType::Village => generate_village(&mut rng),
        SettlementType::Town => generate_town(&mut rng),
        SettlementType::City => generate_city(&mut rng),
    }
}

/// Generate a small village (30x30)
fn generate_village(rng: &mut StdRng) -> Map {
    let width = 30;
    let height = 30;
    let mut map = Map::new(width, height);

    // Fill with grass
    for tile in map.tiles.iter_mut() {
        *tile = Tile::Floor;
    }

    // Add border walls (village fence/wall)
    for x in 0..width {
        let idx1 = map.xy_idx(x, 0);
        let idx2 = map.xy_idx(x, height - 1);
        map.tiles[idx1] = Tile::Wall;
        map.tiles[idx2] = Tile::Wall;
    }
    for y in 0..height {
        let idx1 = map.xy_idx(0, y);
        let idx2 = map.xy_idx(width - 1, y);
        map.tiles[idx1] = Tile::Wall;
        map.tiles[idx2] = Tile::Wall;
    }

    // Add entrance (gap in bottom wall)
    let entrance_x = width / 2;
    let idx1 = map.xy_idx(entrance_x, height - 1);
    let idx2 = map.xy_idx(entrance_x - 1, height - 1);
    let idx3 = map.xy_idx(entrance_x + 1, height - 1);
    map.tiles[idx1] = Tile::Floor;
    map.tiles[idx2] = Tile::Floor;
    map.tiles[idx3] = Tile::Floor;

    // Add 3-5 buildings
    let num_buildings = rng.gen_range(3..=5);
    for _ in 0..num_buildings {
        add_building(&mut map, rng, 4, 6);
    }

    // Add central well/feature
    let center_x = width / 2;
    let center_y = height / 2;
    if map.in_bounds(center_x, center_y) {
        let idx = map.xy_idx(center_x, center_y);
        map.tiles[idx] = Tile::Wall; // Well represented as wall for now
    }

    map
}

/// Generate a medium town (50x50)
fn generate_town(rng: &mut StdRng) -> Map {
    let width = 50;
    let height = 50;
    let mut map = Map::new(width, height);

    // Fill with floor
    for tile in map.tiles.iter_mut() {
        *tile = Tile::Floor;
    }

    // Add town walls
    for x in 0..width {
        let idx1 = map.xy_idx(x, 0);
        let idx2 = map.xy_idx(x, height - 1);
        map.tiles[idx1] = Tile::Wall;
        map.tiles[idx2] = Tile::Wall;
    }
    for y in 0..height {
        let idx1 = map.xy_idx(0, y);
        let idx2 = map.xy_idx(width - 1, y);
        map.tiles[idx1] = Tile::Wall;
        map.tiles[idx2] = Tile::Wall;
    }

    // Add main entrance (south)
    let entrance_x = width / 2;
    for dx in -2..=2 {
        let x = entrance_x + dx;
        if map.in_bounds(x, height - 1) {
            let idx = map.xy_idx(x, height - 1);
            map.tiles[idx] = Tile::Floor;
        }
    }

    // Add main street (vertical)
    for y in 1..height - 1 {
        for dx in -1..=1 {
            let x = entrance_x + dx;
            if map.in_bounds(x, y) {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    // Add cross street (horizontal at center)
    let center_y = height / 2;
    for x in 1..width - 1 {
        for dy in -1..=1 {
            let y = center_y + dy;
            if map.in_bounds(x, y) {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    // Add 8-12 buildings
    let num_buildings = rng.gen_range(8..=12);
    for _ in 0..num_buildings {
        add_building(&mut map, rng, 5, 8);
    }

    // Add town square (center)
    for dx in -3..=3 {
        for dy in -3..=3 {
            let x = entrance_x + dx;
            let y = center_y + dy;
            if map.in_bounds(x, y) {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    map
}

/// Generate a large city (80x80)
fn generate_city(rng: &mut StdRng) -> Map {
    let width = 80;
    let height = 80;
    let mut map = Map::new(width, height);

    // Fill with floor
    for tile in map.tiles.iter_mut() {
        *tile = Tile::Floor;
    }

    // Add city walls
    for x in 0..width {
        let idx1 = map.xy_idx(x, 0);
        let idx2 = map.xy_idx(x, height - 1);
        map.tiles[idx1] = Tile::Wall;
        map.tiles[idx2] = Tile::Wall;
    }
    for y in 0..height {
        let idx1 = map.xy_idx(0, y);
        let idx2 = map.xy_idx(width - 1, y);
        map.tiles[idx1] = Tile::Wall;
        map.tiles[idx2] = Tile::Wall;
    }

    // Add main entrance (south gate)
    let entrance_x = width / 2;
    for dx in -3..=3 {
        let x = entrance_x + dx;
        if map.in_bounds(x, height - 1) {
            let idx = map.xy_idx(x, height - 1);
            map.tiles[idx] = Tile::Floor;
        }
    }

    // Add grid of streets
    for street_x in (10..width).step_by(15) {
        for y in 1..height - 1 {
            for dx in 0..2 {
                let x = street_x + dx;
                if map.in_bounds(x, y) {
                    let idx = map.xy_idx(x, y);
                    map.tiles[idx] = Tile::Floor;
                }
            }
        }
    }

    for street_y in (10..height).step_by(15) {
        for x in 1..width - 1 {
            for dy in 0..2 {
                let y = street_y + dy;
                if map.in_bounds(x, y) {
                    let idx = map.xy_idx(x, y);
                    map.tiles[idx] = Tile::Floor;
                }
            }
        }
    }

    // Add 15-25 buildings
    let num_buildings = rng.gen_range(15..=25);
    for _ in 0..num_buildings {
        add_building(&mut map, rng, 6, 10);
    }

    // Add central plaza
    let center_x = width / 2;
    let center_y = height / 2;
    for dx in -5..=5 {
        for dy in -5..=5 {
            let x = center_x + dx;
            let y = center_y + dy;
            if map.in_bounds(x, y) {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
    }

    map
}

/// Add a building to the map
fn add_building(map: &mut Map, rng: &mut StdRng, min_size: i32, max_size: i32) {
    let max_attempts = 50;

    for _ in 0..max_attempts {
        let w = rng.gen_range(min_size..=max_size);
        let h = rng.gen_range(min_size..=max_size);
        let x = rng.gen_range(2..map.width - w - 2);
        let y = rng.gen_range(2..map.height - h - 2);

        // Check if area is clear (all floor tiles)
        let mut clear = true;
        for bx in x - 1..=x + w {
            for by in y - 1..=y + h {
                if !map.in_bounds(bx, by) {
                    clear = false;
                    break;
                }
                let idx = map.xy_idx(bx, by);
                if map.tiles[idx] != Tile::Floor {
                    clear = false;
                    break;
                }
            }
            if !clear {
                break;
            }
        }

        if !clear {
            continue;
        }

        // Place building walls
        for bx in x..x + w {
            let idx1 = map.xy_idx(bx, y);
            let idx2 = map.xy_idx(bx, y + h - 1);
            map.tiles[idx1] = Tile::Wall;
            map.tiles[idx2] = Tile::Wall;
        }
        for by in y..y + h {
            let idx1 = map.xy_idx(x, by);
            let idx2 = map.xy_idx(x + w - 1, by);
            map.tiles[idx1] = Tile::Wall;
            map.tiles[idx2] = Tile::Wall;
        }

        // Add door (random side)
        let door_side = rng.gen_range(0..4);
        let (door_x, door_y) = match door_side {
            0 => (x + w / 2, y),                    // North
            1 => (x + w - 1, y + h / 2),             // East
            2 => (x + w / 2, y + h - 1),             // South
            _ => (x, y + h / 2),                     // West
        };

        if map.in_bounds(door_x, door_y) {
            let idx = map.xy_idx(door_x, door_y);
            map.tiles[idx] = Tile::Floor;
        }

        // Fill interior with floor
        for bx in x + 1..x + w - 1 {
            for by in y + 1..y + h - 1 {
                if map.in_bounds(bx, by) {
                    let idx = map.xy_idx(bx, by);
                    map.tiles[idx] = Tile::Floor;
                }
            }
        }

        return; // Building placed successfully
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_village() {
        let settlement = Settlement::new(
            0,
            "Test Village".to_string(),
            SettlementType::Village,
            (10, 10),
            30,
            762,
        );

        let map = generate_settlement_map(&settlement, 12345);

        assert_eq!(map.width, 30);
        assert_eq!(map.height, 30);

        // Check entrance exists
        let entrance_x = map.width / 2;
        let entrance_idx = map.xy_idx(entrance_x, map.height - 1);
        assert_eq!(map.tiles[entrance_idx], Tile::Floor);
    }

    #[test]
    fn test_generate_town() {
        let settlement = Settlement::new(
            1,
            "Test Town".to_string(),
            SettlementType::Town,
            (20, 20),
            250,
            762,
        );

        let map = generate_settlement_map(&settlement, 54321);

        assert_eq!(map.width, 50);
        assert_eq!(map.height, 50);
    }

    #[test]
    fn test_generate_city() {
        let settlement = Settlement::new(
            2,
            "Test City".to_string(),
            SettlementType::City,
            (30, 30),
            3000,
            762,
        );

        let map = generate_settlement_map(&settlement, 99999);

        assert_eq!(map.width, 80);
        assert_eq!(map.height, 80);
    }

    #[test]
    fn test_deterministic_generation() {
        let settlement = Settlement::new(
            0,
            "Test".to_string(),
            SettlementType::Village,
            (10, 10),
            30,
            762,
        );

        let map1 = generate_settlement_map(&settlement, 12345);
        let map2 = generate_settlement_map(&settlement, 12345);

        // Same seed should produce same map
        assert_eq!(map1.tiles, map2.tiles);
    }
}
