use crate::State;

pub fn run(state: &mut State, dt: f64) {
    for e in &mut state.entities {
        e.vel += e.acc * dt;
        e.pos += e.vel * dt;
    }
    for p in &mut state.particles {
        p.vel += p.acc * dt;
        p.pos += p.vel * dt;
    }
    state.burger.vel += state.burger.acc * dt;
    state.burger.pos += state.burger.vel * dt;
}
