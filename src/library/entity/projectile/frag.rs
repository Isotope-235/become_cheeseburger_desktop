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

    fn self_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }
    #[allow(unused_variables)]
    fn effect_on_hit(&self, asset_manager: &AssetLoader) -> StateEffect {
        StateEffect { burger_damage: self.bhv.hp, ..Default::default() }
    }
}
impl TakeEffect for Pos<Frag> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage
    }
}
pub struct FragChild {
    pub hp: f64
}
impl HitBox for Pos<FragChild> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 4.00)
    }
}
impl FragChild {
    pub fn new(pos: Vector2, vel: Vector2, acc: Vector2) -> Pos<FragChild> {
        Pos { pos, vel, acc, ..Pos::default() }
    }
}
impl Default for FragChild {
    fn default() -> Self {
        FragChild { hp: 2.00 }
    }
}
impl OnHit for Pos<FragChild> {

    fn self_effect_on_hit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }
    #[allow(unused_variables)]
    fn effect_on_hit(&self, asset_manager: &AssetLoader) -> StateEffect {
        StateEffect { burger_damage: self.bhv.hp, ..Default::default() }
    }
}
impl TakeEffect for Pos<FragChild> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage;
    }
}
