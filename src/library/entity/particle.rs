use crate::*;
pub struct Particle {
    pub lifetime: f64,
    pub friction: f64,
    pub color: Color
}
impl Particle {
    pub fn new(pos: V2, vel: V2, acc: V2, fric: f64, lifetime: f64, color: Color) -> Pos<Particle> {
        Pos {
            pos,
            vel,
            acc,
            age: 0.00,
            bhv: Particle { lifetime, friction: fric, color },
        }
    }
    pub fn from_center(
        number: usize,
        offset: f64,
        pos: V2,
        vel: f64,
        acc: f64,
        fric: f64,
        lifetime: f64,
        color: Color
    ) -> Vec<Pos<Particle>> {
        let mut output = Vec::with_capacity(number);
        for i in 0..number {
            let angle = 2.00 * PI * (offset + (i as f64) / (number as f64));
            let vel = V2::from(angle) *  (vel * 0.90 + rand(vel * 0.20));
            let acc = V2::from(angle) * acc;
            output.push(Particle::new(pos, vel, acc, fric, rand(lifetime), color));
        }
        output
    }
}
