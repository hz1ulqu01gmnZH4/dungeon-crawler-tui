use crate::ecs::{resources::Resources, RunMode, Position};
use crate::systems;
use crate::ui;
use crate::save::{quick_save, quick_load};
use hecs::World;
use ratatui::{backend::Backend, Terminal};

fn update_camera(world: &World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player_entity {
        if let Ok(pos) = world.get::<&Position>(player_entity) {
            resources.camera.center_on(pos.x, pos.y);
        }
    }
}

pub struct App {
    pub world: World,
    pub resources: Resources,
    pub seed: u64,
}

impl App {
    pub fn new(world: World, resources: Resources, seed: u64) -> Self {
        Self { world, resources, seed }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
        loop {
            // Handle input
            let should_continue = systems::handle_input(&mut self.world, &mut self.resources)?;
            if !should_continue {
                break;
            }

            // Run game logic based on mode
            match self.resources.mode {
                RunMode::AwaitingInput => {
                    // Just waiting for input, nothing to do
                }
                RunMode::PlayerTurn => {
                    self.run_player_turn();
                    self.resources.mode = RunMode::MonstersTurn;
                }
                RunMode::MonstersTurn => {
                    self.run_monster_turn();
                    self.resources.mode = RunMode::AwaitingInput;
                }
                RunMode::GameOver => {
                    // Stay in game over state
                }
            }

            // Render
            if self.resources.mode == RunMode::GameOver {
                terminal.draw(|f| render_game_over(f))?;
            } else {
                terminal.draw(|f| ui::render(f, &self.world, &self.resources))?;
            }

            // Small delay to prevent CPU spinning
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        Ok(())
    }

    fn run_player_turn(&mut self) {
        systems::movement_system(&mut self.world, &mut self.resources);
        update_camera(&self.world, &mut self.resources);

        // Inventory systems
        systems::pickup_system(&mut self.world, &mut self.resources);
        systems::drop_system(&mut self.world, &mut self.resources);
        systems::use_item_system(&mut self.world, &mut self.resources);
        systems::equip_system(&mut self.world, &mut self.resources);
        systems::unequip_system(&mut self.world, &mut self.resources);

        systems::melee_combat_system(&mut self.world, &mut self.resources);
        systems::death_system(&mut self.world, &mut self.resources);
        systems::update_fov(&mut self.world, &mut self.resources);
    }

    fn run_monster_turn(&mut self) {
        systems::monster_ai_system(&mut self.world, &self.resources);
        systems::movement_system(&mut self.world, &mut self.resources);
        systems::melee_combat_system(&mut self.world, &mut self.resources);
        systems::death_system(&mut self.world, &mut self.resources);
        systems::update_fov(&mut self.world, &mut self.resources);
    }

    pub fn save_game(&mut self) -> anyhow::Result<()> {
        quick_save(&self.world, &self.resources, self.seed)?;
        self.resources.log.add("Game saved.");
        Ok(())
    }

    pub fn load_game(&mut self) -> anyhow::Result<()> {
        self.seed = quick_load(&mut self.world, &mut self.resources)?;
        update_camera(&self.world, &mut self.resources);
        self.resources.log.add("Game loaded.");
        Ok(())
    }
}

fn render_game_over(frame: &mut ratatui::Frame) {
    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, Paragraph},
    };

    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(area);

    let game_over_text = Paragraph::new("GAME OVER\n\nPress 'q' to quit")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(game_over_text, chunks[1]);
}
