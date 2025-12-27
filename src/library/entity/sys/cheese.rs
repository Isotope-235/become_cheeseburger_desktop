use crate::{
    State,
    library::{AssetLoader, Particle, cheese}
};

const BURGER_SIZE: f64 = 2.00;

pub fn run(state: &mut State, assets: &AssetLoader) {
    let cheese_dist = (state.cheese.pos - state.burger.pos).len();
    if cheese_dist < (BURGER_SIZE + 7.00) {
        state.score += 100;
        assets.play_sound("heal"); // TODO: cheese sound
        state.particles.extend(Particle::from_center(
            5,
            state.cheese.pos,
            0.33,
            *assets.color("cheese")
        ));

        let new_pos = cheese::create_next_pos(state.cheese.next_pos);
        state.cheese.pos = state.cheese.next_pos;
        state.cheese.next_pos = new_pos;
    }
}
