use crate::*;

pub struct Laser {
    pub hp: f64,
}

impl HitBox for Pos<Laser> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 3.00)
    }
}

impl Laser {
    pub fn new(pos: Vector2, vel: Vector2) -> Pos<Laser> {
        Pos { pos, vel, ..Pos::default() }
    }
}

impl Default for Laser {
    fn default() -> Self {
        Laser { hp: 5.00 }
    }
}

impl OnHit for Pos<Laser> {
    fn target_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_on_hit(&self) -> Effect {
        self.target_effect_on_hit()
    }
}

impl TakeEffect for Pos<Laser> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}
