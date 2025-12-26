use crate::{State, library::cheese};

pub fn run(state: &mut State) {
    if state.cheese.hp < 1e-10 {
        let new_pos = cheese::create_next_pos(state.cheese.next_pos);
        state.cheese.pos = state.cheese.next_pos;
        state.cheese.next_pos = new_pos;
        state.cheese.hp = 1.00;
    }
}
