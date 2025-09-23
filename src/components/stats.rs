pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
}

impl Stats {
    pub fn new(health: i32, attack: i32, defense: i32) -> Self {
        Self {
            health,
            max_health: health,
            attack,
            defense,
        }
    }
}