use crate::*;

pub struct Slug {
    pub hp: f64
}
impl Hitbox for Pos<Slug> {
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 10.00)
    }
}
impl Slug {
    pub fn new(pos: V2, vel: V2) -> Pos<Slug> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for Slug {
    fn default() -> Self {
        Slug { hp: 7.00 }
    }
}
impl Onhit for Pos<Slug> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_onhit(&self) -> Effect {
        self.target_effect_onhit()
    }
}
impl TakeEffect for Pos<Slug> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage
    }
}