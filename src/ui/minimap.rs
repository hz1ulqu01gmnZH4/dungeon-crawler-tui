use crate::ecs::resources::Resources;
use crate::ecs::Position;
use crate::map::Tile;
use hecs::World;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Minimap size configuration
pub struct MinimapConfig {
    pub width: u16,
    pub height: u16,
    pub enabled: bool,
}

impl Default for MinimapConfig {
    fn default() -> Self {
        Self {
            width: 20,
            height: 15,
            enabled: true,
        }
    }
}

/// Renders a small overview map in the corner
pub struct MinimapRenderer {
    config: MinimapConfig,
}

impl MinimapRenderer {
    pub fn new(config: MinimapConfig) -> Self {
        Self { config }
    }

    /// Render minimap in the specified area
    pub fn render(&self, frame: &mut Frame, world: &World, resources: &Resources, area: Rect) {
        if !self.config.enabled {
            return;
        }

        let map = resources.world.maps.active_map();

        // Calculate the area for the minimap (bottom-right corner by default)
        let minimap_width = self.config.width.min(area.width);
        let minimap_height = self.config.height.min(area.height);

        let minimap_rect = Rect {
            x: area.x + area.width.saturating_sub(minimap_width),
            y: area.y + area.height.saturating_sub(minimap_height),
            width: minimap_width,
            height: minimap_height,
        };

        // Calculate how much of the map each minimap cell represents
        let scale_x = map.width as f32 / (minimap_width.saturating_sub(2) as f32);
        let scale_y = map.height as f32 / (minimap_height.saturating_sub(2) as f32);

        // Find player position
        let mut player_pos = None;
        for (_, pos) in world.query::<&Position>().iter() {
            if pos.layer == resources.world.maps.active {
                player_pos = Some((pos.x, pos.y));
                break;
            }
        }

        // Build minimap display
        let mut lines = Vec::new();
        for screen_y in 0..(minimap_height.saturating_sub(2)) {
            let mut spans = Vec::new();
            for screen_x in 0..(minimap_width.saturating_sub(2)) {
                // Calculate corresponding map position
                let map_x = (screen_x as f32 * scale_x) as i32;
                let map_y = (screen_y as f32 * scale_y) as i32;

                // Check if this is the player's position
                let is_player = if let Some((px, py)) = player_pos {
                    let player_screen_x = (px as f32 / scale_x) as u16;
                    let player_screen_y = (py as f32 / scale_y) as u16;
                    player_screen_x == screen_x && player_screen_y == screen_y
                } else {
                    false
                };

                if is_player {
                    // Show player as bright marker
                    spans.push(Span::styled("@", Style::default().fg(Color::Yellow)));
                } else if map.in_bounds(map_x, map_y) {
                    let idx = map.xy_idx(map_x, map_y);

                    // Only show revealed tiles
                    if map.revealed[idx] {
                        let tile = map.tiles[idx];
                        let (glyph, color) = self.minimap_tile_display(tile, map.visible[idx]);
                        spans.push(Span::styled(glyph.to_string(), Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw(" "));
                    }
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" Map "))
            .style(Style::default().fg(Color::White));

        frame.render_widget(paragraph, minimap_rect);
    }

    /// Get display character and color for minimap tiles
    fn minimap_tile_display(&self, tile: Tile, visible: bool) -> (char, Color) {
        let base_color = match tile {
            Tile::Floor => Color::DarkGray,
            Tile::Wall => Color::White,
            Tile::ClosedDoor => Color::Rgb(139, 69, 19), // Brown
            Tile::OpenDoor => Color::Rgb(180, 140, 50), // Lighter brown
            Tile::StairsUp => Color::Cyan,
            Tile::StairsDown => Color::Magenta,
        };

        let glyph = match tile {
            Tile::Floor => '·',
            Tile::Wall => '█',
            Tile::ClosedDoor => '+',
            Tile::OpenDoor => '/',
            Tile::StairsUp => '<',
            Tile::StairsDown => '>',
        };

        let color = if visible {
            base_color
        } else {
            // Dim tiles that aren't currently visible
            match tile {
                Tile::Floor => Color::Rgb(40, 40, 40),
                Tile::Wall => Color::Gray,
                Tile::ClosedDoor => Color::Rgb(80, 50, 30),
                Tile::OpenDoor => Color::Rgb(100, 80, 40),
                Tile::StairsUp => Color::Rgb(60, 80, 80),
                Tile::StairsDown => Color::Rgb(80, 60, 80),
            }
        };

        (glyph, color)
    }
}

/// Calculate minimap position for bottom-right corner
pub fn minimap_area(parent: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: parent.x + parent.width.saturating_sub(width + 1),
        y: parent.y + parent.height.saturating_sub(height + 1),
        width: width.min(parent.width),
        height: height.min(parent.height),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimap_config_default() {
        let config = MinimapConfig::default();
        assert_eq!(config.width, 20);
        assert_eq!(config.height, 15);
        assert!(config.enabled);
    }

    #[test]
    fn test_minimap_renderer_creation() {
        let config = MinimapConfig::default();
        let renderer = MinimapRenderer::new(config);
        assert!(renderer.config.enabled);
    }

    #[test]
    fn test_minimap_area_calculation() {
        let parent = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 50,
        };

        let minimap = minimap_area(parent, 20, 15);

        // Should be in bottom-right
        assert!(minimap.x > parent.x);
        assert!(minimap.y > parent.y);
        assert_eq!(minimap.width, 20);
        assert_eq!(minimap.height, 15);
    }

    #[test]
    fn test_minimap_tile_display() {
        let config = MinimapConfig::default();
        let renderer = MinimapRenderer::new(config);

        let (glyph, _) = renderer.minimap_tile_display(Tile::Floor, true);
        assert_eq!(glyph, '·');

        let (glyph, _) = renderer.minimap_tile_display(Tile::Wall, true);
        assert_eq!(glyph, '█');

        let (glyph, _) = renderer.minimap_tile_display(Tile::ClosedDoor, true);
        assert_eq!(glyph, '+');

        let (glyph, _) = renderer.minimap_tile_display(Tile::OpenDoor, true);
        assert_eq!(glyph, '/');

        // Test that invisible tiles have different colors
        let (_, visible_color) = renderer.minimap_tile_display(Tile::Wall, true);
        let (_, invisible_color) = renderer.minimap_tile_display(Tile::Wall, false);
        assert_ne!(visible_color, invisible_color, "Visible and invisible tiles should have different colors");
    }

    #[test]
    fn test_minimap_disabled() {
        let mut config = MinimapConfig::default();
        config.enabled = false;

        let renderer = MinimapRenderer::new(config);
        assert!(!renderer.config.enabled);
    }
}
