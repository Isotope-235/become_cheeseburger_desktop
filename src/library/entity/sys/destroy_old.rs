use crate::{
    State,
    library::{AssetLoader, Entity, Radians, Vector2, component},
};

pub fn run(state: &mut State, assets: &AssetLoader) {
    let mut flak = Vec::new();
    let mut lasers = Vec::new();

    state.entities.retain(|e| match e.lifespan {
        Some(ref ls) => {
            let retain = e.age < ls.time;
            if let Some(ref effect) = ls.on_ended
                && !retain
            {
                match *effect {
                    component::EndedEffect::Flak => flak.push(e.pos),
                    component::EndedEffect::Warning { dir } => lasers.push((e.pos, dir)),
                }
            }
            retain
        }
        None => true,
    });
    state.particles.retain(|p| p.age < p.lifespan.time);

    let num = 8;
    for pos in flak {
        for i in 0..8 {
            let dir = f64::from(i).as_radians() / f64::from(num);
            state.entities.push(Entity {
                pos,
                acc: Vector2::from(dir) * 0.01,
                lifespan: Some(component::Lifespan {
                    time: 300.,
                    on_ended: None,
                }),
                hp: Some(2.00),
                collision: Some(component::Collision {
                    dmg: 2.00,
                    range: 4.00,
                    sound: None,
                }),
                draw: Some(component::Draw::Sprite {
                    name: "flak_child",
                    rotate: false,
                }),
                ..Default::default()
            });
        }
    }

    for (pos, dir) in lasers {
        assets.play_sound("laser");
        state.entities.push(Entity {
            pos: pos - dir * 40.00,
            vel: dir * 7.00,
            lifespan: Some(component::Lifespan {
                time: 500.,
                on_ended: None,
            }),
            hp: Some(5.00),
            collision: Some(component::Collision {
                dmg: 5.00,
                range: 3.00,
                sound: None,
            }),
            draw: Some(component::Draw::Laser),
            ..Default::default()
        });
    }
}
