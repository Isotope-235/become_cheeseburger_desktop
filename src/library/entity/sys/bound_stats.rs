use crate::State;

pub fn run(state: &mut State) {
    state.burger.hp = std::cmp::min(state.burger.hp, state.burger.max_hp());
}
