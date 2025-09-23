use std::io::{self, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};

pub struct Renderer {
    pub width: u16,
    pub height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn clear(&self) -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
    }

    pub fn draw_at(&self, x: u16, y: u16, ch: char) -> io::Result<()> {
        execute!(io::stdout(), MoveTo(x, y))?;
        print!("{}", ch);
        io::stdout().flush()
    }
}