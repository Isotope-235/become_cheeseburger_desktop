use crate::{
    State,
    library::{AssetLoader, Particle, Vector2}
};

const BURGER_SIZE: f64 = 2.00;

pub fn run(state: &mut State, assets: &AssetLoader) {
    if state.burger.is_targetable() {
        let mut dmg = 0;
        for e in &mut state.entities {
            let Some(effect) = effect::of(e.class) else {
                continue;
            };

            let center_dist = (e.pos - state.burger.pos).len();

            if center_dist < (BURGER_SIZE + effect.range) {
                dmg += effect.dmg;
                e.alive = false;
                if let Some(snd) = effect.sound {
                    assets.play_sound(snd);
                }
                if effect.make_particles {
                    make_particles(e.pos, &mut state.particles, assets);
                }
            }
        }
        if dmg > 0 {
            assets.play_sound("damage");
            state.freeze += f64::from(dmg);
        }
        state.burger.hp -= dmg;
    }
}

fn make_particles(pos: Vector2, particles: &mut Vec<Particle>, assets: &AssetLoader) {
    let mut new = Particle::from_center(6, pos, 0.20, *assets.color("heart"));
    particles.append(&mut new);
}

struct Effect {
    dmg:            i32,
    range:          f64,
    sound:          Option<&'static str>,
    make_particles: bool
}

mod effect {
    use super::Effect;
    use crate::library::component::Class;

    fn zero() -> Effect {
        Effect {
            dmg:            0,
            range:          0.00,
            sound:          None,
            make_particles: false
        }
    }

    fn basic(dmg: i32, range: f64) -> Effect {
        Effect {
            dmg,
            range,
            ..zero()
        }
    }

    pub fn of(class: Class) -> Option<Effect> {
        use Class as C;
        Some(match class {
            C::None | C::Warning { .. } => return None,
            C::Bullet => basic(3, 3.00),
            C::Slug => basic(7, 8.00),
            C::Flak => basic(5, 7.00),
            C::HealthPack => Effect {
                dmg:            -4,
                range:          7.00,
                sound:          Some("heal"),
                make_particles: true
            },
            C::FlakChild => basic(2, 4.00),
            C::Laser => basic(5, 3.00)
        })
    }
}
