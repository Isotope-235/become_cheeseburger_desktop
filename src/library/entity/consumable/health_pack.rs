use crate::*;

pub struct HealthPack {
    pub hp: f64
}
impl HitBox for Pos<HealthPack>{
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 5.00)
    }
}
impl HealthPack {
    pub fn new(pos: Vector2, vel: Vector2) -> Pos<Self> {
        Pos { pos, vel, ..Pos::default() }
    }
}
impl Default for HealthPack {
    fn default() -> Self {
        HealthPack { hp: 1.00 }
    }    
}
impl OnHit for Pos<HealthPack> {
    fn self_effect_on_hit(&self) -> Effect {
        Effect { damage: 1.00 }
    }
    fn effect_on_hit(&self, asset_loader: &AssetLoader) -> StateEffect {
        asset_loader.play_sound("heal");
        let particles = Particle::from_center(6, rand(1.00), self.pos, 4.00, 0.00, 0.20, 20.00, *asset_loader.color("heart"));
        StateEffect { burger_damage: -2.00, particles, ..StateEffect::default() }
    }
}
impl TakeEffect for Pos<HealthPack> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage } = effect;
        self.bhv.hp -= damage;
    }
}
