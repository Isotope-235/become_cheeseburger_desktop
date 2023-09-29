use crate::*;

pub struct Laser {
    pub hp: f64
}
impl Hitbox for Pos<Laser> {
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 3.00)
    }
}
impl Laser {
    pub fn new(pos: V2, vel: V2) -> Pos<Laser> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for Laser {
    fn default() -> Self {
        Laser { hp: 5.00 }
    }
}
impl Onhit for Pos<Laser> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_onhit(&self) -> Effect {
        self.target_effect_onhit()
    }
}
impl TakeEffect for Pos<Laser> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}