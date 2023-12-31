use crate::*;

pub struct Player {
    pub hp: f64,
    pub invuln: f64,
    pub dash_charge: f64,
}

impl HitBox for Pos<Player> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 2.00)
    }
}

impl Player {
    pub fn new(pos: Vector2) -> Pos<Self> {
        Pos {
            pos,
            vel: Vector2::ZERO,
            acc: Vector2::ZERO,
            age: 0.00,
            bhv: Player {
                hp: 8.00,
                invuln: 0.00,
                dash_charge: 1.00,
            },
        }
    }
}

impl Pos<Player> {
    pub fn is_alive(&self) -> bool {
        self.bhv.hp > 1e-10
    }
    pub fn stays_in_bounds(&mut self) {
        let bounds = CENTER * 2.00;
        let Vector2(x, y) = self.pos;
        if x < 0.00 || x > bounds.0 {
            self.vel.0 *= -1.00;
        }
        if y < 0.00 || y > bounds.1 {
            self.vel.1 *= -1.00;
        }
        self.pos.0 = self.pos.0.max(0.00).min(bounds.0);
        self.pos.1 = self.pos.1.max(0.00).min(bounds.1);
    }
    pub fn max_hp(&self) -> f64 {
        8.00
    }
    pub fn dash(&mut self, input: &Input, asset_loader: &AssetLoader) {
        asset_loader.play_sound("dash");
        let charge_used = self.bhv.dash_charge;
        self.vel += input.dir().normal() * charge_used * 7.00;
        self.bhv.invuln = charge_used * 15.00;
        self.bhv.dash_charge -= charge_used;
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
