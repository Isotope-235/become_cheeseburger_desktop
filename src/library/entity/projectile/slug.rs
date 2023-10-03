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
    pub fn new(pos: Vector2, vel: Vector2) -> Pos<Slug> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for Slug {
    fn default() -> Self {
        Slug { hp: 7.00 }
    }
}
impl OnHit for Pos<Slug> {

    fn self_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }
    #[allow(unused_variables)]
    fn effect_on_hit(&self, asset_manager: &AssetLoader) -> StateEffect {
        StateEffect { burger_damage: self.bhv.hp, ..Default::default() }
    }
}
impl TakeEffect for Pos<Slug> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage
    }
}
