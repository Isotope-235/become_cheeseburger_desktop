use crate::*;
pub struct Cheese {
    pub hp: f64
}
impl Cheese {
    pub fn new(pos: V2) -> Pos<Cheese> {
        Pos { 
            pos,
            vel: V2::ZERO,
            acc: V2::ZERO,
            age: 0.00,
            bhv: Cheese {
                hp: 1.00,
            }
        }
    }
}

impl Hitbox for Pos<Cheese> {
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 5.00)
    }
}
impl Onhit for Pos<Cheese> {
    fn self_effect_onhit(&self) -> Effect {
        Effect { damage: 1.00, ..Effect::default() }
    }

    fn state_effect_onhit(&self) -> StateEffect {
        StateEffect { score: 100, ..StateEffect::default() }
    }
}
impl TakeEffect for Pos<Cheese> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage;
    }
}
impl Behaviour for Cheese {
    fn update(this: &mut Pos<Self>, input: &Input) {
        if this.bhv.hp < 1e-10 {
            let V2(x, y) = center();
            this.pos = V2(rand(x), rand(y)) + center() * 0.50;
            this.bhv.hp = 1.00;
        }
    }
}