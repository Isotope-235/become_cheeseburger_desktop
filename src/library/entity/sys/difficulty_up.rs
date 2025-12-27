use crate::State;

pub fn run(state: &mut State, dt: f64) {
    state.difficulty += 0.10 * dt;
}
