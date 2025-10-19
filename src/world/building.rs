use crate::map::{Map, Tile};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    House,
    Inn,
    Shop,
    Temple,
    Blacksmith,
    Library,
    Warehouse,
}

impl BuildingType {
    pub fn name(&self) -> &str {
        match self {
            BuildingType::House => "House",
            BuildingType::Inn => "Inn",
            BuildingType::Shop => "Shop",
            BuildingType::Temple => "Temple",
            BuildingType::Blacksmith => "Blacksmith",
            BuildingType::Library => "Library",
            BuildingType::Warehouse => "Warehouse",
        }
    }

    pub fn all_types() -> Vec<BuildingType> {
        vec![
            BuildingType::House,
            BuildingType::Inn,
            BuildingType::Shop,
            BuildingType::Temple,
            BuildingType::Blacksmith,
            BuildingType::Library,
            BuildingType::Warehouse,
        ]
    }

    /// Get typical dimensions for this building type
    pub fn dimensions(&self) -> (i32, i32) {
        match self {
            BuildingType::House => (8, 8),
            BuildingType::Inn => (14, 12),
            BuildingType::Shop => (10, 8),
            BuildingType::Temple => (16, 14),
            BuildingType::Blacksmith => (10, 10),
            BuildingType::Library => (14, 14),
            BuildingType::Warehouse => (12, 16),
        }
    }
}

#[derive(Clone)]
pub struct Building {
    pub building_type: BuildingType,
    pub position: (i32, i32), // Position on settlement map
    pub entrance: (i32, i32), // Local entrance position within building
    pub interior: Map,
}

/// Generate a building interior
pub fn generate_building_interior(building_type: BuildingType, seed: u64) -> Map {
    let mut rng = StdRng::seed_from_u64(seed);
    let (width, height) = building_type.dimensions();

    let mut map = Map::new(width, height);

    // Fill with floors
    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = Tile::Floor;
        }
    }

    // Add walls around perimeter
    for x in 0..width {
        let top_idx = map.xy_idx(x, 0);
        let bottom_idx = map.xy_idx(x, height - 1);
        map.tiles[top_idx] = Tile::Wall;
        map.tiles[bottom_idx] = Tile::Wall;
    }
    for y in 0..height {
        let left_idx = map.xy_idx(0, y);
        let right_idx = map.xy_idx(width - 1, y);
        map.tiles[left_idx] = Tile::Wall;
        map.tiles[right_idx] = Tile::Wall;
    }

    // Add entrance at bottom center
    let entrance_x = width / 2;
    let entrance_idx = map.xy_idx(entrance_x, height - 1);
    map.tiles[entrance_idx] = Tile::Floor;

    // Generate interior based on building type
    match building_type {
        BuildingType::House => generate_house_interior(&mut map, &mut rng),
        BuildingType::Inn => generate_inn_interior(&mut map, &mut rng),
        BuildingType::Shop => generate_shop_interior(&mut map, &mut rng),
        BuildingType::Temple => generate_temple_interior(&mut map, &mut rng),
        BuildingType::Blacksmith => generate_blacksmith_interior(&mut map, &mut rng),
        BuildingType::Library => generate_library_interior(&mut map, &mut rng),
        BuildingType::Warehouse => generate_warehouse_interior(&mut map, &mut rng),
    }

    // Mark all tiles as visible and revealed for interiors
    for i in 0..map.tiles.len() {
        map.visible[i] = true;
        map.revealed[i] = true;
    }

    map
}

fn generate_house_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Divide into rooms with internal walls
    // Create a bedroom (top half) and living area (bottom half)
    let divider_y = height / 2;

    // Horizontal wall dividing rooms
    for x in 1..width - 1 {
        if x != width / 2 { // Leave doorway
            let idx = map.xy_idx(x, divider_y);
            map.tiles[idx] = Tile::Wall;
        }
    }

    // Add some furniture (using walls as placeholders)
    // Bed in bedroom (top left)
    if width > 3 && height > 3 {
        let bed_x = 2;
        let bed_y = 2;
        let idx = map.xy_idx(bed_x, bed_y);
        map.tiles[idx] = Tile::Wall; // Furniture placeholder
    }

    // Table in living area (avoid blocking entrance)
    if width > 4 && height > 6 {
        let table_x = width / 2;
        let table_y = divider_y + 2;
        // Don't place table at entrance or directly inside entrance
        if table_y < height - 2 {
            let idx = map.xy_idx(table_x, table_y);
            map.tiles[idx] = Tile::Wall; // Furniture placeholder
        }
    }

    // Add some random clutter
    for _ in 0..rng.gen_range(2..5) {
        let x = rng.gen_range(2..width - 2);
        let y = rng.gen_range(2..height - 2);
        let idx = map.xy_idx(x, y);
        if map.tiles[idx] == Tile::Floor {
            // Only place on empty floors
            map.tiles[idx] = Tile::Wall; // Clutter
        }
    }
}

fn generate_inn_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Common room takes up bottom 2/3
    let common_room_height = (height * 2) / 3;

    // Bar counter along one wall
    let bar_x = 2;
    for y in 2..5 {
        if y < height - 1 {
            let idx = map.xy_idx(bar_x, y);
            map.tiles[idx] = Tile::Wall; // Bar counter
        }
    }

    // Tables in common room
    for table_num in 0..3 {
        let table_x = 5 + table_num * 3;
        let table_y = common_room_height / 2;
        if table_x < width - 2 {
            let idx = map.xy_idx(table_x, table_y);
            map.tiles[idx] = Tile::Wall; // Table
        }
    }

    // Rooms upstairs (top 1/3)
    let room_divider_y = common_room_height;
    for x in 1..width - 1 {
        let idx = map.xy_idx(x, room_divider_y);
        map.tiles[idx] = Tile::Wall;
    }

    // Add doorways to upstairs rooms
    for door_x in &[width / 3, (width * 2) / 3] {
        if *door_x > 0 && *door_x < width {
            let idx = map.xy_idx(*door_x, room_divider_y);
            map.tiles[idx] = Tile::Floor;
        }
    }

    // Divide upstairs into 2-3 rooms (avoid blocking entrance)
    let entrance_x = width / 2;
    let vertical_divider = if entrance_x > 1 { entrance_x - 1 } else { entrance_x + 1 };
    for y in room_divider_y + 1..height - 1 {
        let idx = map.xy_idx(vertical_divider, y);
        map.tiles[idx] = Tile::Wall;
    }
}

fn generate_shop_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Counter dividing customer area from storage
    let counter_y = height / 2;
    for x in 1..width - 1 {
        if x < 3 || x > width - 4 { // Leave openings on sides
            let idx = map.xy_idx(x, counter_y);
            map.tiles[idx] = Tile::Wall; // Counter
        }
    }

    // Shelves along walls (back area)
    for x in 2..width - 2 {
        let idx = map.xy_idx(x, 2);
        map.tiles[idx] = Tile::Wall; // Shelf
    }

    // Display tables in customer area
    for i in 0..2 {
        let x = 3 + i * 3;
        let y = counter_y + 2;
        if x < width - 2 && y < height - 2 {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = Tile::Wall; // Display table
        }
    }
}

fn generate_temple_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Main altar at far end
    let altar_x = width / 2;
    let altar_y = 2;
    for dx in -1..=1 {
        for dy in 0..2 {
            let x = altar_x + dx;
            let y = altar_y + dy;
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Wall; // Altar
            }
        }
    }

    // Pews/benches in rows
    for row in 0..3 {
        let y = height / 2 + row * 2;
        if y < height - 3 {
            for x_offset in &[-3, 3] {
                let x = width / 2 + x_offset;
                if x > 1 && x < width - 2 {
                    for dx in 0..2 {
                        let idx = map.xy_idx(x + dx, y);
                        map.tiles[idx] = Tile::Wall; // Pew
                    }
                }
            }
        }
    }

    // Side chambers
    let chamber_y = height / 3;
    for side in &[2, width - 4] {
        for y in chamber_y..chamber_y + 3 {
            if y < height - 1 {
                let idx = map.xy_idx(*side, y);
                map.tiles[idx] = Tile::Wall; // Partition
            }
        }
    }
}

fn generate_blacksmith_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Forge/furnace in corner
    let forge_x = 2;
    let forge_y = 2;
    for dx in 0..2 {
        for dy in 0..2 {
            let idx = map.xy_idx(forge_x + dx, forge_y + dy);
            map.tiles[idx] = Tile::Wall; // Forge
        }
    }

    // Anvil near forge
    let anvil_idx = map.xy_idx(forge_x + 3, forge_y + 1);
    map.tiles[anvil_idx] = Tile::Wall;

    // Weapon racks along walls
    for x in 2..width - 2 {
        if x > 5 { // Skip area near forge
            let idx = map.xy_idx(x, height - 3);
            map.tiles[idx] = Tile::Wall; // Weapon rack
        }
    }

    // Work table
    let table_x = width / 2;
    let table_y = height / 2;
    let idx = map.xy_idx(table_x, table_y);
    map.tiles[idx] = Tile::Wall;

    // Storage boxes
    for i in 0..2 {
        let x = width - 3;
        let y = 2 + i * 2;
        let idx = map.xy_idx(x, y);
        map.tiles[idx] = Tile::Wall; // Storage
    }
}

fn generate_library_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Bookshelves along walls
    for x in 2..width - 2 {
        for y in &[2, height - 3] {
            let idx = map.xy_idx(x, *y);
            map.tiles[idx] = Tile::Wall; // Bookshelf
        }
    }

    // Reading tables in center area
    for table_num in 0..2 {
        let x = width / 3 + table_num * (width / 3);
        let y = height / 2;
        if x > 2 && x < width - 3 {
            for dx in 0..2 {
                let idx = map.xy_idx(x + dx, y);
                map.tiles[idx] = Tile::Wall; // Table
            }
        }
    }

    // Additional bookshelf rows
    let shelf_x = width / 4;
    for y in 4..height - 4 {
        if y % 3 == 0 { // Leave gaps for aisles
            let idx = map.xy_idx(shelf_x, y);
            map.tiles[idx] = Tile::Wall; // Bookshelf

            let idx2 = map.xy_idx(width - shelf_x, y);
            map.tiles[idx2] = Tile::Wall; // Bookshelf
        }
    }
}

fn generate_warehouse_interior(map: &mut Map, rng: &mut StdRng) {
    let width = map.width;
    let height = map.height;

    // Rows of crates/boxes
    for row in 0..3 {
        let y = 3 + row * 4;
        if y < height - 3 {
            for col in 0..2 {
                let x = 3 + col * 5;
                if x < width - 3 {
                    // 2x2 crate stack
                    for dx in 0..2 {
                        for dy in 0..2 {
                            let idx = map.xy_idx(x + dx, y + dy);
                            map.tiles[idx] = Tile::Wall; // Crate
                        }
                    }
                }
            }
        }
    }

    // Loading area near entrance (keep clear)
    // Already clear by default

    // Storage along side walls
    for y in 2..height - 2 {
        if y % 3 != 0 { // Leave gaps
            for side_x in &[1, width - 2] {
                let idx = map.xy_idx(*side_x, y);
                if map.tiles[idx] == Tile::Floor {
                    map.tiles[idx] = Tile::Wall; // Storage
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_dimensions() {
        let house = BuildingType::House;
        let (w, h) = house.dimensions();
        assert!(w > 0 && h > 0);
        assert_eq!((w, h), (8, 8));
    }

    #[test]
    fn test_building_types() {
        let types = BuildingType::all_types();
        assert_eq!(types.len(), 7);
        assert!(types.contains(&BuildingType::House));
        assert!(types.contains(&BuildingType::Temple));
    }

    #[test]
    fn test_generate_building_interior() {
        for building_type in BuildingType::all_types() {
            let map = generate_building_interior(building_type, 12345);
            let (width, height) = building_type.dimensions();

            assert_eq!(map.width, width);
            assert_eq!(map.height, height);

            // Check perimeter is walls (except entrance)
            let mut has_entrance = false;
            for x in 0..width {
                let bottom_idx = map.xy_idx(x, height - 1);
                if map.tiles[bottom_idx] == Tile::Floor {
                    has_entrance = true;
                }
            }
            assert!(has_entrance, "Building should have an entrance");

            // Check corners are walls
            assert_eq!(map.tiles[map.xy_idx(0, 0)], Tile::Wall);
            assert_eq!(map.tiles[map.xy_idx(width - 1, 0)], Tile::Wall);
        }
    }

    #[test]
    fn test_house_interior_has_room_divider() {
        let map = generate_building_interior(BuildingType::House, 12345);
        let width = map.width;
        let height = map.height;
        let divider_y = height / 2;

        // Check that there's a horizontal wall dividing the house
        let mut has_divider_wall = false;
        for x in 1..width - 1 {
            let idx = map.xy_idx(x, divider_y);
            if map.tiles[idx] == Tile::Wall {
                has_divider_wall = true;
                break;
            }
        }
        assert!(has_divider_wall, "House should have room divider");

        // Check there's a doorway in the divider
        let mut has_doorway = false;
        for x in 1..width - 1 {
            let idx = map.xy_idx(x, divider_y);
            if map.tiles[idx] == Tile::Floor {
                has_doorway = true;
                break;
            }
        }
        assert!(has_doorway, "House divider should have doorway");
    }

    #[test]
    fn test_inn_interior_has_bar_and_tables() {
        let map = generate_building_interior(BuildingType::Inn, 12345);
        let width = map.width;
        let height = map.height;

        // Count furniture (walls that aren't perimeter)
        let mut furniture_count = 0;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == Tile::Wall {
                    furniture_count += 1;
                }
            }
        }

        // Inn should have bar counter, tables, and room dividers
        assert!(furniture_count > 10, "Inn should have substantial furniture/fixtures (bar, tables, dividers)");

        // Check for horizontal divider (between common room and upstairs)
        let common_room_height = (height * 2) / 3;
        let mut has_upstairs_divider = false;
        for x in 1..width - 1 {
            let idx = map.xy_idx(x, common_room_height);
            if map.tiles[idx] == Tile::Wall {
                has_upstairs_divider = true;
                break;
            }
        }
        assert!(has_upstairs_divider, "Inn should have divider between common room and upstairs");
    }

    #[test]
    fn test_shop_interior_has_counter() {
        let map = generate_building_interior(BuildingType::Shop, 12345);
        let width = map.width;
        let height = map.height;
        let counter_y = height / 2;

        // Check that there's a counter dividing the shop
        let mut has_counter = false;
        for x in 1..width - 1 {
            let idx = map.xy_idx(x, counter_y);
            if map.tiles[idx] == Tile::Wall {
                has_counter = true;
                break;
            }
        }
        assert!(has_counter, "Shop should have counter");

        // Check there are openings in the counter (side passages)
        let mut has_opening = false;
        for x in 3..width - 4 {
            let idx = map.xy_idx(x, counter_y);
            if map.tiles[idx] == Tile::Floor {
                has_opening = true;
                break;
            }
        }
        assert!(has_opening, "Shop counter should have openings for passage");
    }

    #[test]
    fn test_temple_interior_has_altar() {
        let map = generate_building_interior(BuildingType::Temple, 12345);
        let width = map.width;
        let altar_x = width / 2;
        let altar_y = 2;

        // Check altar area has walls (furniture)
        let mut altar_tiles = 0;
        for dx in -1..=1 {
            for dy in 0..2 {
                let x = altar_x + dx;
                let y = altar_y + dy;
                if x > 0 && x < width - 1 && y > 0 {
                    let idx = map.xy_idx(x, y);
                    if map.tiles[idx] == Tile::Wall {
                        altar_tiles += 1;
                    }
                }
            }
        }
        assert!(altar_tiles >= 3, "Temple should have altar structure at far end");
    }

    #[test]
    fn test_blacksmith_interior_has_forge() {
        let map = generate_building_interior(BuildingType::Blacksmith, 12345);
        let forge_x = 2;
        let forge_y = 2;

        // Check forge area (2x2 in corner)
        let mut forge_tiles = 0;
        for dx in 0..2 {
            for dy in 0..2 {
                let idx = map.xy_idx(forge_x + dx, forge_y + dy);
                if map.tiles[idx] == Tile::Wall {
                    forge_tiles += 1;
                }
            }
        }
        assert_eq!(forge_tiles, 4, "Blacksmith should have 2x2 forge in corner");

        // Check anvil position
        let anvil_idx = map.xy_idx(forge_x + 3, forge_y + 1);
        assert_eq!(map.tiles[anvil_idx], Tile::Wall, "Blacksmith should have anvil near forge");
    }

    #[test]
    fn test_library_interior_has_bookshelves() {
        let map = generate_building_interior(BuildingType::Library, 12345);
        let width = map.width;
        let height = map.height;

        // Count bookshelves along top and bottom walls
        let mut bookshelf_count = 0;
        for x in 2..width - 2 {
            let top_idx = map.xy_idx(x, 2);
            let bottom_idx = map.xy_idx(x, height - 3);
            if map.tiles[top_idx] == Tile::Wall {
                bookshelf_count += 1;
            }
            if map.tiles[bottom_idx] == Tile::Wall {
                bookshelf_count += 1;
            }
        }

        assert!(bookshelf_count >= 10, "Library should have many bookshelves");

        // Count reading tables (should have some furniture in center)
        let mut center_furniture = 0;
        for y in height / 2 - 1..=height / 2 + 1 {
            for x in width / 4..width - width / 4 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == Tile::Wall {
                    center_furniture += 1;
                }
            }
        }
        assert!(center_furniture > 0, "Library should have reading tables in center");
    }

    #[test]
    fn test_warehouse_interior_has_crates() {
        let map = generate_building_interior(BuildingType::Warehouse, 12345);
        let width = map.width;
        let height = map.height;

        // Count crates/storage (walls that aren't perimeter)
        let mut storage_count = 0;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == Tile::Wall {
                    storage_count += 1;
                }
            }
        }

        // Warehouse should have many crates and storage
        assert!(storage_count > 15, "Warehouse should have many crates and storage boxes");

        // Check that there are some clear paths (not completely filled)
        let mut clear_tiles = 0;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == Tile::Floor {
                    clear_tiles += 1;
                }
            }
        }
        assert!(clear_tiles > 10, "Warehouse should have clear paths between storage");
    }

    #[test]
    fn test_deterministic_building_generation() {
        // Same seed should produce identical interiors
        let seed = 98765;

        for building_type in BuildingType::all_types() {
            let map1 = generate_building_interior(building_type, seed);
            let map2 = generate_building_interior(building_type, seed);

            assert_eq!(map1.width, map2.width);
            assert_eq!(map1.height, map2.height);
            assert_eq!(map1.tiles.len(), map2.tiles.len());

            // Check that all tiles match
            for i in 0..map1.tiles.len() {
                assert_eq!(
                    map1.tiles[i], map2.tiles[i],
                    "Tiles at index {} should match for {} with seed {}",
                    i, building_type.name(), seed
                );
            }
        }
    }

    #[test]
    fn test_different_seeds_produce_different_layouts() {
        // Different seeds should produce different layouts (at least for randomized buildings)
        let seed1 = 11111;
        let seed2 = 99999;

        // House has random clutter, so it should differ
        let house1 = generate_building_interior(BuildingType::House, seed1);
        let house2 = generate_building_interior(BuildingType::House, seed2);

        // Count differences in tiles
        let mut differences = 0;
        for i in 0..house1.tiles.len() {
            if house1.tiles[i] != house2.tiles[i] {
                differences += 1;
            }
        }

        // Should have at least some differences due to random clutter
        assert!(differences > 0, "Different seeds should produce different house layouts");
    }

    #[test]
    fn test_entrance_is_accessible() {
        // Verify entrance is not blocked by furniture
        for building_type in BuildingType::all_types() {
            let map = generate_building_interior(building_type, 12345);
            let (width, height) = building_type.dimensions();
            let entrance_x = width / 2;

            // Check entrance tile is floor
            let entrance_idx = map.xy_idx(entrance_x, height - 1);
            assert_eq!(
                map.tiles[entrance_idx],
                Tile::Floor,
                "{} entrance should be floor tile",
                building_type.name()
            );

            // Check tile immediately inside entrance is accessible
            let inside_idx = map.xy_idx(entrance_x, height - 2);
            assert_eq!(
                map.tiles[inside_idx],
                Tile::Floor,
                "{} entrance should lead to accessible floor tile",
                building_type.name()
            );
        }
    }

    #[test]
    fn test_all_interiors_fully_visible() {
        // Building interiors should have all tiles marked as visible and revealed
        for building_type in BuildingType::all_types() {
            let map = generate_building_interior(building_type, 12345);

            for i in 0..map.tiles.len() {
                assert!(
                    map.visible[i],
                    "{} interior tile {} should be visible",
                    building_type.name(),
                    i
                );
                assert!(
                    map.revealed[i],
                    "{} interior tile {} should be revealed",
                    building_type.name(),
                    i
                );
            }
        }
    }

    #[test]
    fn test_building_type_names() {
        assert_eq!(BuildingType::House.name(), "House");
        assert_eq!(BuildingType::Inn.name(), "Inn");
        assert_eq!(BuildingType::Shop.name(), "Shop");
        assert_eq!(BuildingType::Temple.name(), "Temple");
        assert_eq!(BuildingType::Blacksmith.name(), "Blacksmith");
        assert_eq!(BuildingType::Library.name(), "Library");
        assert_eq!(BuildingType::Warehouse.name(), "Warehouse");
    }

    #[test]
    fn test_all_building_dimensions_positive() {
        for building_type in BuildingType::all_types() {
            let (w, h) = building_type.dimensions();
            assert!(w > 5, "{} width should be at least 6", building_type.name());
            assert!(h > 5, "{} height should be at least 6", building_type.name());
            assert!(w <= 20, "{} width should be reasonable", building_type.name());
            assert!(h <= 20, "{} height should be reasonable", building_type.name());
        }
    }
}
