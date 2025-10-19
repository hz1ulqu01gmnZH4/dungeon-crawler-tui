use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dungeon_clawler_tui::{
    ecs::*,
    game::App,
    map::generate_dungeon,
};
use hecs::World;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run game
    let result = run_game(&mut terminal);

    // Terminal cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}

fn run_game<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let seed = 12345u64;
    let (world, resources) = create_game_world(seed);
    let mut app = App::new(world, resources, seed);
    app.run(terminal)
}

fn create_game_world(seed: u64) -> (World, Resources) {
    let mut world = World::new();
    let mut resources = Resources::new(80, 50, seed);

    // Generate dungeon
    let rooms = generate_dungeon(
        resources.maps.active_map_mut(),
        &mut resources.rng,
        30,  // max_rooms
        5,   // min_size
        10,  // max_size
    );

    // Spawn player in first room
    if let Some(first_room) = rooms.first() {
        let (px, py) = first_room.center();
        let player = world.spawn((
            Position::new(px, py, RealityLayer::Normal),
            Renderable {
                glyph: '@',
                fg: ratatui::style::Color::Yellow,
                bg: ratatui::style::Color::Reset,
                z: 10,
            },
            Player,
            Name("Player".to_string()),
            CombatStats::new(30, 5, 2),
            TriMeter::new(),
            Viewshed::new(8),
            BlocksMovement,
            Inventory::default_player(),
        ));

        resources.player_entity = Some(player);
        resources.camera.center_on(px, py);
        resources.log.add("Welcome to the Dungeon Clawler!");
        resources.log.add("Use hjkl or arrow keys to move. Press 'q' to quit.");
        resources.log.add("Press 'g' to pickup items, 'i' for inventory, 'S' to save.");
    }

    // Spawn monsters and items in other rooms
    for room in rooms.iter().skip(1) {
        let (mx, my) = room.center();

        // 50% chance to spawn a monster
        if rand::random::<bool>() {
            let monster_type: i32 = rand::random::<i32>() % 3;
            let (glyph, name, hp, power, defense) = match monster_type {
                0 => ('g', "Goblin", 8, 3, 1),
                1 => ('o', "Orc", 12, 4, 1),
                _ => ('t', "Troll", 16, 5, 2),
            };

            world.spawn((
                Position::new(mx, my, RealityLayer::Normal),
                Renderable {
                    glyph,
                    fg: ratatui::style::Color::Red,
                    bg: ratatui::style::Color::Reset,
                    z: 5,
                },
                Monster,
                Name(name.to_string()),
                CombatStats::new(hp, power, defense),
                Viewshed::new(6),
                BlocksMovement,
            ));
        }

        // Spawn items in room
        let mut room_tiles = Vec::new();
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                room_tiles.push((x, y));
            }
        }
        dungeon_clawler_tui::spawn_items_in_room(&mut world, &room_tiles, &mut resources.rng);
    }

    (world, resources)
}
