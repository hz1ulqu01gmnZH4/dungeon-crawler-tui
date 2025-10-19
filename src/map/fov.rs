use crate::map::Map;

pub fn compute_fov(map: &Map, x: i32, y: i32, range: i32) -> Vec<(i32, i32)> {
    let mut visible = Vec::new();

    // Add center point
    visible.push((x, y));

    // Cast rays in a circle
    for angle in 0..360 {
        let rad = (angle as f32).to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        let mut blocked = false;
        for distance in 1..=range {
            if blocked {
                break;
            }

            let target_x = x + (cos * distance as f32).round() as i32;
            let target_y = y + (sin * distance as f32).round() as i32;

            if !map.in_bounds(target_x, target_y) {
                break;
            }

            visible.push((target_x, target_y));

            if map.is_opaque(target_x, target_y) {
                blocked = true;
            }
        }
    }

    // Remove duplicates
    visible.sort_unstable();
    visible.dedup();
    visible
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Tile;

    fn create_empty_room(width: i32, height: i32) -> Map {
        let mut map = Map::new(width, height);
        // Make interior walkable (floor)
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Floor;
            }
        }
        // Walls remain around the border
        map
    }

    #[test]
    fn test_fov_empty_room_sees_all() {
        let map = create_empty_room(10, 10);
        let origin_x = 5;
        let origin_y = 5;
        let range = 8;

        let visible = compute_fov(&map, origin_x, origin_y, range);

        // Should see most of the room (at least 50 tiles in a 10x10 room)
        assert!(visible.len() > 50, "Expected to see most of empty room, saw {} tiles", visible.len());

        // Origin should always be visible
        assert!(visible.contains(&(origin_x, origin_y)), "Origin should be visible");

        // Should see adjacent tiles
        assert!(visible.contains(&(origin_x + 1, origin_y)), "Should see tile to the right");
        assert!(visible.contains(&(origin_x - 1, origin_y)), "Should see tile to the left");
        assert!(visible.contains(&(origin_x, origin_y + 1)), "Should see tile below");
        assert!(visible.contains(&(origin_x, origin_y - 1)), "Should see tile above");
    }

    #[test]
    fn test_fov_blocked_by_wall() {
        let mut map = create_empty_room(10, 10);

        // Place a wall to block vision
        let wall_x = 5;
        let wall_y = 3;
        let idx = map.xy_idx(wall_x, wall_y);
        map.tiles[idx] = Tile::Wall;

        let origin_x = 5;
        let origin_y = 5;
        let range = 8;

        let visible = compute_fov(&map, origin_x, origin_y, range);

        // Wall should be visible (you see the wall)
        assert!(visible.contains(&(wall_x, wall_y)), "Wall itself should be visible");

        // Tiles behind the wall should NOT be visible
        assert!(!visible.contains(&(wall_x, wall_y - 1)), "Should NOT see tile behind wall (y-1)");
        assert!(!visible.contains(&(wall_x, wall_y - 2)), "Should NOT see tile behind wall (y-2)");
    }

    #[test]
    fn test_fov_range_limit() {
        let map = create_empty_room(20, 20);
        let origin_x = 10;
        let origin_y = 10;
        let range = 5;

        let visible = compute_fov(&map, origin_x, origin_y, range);

        // Should not see tiles beyond range
        // Far corners should definitely be out of range
        assert!(!visible.contains(&(1, 1)), "Should NOT see far corner (1,1) with range 5 from (10,10)");
        assert!(!visible.contains(&(18, 18)), "Should NOT see far corner (18,18) with range 5 from (10,10)");

        // Should see tiles within range
        assert!(visible.contains(&(origin_x + 3, origin_y)), "Should see tile 3 steps away");
        assert!(visible.contains(&(origin_x, origin_y + 4)), "Should see tile 4 steps away");
    }

    #[test]
    fn test_fov_origin_always_visible() {
        let map = create_empty_room(10, 10);

        // Test various ranges including very small
        for range in [1, 3, 5, 8, 10] {
            let visible = compute_fov(&map, 5, 5, range);
            assert!(visible.contains(&(5, 5)), "Origin should always be visible (range {})", range);
        }
    }

    #[test]
    fn test_fov_doors_block_when_closed() {
        let mut map = create_empty_room(10, 10);

        // Place a closed door
        let door_x = 5;
        let door_y = 3;
        let idx = map.xy_idx(door_x, door_y);
        map.tiles[idx] = Tile::ClosedDoor;

        let visible = compute_fov(&map, 5, 5, 8);

        // Closed door should be visible but block vision beyond it
        assert!(visible.contains(&(door_x, door_y)), "Closed door should be visible");
        assert!(!visible.contains(&(door_x, door_y - 1)), "Should NOT see through closed door");
    }

    #[test]
    fn test_fov_open_doors_do_not_block() {
        let mut map = create_empty_room(10, 10);

        // Place an open door
        let door_x = 5;
        let door_y = 3;
        let idx = map.xy_idx(door_x, door_y);
        map.tiles[idx] = Tile::OpenDoor;

        let visible = compute_fov(&map, 5, 5, 8);

        // Open door should be visible and NOT block vision
        assert!(visible.contains(&(door_x, door_y)), "Open door should be visible");
        assert!(visible.contains(&(door_x, door_y - 1)), "Should see through open door");
    }

    #[test]
    fn test_fov_out_of_bounds() {
        let map = create_empty_room(10, 10);

        // FOV from corner position
        let visible = compute_fov(&map, 1, 1, 5);

        // Should handle out-of-bounds gracefully (not panic)
        // Should see origin and some tiles
        assert!(visible.contains(&(1, 1)), "Should see origin");
        assert!(visible.len() > 5, "Should see multiple tiles from corner");

        // All visible tiles should be in bounds
        for (x, y) in &visible {
            assert!(map.in_bounds(*x, *y), "All visible tiles should be in bounds: ({}, {})", x, y);
        }
    }
}
