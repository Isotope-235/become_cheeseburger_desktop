use crate::*;

pub struct Warning {
    dir: Vector2,
    pub delay: f64,
}

impl Warning {
    pub fn new(pos: Vector2, dir: Vector2, delay: f64) -> Pos<Self> {
        Pos {
            pos,
            bhv: Self { dir, delay },
            ..Pos::default()
        }
    }
}
impl Default for Warning {
    fn default() -> Self {
        Self{
            dir: Vector2::ZERO,
            delay: 0.00,
        }
    }
}
impl Pos<Warning> {
    pub fn will_live(&self) -> bool {
        self.age < 60.00 + self.bhv.delay
    }
    pub fn dir(&self) -> Vector2 {
        self.bhv.dir
    }
    pub fn is_visible(&self) -> bool {
        self.age >= self.bhv.delay
    }
}
