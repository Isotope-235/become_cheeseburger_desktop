use crate::{
    State,
    library::{AssetLoader, Particle, component::Class},
};

const BURGER_SIZE: f64 = 2.00;

pub fn run(state: &mut State, assets: &AssetLoader) {
    let cheese_dist = (state.cheese.pos - state.burger.pos).len();
    if cheese_dist < (BURGER_SIZE + 7.00) {
        state.cheese.hp = 0.00;
        state.score += 100;
        assets.play_sound("heal"); // TODO: cheese sound
        state.particles.extend(Particle::from_center(
            5,
            state.cheese.pos,
            0.33,
            *assets.color("cheese"),
        ));
    }

    if state.burger.is_targetable() {
        let mut dmg = 0.00;
        for e in &mut state.entities {
            let Some(ref col) = e.collision else {
                continue;
            };

            let center_dist = (e.pos - state.burger.pos).len();

            if center_dist < (BURGER_SIZE + col.range) {
                dmg += col.dmg;
                if let Some(ref mut hp) = e.hp {
                    *hp = 0.00;
                }
                if let Some(snd) = col.sound {
                    assets.play_sound(snd);
                }
                if e.class == Class::HealthPack {
                    let particles = crate::library::Particle::from_center(
                        6,
                        e.pos,
                        0.20,
                        *assets.color("heart"),
                    );
                    state.particles.extend(particles);
                }
            }
        }
        if dmg > 0.00 {
            assets.play_sound("damage");
            state.freeze += dmg;
        }
        state.burger.hp -= dmg;
    }
}
