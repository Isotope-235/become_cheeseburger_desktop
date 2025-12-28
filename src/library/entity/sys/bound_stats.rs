use crate::{State, library::Player};

pub fn run(state: &mut State) {
    state.burger.hp = std::cmp::min(state.burger.hp, Player::max_hp());
}
