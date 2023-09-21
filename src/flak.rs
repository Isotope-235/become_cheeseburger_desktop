use crate::*;

pub struct Flak {
    hp: f64
}
impl Hitbox for Pos<Flak> {
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 7.00)
    }
}
impl Flak {
    pub fn new(pos: V2, vel: V2) -> Pos<Flak> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for Flak {
    fn default() -> Self {
        Flak { hp: 5.00 }
    }
}
impl Pos<Flak> {
    pub fn will_live(&self) -> bool {
        self.bhv.hp > 1e-10 && self.age < 200.00
    }
}
impl Onhit for Pos<Flak> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_onhit(&self) -> Effect {
        self.target_effect_onhit()
    }
}
impl TakeEffect for Pos<Flak> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage
    }
}
pub struct FlakChild {
    pub hp: f64
}
impl Hitbox for Pos<FlakChild> {
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 4.00)
    }
}
impl FlakChild {
    pub fn new(pos: V2, vel: V2, acc: V2) -> Pos<FlakChild> {
        Pos { pos, vel, acc, ..Pos::default() }
    }
}
impl Default for FlakChild {
    fn default() -> Self {
        FlakChild { hp: 2.00 }
    }
}
impl Onhit for Pos<FlakChild> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_onhit(&self) -> Effect {
        self.target_effect_onhit()
    }
}
impl TakeEffect for Pos<FlakChild> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage;
    }
}