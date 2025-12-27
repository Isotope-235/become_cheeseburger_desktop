use crate::{
    State,
    library::{
        AssetLoader, Entity, Radians, Vector2,
        component::{self, Class}
    }
};

pub fn run(state: &mut State, assets: &AssetLoader) {
    let mut flak = Vec::new();
    let mut lasers = Vec::new();

    state.entities.retain(|e| {
        if e.age >= e.lifespan {
            match e.class {
                Class::Flak => flak.push(e.pos),
                Class::Warning { dir } => lasers.push((e.pos, dir)),
                _ => ()
            }
            false
        } else {
            true
        }
    });
    state.particles.retain(|p| p.age < p.lifespan);

    let num = 8;
    for pos in flak {
        for i in 0..8 {
            let dir = f64::from(i).as_radians() / f64::from(num);
            state.entities.push(Entity {
                class: component::Class::FlakChild,
                pos,
                acc: Vector2::from(dir) * 0.01,
                lifespan: 300.,
                ..Default::default()
            });
        }
    }

    for (pos, dir) in lasers {
        assets.play_sound("laser");
        state.entities.push(Entity {
            class: component::Class::Laser,
            pos: pos - dir * 40.00,
            vel: dir * 7.00,
            lifespan: 500.,
            ..Default::default()
        });
    }
}
