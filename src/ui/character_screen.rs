/// Character screen renderer
///
/// Displays player stats, equipment, and TriMeter values

use crate::ecs::{CombatStats, EquipSlot, Inventory, ItemData, Equipable, TriMeter, Name};
use crate::ecs::resources::Resources;
use hecs::World;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_character_screen(frame: &mut Frame, world: &World, resources: &Resources) {
    let area = frame.area();

    // Split into main sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),     // Main content
            Constraint::Length(3),   // Help
        ])
        .split(area);

    // Title
    render_title(frame, chunks[0]);

    // Main content - split into 2 columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // Stats + TriMeter
            Constraint::Percentage(50),  // Equipment
        ])
        .split(chunks[1]);

    // Get player data
    if let Some(player_entity) = resources.player_entity {
        // Left column: Stats and TriMeter
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),  // Stats
                Constraint::Percentage(40),  // TriMeter
            ])
            .split(main_chunks[0]);

        if let Ok(stats) = world.get::<&CombatStats>(player_entity) {
            render_stats(frame, &*stats, left_chunks[0]);
        }

        if let Ok(trimeter) = world.get::<&TriMeter>(player_entity) {
            render_trimeter(frame, &*trimeter, left_chunks[1]);
        }

        // Right column: Equipment
        if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
            render_equipment_full(frame, world, &*inventory, main_chunks[1]);
        }
    }

    // Help text
    render_help(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Character")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_stats(frame: &mut Frame, stats: &CombatStats, area: Rect) {
    let mut lines = Vec::new();

    // HP
    let hp_percent = (stats.hp as f32 / stats.max_hp as f32 * 100.0) as i32;
    let hp_color = if hp_percent > 75 {
        Color::Green
    } else if hp_percent > 25 {
        Color::Yellow
    } else {
        Color::Red
    };

    lines.push(Line::from(vec![
        Span::styled("HP: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}/{}", stats.hp, stats.max_hp),
            Style::default().fg(hp_color).add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));

    // Power
    lines.push(Line::from(vec![
        Span::styled("Power: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", stats.power),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
    ]));

    // Defense
    lines.push(Line::from(vec![
        Span::styled("Defense: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", stats.defense),
            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
        ),
    ]));

    let stats_widget = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Stats "))
        .wrap(Wrap { trim: true });

    frame.render_widget(stats_widget, area);
}

fn render_trimeter(frame: &mut Frame, trimeter: &TriMeter, area: Rect) {
    let mut lines = Vec::new();

    // Insight
    lines.push(Line::from(vec![
        Span::styled("Insight: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", trimeter.insight),
            Style::default().fg(Color::Cyan),
        ),
    ]));

    // Sanity
    let sanity_color = if trimeter.sanity > 70 {
        Color::Green
    } else if trimeter.sanity > 30 {
        Color::Yellow
    } else {
        Color::Red
    };

    lines.push(Line::from(vec![
        Span::styled("Sanity: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", trimeter.sanity),
            Style::default().fg(sanity_color),
        ),
    ]));

    // Notice
    lines.push(Line::from(vec![
        Span::styled("Notice: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", trimeter.notice),
            Style::default().fg(Color::Magenta),
        ),
    ]));

    let trimeter_widget = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" TriMeter "))
        .wrap(Wrap { trim: true });

    frame.render_widget(trimeter_widget, area);
}

fn render_equipment_full(frame: &mut Frame, world: &World, inventory: &Inventory, area: Rect) {
    let mut lines = Vec::new();

    // Show all equipment slots
    for slot in EquipSlot::all() {
        let (equipped_name, bonuses) = if let Some(item_entity) = inventory.get_equipped(slot) {
            let name = if let Ok(item_data) = world.get::<&ItemData>(item_entity) {
                item_data.name.clone()
            } else {
                "Unknown".to_string()
            };

            let bonus_str = if let Ok(equipable) = world.get::<&Equipable>(item_entity) {
                let mut parts = Vec::new();
                if equipable.power_bonus != 0 {
                    parts.push(format!("+{} Pow", equipable.power_bonus));
                }
                if equipable.defense_bonus != 0 {
                    parts.push(format!("+{} Def", equipable.defense_bonus));
                }
                if parts.is_empty() {
                    String::new()
                } else {
                    format!(" ({})", parts.join(", "))
                }
            } else {
                String::new()
            };

            (name, bonus_str)
        } else {
            ("(empty)".to_string(), String::new())
        };

        let is_equipped = inventory.get_equipped(slot).is_some();
        let name_style = if is_equipped {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{}: ", slot.name()), Style::default().fg(Color::Cyan)),
            Span::styled(equipped_name, name_style),
            Span::styled(bonuses, Style::default().fg(Color::Yellow)),
        ]));
    }

    let equipment = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Equipment "))
        .wrap(Wrap { trim: true });

    frame.render_widget(equipment, area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled("@/ESC", Style::default().fg(Color::Yellow)),
        Span::raw(": Close"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(help, area);
}
