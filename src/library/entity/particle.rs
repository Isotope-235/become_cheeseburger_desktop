use std::f64::consts::TAU;

use crate::{library::component::Lifespan, *};
pub struct Particle {
    pub pos:      Vector2,
    pub vel:      Vector2,
    pub acc:      Vector2,
    pub age:      f64,
    pub lifespan: Lifespan,
    pub friction: f64,
    pub color:    Color
}

impl Particle {
    pub fn new(
        pos: Vector2,
        vel: Vector2,
        acc: Vector2,
        fric: f64,
        lifetime: f64,
        color: Color
    ) -> Particle {
        Self {
            pos,
            vel,
            acc,
            age: 0.00,
            lifespan: Lifespan {
                time:     lifetime,
                on_ended: None
            },
            friction: fric,
            color
        }
    }
    /// TODO: find a better way to do this
    #[allow(clippy::too_many_arguments)]
    pub fn from_center(number: usize, pos: Vector2, fric: f64, color: Color) -> Vec<Particle> {
        let mut output = Vec::with_capacity(number);
        let offset = rand(1.00);
        let vel = 4.00;
        let random_offset = rand(TAU);
        for i in 0..number {
            let angle = random_offset + TAU * (offset + i as f64 / (number as f64));
            let vel = Vector2::from(angle) * (vel * 0.90 + rand(vel * 0.20));
            output.push(Particle::new(
                pos,
                vel,
                Vector2::ZERO,
                fric,
                rand(20.),
                color
            ));
        }
        output
    }
}
