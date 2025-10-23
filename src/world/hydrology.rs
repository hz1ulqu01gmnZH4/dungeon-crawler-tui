//! Hydrology system for realistic river generation using Priority-Flood algorithm
//!
//! This module implements a physically-based river generation system that:
//! - Fills depressions using Priority-Flood to ensure drainage
//! - Computes flow directions (D8 algorithm)
//! - Accumulates flow with biome-based rainfall/infiltration
//! - Generates connected river networks that drain to lakes/borders

use super::{Overmap, TerrainType, BiomeId};
use noise::NoiseFn;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

/// Priority queue entry for flood-fill algorithm
#[derive(Copy, Clone, Debug)]
struct PriorityCell {
    x: i32,
    y: i32,
    elevation: f64,
}

impl PartialEq for PriorityCell {
    fn eq(&self, other: &Self) -> bool {
        self.elevation == other.elevation && self.x == other.x && self.y == other.y
    }
}

impl Eq for PriorityCell {}

impl PartialOrd for PriorityCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityCell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse comparison (lower elevation = higher priority)
        // Break ties deterministically with coordinates
        other.elevation.partial_cmp(&self.elevation)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

/// D8 flow direction (8 cardinal + diagonal directions)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlowDirection {
    East = 0,
    SouthEast = 1,
    South = 2,
    SouthWest = 3,
    West = 4,
    NorthWest = 5,
    North = 6,
    NorthEast = 7,
    None = 8, // Sink (lake, border, or pit)
}

impl FlowDirection {
    /// Get the (dx, dy) offset for this direction
    pub fn offset(&self) -> (i32, i32) {
        match self {
            FlowDirection::East => (1, 0),
            FlowDirection::SouthEast => (1, 1),
            FlowDirection::South => (0, 1),
            FlowDirection::SouthWest => (-1, 1),
            FlowDirection::West => (-1, 0),
            FlowDirection::NorthWest => (-1, -1),
            FlowDirection::North => (0, -1),
            FlowDirection::NorthEast => (1, -1),
            FlowDirection::None => (0, 0),
        }
    }

    /// Get opposite direction (for upstream checks)
    pub fn opposite(&self) -> FlowDirection {
        match self {
            FlowDirection::East => FlowDirection::West,
            FlowDirection::SouthEast => FlowDirection::NorthWest,
            FlowDirection::South => FlowDirection::North,
            FlowDirection::SouthWest => FlowDirection::NorthEast,
            FlowDirection::West => FlowDirection::East,
            FlowDirection::NorthWest => FlowDirection::SouthEast,
            FlowDirection::North => FlowDirection::South,
            FlowDirection::NorthEast => FlowDirection::SouthWest,
            FlowDirection::None => FlowDirection::None,
        }
    }
}

/// Hydrology generator for creating realistic river networks
pub struct HydrologyGenerator<N: NoiseFn<f64, 2>> {
    width: i32,
    height: i32,
    seed: u64,
    elevation_noise: N,
}

impl<N: NoiseFn<f64, 2>> HydrologyGenerator<N> {
    pub fn new(width: i32, height: i32, seed: u64, elevation_noise: N) -> Self {
        Self {
            width,
            height,
            seed,
            elevation_noise,
        }
    }

    /// Generate river network using Priority-Flood algorithm
    pub fn generate_rivers(&self, overmap: &mut Overmap) {
        println!("  [Hydrology] Step 1: Computing elevation field...");
        let elevation = self.compute_elevation_field();

        println!("  [Hydrology] Step 2: Identifying sinks (lakes + borders)...");
        let sinks = self.identify_sinks(overmap);

        println!("  [Hydrology] Step 3: Priority-Flood fill depressions...");
        let filled_elevation = self.priority_flood_fill(&elevation, &sinks);

        println!("  [Hydrology] Step 4: Computing D8 flow directions...");
        let flow_directions = self.compute_flow_directions(&filled_elevation);

        println!("  [Hydrology] Step 5: Computing biome-based runoff...");
        let runoff = self.compute_runoff(overmap);

        println!("  [Hydrology] Step 6: Accumulating flow...");
        let flow_accumulation = self.accumulate_flow(&flow_directions, &runoff);

        println!("  [Hydrology] Step 7: Selecting river channels...");
        let rivers = self.select_rivers(&flow_accumulation, &flow_directions, &sinks);

        println!("  [Hydrology] Step 8: Applying rivers to overmap...");
        self.apply_rivers_to_overmap(overmap, &rivers);

        println!("  [Hydrology] Complete! Generated {} river tiles", rivers.len());
    }

    /// Compute elevation field from noise (normalized to 0.0-1.0)
    fn compute_elevation_field(&self) -> Vec<f64> {
        let mut elevation = Vec::with_capacity((self.width * self.height) as usize);
        let scale = 0.05;

        for y in 0..self.height {
            for x in 0..self.width {
                let e = self.elevation_noise.get([x as f64 * scale, y as f64 * scale]);
                let normalized = (e + 1.0) / 2.0; // Map from [-1,1] to [0,1]
                elevation.push(normalized);
            }
        }

        elevation
    }

    /// Identify sinks: existing lakes + border cells
    fn identify_sinks(&self, overmap: &Overmap) -> HashSet<usize> {
        let mut sinks = HashSet::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);

                // Border cells are sinks (map edge)
                if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
                    sinks.insert(idx);
                    continue;
                }

                // Lakes are sinks
                if let Some(tile) = overmap.get_tile(x, y) {
                    if tile.terrain == TerrainType::Lake {
                        sinks.insert(idx);
                    }
                }
            }
        }

        sinks
    }

    /// Priority-Flood algorithm to fill depressions while preserving sinks
    ///
    /// Algorithm:
    /// 1. Add all sinks to priority queue at their current elevation
    /// 2. Add all border cells to priority queue
    /// 3. Process cells in order of increasing elevation
    /// 4. For each cell, ensure it's at least as high as its lowest processed neighbor
    /// 5. This guarantees every cell has a downhill path to a sink
    fn priority_flood_fill(&self, elevation: &[f64], sinks: &HashSet<usize>) -> Vec<f64> {
        let mut filled = elevation.to_vec();
        let mut processed = vec![false; (self.width * self.height) as usize];
        let mut queue = BinaryHeap::new();

        // Initialize: Add all sinks to queue
        for &sink_idx in sinks.iter() {
            let (x, y) = self.idx_xy(sink_idx);
            queue.push(PriorityCell {
                x,
                y,
                elevation: elevation[sink_idx],
            });
            processed[sink_idx] = true;
        }

        // Process cells in order of increasing elevation
        while let Some(cell) = queue.pop() {
            let idx = self.xy_idx(cell.x, cell.y);

            // Check all 8 neighbors
            for (nx, ny) in self.get_neighbors_8(cell.x, cell.y) {
                let n_idx = self.xy_idx(nx, ny);

                if processed[n_idx] {
                    continue;
                }

                // Ensure neighbor is at least as high as current cell
                // This fills depressions by raising the elevation
                filled[n_idx] = filled[n_idx].max(cell.elevation);

                queue.push(PriorityCell {
                    x: nx,
                    y: ny,
                    elevation: filled[n_idx],
                });
                processed[n_idx] = true;
            }
        }

        filled
    }

    /// Compute D8 flow directions (each cell flows to steepest downslope neighbor)
    fn compute_flow_directions(&self, elevation: &[f64]) -> Vec<FlowDirection> {
        let mut flow_dirs = vec![FlowDirection::None; (self.width * self.height) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);
                let current_elev = elevation[idx];

                let mut steepest_dir = FlowDirection::None;
                let mut steepest_drop = 0.0;

                // Check all 8 neighbors
                let neighbors = [
                    (FlowDirection::East, (1, 0)),
                    (FlowDirection::SouthEast, (1, 1)),
                    (FlowDirection::South, (0, 1)),
                    (FlowDirection::SouthWest, (-1, 1)),
                    (FlowDirection::West, (-1, 0)),
                    (FlowDirection::NorthWest, (-1, -1)),
                    (FlowDirection::North, (0, -1)),
                    (FlowDirection::NorthEast, (1, -1)),
                ];

                for (dir, (dx, dy)) in neighbors.iter() {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx < 0 || ny < 0 || nx >= self.width || ny >= self.height {
                        continue;
                    }

                    let n_idx = self.xy_idx(nx, ny);
                    let neighbor_elev = elevation[n_idx];
                    let drop = current_elev - neighbor_elev;

                    // Deterministic tie-breaking using hash of position + seed
                    if drop > steepest_drop ||
                       (drop == steepest_drop && self.tie_breaker(x, y, *dir as u8)) {
                        steepest_drop = drop;
                        steepest_dir = *dir;
                    }
                }

                flow_dirs[idx] = steepest_dir;
            }
        }

        flow_dirs
    }

    /// Deterministic tie-breaking for flow direction
    fn tie_breaker(&self, x: i32, y: i32, dir: u8) -> bool {
        // Use simple hash of coordinates + seed + direction
        let hash = (x as u64)
            .wrapping_mul(73856093)
            .wrapping_add((y as u64).wrapping_mul(19349663))
            .wrapping_add(self.seed)
            .wrapping_add(dir as u64);

        (hash % 2) == 0
    }

    /// Compute runoff based on biome rainfall and infiltration
    fn compute_runoff(&self, overmap: &Overmap) -> Vec<f64> {
        let mut runoff = Vec::with_capacity((self.width * self.height) as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = overmap.get_tile(x, y) {
                    let (rainfall, infiltration) = self.biome_hydrology_params(tile.biome);
                    let r = rainfall * (1.0 - infiltration);
                    runoff.push(r);
                } else {
                    runoff.push(0.5); // Default
                }
            }
        }

        runoff
    }

    /// Get rainfall and infiltration parameters for a biome
    fn biome_hydrology_params(&self, biome: BiomeId) -> (f64, f64) {
        // Returns (rainfall, infiltration)
        match biome {
            BiomeId::DeepWilderness => (1.2, 0.3), // High rain, moderate infiltration
            BiomeId::Forest => (1.0, 0.2),          // Good rain, low infiltration
            BiomeId::Plains => (0.8, 0.3),          // Moderate rain, moderate infiltration
            BiomeId::Hills => (0.9, 0.25),          // Good rain, some infiltration
            BiomeId::Mountains => (1.1, 0.2),       // High rain (orographic), low infiltration
            BiomeId::Wetlands => (1.3, 0.1),        // Very high rain, very low infiltration
        }
    }

    /// Accumulate flow using topological ordering
    fn accumulate_flow(&self, flow_dirs: &[FlowDirection], runoff: &[f64]) -> Vec<f64> {
        let mut accumulation = runoff.to_vec();

        // Process cells in reverse topological order (bottom-up from sinks)
        // For simplicity, we'll use multiple passes until convergence
        for _pass in 0..50 {
            let mut changed = false;

            for y in 0..self.height {
                for x in 0..self.width {
                    let idx = self.xy_idx(x, y);
                    let flow_dir = flow_dirs[idx];

                    if flow_dir == FlowDirection::None {
                        continue; // Sink
                    }

                    let (dx, dy) = flow_dir.offset();
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx < 0 || ny < 0 || nx >= self.width || ny >= self.height {
                        continue;
                    }

                    let n_idx = self.xy_idx(nx, ny);
                    let old_accum = accumulation[n_idx];
                    accumulation[n_idx] += accumulation[idx];

                    if (accumulation[n_idx] - old_accum).abs() > 0.001 {
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        accumulation
    }

    /// Select river channels based on flow accumulation threshold
    fn select_rivers(
        &self,
        flow_accum: &[f64],
        flow_dirs: &[FlowDirection],
        sinks: &HashSet<usize>,
    ) -> HashSet<usize> {
        let mut rivers = HashSet::new();

        // Find threshold using binary search to hit target length
        let target_river_tiles = ((self.width * self.height) as f64 * 0.05) as usize; // 5% of map
        let threshold = self.find_accumulation_threshold(flow_accum, target_river_tiles);

        println!("  [Hydrology]   Threshold: {:.2}, Target: {} tiles", threshold, target_river_tiles);

        // Trace paths from high-accumulation cells to sinks
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);

                if flow_accum[idx] >= threshold {
                    // Trace downstream path
                    self.trace_river_path(x, y, flow_dirs, sinks, &mut rivers);
                }
            }
        }

        // Ensure each sink has at least one inlet
        self.ensure_sink_inlets(flow_accum, flow_dirs, sinks, &mut rivers);

        rivers
    }

    /// Binary search for accumulation threshold
    fn find_accumulation_threshold(&self, flow_accum: &[f64], target_tiles: usize) -> f64 {
        let mut sorted: Vec<f64> = flow_accum.iter().copied().collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

        if target_tiles >= sorted.len() {
            return 0.0;
        }

        sorted[target_tiles.min(sorted.len() - 1)]
    }

    /// Trace river path from cell to sink
    fn trace_river_path(
        &self,
        start_x: i32,
        start_y: i32,
        flow_dirs: &[FlowDirection],
        sinks: &HashSet<usize>,
        rivers: &mut HashSet<usize>,
    ) {
        let mut x = start_x;
        let mut y = start_y;
        let max_steps = (self.width * self.height) as usize;

        for _ in 0..max_steps {
            let idx = self.xy_idx(x, y);
            rivers.insert(idx);

            if sinks.contains(&idx) {
                break; // Reached sink
            }

            let flow_dir = flow_dirs[idx];
            if flow_dir == FlowDirection::None {
                break; // No flow (shouldn't happen after flood-fill)
            }

            let (dx, dy) = flow_dir.offset();
            x += dx;
            y += dy;

            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                break; // Out of bounds
            }
        }
    }

    /// Ensure each non-border sink has at least one river flowing into it
    fn ensure_sink_inlets(
        &self,
        flow_accum: &[f64],
        flow_dirs: &[FlowDirection],
        sinks: &HashSet<usize>,
        rivers: &mut HashSet<usize>,
    ) {
        for &sink_idx in sinks.iter() {
            let (sx, sy) = self.idx_xy(sink_idx);

            // Skip border sinks
            if sx == 0 || sy == 0 || sx == self.width - 1 || sy == self.height - 1 {
                continue;
            }

            // Find highest-accumulation neighbor that flows into this sink
            let mut best_inlet: Option<(i32, i32, f64)> = None;

            for (nx, ny) in self.get_neighbors_8(sx, sy) {
                let n_idx = self.xy_idx(nx, ny);
                let flow_dir = flow_dirs[n_idx];

                if flow_dir == FlowDirection::None {
                    continue;
                }

                let (dx, dy) = flow_dir.offset();
                if nx + dx == sx && ny + dy == sy {
                    // This neighbor flows into the sink
                    let accum = flow_accum[n_idx];
                    if best_inlet.is_none() || accum > best_inlet.unwrap().2 {
                        best_inlet = Some((nx, ny, accum));
                    }
                }
            }

            // Trace path from best inlet
            if let Some((inlet_x, inlet_y, _)) = best_inlet {
                self.trace_river_path(inlet_x, inlet_y, flow_dirs, sinks, rivers);
            }
        }
    }

    /// Apply generated rivers to the overmap
    fn apply_rivers_to_overmap(&self, overmap: &mut Overmap, rivers: &HashSet<usize>) {
        for &idx in rivers.iter() {
            let (x, y) = self.idx_xy(idx);

            if let Some(tile) = overmap.get_tile_mut(x, y) {
                // Don't overwrite lakes or settlements
                if !matches!(tile.terrain, TerrainType::Lake | TerrainType::Settlement) {
                    tile.terrain = TerrainType::River;
                }
            }
        }
    }

    /// Get 8 neighbors (cardinal + diagonal)
    fn get_neighbors_8(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::with_capacity(8);

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && ny >= 0 && nx < self.width && ny < self.height {
                    neighbors.push((nx, ny));
                }
            }
        }

        neighbors
    }

    /// Convert (x, y) to flat index
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Convert flat index to (x, y)
    fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let idx = idx as i32;
        (idx % self.width, idx / self.width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use noise::Perlin;

    #[test]
    fn test_priority_flood_fills_depressions() {
        let noise = Perlin::new(12345);
        let hydro = HydrologyGenerator::new(10, 10, 12345, noise);

        // Create elevation with a depression
        let mut elevation = vec![1.0; 100];
        elevation[55] = 0.5; // Depression in middle

        let mut sinks = HashSet::new();
        // Border cells are sinks
        for i in 0..10 {
            sinks.insert(i); // Top row
            sinks.insert(90 + i); // Bottom row
            sinks.insert(i * 10); // Left column
            sinks.insert(i * 10 + 9); // Right column
        }

        let filled = hydro.priority_flood_fill(&elevation, &sinks);

        // Depression should be filled
        assert!(filled[55] >= 1.0);
    }

    #[test]
    fn test_flow_directions_point_downhill() {
        let noise = Perlin::new(54321);
        let hydro = HydrologyGenerator::new(5, 5, 54321, noise);

        // Create simple slope from top-left to bottom-right
        let mut elevation = vec![0.0; 25];
        for y in 0..5 {
            for x in 0..5 {
                elevation[y * 5 + x] = ((x + y) as f64) / 10.0;
            }
        }

        let flow_dirs = hydro.compute_flow_directions(&elevation);

        // Cell (1,1) should flow towards bottom-right
        let flow = flow_dirs[1 * 5 + 1];
        assert!(matches!(flow, FlowDirection::East | FlowDirection::South | FlowDirection::SouthEast));
    }

    #[test]
    fn test_biome_hydrology_params() {
        let noise = Perlin::new(99999);
        let hydro = HydrologyGenerator::new(10, 10, 99999, noise);

        let (rain_forest, inf_forest) = hydro.biome_hydrology_params(BiomeId::Forest);
        let (rain_wetland, inf_wetland) = hydro.biome_hydrology_params(BiomeId::Wetlands);

        // Wetlands should have more rainfall and less infiltration
        assert!(rain_wetland > rain_forest);
        assert!(inf_wetland < inf_forest);
    }
}
