/// Examine mode renderer - displays information about tiles and objects
///
/// Press 'x' to enter examine mode, move cursor with hjkl/arrows, press 'x' or ESC to exit

use crate::ecs::{Position, Name, CombatStats, ItemData, Equipable, Consumable, Monster, RealityLayer};
use crate::ecs::resources::Resources;
use crate::map::Tile;
use hecs::World;
use ratatui::{
    Frame,
    layout::{Rect, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Color, Style, Modifier},
};

pub fn render_examine_mode(frame: &mut Frame, world: &World, resources: &Resources) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),       // Map area
            Constraint::Length(12),    // Examine info panel
        ])
        .split(frame.area());

    // Render examine info panel at bottom
    render_examine_info(frame, world, resources, chunks[1]);

    // Cursor is rendered by main renderer overlay
}

fn render_examine_info(frame: &mut Frame, world: &World, resources: &Resources, area: Rect) {
    let (cx, cy) = resources.examine_cursor;

    // Get the map based on current depth
    let map = if resources.current_depth > 0 {
        // In dungeon
        resources.dungeon_levels.get(&resources.current_depth)
    } else if let Some(location_id) = resources.current_location {
        // In settlement
        resources.settlement_maps.get(&location_id)
    } else {
        // On surface/wilderness
        Some(resources.maps.active_map())
    };

    let mut info_lines = Vec::new();

    // Title
    info_lines.push(format!("Examining: ({}, {})", cx, cy));
    info_lines.push(String::new());

    // Terrain info
    if let Some(map) = map {
        let idx = map.xy_idx(cx, cy);
        if idx < map.tiles.len() {
            let tile = &map.tiles[idx];
            let terrain_desc = match tile {
                Tile::Floor => "Stone floor - worn smooth by countless footsteps.",
                Tile::Wall => "Solid stone wall - cold and unyielding.",
                Tile::ClosedDoor => "Closed door - solid oak construction. Press 'o' to open.",
                Tile::OpenDoor => "Open door - the way is clear. Press 'c' to close.",
                Tile::StairsUp => "Stairs leading upward. Press '<' to ascend.",
                Tile::StairsDown => "Stairs leading downward. Press '>' to descend.",
            };

            info_lines.push(format!("Terrain: {}", terrain_desc));

            if map.visible[idx] {
                info_lines.push("Status: Currently visible".to_string());
            } else if map.revealed[idx] {
                info_lines.push("Status: Remembered (not currently visible)".to_string());
            } else {
                info_lines.push("Status: Unexplored".to_string());
            }
        }
    }

    // Check for entities at this position
    let mut found_entities = Vec::new();
    for (entity, (pos, name)) in world.query::<(&Position, &Name)>().iter() {
        if pos.x == cx && pos.y == cy {
            found_entities.push((entity, name.0.clone()));
        }
    }

    if !found_entities.is_empty() {
        info_lines.push(String::new());
        info_lines.push("Entities here:".to_string());

        for (entity, name) in found_entities {
            info_lines.push(format!("  {}", name));

            // Add details for monsters
            if let Ok(stats) = world.get::<&CombatStats>(entity) {
                info_lines.push(format!("    HP: {}/{}", stats.hp, stats.max_hp));
                info_lines.push(format!("    Power: {}, Defense: {}", stats.power, stats.defense));
            }

            // Add details for items
            if let Ok(item_data) = world.get::<&ItemData>(entity) {
                info_lines.push(format!("    {}", item_data.description));

                if let Ok(equipable) = world.get::<&Equipable>(entity) {
                    info_lines.push(format!(
                        "    Equipment: {:?} (+{} Pow, +{} Def)",
                        equipable.slot, equipable.power_bonus, equipable.defense_bonus
                    ));
                }

                if let Ok(consumable) = world.get::<&Consumable>(entity) {
                    if consumable.hp_restore > 0 {
                        info_lines.push(format!("    Heals: {} HP", consumable.hp_restore));
                    }
                }
            }
        }
    }

    // Instructions at bottom
    info_lines.push(String::new());
    info_lines.push("Controls: hjkl/arrows to move cursor, x/ESC to exit".to_string());

    let info_text = info_lines.join("\n");

    let paragraph = Paragraph::new(info_text)
        .block(
            Block::default()
                .title("Examine")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
