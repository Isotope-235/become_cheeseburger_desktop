use crate::{
    State,
    library::{
        CENTER, Counter, Entity, SCREEN, Vector2, chance, component::Class, get_shift,
        num_to_corner, num_to_side, rrange, spawn_pos_vel, spawn_pos_vel_from
    }
};

#[allow(clippy::missing_panics_doc)]
pub fn run(state: &mut State, dt: f64) {
    let diff_scale = state.difficulty * 0.01;

    // bullets
    let times = state.counters.bullet.revolve(1.10 + 0.20 * diff_scale, dt);

    for _ in 0..times {
        let side = rrange(4);
        let snake_ch = diff_scale * 0.25;
        if chance(snake_ch / (1.00 + snake_ch)) {
            let direction = num_to_side(side);
            let shift = get_shift(direction, 4.00);
            for i in 0..((2.00 + diff_scale) as i32) {
                let delay = f64::from(i) * 10.00;
                let (pos, vel) = {
                    let side_buffer = 4.00 + delay;
                    let buffer = direction * side_buffer;
                    let pos = CENTER + direction.mul_per(CENTER) + buffer;
                    (pos + shift, direction.negate())
                };
                spawn(state, Class::Bullet, pos, vel * 1.25, 750. + delay);
            }
        } else {
            for i in 0..((1.00 + diff_scale * 2.00) as i32) {
                let delay = f64::from(i) * 10.00;
                let (pos, vel) = spawn_pos_vel_from(side, 4.00 + delay, 4.00);
                spawn(state, Class::Bullet, pos, vel * 1.25, 750. + delay);
            }
        }
    }

    // slugs
    let times = state.counters.slug.revolve(0.125 + 0.025 * diff_scale, dt);

    for _ in 0..times {
        let (pos, vel) = spawn_pos_vel(10.00, 10.00);
        spawn(state, Class::Slug, pos, vel * 0.50, 1500.);
    }

    // warnings
    let times = state.counters.warning.revolve(0.15 + 0.10 * diff_scale, dt);

    for i in 0..(times * diff_scale as i32) {
        let (mut pos, dir) = spawn_pos_vel(-12.00, 12.00);
        // move laser so it targets player
        let shift = crate::rand(30.00) - 15.00;
        if dir.x().abs() < 1e-10 {
            pos.0 = state.burger.pos.x() + shift;
        } else {
            pos.1 = state.burger.pos.y() + shift;
        }
        let delay = f64::from(i) * (15.00);
        spawn(
            state,
            Class::Warning { dir, delay },
            pos,
            Vector2::ZERO,
            60. + delay
        );
    }

    // health packs
    let hp_count = state
        .entities
        .iter()
        .filter(|e| matches!(e.class, Class::HealthPack))
        .count();
    let times = state.counters.health_pack.revolve(
        0.10 * f64::from(
            (state.burger.missing_hp() - i32::try_from(hp_count * 2).unwrap()).clamp(0, 8)
        ),
        dt
    );

    for _ in 0..times {
        let (pos, vel) = spawn_pos_vel(10.00, 12.00);
        spawn(state, Class::HealthPack, pos, vel * 0.30, 500.);
    }

    // frag
    let times = state.counters.frag.revolve(0.10 + 0.02 * diff_scale, dt);

    for _ in 0..times {
        let (pos, vel) = spawn_pos_vel(4.00, 4.00);
        spawn(state, Class::Flak, pos, vel * 0.50, 200.);
    }

    let times = state
        .counters
        .cross
        .revolve((-0.25 + 0.135 * diff_scale).max(0.00), dt);

    for _ in 0..times {
        for i in 0..4 {
            let starting_point = SCREEN.mul_per(num_to_corner(i));
            let direction = CENTER - starting_point;
            let vel = direction.normal();
            for ii in 0..3 {
                let pos = starting_point - vel * 10.00 * f64::from(ii);
                spawn(state, Class::Bullet, pos, vel * 1.75, 750.);
            }
        }
    }
}

fn spawn(state: &mut State, class: Class, pos: Vector2, vel: Vector2, lifespan: f64) {
    state.entities.push(Entity {
        class,
        pos,
        vel,
        lifespan,
        ..Default::default()
    });
}
