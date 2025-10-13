use super::terrain::TerrainType;
use serde::{Deserialize, Serialize};

/// Unique identifier for locations (settlements, dungeons, etc.)
pub type LocationId = usize;

/// A single tile on the overmap
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OvermapTile {
    /// World coordinates
    pub x: i32,
    pub y: i32,

    /// Base terrain type
    pub terrain: TerrainType,

    /// Has the player discovered this tile?
    pub discovered: bool,

    /// Has the player visited this tile?
    pub visited: bool,

    /// Corruption level (0-100)
    pub corruption: u8,

    /// Optional reference to a location at this tile
    pub location: Option<LocationId>,
}

impl OvermapTile {
    pub fn new(x: i32, y: i32, terrain: TerrainType) -> Self {
        Self {
            x,
            y,
            terrain,
            discovered: false,
            visited: false,
            corruption: 0,
            location: None,
        }
    }
}

/// The overworld map - a large grid representing the game world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Overmap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<OvermapTile>,
    pub seed: u64,
}

impl Overmap {
    /// Create a new overmap with given dimensions
    pub fn new(width: i32, height: i32, seed: u64) -> Self {
        let size = (width * height) as usize;
        let mut tiles = Vec::with_capacity(size);

        // Initialize with plains
        for y in 0..height {
            for x in 0..width {
                tiles.push(OvermapTile::new(x, y, TerrainType::Plains));
            }
        }

        Self {
            width,
            height,
            tiles,
            seed,
        }
    }

    /// Convert x, y coordinates to index
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Convert index to x, y coordinates
    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    /// Check if coordinates are within bounds
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Get tile at coordinates (immutable)
    pub fn get_tile(&self, x: i32, y: i32) -> Option<&OvermapTile> {
        if !self.in_bounds(x, y) {
            return None;
        }
        let idx = self.xy_idx(x, y);
        self.tiles.get(idx)
    }

    /// Get tile at coordinates (mutable)
    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut OvermapTile> {
        if !self.in_bounds(x, y) {
            return None;
        }
        let idx = self.xy_idx(x, y);
        self.tiles.get_mut(idx)
    }

    /// Check if tile is walkable
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.terrain.is_walkable()
        } else {
            false
        }
    }

    /// Mark a tile as discovered
    pub fn discover_tile(&mut self, x: i32, y: i32) {
        if let Some(tile) = self.get_tile_mut(x, y) {
            tile.discovered = true;
        }
    }

    /// Mark a tile as visited
    pub fn visit_tile(&mut self, x: i32, y: i32) {
        if let Some(tile) = self.get_tile_mut(x, y) {
            tile.discovered = true;
            tile.visited = true;
        }
    }

    /// Get neighbors of a tile (4-directional)
    pub fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if self.in_bounds(nx, ny) {
                neighbors.push((nx, ny));
            }
        }

        neighbors
    }

    /// Get all tiles in a radius (circular)
    pub fn get_tiles_in_radius(&self, center_x: i32, center_y: i32, radius: i32) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        let radius_sq = radius * radius;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let x = center_x + dx;
                let y = center_y + dy;

                if self.in_bounds(x, y) {
                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq <= radius_sq {
                        tiles.push((x, y));
                    }
                }
            }
        }

        tiles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overmap_creation() {
        let overmap = Overmap::new(50, 50, 12345);
        assert_eq!(overmap.width, 50);
        assert_eq!(overmap.height, 50);
        assert_eq!(overmap.tiles.len(), 2500);
    }

    #[test]
    fn test_xy_idx_conversion() {
        let overmap = Overmap::new(10, 10, 12345);

        // Test conversion
        let idx = overmap.xy_idx(5, 3);
        assert_eq!(idx, 35);

        let (x, y) = overmap.idx_xy(35);
        assert_eq!(x, 5);
        assert_eq!(y, 3);
    }

    #[test]
    fn test_bounds_checking() {
        let overmap = Overmap::new(10, 10, 12345);

        assert!(overmap.in_bounds(0, 0));
        assert!(overmap.in_bounds(9, 9));
        assert!(!overmap.in_bounds(-1, 0));
        assert!(!overmap.in_bounds(0, -1));
        assert!(!overmap.in_bounds(10, 0));
        assert!(!overmap.in_bounds(0, 10));
    }

    #[test]
    fn test_tile_access() {
        let mut overmap = Overmap::new(10, 10, 12345);

        // Get tile
        let tile = overmap.get_tile(5, 5);
        assert!(tile.is_some());

        // Modify tile
        if let Some(tile) = overmap.get_tile_mut(5, 5) {
            tile.terrain = TerrainType::Forest;
            tile.discovered = true;
        }

        // Verify modification
        let tile = overmap.get_tile(5, 5).unwrap();
        assert_eq!(tile.terrain, TerrainType::Forest);
        assert!(tile.discovered);
    }

    #[test]
    fn test_neighbors() {
        let overmap = Overmap::new(10, 10, 12345);

        // Center tile should have 4 neighbors
        let neighbors = overmap.get_neighbors(5, 5);
        assert_eq!(neighbors.len(), 4);

        // Corner tile should have 2 neighbors
        let neighbors = overmap.get_neighbors(0, 0);
        assert_eq!(neighbors.len(), 2);

        // Edge tile should have 3 neighbors
        let neighbors = overmap.get_neighbors(5, 0);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn test_radius_tiles() {
        let overmap = Overmap::new(20, 20, 12345);

        // Radius 0 should return just the center
        let tiles = overmap.get_tiles_in_radius(10, 10, 0);
        assert_eq!(tiles.len(), 1);

        // Radius 1 should return ~5 tiles (center + 4 cardinal)
        let tiles = overmap.get_tiles_in_radius(10, 10, 1);
        assert!(tiles.len() >= 5 && tiles.len() <= 9);

        // Radius 2 should return more tiles
        let tiles = overmap.get_tiles_in_radius(10, 10, 2);
        assert!(tiles.len() > 9);
    }
}
