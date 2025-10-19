/// Inventory UI renderer
///
/// Displays player inventory, equipment, and item details

use crate::ecs::{CombatStats, EquipSlot, Inventory, ItemData, Equipable, Consumable, Stackable};
use crate::ecs::resources::Resources;
use hecs::World;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_inventory(frame: &mut Frame, world: &World, resources: &Resources) {
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

    // Main content - split into 3 columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),  // Item list
            Constraint::Percentage(30),  // Equipment
            Constraint::Percentage(30),  // Item details
        ])
        .split(chunks[1]);

    // Get player inventory
    if let Some(player_entity) = resources.player_entity {
        if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
            render_item_list(frame, world, &*inventory, resources.inventory_selection, main_chunks[0]);
            render_equipment(frame, world, &*inventory, main_chunks[1]);
            render_item_details(frame, world, &*inventory, resources.inventory_selection, main_chunks[2]);
        }

        // Also show player stats in equipment panel
        if let Ok(stats) = world.get::<&CombatStats>(player_entity) {
            render_player_stats(frame, &*stats, main_chunks[1]);
        }
    }

    // Help text
    render_help(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Inventory")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_item_list(frame: &mut Frame, world: &World, inventory: &Inventory, selected: usize, area: Rect) {
    let mut items = Vec::new();

    // List all items with letter labels (a-z)
    for (idx, &item_entity) in inventory.items.iter().enumerate() {
        let letter = (b'a' + idx as u8) as char;

        let item_name = if let Ok(item_data) = world.get::<&ItemData>(item_entity) {
            item_data.name.clone()
        } else {
            "Unknown Item".to_string()
        };

        // Check for stack quantity
        let stack_info = if let Ok(stackable) = world.get::<&Stackable>(item_entity) {
            if stackable.quantity > 1 {
                format!(" x{}", stackable.quantity)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Check if equipped
        let is_equipped = inventory.equipped.values().any(|&e| e == item_entity);
        let equipped_marker = if is_equipped { " [E]" } else { "" };

        let style = if idx == selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let line = format!(" {} - {}{}{}", letter, item_name, stack_info, equipped_marker);
        items.push(ListItem::new(line).style(style));
    }

    if items.is_empty() {
        items.push(ListItem::new(" (empty)").style(Style::default().fg(Color::Gray)));
    }

    let item_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" Items ({}/{}) ", inventory.items.len(), inventory.capacity)));

    frame.render_widget(item_list, area);
}

fn render_equipment(frame: &mut Frame, world: &World, inventory: &Inventory, area: Rect) {
    let mut lines = Vec::new();

    // Show all equipment slots
    for slot in EquipSlot::all() {
        let equipped_name = if let Some(item_entity) = inventory.get_equipped(slot) {
            if let Ok(item_data) = world.get::<&ItemData>(item_entity) {
                item_data.name.clone()
            } else {
                "Unknown".to_string()
            }
        } else {
            "(empty)".to_string()
        };

        let style = if inventory.get_equipped(slot).is_some() {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{}: ", slot.name()), Style::default().fg(Color::Cyan)),
            Span::styled(equipped_name, style),
        ]));
    }

    let equipment = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Equipment "))
        .wrap(Wrap { trim: true });

    frame.render_widget(equipment, area);
}

fn render_player_stats(frame: &mut Frame, stats: &CombatStats, area: Rect) {
    // This would overlay or append to equipment section
    // For now, we'll show it at the bottom of the equipment area
    // In a more complete implementation, we'd split the area further
}

fn render_item_details(frame: &mut Frame, world: &World, inventory: &Inventory, selected: usize, area: Rect) {
    let mut lines = Vec::new();

    if selected < inventory.items.len() {
        let item_entity = inventory.items[selected];

        // Item name
        if let Ok(item_data) = world.get::<&ItemData>(item_entity) {
            lines.push(Line::from(Span::styled(
                item_data.name.clone(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            // Short description
            lines.push(Line::from(item_data.description.clone()));
            lines.push(Line::from(""));

            // Detailed description (if available)
            if !item_data.detailed_description.is_empty() {
                for line in item_data.detailed_description.split("\\n") {
                    lines.push(Line::from(line.to_string()));
                }
                lines.push(Line::from(""));
            }

            // Category
            lines.push(Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::Gray)),
                Span::raw(item_data.category.name()),
            ]));

            // Stack info
            if let Ok(stackable) = world.get::<&Stackable>(item_entity) {
                lines.push(Line::from(vec![
                    Span::styled("Quantity: ", Style::default().fg(Color::Gray)),
                    Span::raw(format!("{}/{}", stackable.quantity, item_data.max_stack)),
                ]));
            }

            // Weight and value
            lines.push(Line::from(vec![
                Span::styled("Weight: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{}", item_data.weight)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Value: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{} gold", item_data.value)),
            ]));
            lines.push(Line::from(""));
        }

        // Equipment stats
        if let Ok(equipable) = world.get::<&Equipable>(item_entity) {
            lines.push(Line::from(vec![
                Span::styled("Slot: ", Style::default().fg(Color::Cyan)),
                Span::raw(equipable.slot.name()),
            ]));

            if equipable.power_bonus != 0 {
                lines.push(Line::from(vec![
                    Span::styled("Power: ", Style::default().fg(Color::Green)),
                    Span::raw(format!("{:+}", equipable.power_bonus)),
                ]));
            }

            if equipable.defense_bonus != 0 {
                lines.push(Line::from(vec![
                    Span::styled("Defense: ", Style::default().fg(Color::Blue)),
                    Span::raw(format!("{:+}", equipable.defense_bonus)),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Consumable info
        if let Ok(consumable) = world.get::<&Consumable>(item_entity) {
            lines.push(Line::from(vec![
                Span::styled("HP Restore: ", Style::default().fg(Color::Red)),
                Span::raw(format!("{:+}", consumable.hp_restore)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Uses: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{}", consumable.uses)),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No item selected",
            Style::default().fg(Color::Gray),
        )));
    }

    let details = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Details "))
        .wrap(Wrap { trim: true });

    frame.render_widget(details, area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Select | "),
        Span::styled("e", Style::default().fg(Color::Yellow)),
        Span::raw(": Equip/Unequip | "),
        Span::styled("d", Style::default().fg(Color::Yellow)),
        Span::raw(": Drop | "),
        Span::styled("u", Style::default().fg(Color::Yellow)),
        Span::raw(": Use | "),
        Span::styled("i/ESC", Style::default().fg(Color::Yellow)),
        Span::raw(": Close"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(help, area);
}
