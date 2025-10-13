use crate::world::{Overmap, TerrainType};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct OvermapRenderer {
    viewport_width: i32,
    viewport_height: i32,
}

impl OvermapRenderer {
    pub fn new(viewport_width: i32, viewport_height: i32) -> Self {
        Self {
            viewport_width,
            viewport_height,
        }
    }

    /// Render the overmap centered on player position
    pub fn render(
        &self,
        frame: &mut ratatui::Frame,
        overmap: &Overmap,
        player_pos: (i32, i32),
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),        // Map area
                Constraint::Length(5),     // Legend
                Constraint::Length(3),     // Instructions
            ])
            .split(area);

        // Render map
        self.render_map(frame, overmap, player_pos, chunks[0]);

        // Render legend
        self.render_legend(frame, chunks[1]);

        // Render instructions
        self.render_instructions(frame, chunks[2]);
    }

    fn render_map(
        &self,
        frame: &mut ratatui::Frame,
        overmap: &Overmap,
        player_pos: (i32, i32),
        area: Rect,
    ) {
        let block = Block::default()
            .title(" World Map ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Calculate viewport bounds centered on player
        let (player_x, player_y) = player_pos;
        let half_width = (inner_area.width as i32) / 2;
        let half_height = (inner_area.height as i32) / 2;

        let viewport_min_x = player_x - half_width;
        let viewport_min_y = player_y - half_height;
        let viewport_max_x = player_x + half_width;
        let viewport_max_y = player_y + half_height;

        // Render each visible tile
        for screen_y in 0..inner_area.height {
            for screen_x in 0..inner_area.width {
                let world_x = viewport_min_x + screen_x as i32;
                let world_y = viewport_min_y + screen_y as i32;

                let (glyph, color) = if world_x == player_x && world_y == player_y {
                    // Player position
                    ('@', Color::Yellow)
                } else if let Some(tile) = overmap.get_tile(world_x, world_y) {
                    if tile.discovered {
                        let terrain_glyph = tile.terrain.glyph();
                        let terrain_color = Self::terrain_color(&tile.terrain, tile.visited);
                        (terrain_glyph, terrain_color)
                    } else {
                        // Undiscovered tile
                        (' ', Color::Black)
                    }
                } else {
                    // Out of bounds
                    (' ', Color::DarkGray)
                };

                let x = inner_area.x + screen_x;
                let y = inner_area.y + screen_y;
                if x < inner_area.right() && y < inner_area.bottom() {
                    let cell = frame.buffer_mut().get_mut(x, y);
                    cell.set_char(glyph);
                    cell.set_fg(color);
                }
            }
        }
    }

    fn render_legend(&self, frame: &mut ratatui::Frame, area: Rect) {
        let block = Block::default()
            .title(" Legend ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let legend_items = vec![
            Line::from(vec![
                Span::styled("@", Style::default().fg(Color::Yellow)),
                Span::raw(" You  "),
                Span::styled(".", Style::default().fg(Color::Green)),
                Span::raw(" Plains  "),
                Span::styled("t", Style::default().fg(Color::Green)),
                Span::raw(" Forest  "),
                Span::styled("^", Style::default().fg(Color::White)),
                Span::raw(" Mountains  "),
                Span::styled("~", Style::default().fg(Color::Blue)),
                Span::raw(" Water"),
            ]),
            Line::from(vec![
                Span::styled("=", Style::default().fg(Color::Yellow)),
                Span::raw(" Road  "),
                Span::styled("⌂", Style::default().fg(Color::Cyan)),
                Span::raw(" Settlement  "),
                Span::styled("▼", Style::default().fg(Color::Red)),
                Span::raw(" Dungeon  "),
                Span::styled("%", Style::default().fg(Color::Rgb(100, 80, 60))),
                Span::raw(" Swamp"),
            ]),
        ];

        let paragraph = Paragraph::new(legend_items)
            .block(block)
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, area);
    }

    fn render_instructions(&self, frame: &mut ratatui::Frame, area: Rect) {
        let block = Block::default()
            .title(" Controls ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let instructions = vec![Line::from(vec![
            Span::styled(
                "Arrow Keys",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Move  "),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Enter Location  "),
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Toggle Map View  "),
            Span::styled(
                "Q",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Quit"),
        ])];

        let paragraph = Paragraph::new(instructions)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    /// Get color for terrain type
    fn terrain_color(terrain: &TerrainType, visited: bool) -> Color {
        let base_color = match terrain {
            TerrainType::Plains => Color::Green,
            TerrainType::Forest => Color::Rgb(34, 139, 34), // Forest green
            TerrainType::DenseForest => Color::Rgb(0, 100, 0), // Dark green
            TerrainType::Hills => Color::Rgb(139, 119, 101), // Saddle brown
            TerrainType::Mountains => Color::White,
            TerrainType::Lake => Color::Blue,
            TerrainType::River => Color::Cyan,
            TerrainType::Swamp => Color::Rgb(100, 80, 60), // Brown-green
            TerrainType::Road => Color::Yellow,
            TerrainType::KingRoad => Color::Rgb(255, 215, 0), // Gold
            TerrainType::TradePath => Color::Rgb(210, 180, 140), // Tan
            TerrainType::Settlement => Color::Cyan,
            TerrainType::Dungeon => Color::Red,
            TerrainType::SpecialLocation => Color::Magenta,
        };

        // Dim unvisited tiles
        if !visited {
            Self::dim_color(base_color)
        } else {
            base_color
        }
    }

    /// Dim a color for unvisited tiles
    fn dim_color(color: Color) -> Color {
        match color {
            Color::Rgb(r, g, b) => Color::Rgb(r / 2, g / 2, b / 2),
            Color::Green => Color::DarkGray,
            Color::Blue => Color::Rgb(0, 0, 128),
            Color::Cyan => Color::Rgb(0, 128, 128),
            Color::Yellow => Color::Rgb(128, 128, 0),
            Color::White => Color::Gray,
            Color::Red => Color::Rgb(128, 0, 0),
            Color::Magenta => Color::Rgb(128, 0, 128),
            _ => Color::DarkGray,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = OvermapRenderer::new(80, 40);
        assert_eq!(renderer.viewport_width, 80);
        assert_eq!(renderer.viewport_height, 40);
    }

    #[test]
    fn test_terrain_colors() {
        let plains_color = OvermapRenderer::terrain_color(&TerrainType::Plains, true);
        let mountains_color = OvermapRenderer::terrain_color(&TerrainType::Mountains, true);

        // Just verify they return colors (not testing exact values)
        assert!(matches!(plains_color, Color::Green));
        assert!(matches!(mountains_color, Color::White));
    }

    #[test]
    fn test_dim_color() {
        let bright = Color::Rgb(200, 200, 200);
        let dimmed = OvermapRenderer::dim_color(bright);

        if let Color::Rgb(r, g, b) = dimmed {
            assert!(r < 200 && g < 200 && b < 200);
        } else {
            panic!("Expected RGB color");
        }
    }
}
