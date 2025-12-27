use crate::{
    State,
    library::{AssetLoader, Input}
};

pub fn run(state: &mut State, dt: f64, input: &Input, assets: &AssetLoader) {
    let burger = &mut state.burger;
    burger.vel = input.dir().normal() * (0.55) * dt + burger.vel * 0.675f64.powf(dt);
    burger.invuln = (burger.invuln - dt).max(0.00);
    burger.dash_charge = (burger.dash_charge + 0.01 * dt).min(1.00);
    burger.hp = burger.hp.min(burger.max_hp());
    if input.space.is_pressed() && burger.can_dash() && input.dir().len() > 0.00 {
        burger.dash(input, assets);
    }
}
