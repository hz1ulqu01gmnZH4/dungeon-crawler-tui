use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Debug)]
enum GameInput {
    Move(MoveDirection),
    Quit,
    Pause,
    Resume,
}

#[derive(Debug, Clone, Copy)]
enum MoveDirection {
    North,
    South,
    East,
    West,
}

struct Game {
    state: GameState,
    player_pos: (u16, u16),
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::MainMenu,
            player_pos: (10, 10),
        }
    }

    fn handle_input(&mut self, input: GameInput) {
        match (&self.state, input) {
            (GameState::MainMenu, GameInput::Quit) => self.state = GameState::GameOver,
            (GameState::MainMenu, _) => self.state = GameState::Playing,
            (GameState::Playing, GameInput::Move(dir)) => self.move_player(dir),
            (GameState::Playing, GameInput::Pause) => self.state = GameState::Paused,
            (GameState::Playing, GameInput::Quit) => self.state = GameState::GameOver,
            (GameState::Paused, GameInput::Resume) => self.state = GameState::Playing,
            (GameState::Paused, GameInput::Quit) => self.state = GameState::GameOver,
            _ => {}
        }
    }

    fn move_player(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::North if self.player_pos.1 > 0 => self.player_pos.1 -= 1,
            MoveDirection::South if self.player_pos.1 < 20 => self.player_pos.1 += 1,
            MoveDirection::West if self.player_pos.0 > 0 => self.player_pos.0 -= 1,
            MoveDirection::East if self.player_pos.0 < 40 => self.player_pos.0 += 1,
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut game = Game::new();
    
    let result = run_game_loop(&mut terminal, &mut game);
    
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    result
}

fn run_game_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, game: &mut Game) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(f.area());
            
            let game_area = Block::default()
                .title("Dungeon Clawler")
                .borders(Borders::ALL);
            
            let content = match game.state {
                GameState::MainMenu => "Press any key to start, 'q' to quit".to_string(),
                GameState::Playing => {
                    let mut display = vec![vec!['.'; 41]; 21];
                    display[game.player_pos.1 as usize][game.player_pos.0 as usize] = '@';
                    display.iter()
                        .map(|row| row.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join("\n")
                }
                GameState::Paused => "PAUSED - Press 'p' to resume, 'q' to quit".to_string(),
                GameState::GameOver => "Game Over! Thanks for playing.".to_string(),
            };
            
            let game_widget = Paragraph::new(content)
                .block(game_area)
                .style(Style::default().fg(Color::White));
            
            f.render_widget(game_widget, chunks[0]);
            
            let status = Paragraph::new(format!("State: {:?} | Position: {:?}", game.state, game.player_pos))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));
            
            f.render_widget(status, chunks[1]);
        })?;
        
        if game.state == GameState::GameOver {
            break;
        }
        
        if let Some(input) = handle_input()? {
            game.handle_input(input);
        }
    }
    
    Ok(())
}

fn handle_input() -> Result<Option<GameInput>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            return Ok(Some(match key.code {
                KeyCode::Char('q') => GameInput::Quit,
                KeyCode::Char('p') => GameInput::Pause,
                KeyCode::Char('r') => GameInput::Resume,
                KeyCode::Char('h') | KeyCode::Left => GameInput::Move(MoveDirection::West),
                KeyCode::Char('j') | KeyCode::Down => GameInput::Move(MoveDirection::South),
                KeyCode::Char('k') | KeyCode::Up => GameInput::Move(MoveDirection::North),
                KeyCode::Char('l') | KeyCode::Right => GameInput::Move(MoveDirection::East),
                _ => return Ok(None),
            }));
        }
    }
    Ok(None)
}