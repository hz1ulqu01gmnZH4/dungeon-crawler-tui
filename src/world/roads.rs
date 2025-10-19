use crate::world::{Overmap, TerrainType, Settlement, SettlementType};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadType {
    KingRoad,    // Major roads between cities
    TradeRoute,  // Roads between towns
    Path,        // Small paths to villages
}

impl RoadType {
    pub fn name(&self) -> &str {
        match self {
            RoadType::KingRoad => "King Road",
            RoadType::TradeRoute => "Trade Route",
            RoadType::Path => "Path",
        }
    }

    pub fn glyph(&self) -> char {
        match self {
            RoadType::KingRoad => '═',
            RoadType::TradeRoute => '─',
            RoadType::Path => '·',
        }
    }

    pub fn travel_speed_multiplier(&self) -> f32 {
        match self {
            RoadType::KingRoad => 1.5,    // 50% faster
            RoadType::TradeRoute => 1.3,  // 30% faster
            RoadType::Path => 1.1,        // 10% faster
        }
    }
}

#[derive(Debug, Clone)]
pub struct Road {
    pub road_type: RoadType,
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub tiles: Vec<(i32, i32)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathNode {
    position: (i32, i32),
    cost: i32,
    heuristic: i32,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Calculate terrain cost for road building
fn terrain_cost(terrain: TerrainType) -> i32 {
    match terrain {
        TerrainType::Plains => 10,
        TerrainType::Forest => 20,
        TerrainType::DenseForest => 40,
        TerrainType::Hills => 30,
        TerrainType::Mountains => 100, // Very expensive, avoid if possible
        TerrainType::Swamp => 50,
        TerrainType::Lake => 200, // Can't build roads over water
        TerrainType::River => 150, // Expensive to bridge
        TerrainType::Road => 5, // Already a road, very cheap
        TerrainType::KingRoad => 3, // Existing major road
        TerrainType::TradePath => 4, // Existing minor road
        TerrainType::Settlement => 10, // Can route through settlements
        TerrainType::Dungeon => 50, // Avoid dungeons
        TerrainType::SpecialLocation => 50, // Avoid special locations
    }
}

/// A* pathfinding for road generation
fn find_path(
    overmap: &Overmap,
    start: (i32, i32),
    end: (i32, i32),
    existing_roads: &HashSet<(i32, i32)>,
) -> Option<Vec<(i32, i32)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    let heuristic = |pos: (i32, i32)| -> i32 {
        ((pos.0 - end.0).abs() + (pos.1 - end.1).abs()) * 10
    };

    g_score.insert(start, 0);
    open_set.push(PathNode {
        position: start,
        cost: 0,
        heuristic: heuristic(start),
    });

    while let Some(PathNode { position: current, .. }) = open_set.pop() {
        if current == end {
            // Reconstruct path
            let mut path = vec![current];
            let mut current_pos = current;
            while let Some(&prev) = came_from.get(&current_pos) {
                path.push(prev);
                current_pos = prev;
            }
            path.reverse();
            return Some(path);
        }

        let current_g = *g_score.get(&current).unwrap_or(&i32::MAX);

        // Check 8 neighbors
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor = (current.0 + dx, current.1 + dy);

                if !overmap.in_bounds(neighbor.0, neighbor.1) {
                    continue;
                }

                let tile = overmap.get_tile(neighbor.0, neighbor.1);
                if tile.is_none() {
                    continue;
                }

                let terrain = tile.unwrap().terrain;
                let mut cost = terrain_cost(terrain);

                // Diagonal movement costs more
                if dx != 0 && dy != 0 {
                    cost = (cost as f32 * 1.4) as i32;
                }

                // Discount if road already exists here
                if existing_roads.contains(&neighbor) {
                    cost = cost / 4; // Much cheaper to use existing roads
                }

                let tentative_g = current_g + cost;
                let neighbor_g = *g_score.get(&neighbor).unwrap_or(&i32::MAX);

                if tentative_g < neighbor_g {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g);
                    open_set.push(PathNode {
                        position: neighbor,
                        cost: tentative_g,
                        heuristic: heuristic(neighbor),
                    });
                }
            }
        }
    }

    None // No path found
}

/// Determine road type based on settlement types
fn determine_road_type(from: &Settlement, to: &Settlement) -> RoadType {
    match (from.settlement_type, to.settlement_type) {
        (SettlementType::City, SettlementType::City) => RoadType::KingRoad,
        (SettlementType::City, _) | (_, SettlementType::City) => RoadType::TradeRoute,
        (SettlementType::Town, SettlementType::Town) => RoadType::TradeRoute,
        (SettlementType::Town, _) | (_, SettlementType::Town) => RoadType::Path,
        _ => RoadType::Path,
    }
}

/// Calculate distance between two points
fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// Generate roads connecting all settlements
pub fn generate_roads(overmap: &Overmap, settlements: &[Settlement]) -> Vec<Road> {
    if settlements.len() < 2 {
        return Vec::new();
    }

    let mut roads = Vec::new();
    let mut existing_road_tiles = HashSet::new();

    // Sort settlements by importance (cities first, then towns, then villages)
    let mut sorted_settlements = settlements.to_vec();
    sorted_settlements.sort_by_key(|s| match s.settlement_type {
        SettlementType::City => 0,
        SettlementType::Town => 1,
        SettlementType::Village => 2,
    });

    // Connect each settlement to its nearest neighbors
    let mut connected = HashSet::new();
    connected.insert(sorted_settlements[0].id);

    // Use Prim's algorithm variant to ensure all settlements are connected
    while connected.len() < sorted_settlements.len() {
        let mut best_connection: Option<(usize, usize, i32)> = None;

        // Find the shortest road from a connected settlement to an unconnected one
        for (i, from_settlement) in sorted_settlements.iter().enumerate() {
            if !connected.contains(&from_settlement.id) {
                continue;
            }

            for (j, to_settlement) in sorted_settlements.iter().enumerate() {
                if connected.contains(&to_settlement.id) || i == j {
                    continue;
                }

                let dist = distance(from_settlement.position, to_settlement.position);

                if let Some((_, _, best_dist)) = best_connection {
                    if dist < best_dist {
                        best_connection = Some((i, j, dist));
                    }
                } else {
                    best_connection = Some((i, j, dist));
                }
            }
        }

        // Build the best connection
        if let Some((from_idx, to_idx, _)) = best_connection {
            let from = &sorted_settlements[from_idx];
            let to = &sorted_settlements[to_idx];

            if let Some(path) = find_path(overmap, from.position, to.position, &existing_road_tiles) {
                let road_type = determine_road_type(from, to);

                // Add all tiles to existing roads set
                for &tile in &path {
                    existing_road_tiles.insert(tile);
                }

                roads.push(Road {
                    road_type,
                    start: from.position,
                    end: to.position,
                    tiles: path,
                });

                connected.insert(to.id);
            }
        } else {
            break; // No more connections possible
        }
    }

    // Add extra roads between nearby major settlements
    for i in 0..sorted_settlements.len() {
        if sorted_settlements[i].settlement_type == SettlementType::Village {
            continue; // Only connect cities and towns with extra roads
        }

        for j in (i + 1)..sorted_settlements.len() {
            if sorted_settlements[j].settlement_type == SettlementType::Village {
                continue;
            }

            let dist = distance(
                sorted_settlements[i].position,
                sorted_settlements[j].position,
            );

            // Connect if they're reasonably close
            // Cities connect at longer distances than towns
            let max_distance = match (sorted_settlements[i].settlement_type, sorted_settlements[j].settlement_type) {
                (SettlementType::City, SettlementType::City) => 60,  // Cities connect across longer distances
                (SettlementType::City, _) | (_, SettlementType::City) => 40,  // City-Town connections
                _ => 30,  // Town-Town connections
            };

            if dist < max_distance {
                let from = &sorted_settlements[i];
                let to = &sorted_settlements[j];

                // Check if a direct road already exists
                let already_connected = roads.iter().any(|r| {
                    (r.start == from.position && r.end == to.position) ||
                    (r.start == to.position && r.end == from.position)
                });

                if !already_connected {
                    if let Some(path) = find_path(overmap, from.position, to.position, &existing_road_tiles) {
                        let road_type = determine_road_type(from, to);

                        for &tile in &path {
                            existing_road_tiles.insert(tile);
                        }

                        roads.push(Road {
                            road_type,
                            start: from.position,
                            end: to.position,
                            tiles: path,
                        });
                    }
                }
            }
        }
    }

    roads
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_cost() {
        assert!(terrain_cost(TerrainType::Plains) < terrain_cost(TerrainType::Mountains));
        assert!(terrain_cost(TerrainType::Lake) > terrain_cost(TerrainType::Forest));
    }

    #[test]
    fn test_road_type_determination() {
        let city1 = Settlement {
            id: 0,
            name: "City1".to_string(),
            settlement_type: SettlementType::City,
            position: (0, 0),
            population: 5000,
            founded_year: 1,
        };
        let city2 = Settlement {
            id: 1,
            name: "City2".to_string(),
            settlement_type: SettlementType::City,
            position: (10, 10),
            population: 5000,
            founded_year: 1,
        };
        let town = Settlement {
            id: 2,
            name: "Town".to_string(),
            settlement_type: SettlementType::Town,
            position: (5, 5),
            population: 1000,
            founded_year: 1,
        };

        assert_eq!(determine_road_type(&city1, &city2), RoadType::KingRoad);
        assert_eq!(determine_road_type(&city1, &town), RoadType::TradeRoute);
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance((0, 0), (3, 4)), 7);
        assert_eq!(distance((10, 10), (10, 10)), 0);
    }

    #[test]
    fn test_road_generation_connects_all_settlements() {
        // Create a simple overmap with plains
        let mut overmap = Overmap::new(50, 50, 12345);

        // Fill with plains for easy pathfinding
        for tile in overmap.tiles.iter_mut() {
            tile.terrain = TerrainType::Plains;
        }

        // Create 5 settlements at various positions
        let settlements = vec![
            Settlement::new(0, "City1".to_string(), SettlementType::City, (10, 10), 5000, 1),
            Settlement::new(1, "Town1".to_string(), SettlementType::Town, (20, 15), 1000, 1),
            Settlement::new(2, "Village1".to_string(), SettlementType::Village, (30, 20), 100, 1),
            Settlement::new(3, "Town2".to_string(), SettlementType::Town, (15, 30), 1200, 1),
            Settlement::new(4, "Village2".to_string(), SettlementType::Village, (35, 35), 80, 1),
        ];

        let roads = generate_roads(&overmap, &settlements);

        // Should have at least 4 roads to connect 5 settlements (minimum spanning tree)
        assert!(roads.len() >= 4, "Expected at least 4 roads, got {}", roads.len());

        // Verify all settlements are reachable
        let mut connected = std::collections::HashSet::new();
        connected.insert(settlements[0].id);

        // Build adjacency from roads
        let mut adj: std::collections::HashMap<(i32, i32), Vec<(i32, i32)>> = std::collections::HashMap::new();
        for road in &roads {
            adj.entry(road.start).or_insert_with(Vec::new).push(road.end);
            adj.entry(road.end).or_insert_with(Vec::new).push(road.start);
        }

        // BFS to find all connected settlements
        let mut queue = vec![settlements[0].position];
        let mut visited = std::collections::HashSet::new();
        visited.insert(settlements[0].position);

        while let Some(pos) = queue.pop() {
            if let Some(neighbors) = adj.get(&pos) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push(neighbor);

                        // Mark settlement as connected if we reached it
                        for settlement in &settlements {
                            if settlement.position == neighbor {
                                connected.insert(settlement.id);
                            }
                        }
                    }
                }
            }
        }

        // All settlements should be connected
        assert_eq!(connected.len(), settlements.len(),
            "Not all settlements are connected. Connected: {}, Total: {}",
            connected.len(), settlements.len());
    }

    #[test]
    fn test_road_types_correct() {
        let mut overmap = Overmap::new(30, 30, 99999);
        for tile in overmap.tiles.iter_mut() {
            tile.terrain = TerrainType::Plains;
        }

        let settlements = vec![
            Settlement::new(0, "City1".to_string(), SettlementType::City, (5, 5), 5000, 1),
            Settlement::new(1, "City2".to_string(), SettlementType::City, (25, 25), 6000, 1),
            Settlement::new(2, "Town1".to_string(), SettlementType::Town, (15, 15), 1000, 1),
        ];

        let roads = generate_roads(&overmap, &settlements);

        // Check that we have at least one King Road (between cities)
        let has_king_road = roads.iter().any(|r| r.road_type == RoadType::KingRoad);
        assert!(has_king_road, "Should have at least one King Road between cities");

        // Check that roads have non-empty paths
        for road in &roads {
            assert!(!road.tiles.is_empty(), "Road path should not be empty");
            assert!(road.tiles.len() > 0, "Road should have tiles");
        }
    }

    #[test]
    fn test_pathfinding_with_obstacles() {
        let mut overmap = Overmap::new(20, 20, 54321);

        // Fill with plains
        for tile in overmap.tiles.iter_mut() {
            tile.terrain = TerrainType::Plains;
        }

        // Add a mountain barrier
        for y in 5..15 {
            let idx = overmap.xy_idx(10, y);
            overmap.tiles[idx].terrain = TerrainType::Mountains;
        }

        // Create settlements on opposite sides of barrier
        let start = (5, 10);
        let end = (15, 10);

        let existing_roads = HashSet::new();
        let path = find_path(&overmap, start, end, &existing_roads);

        assert!(path.is_some(), "Should find a path around the mountain barrier");

        if let Some(path) = path {
            assert!(path.len() > 0, "Path should have tiles");
            assert_eq!(path[0], start, "Path should start at start position");
            assert_eq!(path[path.len() - 1], end, "Path should end at end position");

            // Verify path doesn't go through mountains
            for &pos in &path {
                if let Some(tile) = overmap.get_tile(pos.0, pos.1) {
                    assert_ne!(tile.terrain, TerrainType::Mountains,
                        "Path should not go through mountains");
                }
            }
        }
    }

    #[test]
    fn test_no_roads_with_single_settlement() {
        let overmap = Overmap::new(20, 20, 11111);
        let settlements = vec![
            Settlement::new(0, "Lonely".to_string(), SettlementType::Village, (10, 10), 50, 1),
        ];

        let roads = generate_roads(&overmap, &settlements);
        assert_eq!(roads.len(), 0, "Should have no roads with only one settlement");
    }

    #[test]
    fn test_road_speed_multipliers() {
        let king_road = RoadType::KingRoad;
        let trade_route = RoadType::TradeRoute;
        let path = RoadType::Path;

        assert!(king_road.travel_speed_multiplier() > trade_route.travel_speed_multiplier());
        assert!(trade_route.travel_speed_multiplier() > path.travel_speed_multiplier());
        assert!(path.travel_speed_multiplier() > 1.0, "All roads should be faster than no road");
    }

    #[test]
    fn test_deterministic_road_generation() {
        let overmap1 = Overmap::new(30, 30, 77777);
        let overmap2 = Overmap::new(30, 30, 77777);

        let settlements = vec![
            Settlement::new(0, "City".to_string(), SettlementType::City, (10, 10), 5000, 1),
            Settlement::new(1, "Town".to_string(), SettlementType::Town, (20, 20), 1000, 1),
        ];

        let roads1 = generate_roads(&overmap1, &settlements);
        let roads2 = generate_roads(&overmap2, &settlements);

        // Same seed and settlements should produce same roads
        assert_eq!(roads1.len(), roads2.len(), "Same seed should produce same number of roads");

        for (r1, r2) in roads1.iter().zip(roads2.iter()) {
            assert_eq!(r1.road_type, r2.road_type, "Road types should match");
            assert_eq!(r1.tiles.len(), r2.tiles.len(), "Road lengths should match");
        }
    }
}
