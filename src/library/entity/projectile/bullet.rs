use crate::*;
pub struct Bullet {
    pub hp: f64
}
impl HitBox for Pos<Bullet>{
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 3.00)
    }
}
impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2, extra_lifetime: f64) -> Pos<Self> {
        Pos { pos, vel, acc: Vector2::ZERO, age: -extra_lifetime, bhv: Bullet { hp: 3.00 } }
    }
}
impl OnHit for Pos<Bullet> {
    fn target_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_on_hit(&self) -> Effect {
        self.target_effect_on_hit()
    }
}
impl TakeEffect for Pos<Bullet> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}
