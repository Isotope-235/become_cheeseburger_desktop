use crate::*;

pub struct Player {
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub hp: f64,
    pub invuln: f64,
    pub dash_charge: f64,
}

impl Player {
    pub fn new(pos: Vector2) -> Self {
        Self {
            pos,
            vel: Vector2::ZERO,
            acc: Vector2::ZERO,
            hp: 8.00,
            invuln: 0.00,
            dash_charge: 1.00,
        }
    }
}

impl Player {
    pub fn is_alive(&self) -> bool {
        self.hp > 1e-10
    }
    pub fn max_hp(&self) -> f64 {
        8.00
    }
    pub fn dash(&mut self, input: &Input, asset_loader: &AssetLoader) {
        asset_loader.play_sound("dash");
        let charge_used = self.dash_charge;
        self.vel += input.dir().normal() * charge_used * 7.00;
        self.invuln = charge_used * 15.00;
        self.dash_charge -= charge_used;
    }
    pub fn can_dash(&self) -> bool {
        self.dash_charge >= 1.00
    }
    pub fn is_targetable(&self) -> bool {
        self.invuln <= 0.00
    }
    pub fn missing_hp(&self) -> f64 {
        self.max_hp() - self.hp
    }
}
