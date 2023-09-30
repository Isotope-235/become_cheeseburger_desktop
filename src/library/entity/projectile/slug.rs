use crate::*;

pub struct Slug {
    pub hp: f64
}
impl HitBox for Pos<Slug> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 10.00)
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
impl OnHit for Pos<Slug> {
    fn target_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_on_hit(&self) -> Effect {
        self.target_effect_on_hit()
    }
}
impl TakeEffect for Pos<Slug> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage
    }
}
