use crate::map::{Map, Tile};
use std::collections::HashMap;

/// Size of a chunk in tiles (32x32)
pub const CHUNK_SIZE: i32 = 32;

/// Radius of chunks to keep loaded around the player (in chunks)
pub const LOAD_RADIUS: i32 = 2;

/// A chunk is a fixed-size section of the map
#[derive(Clone)]
pub struct Chunk {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub tiles: Vec<Tile>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
}

impl Chunk {
    /// Create a new empty chunk
    pub fn new(chunk_x: i32, chunk_y: i32) -> Self {
        let size = (CHUNK_SIZE * CHUNK_SIZE) as usize;
        Self {
            chunk_x,
            chunk_y,
            tiles: vec![Tile::Wall; size],
            visible: vec![false; size],
            revealed: vec![false; size],
        }
    }

    /// Get the world position of the top-left corner of this chunk
    pub fn world_position(&self) -> (i32, i32) {
        (self.chunk_x * CHUNK_SIZE, self.chunk_y * CHUNK_SIZE)
    }

    /// Convert chunk-local coordinates to index
    pub fn xy_idx(&self, local_x: i32, local_y: i32) -> usize {
        (local_y * CHUNK_SIZE + local_x) as usize
    }

    /// Check if local coordinates are valid within this chunk
    pub fn in_bounds(&self, local_x: i32, local_y: i32) -> bool {
        local_x >= 0 && local_x < CHUNK_SIZE && local_y >= 0 && local_y < CHUNK_SIZE
    }
}

/// Manages loading and unloading of map chunks
pub struct ChunkManager {
    /// All loaded chunks, keyed by (chunk_x, chunk_y)
    pub loaded_chunks: HashMap<(i32, i32), Chunk>,

    /// The full map (for fallback when chunks aren't implemented everywhere)
    pub full_map: Map,

    /// Whether chunk system is enabled
    pub enabled: bool,
}

impl ChunkManager {
    /// Create a new chunk manager with an initial map
    pub fn new(map: Map) -> Self {
        Self {
            loaded_chunks: HashMap::new(),
            full_map: map,
            enabled: false, // Disabled by default for now
        }
    }

    /// Convert world coordinates to chunk coordinates
    pub fn world_to_chunk(world_x: i32, world_y: i32) -> (i32, i32) {
        (
            if world_x >= 0 { world_x / CHUNK_SIZE } else { (world_x - CHUNK_SIZE + 1) / CHUNK_SIZE },
            if world_y >= 0 { world_y / CHUNK_SIZE } else { (world_y - CHUNK_SIZE + 1) / CHUNK_SIZE },
        )
    }

    /// Convert world coordinates to chunk-local coordinates
    pub fn world_to_local(world_x: i32, world_y: i32) -> (i32, i32) {
        let local_x = world_x.rem_euclid(CHUNK_SIZE);
        let local_y = world_y.rem_euclid(CHUNK_SIZE);
        (local_x, local_y)
    }

    /// Get a tile at world coordinates (chunk system or fallback to full map)
    pub fn get_tile(&self, world_x: i32, world_y: i32) -> Option<Tile> {
        if !self.enabled {
            // Use full map when chunk system is disabled
            if !self.full_map.in_bounds(world_x, world_y) {
                return None;
            }
            let idx = self.full_map.xy_idx(world_x, world_y);
            return Some(self.full_map.tiles[idx]);
        }

        let (chunk_x, chunk_y) = Self::world_to_chunk(world_x, world_y);
        let chunk = self.loaded_chunks.get(&(chunk_x, chunk_y))?;

        let (local_x, local_y) = Self::world_to_local(world_x, world_y);
        if !chunk.in_bounds(local_x, local_y) {
            return None;
        }

        let idx = chunk.xy_idx(local_x, local_y);
        Some(chunk.tiles[idx])
    }

    /// Set a tile at world coordinates
    pub fn set_tile(&mut self, world_x: i32, world_y: i32, tile: Tile) {
        if !self.enabled {
            // Use full map when chunk system is disabled
            if !self.full_map.in_bounds(world_x, world_y) {
                return;
            }
            let idx = self.full_map.xy_idx(world_x, world_y);
            self.full_map.tiles[idx] = tile;
            return;
        }

        let (chunk_x, chunk_y) = Self::world_to_chunk(world_x, world_y);

        // Load chunk if not loaded
        if !self.loaded_chunks.contains_key(&(chunk_x, chunk_y)) {
            self.load_chunk(chunk_x, chunk_y);
        }

        if let Some(chunk) = self.loaded_chunks.get_mut(&(chunk_x, chunk_y)) {
            let (local_x, local_y) = Self::world_to_local(world_x, world_y);
            if chunk.in_bounds(local_x, local_y) {
                let idx = chunk.xy_idx(local_x, local_y);
                chunk.tiles[idx] = tile;
            }
        }
    }

    /// Load chunks around a player position
    pub fn update_loaded_chunks(&mut self, player_x: i32, player_y: i32) {
        if !self.enabled {
            return;
        }

        let (player_chunk_x, player_chunk_y) = Self::world_to_chunk(player_x, player_y);

        // Load chunks within radius
        for dy in -LOAD_RADIUS..=LOAD_RADIUS {
            for dx in -LOAD_RADIUS..=LOAD_RADIUS {
                let chunk_x = player_chunk_x + dx;
                let chunk_y = player_chunk_y + dy;

                if !self.loaded_chunks.contains_key(&(chunk_x, chunk_y)) {
                    self.load_chunk(chunk_x, chunk_y);
                }
            }
        }

        // Unload chunks outside radius
        let chunks_to_unload: Vec<(i32, i32)> = self.loaded_chunks
            .keys()
            .filter(|(cx, cy)| {
                let dx = (cx - player_chunk_x).abs();
                let dy = (cy - player_chunk_y).abs();
                dx > LOAD_RADIUS || dy > LOAD_RADIUS
            })
            .copied()
            .collect();

        for chunk_coords in chunks_to_unload {
            self.unload_chunk(chunk_coords.0, chunk_coords.1);
        }
    }

    /// Load a specific chunk
    fn load_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        // Create new chunk and populate from full map if available
        let mut chunk = Chunk::new(chunk_x, chunk_y);

        let (world_start_x, world_start_y) = chunk.world_position();

        // Copy data from full map if within bounds
        for local_y in 0..CHUNK_SIZE {
            for local_x in 0..CHUNK_SIZE {
                let world_x = world_start_x + local_x;
                let world_y = world_start_y + local_y;

                if self.full_map.in_bounds(world_x, world_y) {
                    let map_idx = self.full_map.xy_idx(world_x, world_y);
                    let chunk_idx = chunk.xy_idx(local_x, local_y);

                    chunk.tiles[chunk_idx] = self.full_map.tiles[map_idx];
                    chunk.visible[chunk_idx] = self.full_map.visible[map_idx];
                    chunk.revealed[chunk_idx] = self.full_map.revealed[map_idx];
                }
            }
        }

        self.loaded_chunks.insert((chunk_x, chunk_y), chunk);
    }

    /// Unload a specific chunk (could save to disk here)
    fn unload_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        // In a full implementation, we might save the chunk to disk here
        self.loaded_chunks.remove(&(chunk_x, chunk_y));
    }

    /// Get the number of loaded chunks
    pub fn loaded_count(&self) -> usize {
        self.loaded_chunks.len()
    }

    /// Get active map (either full map or for compatibility)
    pub fn active_map(&self) -> &Map {
        &self.full_map
    }

    /// Get mutable active map
    pub fn active_map_mut(&mut self) -> &mut Map {
        &mut self.full_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new(0, 0);
        assert_eq!(chunk.chunk_x, 0);
        assert_eq!(chunk.chunk_y, 0);
        assert_eq!(chunk.tiles.len(), (CHUNK_SIZE * CHUNK_SIZE) as usize);
    }

    #[test]
    fn test_world_to_chunk() {
        assert_eq!(ChunkManager::world_to_chunk(0, 0), (0, 0));
        assert_eq!(ChunkManager::world_to_chunk(31, 31), (0, 0));
        assert_eq!(ChunkManager::world_to_chunk(32, 32), (1, 1));
        assert_eq!(ChunkManager::world_to_chunk(64, 64), (2, 2));
        assert_eq!(ChunkManager::world_to_chunk(-1, -1), (-1, -1));
    }

    #[test]
    fn test_world_to_local() {
        assert_eq!(ChunkManager::world_to_local(0, 0), (0, 0));
        assert_eq!(ChunkManager::world_to_local(5, 10), (5, 10));
        assert_eq!(ChunkManager::world_to_local(32, 32), (0, 0));
        assert_eq!(ChunkManager::world_to_local(33, 33), (1, 1));
    }

    #[test]
    fn test_chunk_world_position() {
        let chunk = Chunk::new(0, 0);
        assert_eq!(chunk.world_position(), (0, 0));

        let chunk = Chunk::new(1, 2);
        assert_eq!(chunk.world_position(), (32, 64));
    }

    #[test]
    fn test_chunk_bounds() {
        let chunk = Chunk::new(0, 0);
        assert!(chunk.in_bounds(0, 0));
        assert!(chunk.in_bounds(31, 31));
        assert!(!chunk.in_bounds(32, 32));
        assert!(!chunk.in_bounds(-1, -1));
    }

    #[test]
    fn test_chunk_manager_creation() {
        let map = Map::new(100, 100);
        let manager = ChunkManager::new(map);
        assert_eq!(manager.loaded_count(), 0);
        assert!(!manager.enabled);
    }

    #[test]
    fn test_chunk_loading() {
        let map = Map::new(100, 100);
        let mut manager = ChunkManager::new(map);
        manager.enabled = true;

        // Load chunks around origin
        manager.update_loaded_chunks(0, 0);

        // Should load (2*LOAD_RADIUS+1)^2 chunks
        let expected_chunks = ((2 * LOAD_RADIUS + 1) * (2 * LOAD_RADIUS + 1)) as usize;
        assert_eq!(manager.loaded_count(), expected_chunks);
    }

    #[test]
    fn test_chunk_unloading() {
        let map = Map::new(200, 200);
        let mut manager = ChunkManager::new(map);
        manager.enabled = true;

        // Load chunks around origin
        manager.update_loaded_chunks(0, 0);
        let initial_count = manager.loaded_count();
        assert!(initial_count > 0);

        // Move far away - should unload old chunks and load new ones
        manager.update_loaded_chunks(150, 150);

        // Should still have chunks loaded around new position
        assert!(manager.loaded_count() > 0);

        // The chunks should be different (around new position)
        assert!(manager.loaded_chunks.contains_key(&(4, 4))); // Near (150, 150) -> chunk (4, 4)
    }

    #[test]
    fn test_get_set_tile() {
        let mut map = Map::new(100, 100);
        // Set some initial tiles
        let idx = map.xy_idx(5, 5);
        map.tiles[idx] = Tile::Floor;

        let mut manager = ChunkManager::new(map);

        // Test with disabled chunks (uses full map)
        assert_eq!(manager.get_tile(5, 5), Some(Tile::Floor));
        manager.set_tile(10, 10, Tile::Floor);
        assert_eq!(manager.get_tile(10, 10), Some(Tile::Floor));
    }

    #[test]
    fn test_chunk_system_disabled_by_default() {
        let map = Map::new(50, 50);
        let manager = ChunkManager::new(map);

        // Should work with full map when disabled
        assert_eq!(manager.get_tile(0, 0), Some(Tile::Wall));
        assert_eq!(manager.loaded_count(), 0);
    }
}
