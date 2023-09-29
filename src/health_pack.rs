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
impl Onhit for Pos<HealthPack> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: -2.00 }
    }

    fn self_effect_onhit(&self) -> Effect {
        Effect { damage: 1.00 }
    }
    fn state_effect_onhit(&self) -> StateEffect {
        let particles = Particle::from_center(6, rand(1.00), self.pos, 4.00, 0.00, 0.20, 20.00, RED);
        StateEffect { particles, ..StateEffect::default() }
    }
}
impl TakeEffect for Pos<HealthPack> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}