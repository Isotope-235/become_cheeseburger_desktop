#![allow(unused_variables)]

use std::ops::AddAssign;

use crate::*;
pub struct Pos<T> {
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub age: f64,
    pub bhv: T,
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
        Self {
            pos: CENTER,
            vel: Vector2::ZERO,
            acc: Vector2::ZERO,
            age: 0.00,
            bhv: T::default(),
        }
    }
}

pub fn update_all<T>(items: &mut Vec<Pos<T>>, dt: f64) {
    for item in items {
        item.update_pos(dt);
    }
}

pub struct HitInfo<'a> {
    pub state_effect_accumulator: &'a mut StateEffect,
    pub burger_circle: &'a Circle,
    pub asset_loader: &'a AssetLoader,
}

pub fn do_all_hits<T: HitBox + OnHit + TakeEffect>(items: &mut Vec<T>, hit_info: &mut HitInfo) {
    let HitInfo {
        state_effect_accumulator,
        burger_circle,
        asset_loader,
    } = hit_info;
    for item in items {
        if item.hit_circle().is_hitting(hit_info.burger_circle) {
            **state_effect_accumulator += item.effect_on_hit(hit_info.asset_loader);
            item.takes_effect(&item.self_effect_on_hit());
        }
    }
}

pub trait OnHit: Sized {
    fn self_effect_on_hit(&self) -> Effect {
        Effect::default()
    }
    //noinspection RsLiveness
    fn effect_on_hit(&self, asset_manager: &AssetLoader) -> StateEffect {
        StateEffect::default()
    }
}

pub trait TakeEffect {
    fn takes_effect(&mut self, effect: &Effect);
}

pub struct Effect {
    pub damage: f64,
}

impl Default for Effect {
    fn default() -> Self {
        Effect { damage: 0.00 }
    }
}

pub struct StateEffect {
    pub burger_damage: f64,
    pub score: i32,
    pub freeze: f64,
    pub particles: Vec<Pos<Particle>>,
}

impl Default for StateEffect {
    fn default() -> Self {
        StateEffect {
            score: 0,
            freeze: 0.00,
            particles: Vec::new(),
            burger_damage: 0.00,
        }
    }
}

impl AddAssign<StateEffect> for StateEffect {
    fn add_assign(&mut self, rhs: StateEffect) {
        let StateEffect {
            score,
            freeze,
            particles,
            burger_damage,
        } = rhs;
        self.burger_damage += burger_damage;
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
    pos: Vector2,
    rad: f64,
}

impl Circle {
    pub fn new(pos: Vector2, rad: f64) -> Circle {
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
