use crate::components::position::Position;

pub fn move_entity(pos: &mut Position, dx: i32, dy: i32) {
    pos.x = (pos.x + dx).max(0);
    pos.y = (pos.y + dy).max(0);
}