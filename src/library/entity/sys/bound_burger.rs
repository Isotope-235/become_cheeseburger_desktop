use crate::{
    State,
    library::{CENTER, Vector2},
};

pub fn run(state: &mut State) {
    let bounds = CENTER * 2.00;
    let Vector2(x, y) = state.burger.pos;
    if x < 0.00 || x > bounds.0 {
        state.burger.vel.0 *= -1.00;
    }
    if y < 0.00 || y > bounds.1 {
        state.burger.vel.1 *= -1.00;
    }
    state.burger.pos.0 = state.burger.pos.0.max(0.00).min(bounds.0);
    state.burger.pos.1 = state.burger.pos.1.max(0.00).min(bounds.1);
}
