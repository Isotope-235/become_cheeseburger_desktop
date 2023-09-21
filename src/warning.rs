use crate::*;

pub struct Warning {
    dir: V2,
    delay: f64
}

impl Warning {
    pub fn new(pos: V2, dir: V2, delay: f64) -> Pos<Warning> {
        Pos { pos, bhv: Warning { dir, delay }, ..Pos::default() }
    }
}
impl Default for Warning {
    fn default() -> Self {
        Warning { dir: V2::ZERO, delay: 0.00 }
    }
}
impl Pos<Warning> {
    pub fn should_be_removed(&self) -> bool {
        self.age >= 60.00 + self.bhv.delay
    }
    pub fn dir(&self) -> V2 {
        self.bhv.dir
    }
    pub fn is_visible(&self) -> bool {
        self.age >= self.bhv.delay
    }
}