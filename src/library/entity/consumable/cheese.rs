use crate::*;
pub struct Cheese {
    pub hp: f64,
    pub next_pos: Vector2,
}
impl Cheese {
    pub fn new(pos: Vector2, next_pos: Vector2) -> Pos<Cheese> {
        Pos {
            pos,
            vel: Vector2::ZERO,
            acc: Vector2::ZERO,
            age: 0.00,
            bhv: Cheese { hp: 1.00, next_pos },
        }
    }
}

impl HitBox for Pos<Cheese> {
    fn hit_circle(&self) -> pos::Circle {
        pos::Circle::new(self.pos, 7.00)
    }
}
impl OnHit for Pos<Cheese> {
    fn self_effect_on_hit(&self) -> Effect {
        Effect { damage: 1.00 }
    }

    fn effect_on_hit(&self, asset_loader: &AssetLoader) -> StateEffect {
        asset_loader.play_sound("heal"); // TODO: cheese sound
        StateEffect {
            score: 100,
            particles: Particle::from_center(
                5,
                rand(1.00),
                self.pos,
                4.00,
                0.00,
                0.33,
                20.00,
                *asset_loader.color("cheese"),
            ),
            ..StateEffect::default()
        }
    }
}
impl TakeEffect for Pos<Cheese> {
    fn takes_effect(&mut self, effect: &Effect) {
        let Effect { damage, .. } = effect;
        self.bhv.hp -= damage;
    }
}

pub fn create_next_pos(burger_pos: Vector2) -> Vector2 {
    let Vector2(x, y) = CENTER;
    loop {
        let maybe_pos = Vector2(rand(x), rand(y)) + CENTER * 0.50;
        if (burger_pos - maybe_pos).len() > 16.00 {
            return maybe_pos;
        }
    }
}
