use crate::*;
pub struct Cheese {
    pub pos: Vector2,
    pub hp: f64,
    pub next_pos: Vector2,
}
impl Cheese {
    pub fn new(pos: Vector2, next_pos: Vector2) -> Self {
        Self {
            pos,
            hp: 1.00,
            next_pos,
        }
    }
}

pub fn create_next_pos(last_pos: Vector2) -> Vector2 {
    let Vector2(x, y) = CENTER;
    loop {
        let maybe_pos = Vector2(rand(x), rand(y)) + CENTER * 0.50;
        if (last_pos - maybe_pos).len() > 24.00 {
            return maybe_pos;
        }
    }
}
