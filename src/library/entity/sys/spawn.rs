use crate::{
    State,
    library::{
        CENTER, Counter, Entity, SCREEN, Vector2, chance, component::Class, get_shift,
        num_to_corner, num_to_side, rrange, spawn_pos_vel, spawn_pos_vel_from
    }
};

pub fn run(state: &mut State, dt: f64) {
    let diff_scale = state.difficulty * 0.01;

    let (entities, counters) = (&mut state.entities, &mut state.counters);

    counters.bullet.run(1.10 + 0.20 * diff_scale, dt, || {
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
                spawn(entities, Class::Bullet, pos, vel * 1.25, 750. + delay);
            }
        } else {
            for i in 0..((1.00 + diff_scale * 2.00) as i32) {
                let delay = f64::from(i) * 10.00;
                let (pos, vel) = spawn_pos_vel_from(side, 4.00 + delay, 4.00);
                spawn(entities, Class::Bullet, pos, vel * 1.25, 750. + delay);
            }
        }
    });

    counters.slug.run(0.125 + 0.025 * diff_scale, dt, || {
        let (pos, vel) = spawn_pos_vel(10.00, 10.00);
        spawn(entities, Class::Slug, pos, vel * 0.50, 1500.);
    });

    counters.warning.run(0.15 + 0.10 * diff_scale, dt, || {
        for i in 0..diff_scale as i32 {
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
                entities,
                Class::Warning { dir, delay },
                pos,
                Vector2::ZERO,
                60. + delay
            );
        }
    });

    let hp_count = entities
        .iter()
        .filter(|e| matches!(e.class, Class::HealthPack))
        .count();
    counters.health_pack.run(
        0.10 * f64::from(
            (state.burger.missing_hp() - i32::try_from(hp_count * 2).unwrap()).clamp(0, 8)
        ),
        dt,
        || {
            let (pos, vel) = spawn_pos_vel(10.00, 12.00);
            spawn(entities, Class::HealthPack, pos, vel * 0.30, 500.);
        }
    );

    counters.frag.run(0.10 + 0.02 * diff_scale, dt, || {
        let (pos, vel) = spawn_pos_vel(4.00, 4.00);
        spawn(entities, Class::Flak, pos, vel * 0.50, 200.);
    });

    counters
        .cross
        .run((-0.25 + 0.135 * diff_scale).max(0.00), dt, || {
            for i in 0..4 {
                let starting_point = SCREEN.mul_per(num_to_corner(i));
                let direction = CENTER - starting_point;
                let vel = direction.normal();
                for ii in 0..3 {
                    let pos = starting_point - vel * 10.00 * f64::from(ii);
                    spawn(entities, Class::Bullet, pos, vel * 1.75, 750.);
                }
            }
        });
}

fn spawn(entities: &mut Vec<Entity>, class: Class, pos: Vector2, vel: Vector2, lifespan: f64) {
    entities.push(Entity {
        class,
        pos,
        vel,
        lifespan,
        ..Default::default()
    });
}
