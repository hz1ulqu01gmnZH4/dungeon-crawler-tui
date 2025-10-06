use crate::ecs::{Position, Renderable, Player, CombatStats, TriMeter};
use crate::ecs::resources::Resources;
use hecs::World;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, world: &World, resources: &Resources) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),      // Map area
            Constraint::Length(3),    // Status bar
            Constraint::Length(8),    // Message log
        ])
        .split(frame.area());

    render_map(frame, chunks[0], world, resources);
    render_status(frame, chunks[1], world, resources);
    render_log(frame, chunks[2], resources);
}

fn render_map(frame: &mut Frame, area: Rect, world: &World, resources: &Resources) {
    let map = resources.maps.active_map();
    let active_layer = resources.maps.active;

    // Calculate camera bounds
    let cam_x = resources.camera.x;
    let cam_y = resources.camera.y;
    let view_width = area.width.saturating_sub(2) as i32;
    let view_height = area.height.saturating_sub(2) as i32;

    let mut display = vec![vec![' '; view_width as usize]; view_height as usize];
    let mut colors = vec![vec![Color::White; view_width as usize]; view_height as usize];

    // Render tiles
    for screen_y in 0..view_height {
        for screen_x in 0..view_width {
            let world_x = cam_x + screen_x;
            let world_y = cam_y + screen_y;

            if !map.in_bounds(world_x, world_y) {
                continue;
            }

            let idx = map.xy_idx(world_x, world_y);
            let tile = map.tiles[idx];

            if map.visible[idx] {
                display[screen_y as usize][screen_x as usize] = tile.glyph(true);
                colors[screen_y as usize][screen_x as usize] = tile.color(true);
            } else if map.revealed[idx] {
                display[screen_y as usize][screen_x as usize] = tile.glyph(false);
                colors[screen_y as usize][screen_x as usize] = tile.color(false);
            }
        }
    }

    // Collect and sort entities
    let mut renderables = Vec::new();
    for (_, (pos, rend)) in world.query::<(&Position, &Renderable)>().iter() {
        if pos.layer != active_layer {
            continue;
        }

        let screen_x = pos.x - cam_x;
        let screen_y = pos.y - cam_y;

        if screen_x < 0 || screen_x >= view_width || screen_y < 0 || screen_y >= view_height {
            continue;
        }

        // Check if visible
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible[idx] {
            renderables.push((screen_x, screen_y, *rend));
        }
    }

    renderables.sort_by_key(|(_, _, r)| r.z);

    // Render entities
    for (screen_x, screen_y, rend) in renderables {
        display[screen_y as usize][screen_x as usize] = rend.glyph;
        colors[screen_y as usize][screen_x as usize] = rend.fg;
    }

    // Convert to text
    let mut lines = Vec::new();
    for (y, row) in display.iter().enumerate() {
        let mut spans = Vec::new();
        for (x, &ch) in row.iter().enumerate() {
            let color = colors[y][x];
            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        lines.push(Line::from(spans));
    }

    let title = format!(" Dungeon Clawler - Floor 1 [{:?} Layer] ", active_layer);
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(paragraph, area);
}

fn render_status(frame: &mut Frame, area: Rect, world: &World, _resources: &Resources) {
    let mut status_text = String::from("No player found");

    for (entity, (stats, pos, _)) in world.query::<(&CombatStats, &Position, &Player)>().iter() {
        let tri_meter = world.get::<&TriMeter>(entity).ok().map(|t| *t);

        let hp_bar = create_bar(stats.hp, stats.max_hp, 20);
        let mut bars = format!("HP: {} {}/{}", hp_bar, stats.hp, stats.max_hp);

        if let Some(tri) = tri_meter {
            let sanity_bar = create_bar(tri.sanity, 100, 10);
            let insight_bar = create_bar(tri.insight, 100, 10);
            let notice_bar = create_bar(tri.notice, 100, 10);
            bars.push_str(&format!(
                " | Sanity: {} | Insight: {} | Notice: {}",
                sanity_bar, insight_bar, notice_bar
            ));
        }

        status_text = format!("Pos: ({}, {}) | {}", pos.x, pos.y, bars);
        break;
    }

    let paragraph = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title(" Status "))
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(paragraph, area);
}

fn render_log(frame: &mut Frame, area: Rect, resources: &Resources) {
    let messages = resources.log.recent(6);
    let text: Vec<Line> = messages.iter().map(|m| Line::from(m.as_str())).collect();

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Messages "))
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}

fn create_bar(current: i32, max: i32, width: usize) -> String {
    let filled = ((current as f32 / max as f32) * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
