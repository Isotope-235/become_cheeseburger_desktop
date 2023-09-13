use crate::*;
pub struct Bullet {
    hp: f64
}
impl Behaviour for Bullet {
}
impl Hitbox for Pos<Bullet>{
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 3.00)
    }
}
impl Bullet {
    pub fn new(pos: V2, vel: V2, extra_lifetime: f64) -> Pos<Self> {
        Pos { pos, vel, acc: V2::ZERO, age: -extra_lifetime, bhv: Bullet { hp: 3.00 } }
    }
}
impl Pos<Bullet> {
    pub fn should_be_removed(&self) -> bool {
        self.bhv.hp < 1e-10
    }
}
impl Onhit for Pos<Bullet> {
    fn target_effect_onhit(&self) -> Effect {
        Effect { damage: self.bhv.hp }
    }

    fn self_effect_onhit(&self) -> Effect {
        self.target_effect_onhit()
    }
}
impl TakeEffect for Pos<Bullet> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}