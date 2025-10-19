/// Main menu UI - Continue/New Game/Quit

use ratatui::{
    Frame,
    layout::{Rect, Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Color, Style, Modifier},
    text::{Line, Span},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuOption {
    Continue,
    NewGame,
    Quit,
}

impl MenuOption {
    pub fn all() -> &'static [MenuOption] {
        &[MenuOption::Continue, MenuOption::NewGame, MenuOption::Quit]
    }

    pub fn name(&self) -> &'static str {
        match self {
            MenuOption::Continue => "Continue Game",
            MenuOption::NewGame => "New Game",
            MenuOption::Quit => "Quit",
        }
    }

    pub fn next(&self) -> MenuOption {
        match self {
            MenuOption::Continue => MenuOption::NewGame,
            MenuOption::NewGame => MenuOption::Quit,
            MenuOption::Quit => MenuOption::Continue,
        }
    }

    pub fn prev(&self) -> MenuOption {
        match self {
            MenuOption::Continue => MenuOption::Quit,
            MenuOption::NewGame => MenuOption::Continue,
            MenuOption::Quit => MenuOption::NewGame,
        }
    }
}

pub fn render_main_menu(
    frame: &mut Frame,
    selected: MenuOption,
    save_exists: bool,
) {
    let area = frame.area();

    // Create centered layout
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),  // Top padding
            Constraint::Min(15),         // Menu content
            Constraint::Percentage(30),  // Bottom padding
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Left padding
            Constraint::Percentage(50),  // Menu content
            Constraint::Percentage(25),  // Right padding
        ])
        .split(vertical_chunks[1]);

    let menu_area = horizontal_chunks[1];

    // Build menu content
    let mut lines = vec![];

    // Title
    lines.push(Line::from(""));
    lines.push(Line::from(
        Span::styled(
            "DUNGEON CLAWLER",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        )
    ));
    lines.push(Line::from(
        Span::styled(
            "A Roguelike Adventure",
            Style::default().fg(Color::Gray)
        )
    ));
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Menu options
    for option in MenuOption::all() {
        // Disable "Continue" if no save exists
        let is_disabled = *option == MenuOption::Continue && !save_exists;
        let is_selected = *option == selected;

        let style = if is_disabled {
            Style::default().fg(Color::DarkGray)
        } else if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if is_selected { "> " } else { "  " };
        let suffix = if is_disabled { " (no save found)" } else { "" };

        lines.push(Line::from(
            Span::styled(
                format!("{}{}{}", prefix, option.name(), suffix),
                style
            )
        ));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(
        Span::styled(
            "Use ↑/↓ or k/j to select, Enter to confirm",
            Style::default().fg(Color::DarkGray)
        )
    ));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, menu_area);
}
