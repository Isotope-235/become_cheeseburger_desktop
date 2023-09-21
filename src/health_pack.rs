use crate::*;
pub struct HealthPack {
    pub hp: f64
}
impl Hitbox for Pos<HealthPack>{
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 5.00)
    }
}
impl HealthPack {
    pub fn new(pos: V2, vel: V2) -> Pos<Self> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for HealthPack {
    fn default() -> Self {
        HealthPack { hp: 1.00 }
    }    
}
impl Pos<HealthPack> {
    pub fn should_be_removed(&self) -> bool {
        self.bhv.hp < 1e-10
    }
}
impl Onhit for Pos<HealthPack> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: -2.00 }
    }

    fn self_effect_onhit(&self) -> Effect {
        Effect { damage: 1.00 }
    }
}
impl TakeEffect for Pos<HealthPack> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}