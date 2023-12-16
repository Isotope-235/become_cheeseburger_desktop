use std::f64::consts::TAU;

use crate::*;
pub struct Particle {
    pub lifetime: f64,
    pub friction: f64,
    pub color: Color,
}
impl Particle {
    pub fn new(
        pos: Vector2,
        vel: Vector2,
        acc: Vector2,
        fric: f64,
        lifetime: f64,
        color: Color,
    ) -> Pos<Self> {
        Pos {
            pos,
            vel,
            acc,
            age: 0.00,
            bhv: Self {
                lifetime,
                friction: fric,
                color,
            },
        }
    }
    /// TODO: find a better way to do this
    #[allow(clippy::too_many_arguments)]
    pub fn from_center(
        number: usize,
        offset: f64,
        pos: Vector2,
        vel: f64,
        acc: f64,
        fric: f64,
        lifetime: f64,
        color: Color,
    ) -> Vec<Pos<Self>> {
        let mut output = Vec::with_capacity(number);
        let random_offset = rand(TAU);
        for i in 0..number {
            let angle = random_offset + TAU * (offset + i as f64 / (number as f64));
            let vel = Vector2::from(angle) * (vel * 0.90 + rand(vel * 0.20));
            let acc = Vector2::from(angle) * acc;
            output.push(Self::new(pos, vel, acc, fric, rand(lifetime), color));
        }
        output
    }
}
