use crate::*;

pub struct Warning {
    dir: V2
}
impl Behaviour for Warning {}

impl Warning {
    pub fn new(pos: V2, dir: V2) -> Pos<Warning> {
        Pos { pos, bhv: Warning { dir }, ..Pos::default() }
    }
}
impl Default for Warning {
    fn default() -> Self {
        Warning { dir: V2::ZERO }
    }
}
impl Pos<Warning> {
    pub fn should_be_removed(&self) -> bool {
        self.age >= 60.00
    }
    pub fn dir(&self) -> V2 {
        self.bhv.dir
    }
}