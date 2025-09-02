pub mod game {
    pub mod state;
    pub mod world;
}

pub mod components {
    pub mod position;
    pub mod stats;
}

pub mod systems {
    pub mod movement;
    pub mod combat;
}

pub mod ui {
    pub mod renderer;
    pub mod input;
}

pub mod generation {
    pub mod dungeon;
}