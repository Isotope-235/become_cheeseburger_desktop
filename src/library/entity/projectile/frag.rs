use crate::*;

pub struct Frag {
    hp: f64
}
impl HitBox for Pos<Frag> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 7.00)
    }
}
impl Frag {
    pub fn new(pos: Vector2, vel: Vector2) -> Pos<Frag> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for Frag {
    fn default() -> Self {
        Frag { hp: 5.00 }
    }
}
impl Pos<Frag> {
    pub fn will_live(&self) -> bool {
        self.bhv.hp > 1e-10 && self.age < 200.00
    }
}
impl OnHit for Pos<Frag> {
    fn target_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_on_hit(&self) -> Effect {
        self.target_effect_on_hit()
    }
}
impl TakeEffect for Pos<Frag> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage
    }
}
pub struct FlakChild {
    pub hp: f64
}
impl HitBox for Pos<FlakChild> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 4.00)
    }
}
impl FlakChild {
    pub fn new(pos: Vector2, vel: Vector2, acc: Vector2) -> Pos<FlakChild> {
        Pos { pos, vel, acc, ..Pos::default() }
    }
}
impl Default for FlakChild {
    fn default() -> Self {
        FlakChild { hp: 2.00 }
    }
}
impl OnHit for Pos<FlakChild> {
    fn target_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_on_hit(&self) -> Effect {
        self.target_effect_on_hit()
    }
}
impl TakeEffect for Pos<FlakChild> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage;
    }
}
