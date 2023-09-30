use std::ops::AddAssign;


use crate::*;
pub struct Pos<T> {
    pub pos: V2,
    pub vel: V2,
    pub acc: V2,
    pub age: f64,
    pub bhv: T
}
impl<T> Pos<T> {
    pub fn update_pos(&mut self, dt: f64) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.age += 1.00 * dt;
    }
}
impl<T: Default> Default for Pos<T> {
    fn default() -> Self {
        Self { pos: center(), vel: V2::ZERO, acc: V2::ZERO, age: 0.00, bhv: T::default() }
    }
}
pub fn update_all_pos<T>(items: &mut Vec<Pos<T>>, dt: f64) {
    for item in items {
        item.update_pos(dt)
    }
}
pub fn do_all_hits<T: HitBox + OnHit + TakeEffect>(items: &mut Vec<T>, state_effect_accumulator: &mut StateEffect, burger_circle: &Circle, burger_accumulator: &mut Effect, sprite_manager: &SpriteLoader) {
    for item in items {
        if item.hit_circle().is_hitting(burger_circle) {
            *burger_accumulator += item.target_effect_on_hit();
            item.takes_effect(&item.self_effect_on_hit());
            *state_effect_accumulator += item.state_effect_on_hit(sprite_manager);
        }
    }
}

pub trait OnHit: Sized {
    fn target_effect_on_hit(&self) -> Effect {
        Effect::default()
    }
    fn self_effect_on_hit(&self) -> Effect {
        Effect::default()
    }
    //noinspection RsLiveness
    fn state_effect_on_hit(&self, sprite_manager: &SpriteLoader) -> StateEffect {
        StateEffect::default()
    }
}
pub trait TakeEffect {
    fn takes_effect(&mut self, effect: &Effect);
}
pub struct Effect {
    pub damage: f64
}
impl Default for Effect {
    fn default() -> Self {
        Effect { damage: 0.00 }
    }
}
pub struct StateEffect {
    pub score: i32,
    pub freeze: f64,
    pub particles: Vec<Pos<Particle>>,
}
impl Default for StateEffect {
    fn default() -> Self {
        StateEffect { score: 0, freeze: 0.00, particles: Vec::new() }
    }
}
impl AddAssign<StateEffect> for StateEffect {
    fn add_assign(&mut self, rhs: StateEffect) {
        let StateEffect { score, freeze, particles } = rhs;
        self.score += score;
        self.freeze += freeze;
        self.particles.extend(particles);
    }
}
impl AddAssign<Effect> for Effect {
    fn add_assign(&mut self, rhs: Effect) {
        let Effect { damage } = rhs;
        self.damage += damage;
    }
}

pub trait HitBox: Sized {
    fn hit_circle(&self) -> Circle;
}
pub struct Circle {
    pos: V2,
    rad: f64
}
impl Circle {
    pub fn new(pos: V2, rad: f64) -> Circle {
        Circle { pos, rad }
    }
    pub fn _overlap(&self, other: &Circle) -> f64 {
        let dist_between_centers = (self.pos - other.pos).len();
        let combined_radii = self.rad + other.rad;
        (combined_radii - dist_between_centers) / combined_radii
    }
    pub fn is_hitting(&self, other: &Circle) -> bool {
        (other.pos - self.pos).square_len() < (self.rad + other.rad).powi(2)
    }
}