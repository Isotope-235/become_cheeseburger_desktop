use crate::*;
pub struct Bullet {
    pub hp: f64,
}
impl HitBox for Pos<Bullet> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 3.00)
    }
}
impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2, extra_lifetime: f64) -> Pos<Self> {
        Pos {
            pos,
            vel,
            acc: Vector2::ZERO,
            age: -extra_lifetime,
            bhv: Self { hp: 3.00 },
        }
    }
}
impl OnHit for Pos<Bullet> {
    fn self_effect_on_hit(&self) -> Effect {
        Effect {
            damage: self.bhv.hp,
        }
    }
    #[allow(unused_variables)]
    fn effect_on_hit(&self, asset_manager: &AssetLoader) -> StateEffect {
        StateEffect {
            burger_damage: self.bhv.hp,
            ..Default::default()
        }
    }
}
impl TakeEffect for Pos<Bullet> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}
