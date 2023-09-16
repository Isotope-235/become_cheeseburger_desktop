use crate::*;
pub struct Player {
    pub hp: f64,
    pub invuln: f64,
    pub dash_charge: f64
}

impl Behaviour for Player {
    fn update(this: &mut Pos<Self>, input: &Input) {
        this.vel = input.dir().normal() * (0.55) * DT + this.vel * 0.675f64.powf(DT);
        this.bhv.invuln = (this.bhv.invuln - DT).max(0.00);
        this.bhv.dash_charge = (this.bhv.dash_charge + 0.01 * DT).min(1.00);
        this.bhv.hp = this.bhv.hp.min(this.max_hp());
        if input.space && this.can_dash() && input.dir().len() > 0.00 {
            this.dash(input);
        }
    }
}
impl Hitbox for Pos<Player> {
    fn hitcircle(&self) -> Circle {
        Circle::new(self.pos, 2.00)
    }
}

impl Player {
    pub fn new(pos: V2) -> Pos<Self> {
        Pos { pos, vel: V2::ZERO, acc: V2::ZERO, age: 0.00, bhv: Player { hp: 8.00, invuln: 0.00, dash_charge: 1.00 } }
    }
}

impl Pos<Player> {
    pub fn is_alive(&self) -> bool {
        self.bhv.hp > 1e-10
    }
    pub fn stays_in_bounds(&mut self) {
        let bounds = center() * 2.00;
        let V2(x, y) = self.pos;
        if x < 0.00 || x > bounds.0 {
            self.vel.0 = self.vel.0 * -1.00
        }
        if y < 0.00 || y > bounds.1 {
            self.vel.1 = self.vel.1 * -1.00
        }
        self.pos.0 = self.pos.0.max(0.00).min(bounds.0);
        self.pos.1 = self.pos.1.max(0.00).min(bounds.1);
    }
    pub fn max_hp(&self) -> f64 {
        8.00
    }
    pub fn dash(&mut self, input: &Input) {
        let charge_used = self.bhv.dash_charge;
        self.vel = self.vel + input.dir().normal() * charge_used * 7.00;
        self.bhv.invuln = charge_used * 15.00;
        self.bhv.dash_charge = self.bhv.dash_charge - charge_used;
    }
    pub fn can_dash(&self) -> bool {
        self.bhv.dash_charge >= 1.00
    }
    pub fn is_targetable(&self) -> bool {
        self.bhv.invuln <= 0.00
    }
    pub fn missing_hp(&self) -> f64 {
        self.max_hp() - self.bhv.hp
    }
}
impl TakeEffect for Pos<Player> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage;
    }
}