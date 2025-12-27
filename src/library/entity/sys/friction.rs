use crate::State;

pub fn run(state: &mut State, dt: f64) {
    for p in &mut state.particles {
        p.vel *= (1.00 - p.friction).powf(dt);
    }
}
