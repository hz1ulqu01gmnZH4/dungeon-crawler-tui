# Terrain-Based Local Map Generation Plan

## Overview

Implement dynamic local map generation based on overmap terrain type. When entering a location from the overmap, generate an appropriate local map that matches the terrain aesthetics and gameplay.

## Current State

### What Exists
✅ **TerrainType Enum**: 13 terrain types defined
- Plains, Forest, DenseForest, Hills, Mountains
- Lake, River, Swamp
- Road, KingRoad, TradePath
- Settlement, Dungeon, SpecialLocation

✅ **Building Interior Generation**: 7 building types with unique layouts
- Demonstrates we can generate different map styles

✅ **Map Generator**: Generic dungeon generator exists
- Room-based generation with corridors
- Can be adapted for different terrain types

### What's Missing
❌ **Terrain-Specific Generators**: No forest/mountain/swamp map generators
❌ **Transition System**: No link between overmap terrain and local map style
❌ **Tile Varieties**: Limited tile types (Floor, Wall, Door)
❌ **Biome-Specific Features**: No terrain-unique elements

## Implementation Plan

### Phase 2A: Extended Tile System

**File**: `src/map/tile.rs`

Add new tile types for different terrains:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    // Existing
    Floor,
    Wall,
    Door,

    // Natural terrain (NEW)
    Grass,           // Plains
    DirtPath,        // Worn paths
    ShortGrass,      // Lighter grass

    // Forest
    Tree,            // Blocks movement and sight
    Bush,            // Blocks movement, not sight
    Undergrowth,     // Walkable, slow
    DeadTree,        // Scenery

    // Mountains
    Rock,            // Impassable
    Boulder,         // Large rock
    Rubble,          // Walkable rocks
    MountainWall,    // Cliff face
    CaveFloor,       // Inside caves
    CaveWall,        // Cave walls

    // Water
    ShallowWater,    // Walkable, slow
    DeepWater,       // Impassable
    Ice,             // Slippery

    // Swamp
    Mud,             // Very slow movement
    SwampWater,      // Shallow, murky
    DeadTreeSwamp,   // Swamp scenery
    Reeds,           // Tall swamp grass

    // Hills
    HighGround,      // Elevated position
    LowGround,       // Valley
    Slope,           // Transition

    // Special
    Bridge,          // Cross water
    Shrine,          // Points of interest
    Campfire,        // Rest spots
    Statue,          // Scenery
}
```

### Phase 2B: Terrain-Specific Generators

**File**: `src/map/terrain_generators.rs` (NEW)

```rust
use crate::map::{Map, Tile};
use crate::world::TerrainType;
use rand::rngs::StdRng;

/// Generate a local map based on terrain type
pub fn generate_terrain_map(
    terrain: TerrainType,
    width: i32,
    height: i32,
    rng: &mut StdRng,
    seed: u64,
) -> Map {
    match terrain {
        TerrainType::Plains => generate_plains_map(width, height, rng),
        TerrainType::Forest => generate_forest_map(width, height, rng),
        TerrainType::DenseForest => generate_dense_forest_map(width, height, rng),
        TerrainType::Hills => generate_hills_map(width, height, rng),
        TerrainType::Mountains => generate_mountain_map(width, height, rng),
        TerrainType::Lake => generate_lake_shore_map(width, height, rng),
        TerrainType::River => generate_river_map(width, height, rng),
        TerrainType::Swamp => generate_swamp_map(width, height, rng),
        TerrainType::Dungeon => generate_dungeon_map(width, height, rng), // Existing
        _ => generate_generic_map(width, height, rng),
    }
}
```

### Phase 2C: Individual Terrain Generators

#### 1. Plains Generator
```rust
fn generate_plains_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = Map::new(width, height);

    // Fill with grass
    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = if rng.gen::<f32>() < 0.1 {
                Tile::ShortGrass  // Variation
            } else {
                Tile::Grass
            };
        }
    }

    // Add occasional features
    for _ in 0..rng.gen_range(3..8) {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = map.xy_idx(x, y);

        // Random features
        map.tiles[idx] = match rng.gen_range(0..3) {
            0 => Tile::Bush,      // Small obstacle
            1 => Tile::Boulder,   // Rock
            _ => Tile::Shrine,    // Point of interest
        };
    }

    // Add dirt paths connecting edges
    add_natural_paths(&mut map, rng, Tile::DirtPath);

    map
}
```

#### 2. Forest Generator
```rust
fn generate_forest_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = Map::new(width, height);

    // Base: grass floor
    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = Tile::Grass;
        }
    }

    // Cellular automata for tree clusters
    let tree_density = 0.45;  // 45% trees
    for y in 0..height {
        for x in 0..width {
            if rng.gen::<f32>() < tree_density {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = Tile::Tree;
            }
        }
    }

    // Smooth with cellular automata (4-5 rule)
    smooth_terrain(&mut map, Tile::Tree, Tile::Grass, 4, 3);

    // Add clearings (guaranteed open spaces)
    create_clearings(&mut map, rng, 3..6, 8..15);

    // Add undergrowth at forest edges
    add_edge_tiles(&mut map, Tile::Tree, Tile::Undergrowth, 0.6);

    // Occasional dead trees for scenery
    replace_random_tiles(&mut map, Tile::Tree, Tile::DeadTree, 0.05);

    map
}
```

#### 3. Dense Forest Generator
```rust
fn generate_dense_forest_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = generate_forest_map(width, height, rng);

    // Much higher tree density
    let extra_trees = (width * height / 4) as usize;
    for _ in 0..extra_trees {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = map.xy_idx(x, y);

        if map.tiles[idx] == Tile::Grass {
            map.tiles[idx] = Tile::Tree;
        }
    }

    // Even more undergrowth
    replace_random_tiles(&mut map, Tile::Grass, Tile::Undergrowth, 0.4);

    // Add bushes
    for _ in 0..(width * height / 20) {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = map.xy_idx(x, y);

        if map.tiles[idx] == Tile::Grass {
            map.tiles[idx] = Tile::Bush;
        }
    }

    // Smaller clearings
    create_clearings(&mut map, rng, 1..3, 5..8);

    map
}
```

#### 4. Mountain Generator
```rust
fn generate_mountain_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = Map::new(width, height);

    // Generate cave system using cellular automata
    // Start with random rocks
    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = if rng.gen::<f32>() < 0.55 {
                Tile::Rock
            } else {
                Tile::CaveFloor
            };
        }
    }

    // Apply cellular automata to create natural caves
    for _ in 0..4 {
        smooth_terrain(&mut map, Tile::Rock, Tile::CaveFloor, 5, 3);
    }

    // Ensure connectivity
    ensure_connected_caves(&mut map, Tile::CaveFloor, Tile::Rock);

    // Add cave walls around edges
    add_perimeter(&mut map, Tile::CaveWall);

    // Add boulders and rubble
    replace_random_tiles(&mut map, Tile::CaveFloor, Tile::Rubble, 0.15);
    replace_random_tiles(&mut map, Tile::Rock, Tile::Boulder, 0.1);

    // Add stalagmites/columns (decorative rocks)
    for _ in 0..(width * height / 40) {
        let x = rng.gen_range(1..width - 1);
        let y = rng.gen_range(1..height - 1);
        let idx = map.xy_idx(x, y);

        if map.tiles[idx] == Tile::CaveFloor {
            map.tiles[idx] = Tile::Boulder;
        }
    }

    map
}
```

#### 5. Hills Generator
```rust
fn generate_hills_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = Map::new(width, height);

    // Use Perlin/simplex noise for elevation
    let elevation = generate_elevation_map(width, height, rng);

    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            let elev = elevation[idx];

            map.tiles[idx] = if elev > 0.7 {
                Tile::HighGround
            } else if elev < 0.3 {
                Tile::LowGround
            } else if elev > 0.5 && elev < 0.6 {
                Tile::Slope
            } else {
                Tile::Grass
            };
        }
    }

    // Add rocks on high ground
    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == Tile::HighGround && rng.gen::<f32>() < 0.2 {
                map.tiles[idx] = Tile::Rock;
            }
        }
    }

    // Add paths through valleys
    add_natural_paths(&mut map, rng, Tile::DirtPath);

    map
}
```

#### 6. Swamp Generator
```rust
fn generate_swamp_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = Map::new(width, height);

    // Mix of mud, shallow water, and grass
    for y in 0..height {
        for x in 0..width {
            let idx = map.xy_idx(x, y);
            let rand = rng.gen::<f32>();

            map.tiles[idx] = if rand < 0.3 {
                Tile::SwampWater
            } else if rand < 0.6 {
                Tile::Mud
            } else {
                Tile::Grass
            };
        }
    }

    // Smooth to create water patches
    smooth_terrain(&mut map, Tile::SwampWater, Tile::Mud, 4, 2);

    // Add reeds around water
    add_edge_tiles(&mut map, Tile::SwampWater, Tile::Reeds, 0.5);

    // Dead trees scattered throughout
    for _ in 0..(width * height / 30) {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = map.xy_idx(x, y);

        if map.tiles[idx] == Tile::Grass || map.tiles[idx] == Tile::Mud {
            map.tiles[idx] = Tile::DeadTreeSwamp;
        }
    }

    // Ensure some dry paths exist
    create_walkable_paths(&mut map, rng, Tile::Grass);

    map
}
```

#### 7. River Generator
```rust
fn generate_river_map(width: i32, height: i32, rng: &mut StdRng) -> Map {
    let mut map = Map::new(width, height);

    // Fill with grass
    for idx in 0..map.tiles.len() {
        map.tiles[idx] = Tile::Grass;
    }

    // Generate meandering river through middle
    let river_start_y = height / 2;
    let mut river_y = river_start_y;

    for x in 0..width {
        // River meanders up and down
        river_y += rng.gen_range(-1..=1);
        river_y = river_y.clamp(height / 4, (height * 3) / 4);

        // River is 3-5 tiles wide
        let river_width = rng.gen_range(3..=5);
        for dy in 0..river_width {
            let y = river_y + dy - river_width / 2;
            if y >= 0 && y < height {
                let idx = map.xy_idx(x, y);

                // Center is deep, edges are shallow
                map.tiles[idx] = if dy == 0 || dy == river_width - 1 {
                    Tile::ShallowWater
                } else {
                    Tile::DeepWater
                };
            }
        }
    }

    // Add bridges across river
    let bridge_count = rng.gen_range(1..=3);
    for _ in 0..bridge_count {
        let bridge_x = rng.gen_range(width / 4..(width * 3) / 4);
        for y in 0..height {
            let idx = map.xy_idx(bridge_x, y);
            if map.tiles[idx] == Tile::DeepWater || map.tiles[idx] == Tile::ShallowWater {
                map.tiles[idx] = Tile::Bridge;
            }
        }
    }

    // Add riverbank features
    for x in 0..width {
        for y in 0..height {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == Tile::Grass {
                // Check if near water
                if is_adjacent_to_water(&map, x, y) && rng.gen::<f32>() < 0.2 {
                    map.tiles[idx] = Tile::Reeds;
                }
            }
        }
    }

    map
}
```

### Phase 2D: Transition System

**File**: `src/systems/input.rs` (modify)

Update the Enter key handler on overmap:

```rust
// When player presses Enter on overmap
KeyCode::Enter if resources.in_overmap_mode => {
    let player_pos = resources.player_overmap_pos;

    // Get terrain at current position
    if let Some(tile) = resources.overmap.get_tile(player_pos.0, player_pos.1) {
        let terrain = tile.terrain;

        // Generate appropriate local map based on terrain
        match terrain {
            TerrainType::Settlement => {
                // Enter settlement (existing logic)
                enter_settlement(resources, player_pos);
            }
            TerrainType::Dungeon => {
                // Enter dungeon (existing logic)
                enter_dungeon(resources);
            }
            TerrainType::SpecialLocation => {
                // Enter POI
                enter_poi(resources, player_pos);
            }
            _ => {
                // Enter terrain-specific local map (NEW)
                enter_terrain_map(resources, terrain, player_pos);
            }
        }

        resources.in_overmap_mode = false;
        resources.log.add(format!("Entering {}...", terrain_name(terrain)));
    }
}

fn enter_terrain_map(resources: &mut Resources, terrain: TerrainType, pos: (i32, i32)) {
    // Generate seed based on overmap position for consistency
    let seed = hash_position(pos);

    // Generate terrain-specific map
    let new_map = terrain_generators::generate_terrain_map(
        terrain,
        80,
        50,
        &mut resources.rng,
        seed,
    );

    // Replace current map with terrain map
    *resources.maps.active_map_mut() = new_map;

    // Place player in safe starting position
    let start_pos = find_safe_spawn_point(&resources.maps.active_map());
    move_player_to(resources, start_pos);

    // Store terrain type for context
    resources.current_terrain = Some(terrain);
}
```

### Phase 2E: Gameplay Integration

**Terrain-Specific Features**:

```rust
// In movement system
match current_tile {
    Tile::Undergrowth => {
        // Slow movement
        time_cost *= 1.5;
        chance_of_event += 0.1;
    }
    Tile::Mud => {
        // Very slow movement
        time_cost *= 2.0;
        log.add("You trudge through thick mud...");
    }
    Tile::HighGround => {
        // Better visibility
        viewshed.range += 2;
        log.add("From this vantage point, you can see farther.");
    }
    Tile::ShallowWater => {
        // Crossable but slow
        time_cost *= 1.3;
        if rng.gen::<f32>() < 0.1 {
            log.add("You wade through the water.");
        }
    }
    Tile::Ice => {
        // Slippery - chance to slide
        if rng.gen::<f32>() < 0.2 {
            log.add("You slip on the ice!");
            // Slide in random direction
        }
    }
    _ => {}
}
```

## Implementation Priority

### Phase 2.1: Foundation (1-2 weeks)
1. ✅ Extend Tile enum with new terrain tiles
2. ✅ Implement tile rendering and colors
3. ✅ Add tile walkability and sight-blocking logic
4. ✅ Test new tiles in existing dungeons

### Phase 2.2: Basic Generators (2-3 weeks)
1. ✅ Implement Plains generator (simplest)
2. ✅ Implement Forest generator (cellular automata)
3. ✅ Implement Mountain generator (caves)
4. ✅ Test each generator independently

### Phase 2.3: Advanced Generators (2-3 weeks)
1. ✅ Implement Hills generator (elevation-based)
2. ✅ Implement Swamp generator
3. ✅ Implement River generator
4. ✅ Add helper functions (smoothing, connectivity, etc.)

### Phase 2.4: Integration (1-2 weeks)
1. ✅ Connect overmap to terrain generators
2. ✅ Add transition system
3. ✅ Implement position-based seeding
4. ✅ Add "return to overmap" functionality

### Phase 2.5: Polish (1-2 weeks)
1. ✅ Terrain-specific gameplay effects
2. ✅ Special features (shrines, campfires, etc.)
3. ✅ Monsters appropriate to terrain
4. ✅ Items/loot appropriate to terrain

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_plains_generation() {
    let mut rng = StdRng::seed_from_u64(12345);
    let map = generate_plains_map(50, 50, &mut rng);

    // Mostly grass
    let grass_count = count_tiles(&map, Tile::Grass);
    assert!(grass_count > 2000); // > 80%

    // Some features
    let feature_count = count_tiles(&map, Tile::Bush) +
                       count_tiles(&map, Tile::Boulder);
    assert!(feature_count > 3);
}

#[test]
fn test_forest_has_clearings() {
    let mut rng = StdRng::seed_from_u64(12345);
    let map = generate_forest_map(50, 50, &mut rng);

    // Should have traversable paths
    assert!(is_fully_connected(&map, Tile::Grass));
}

#[test]
fn test_mountain_caves_connected() {
    let mut rng = StdRng::seed_from_u64(12345);
    let map = generate_mountain_map(50, 50, &mut rng);

    // All cave floors should be reachable
    assert!(is_fully_connected(&map, Tile::CaveFloor));
}
```

### Integration Tests
```rust
#[test]
fn test_terrain_transition() {
    let mut harness = GameTestHarness::new(12345);

    // Start in overmap mode
    harness.press_key(KeyCode::Tab);

    // Find plains terrain
    let plains_pos = harness.find_terrain(TerrainType::Plains);
    harness.navigate_to(plains_pos);

    // Enter terrain
    harness.press_key(KeyCode::Enter);

    // Should be in plains map
    assert!(!harness.is_in_overmap_mode());
    let map = harness.get_current_map();
    assert!(has_terrain_tiles(&map, &[Tile::Grass, Tile::ShortGrass]));
}
```

## Benefits

### Gameplay
1. **Immersion**: World feels more cohesive and realistic
2. **Variety**: Each journey is unique based on terrain
3. **Strategy**: Different terrains require different approaches
4. **Exploration**: Motivation to explore different biomes

### Technical
1. **Modularity**: Each generator is independent
2. **Extensibility**: Easy to add new terrain types
3. **Reusability**: Generators can combine features
4. **Testing**: Each generator can be tested in isolation

## Future Enhancements

### Phase 3+
- **Weather Effects**: Rain makes mud, snow creates ice
- **Seasons**: Forest changes colors, rivers freeze
- **Biome Blending**: Transition zones between terrains
- **Verticality**: Multi-level caves, tree canopy layers
- **Dynamic**: Terrain changes based on player actions
- **Ecology**: Animals/monsters appropriate to biome

## Conclusion

**Yes, this is definitely planned!**

The terrain-based local map system is a natural evolution that will:
- Make the world feel alive and interconnected
- Provide meaningful variety in gameplay
- Leverage the existing terrain system
- Be implemented incrementally in Phase 2

Current status: **Not implemented yet**, but the foundation exists and the plan is clear.

Estimated effort: **8-12 weeks** for full implementation of all terrain types with polish.

**Would you like to prioritize this feature for early Phase 2 implementation?**
