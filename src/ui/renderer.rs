use crate::ecs::{Position, Renderable, Player, CombatStats, TriMeter};
use crate::ecs::resources::Resources;
use crate::ui::{OvermapRenderer, MinimapRenderer, MinimapConfig};
use crate::world::TimeOfDay;
use hecs::World;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Darken a color based on time of day and weather conditions
fn apply_lighting(color: Color, time_of_day: TimeOfDay, weather_modifier: i32) -> Color {
    // Calculate darkness level (0.0 = pitch black, 1.0 = full brightness)
    let time_brightness = match time_of_day {
        TimeOfDay::Day => 1.0,
        TimeOfDay::Dawn | TimeOfDay::Dusk => 0.7,
        TimeOfDay::Night => 0.4,
        TimeOfDay::DeepNight => 0.3,
    };

    // Weather reduces brightness further (each -1 modifier = -5% brightness)
    let weather_brightness = 1.0 + (weather_modifier as f32 * 0.05);
    let total_brightness = (time_brightness * weather_brightness).max(0.2); // Minimum 20% visibility

    match color {
        Color::Rgb(r, g, b) => {
            let r_dark = (r as f32 * total_brightness) as u8;
            let g_dark = (g as f32 * total_brightness) as u8;
            let b_dark = (b as f32 * total_brightness) as u8;
            Color::Rgb(r_dark, g_dark, b_dark)
        }
        Color::Black => Color::Black,
        Color::DarkGray => {
            if total_brightness < 0.5 { Color::Black } else { Color::DarkGray }
        }
        Color::Gray => {
            if total_brightness < 0.4 { Color::Black }
            else if total_brightness < 0.7 { Color::DarkGray }
            else { Color::Gray }
        }
        Color::White => {
            if total_brightness < 0.4 { Color::DarkGray }
            else if total_brightness < 0.7 { Color::Gray }
            else { Color::White }
        }
        Color::Red => {
            if total_brightness < 0.5 { Color::DarkGray } else { Color::Red }
        }
        Color::Green => {
            if total_brightness < 0.5 { Color::DarkGray } else { Color::Green }
        }
        Color::Yellow => {
            if total_brightness < 0.5 { Color::Gray } else { Color::Yellow }
        }
        Color::Blue => {
            if total_brightness < 0.5 { Color::DarkGray } else { Color::Blue }
        }
        Color::Magenta => {
            if total_brightness < 0.5 { Color::DarkGray } else { Color::Magenta }
        }
        Color::Cyan => {
            if total_brightness < 0.5 { Color::DarkGray } else { Color::Cyan }
        }
        // Leave other colors unchanged
        _ => color,
    }
}

pub fn render(frame: &mut Frame, world: &World, resources: &Resources) {
    // If in main menu, show main menu
    if resources.in_main_menu {
        let save_exists = std::path::Path::new("savegame.json").exists();
        crate::ui::render_main_menu(frame, resources.menu_selection, save_exists);
        return;
    }

    // If in character screen mode, show character screen UI
    if resources.in_character_screen {
        crate::ui::render_character_screen(frame, world, resources);
        return;
    }

    // If in inventory mode, show inventory UI
    if resources.in_inventory_mode {
        crate::ui::render_inventory(frame, world, resources);
        return;
    }

    // If in overmap mode, show overmap instead
    if resources.in_overmap_mode {
        let renderer = OvermapRenderer::new(80, 40);
        renderer.render(frame, &resources.overmap, &resources.settlements, &resources.roads, resources.player_overmap_pos, frame.area());
        return;
    }

    // If in examine mode, show map + examine info panel
    if resources.in_examine_mode {
        crate::ui::render_examine_mode(frame, world, resources);
        // Continue to render map and status, but replace message log with examine info
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),      // Map area
            Constraint::Length(3),    // Status bar
            Constraint::Length(8),    // Message log (or examine info in examine mode)
        ])
        .split(frame.area());

    render_map(frame, chunks[0], world, resources);
    render_status(frame, chunks[1], world, resources);
    render_log(frame, chunks[2], resources);
}

fn render_map(frame: &mut Frame, area: Rect, world: &World, resources: &Resources) {
    // Get map based on current depth
    let map = if resources.current_depth > 0 {
        // In dungeon - get from dungeon_levels
        if let Some(dungeon_map) = resources.dungeon_levels.get(&resources.current_depth) {
            dungeon_map
        } else {
            // Fallback to active map if dungeon level doesn't exist
            resources.maps.active_map()
        }
    } else if let Some(location_id) = resources.current_location {
        // In settlement - get from settlement_maps
        if let Some(settlement_map) = resources.settlement_maps.get(&location_id) {
            settlement_map
        } else {
            resources.maps.active_map()
        }
    } else {
        // On surface/wilderness - use active map
        resources.maps.active_map()
    };

    let active_layer = resources.maps.active;

    // Get lighting conditions
    let time_of_day = resources.world_time.time_of_day();
    let weather_modifier = resources.weather.current_weather.visibility_modifier();

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
                let base_color = tile.color(true);
                colors[screen_y as usize][screen_x as usize] = apply_lighting(base_color, time_of_day, weather_modifier);
            } else if map.revealed[idx] {
                display[screen_y as usize][screen_x as usize] = tile.glyph(false);
                let base_color = tile.color(false);
                // Revealed tiles are even darker (only 50% of lighting effect)
                let revealed_modifier = weather_modifier + (4 - (time_of_day as i32));
                colors[screen_y as usize][screen_x as usize] = apply_lighting(base_color, time_of_day, revealed_modifier);
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

    // Render entities with lighting
    for (screen_x, screen_y, rend) in renderables {
        display[screen_y as usize][screen_x as usize] = rend.glyph;
        colors[screen_y as usize][screen_x as usize] = apply_lighting(rend.fg, time_of_day, weather_modifier);
    }

    // Render examine cursor if in examine mode
    if resources.in_examine_mode {
        let (cx, cy) = resources.examine_cursor;
        let screen_x = cx - cam_x;
        let screen_y = cy - cam_y;

        // Check if cursor is within screen bounds
        if screen_x >= 0 && screen_x < view_width && screen_y >= 0 && screen_y < view_height {
            // Yellow background for cursor position
            colors[screen_y as usize][screen_x as usize] = Color::Yellow;
        }
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

    // Build title with location info and depth
    let title = if resources.current_depth > 0 {
        // In dungeon
        format!(" Dungeon - Depth {} [{:?} Layer] ", resources.current_depth, active_layer)
    } else if let Some(loc_id) = resources.current_location {
        // In settlement
        if let Some(settlement) = resources.settlements.iter().find(|s| s.id == loc_id) {
            format!(" {} ({}) [{:?} Layer] ", settlement.name, settlement.settlement_type.name(), active_layer)
        } else {
            format!(" Dungeon Clawler - Surface [{:?} Layer] ", active_layer)
        }
    } else {
        // On wilderness surface
        format!(" Dungeon Clawler - Surface [{:?} Layer] ", active_layer)
    };

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(paragraph, area);

    // Render minimap overlay
    let minimap_config = MinimapConfig::default();
    let minimap = MinimapRenderer::new(minimap_config);
    minimap.render(frame, world, resources, area);
}

fn render_status(frame: &mut Frame, area: Rect, world: &World, resources: &Resources) {
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

        // Add time and weather info
        let time_str = resources.world_time.time_string();
        let tod = resources.world_time.time_of_day().name();
        let weather = resources.weather.current_weather.name();

        status_text = format!("Pos: ({}, {}) | {} | {} | {} | {}",
            pos.x, pos.y, time_str, tod, weather, bars);
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
