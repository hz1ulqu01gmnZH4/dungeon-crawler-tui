pub mod tile;
pub mod generator;
pub mod fov;
pub mod chunks;
pub mod cache;

pub use tile::*;
pub use generator::*;
pub use self::fov::*;
pub use chunks::*;
pub use cache::*;

use crate::ecs::RealityLayer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            tiles: vec![Tile::Wall; size],
            visible: vec![false; size],
            revealed: vec![false; size],
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx].walkable()
    }

    pub fn is_opaque(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return true;
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx].blocks_sight()
    }

    pub fn clear_visible(&mut self) {
        self.visible.iter_mut().for_each(|v| *v = false);
    }

    pub fn set_visible(&mut self, x: i32, y: i32) {
        if self.in_bounds(x, y) {
            let idx = self.xy_idx(x, y);
            self.visible[idx] = true;
            self.revealed[idx] = true;
        }
    }
}

pub struct MapSet {
    pub active: RealityLayer,
    pub normal: Map,
    pub cosmic: Map,
}

impl MapSet {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            active: RealityLayer::Normal,
            normal: Map::new(width, height),
            cosmic: Map::new(width, height),
        }
    }

    pub fn active_map(&self) -> &Map {
        match self.active {
            RealityLayer::Normal => &self.normal,
            RealityLayer::Cosmic => &self.cosmic,
        }
    }

    pub fn active_map_mut(&mut self) -> &mut Map {
        match self.active {
            RealityLayer::Normal => &mut self.normal,
            RealityLayer::Cosmic => &mut self.cosmic,
        }
    }
}
