use crate::map::Map;
use crate::domain_types::Depth;
use std::collections::HashMap;

/// Simple Point type for map coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Unique identifier for different map types
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum MapId {
    /// The overworld/overmap (not a local Map, but tracked for consistency)
    Overworld,
    /// A settlement interior map by settlement ID
    Settlement(usize),
    /// A dungeon level at a specific depth
    Dungeon(Depth),
    /// A POI (dungeon, cave, etc.) map by POI ID
    POI(usize),
}

/// A location in the game world (map + position)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub map_id: MapId,
    pub pos: Point,
}

impl Location {
    pub fn new(map_id: MapId, x: i32, y: i32) -> Self {
        Self {
            map_id,
            pos: Point::new(x, y),
        }
    }
}

/// Cache for generated maps with on-demand generation
/// This unifies storage for all map types (settlements, dungeons, POIs)
pub struct MapCache {
    /// Currently active map ID
    pub active: MapId,

    /// Generated settlement maps
    settlements: HashMap<usize, Map>,

    /// Generated dungeon levels
    dungeons: HashMap<Depth, Map>,

    /// Generated POI maps
    pois: HashMap<usize, Map>,

    /// Default map dimensions for generation
    default_width: i32,
    default_height: i32,
}

impl MapCache {
    pub fn new(default_width: i32, default_height: i32) -> Self {
        Self {
            active: MapId::Overworld,
            settlements: HashMap::new(),
            dungeons: HashMap::new(),
            pois: HashMap::new(),
            default_width,
            default_height,
        }
    }

    /// Get a reference to a map by ID, generating if necessary
    pub fn get(&self, id: MapId) -> Option<&Map> {
        match id {
            MapId::Overworld => None, // Overworld is handled separately
            MapId::Settlement(sid) => self.settlements.get(&sid),
            MapId::Dungeon(depth) => self.dungeons.get(&depth),
            MapId::POI(pid) => self.pois.get(&pid),
        }
    }

    /// Get a mutable reference to a map by ID
    pub fn get_mut(&mut self, id: MapId) -> Option<&mut Map> {
        match id {
            MapId::Overworld => None, // Overworld is handled separately
            MapId::Settlement(sid) => self.settlements.get_mut(&sid),
            MapId::Dungeon(depth) => self.dungeons.get_mut(&depth),
            MapId::POI(pid) => self.pois.get_mut(&pid),
        }
    }

    /// Insert a map into the cache
    pub fn insert(&mut self, id: MapId, map: Map) {
        match id {
            MapId::Overworld => {}, // Overworld is handled separately
            MapId::Settlement(sid) => {
                self.settlements.insert(sid, map);
            },
            MapId::Dungeon(depth) => {
                self.dungeons.insert(depth, map);
            },
            MapId::POI(pid) => {
                self.pois.insert(pid, map);
            },
        }
    }

    /// Check if a map exists in the cache
    pub fn contains(&self, id: MapId) -> bool {
        match id {
            MapId::Overworld => false, // Overworld is handled separately
            MapId::Settlement(sid) => self.settlements.contains_key(&sid),
            MapId::Dungeon(depth) => self.dungeons.contains_key(&depth),
            MapId::POI(pid) => self.pois.contains_key(&pid),
        }
    }

    /// Get the currently active map
    pub fn current(&self) -> Option<&Map> {
        self.get(self.active)
    }

    /// Get the currently active map (mutable)
    pub fn current_mut(&mut self) -> Option<&mut Map> {
        let active = self.active;
        self.get_mut(active)
    }

    /// Set the active map
    pub fn set_active(&mut self, id: MapId) {
        self.active = id;
    }

    /// Clear all cached maps (useful for testing or memory management)
    pub fn clear(&mut self) {
        self.settlements.clear();
        self.dungeons.clear();
        self.pois.clear();
    }

    /// Get references to the internal storage (for migration/save-load)
    pub fn settlements(&self) -> &HashMap<usize, Map> {
        &self.settlements
    }

    pub fn settlements_mut(&mut self) -> &mut HashMap<usize, Map> {
        &mut self.settlements
    }

    pub fn dungeons(&self) -> &HashMap<Depth, Map> {
        &self.dungeons
    }

    pub fn dungeons_mut(&mut self) -> &mut HashMap<Depth, Map> {
        &mut self.dungeons
    }

    pub fn pois(&self) -> &HashMap<usize, Map> {
        &self.pois
    }

    pub fn pois_mut(&mut self) -> &mut HashMap<usize, Map> {
        &mut self.pois
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_cache_creation() {
        let cache = MapCache::new(80, 50);
        assert_eq!(cache.active, MapId::Overworld);
        assert!(!cache.contains(MapId::Settlement(0)));
        assert!(!cache.contains(MapId::Dungeon(Depth(1))));
    }

    #[test]
    fn test_map_cache_insert_and_get() {
        let mut cache = MapCache::new(80, 50);
        let map = Map::new(80, 50);

        cache.insert(MapId::Settlement(0), map);
        assert!(cache.contains(MapId::Settlement(0)));
        assert!(cache.get(MapId::Settlement(0)).is_some());
    }

    #[test]
    fn test_map_cache_active_map() {
        let mut cache = MapCache::new(80, 50);
        let map = Map::new(80, 50);

        cache.insert(MapId::Dungeon(Depth(1)), map);
        cache.set_active(MapId::Dungeon(Depth(1)));

        assert_eq!(cache.active, MapId::Dungeon(Depth(1)));
        assert!(cache.current().is_some());
    }

    #[test]
    fn test_location_creation() {
        let loc = Location::new(MapId::Settlement(5), 10, 20);
        assert_eq!(loc.map_id, MapId::Settlement(5));
        assert_eq!(loc.pos.x, 10);
        assert_eq!(loc.pos.y, 20);
    }
}
