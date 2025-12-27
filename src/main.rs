//#![windows_subsystem = "windows"]

use std::{f64::consts::PI, ops::Not};

use library::*;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::library::component::Class;

pub mod library;

#[macroquad::main(window())]
async fn main() {
    // get fps via cringe
    let fps = fps::find().await;
    println!("Targeted framerate: {fps}");

    // fps-based timestep
    let dt = DT * 60.00 / f64::from(fps);

    // pixel perfection
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.00, 0.00, SCREEN_X as f32, SCREEN_Y as f32));
    camera.zoom = vec2(camera.zoom.x, -camera.zoom.y);
    set_camera(&camera);

    let mut canvas = Canvas2D::new(SCREEN_X as f32, SCREEN_Y as f32);
    canvas.get_texture_mut().set_filter(FilterMode::Nearest);

    // asset loading
    let mut asset_loader = AssetLoader::new();
    asset_loader
        .load_sprites(vec![
            "burger",
            "burger_invuln",
            "bullet",
            "flak",
            "slug",
            "flak_child",
        ])
        .await
        .load_sprites(vec![
            ("cheese", Color::from_rgba(255, 221, 86, 255)),
            ("heart", Color::from_rgba(221, 16, 85, 255)),
        ])
        .await
        .load_sounds(vec!["explosion", "heal", "laser", "damage", "dash"])
        .await
        .load_sounds(vec![(0.15, true, "music1")])
        .await;

    let joystix = load_ttf_font("joystix.otf").await.unwrap();

    // state init
    let mut state = State::reset();
    let mut ended = false;

    // tests
    let text_params = TextParams {
        font: Some(&joystix),
        ..SCORE_TEXT_PARAMS
    };

    // music
    asset_loader.play_sound("music1");

    // main game loop
    loop {
        // get inputs for this frame
        let input = Input::get();

        if ended.not() {
            state.progress(&input, dt, &asset_loader);
        }

        // draw calls
        set_camera(&canvas.camera);
        clear_background(if state.freeze > 0.00 {
            BG_ON_DAMAGE
        } else {
            BG
        });
        state.draw(&asset_loader);
        let score_text = fill_leading_zeroes(state.score);
        let score_chars = score_text.chars();
        for (i, c) in score_chars.enumerate() {
            draw_text_ex(
                &(c.to_string())[..],
                1.00 + i as f32 * 8.00,
                9.00,
                text_params.clone()
            );
        }

        if ended {
            let game_over = "you did not become cheeseburger";
            let options = TextParams {
                font: Some(&joystix),
                font_size: 80,
                font_scale: 0.125,
                ..SCORE_TEXT_PARAMS
            };
            draw_text_ex(
                &game_over[..12],
                35.00,
                CENTER_Y as f32 - 20.00,
                options.clone()
            );
            draw_text_ex(&game_over[12..], 1.00, CENTER_Y as f32, options.clone());
            draw_text_ex("restart: [r]", 30.00, CENTER_Y as f32 + 20.00, options);
            if is_key_pressed(KeyCode::R) {
                state = State::reset();
            }
        }

        set_default_camera();
        canvas.draw();

        // game should only end after freeze frames are rendered, so this goes after draw calls
        ended = state.game_is_over();

        // wait for the frame timer
        next_frame().await;
    }
}

#[derive(Debug, Default)]
pub struct Counters {
    bullet:      f64,
    slug:        f64,
    warning:     f64,
    health_pack: f64,
    frag:        f64,
    cross:       f64
}

pub struct State {
    difficulty: f64,
    score:      i32,
    freeze:     f64,
    counters:   Counters,

    entities:  Vec<Entity>,
    // instances
    burger:    Player,
    cheese:    Cheese,
    particles: Vec<Particle>
}

impl State {
    fn run_systems(&mut self, dt: f64, input: &Input, assets: &AssetLoader) {
        sys::dash::run(self, dt, input, assets);
        sys::pos::run(self, dt);
        sys::bound_burger::run(self);
        sys::age::run(self, dt);
        sys::player_collide::run(self, assets);
        sys::cheese::run(self, assets);
        sys::friction::run(self, dt);

        sys::destroy_old::run(self, assets);
        sys::destroy_dead::run(&mut self.entities);
    }

    fn progress(&mut self, input: &Input, dt: f64, asset_loader: &AssetLoader) {
        for _ in 0..ITERATIONS {
            if self.freeze > 0.00 {
                self.freeze = (self.freeze - dt).max(0.00);
                continue;
            }
            // data saved for perf
            let diff_scale = self.difficulty * 0.01;

            // spawn_logic

            // bullets
            let times = self.counters.bullet.revolve(1.10 + 0.20 * diff_scale, dt);

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
                        self.entities.push(entity::Entity {
                            class: Class::Bullet,
                            pos,
                            vel: vel * 1.25,
                            lifespan: Some(component::Lifespan {
                                time:     750. + delay,
                                on_ended: None
                            }),
                            draw: Some(component::Draw::Sprite {
                                name:   "bullet",
                                rotate: false
                            }),
                            ..Default::default()
                        });
                    }
                } else {
                    for i in 0..((1.00 + diff_scale * 2.00) as i32) {
                        let delay = f64::from(i) * 10.00;
                        let (pos, vel) = spawn_pos_vel_from(side, 4.00 + delay, 4.00);
                        self.entities.push(entity::Entity {
                            class: Class::Bullet,
                            pos,
                            vel: vel * 1.25,
                            lifespan: Some(component::Lifespan {
                                time:     750. + delay,
                                on_ended: None
                            }),
                            draw: Some(component::Draw::Sprite {
                                name:   "bullet",
                                rotate: false
                            }),
                            ..Default::default()
                        });
                    }
                }
            }

            // cheeses

            // slugs
            let times = self.counters.slug.revolve(0.125 + 0.025 * diff_scale, dt);

            for _ in 0..times {
                let (pos, vel) = spawn_pos_vel(10.00, 10.00);
                self.entities.push(entity::Entity {
                    class: Class::Slug,
                    pos,
                    vel: vel * 0.50,
                    lifespan: Some(component::Lifespan {
                        time:     1500.,
                        on_ended: None
                    }),
                    draw: Some(component::Draw::Sprite {
                        name:   "slug",
                        rotate: true
                    }),
                    ..Default::default()
                });
            }

            // warnings
            let times = self.counters.warning.revolve(0.15 + 0.10 * diff_scale, dt);

            for i in 0..(times * diff_scale as i32) {
                let (mut pos, dir) = spawn_pos_vel(-12.00, 12.00);
                // move laser so it targets player
                let shift = rand(30.00) - 15.00;
                if dir.x().abs() < 1e-10 {
                    pos.0 = self.burger.pos.x() + shift;
                } else {
                    pos.1 = self.burger.pos.y() + shift;
                }
                self.entities.push(entity::Entity {
                    pos,
                    lifespan: Some(component::Lifespan {
                        time:     60. + f64::from(i) * (15.00),
                        on_ended: Some(component::EndedEffect::Warning { dir })
                    }),
                    draw: Some(component::Draw::Warning {
                        delay: f64::from(i) * (15.00)
                    }),
                    ..Default::default()
                });
            }

            // health packs
            let hp_count = self
                .entities
                .iter()
                .filter(|e| e.class == Class::HealthPack)
                .count();
            let times = self.counters.health_pack.revolve(
                0.10 * f64::from(
                    (self.burger.missing_hp() - i32::try_from(hp_count * 2).unwrap()).clamp(0, 8)
                ),
                dt
            );

            for _ in 0..times {
                let (pos, vel) = spawn_pos_vel(10.00, 12.00);
                self.entities.push(entity::Entity {
                    class: Class::HealthPack,
                    pos,
                    vel: vel * 0.30,
                    lifespan: Some(component::Lifespan {
                        time:     500.,
                        on_ended: None
                    }),
                    draw: Some(component::Draw::Sprite {
                        name:   "heart",
                        rotate: false
                    }),
                    ..Default::default()
                });
            }

            // frag
            let times = self.counters.frag.revolve(0.10 + 0.02 * diff_scale, dt);

            for _ in 0..times {
                let (pos, vel) = spawn_pos_vel(4.00, 4.00);
                self.entities.push(entity::Entity {
                    class: Class::Flak,
                    pos,
                    vel: vel * 0.50,
                    lifespan: Some(component::Lifespan {
                        time:     200.,
                        on_ended: Some(component::EndedEffect::Flak)
                    }),
                    draw: Some(component::Draw::Sprite {
                        name:   "flak",
                        rotate: false
                    }),
                    ..Default::default()
                });
            }

            let times = self
                .counters
                .cross
                .revolve((-0.25 + 0.135 * diff_scale).max(0.00), dt);

            for _ in 0..times {
                for i in 0..4 {
                    let starting_point = SCREEN.mul_per(num_to_corner(i));
                    let direction = CENTER - starting_point;
                    let vel = direction.normal();
                    for ii in 0..3 {
                        self.entities.push(entity::Entity {
                            class: Class::Bullet,
                            pos: starting_point - vel * 10.00 * f64::from(ii),
                            vel: vel * 1.75,
                            lifespan: Some(component::Lifespan {
                                time:     750.,
                                on_ended: None
                            }),
                            draw: Some(component::Draw::Sprite {
                                name:   "bullet",
                                rotate: false
                            }),
                            ..Default::default()
                        });
                    }
                }
            }

            // movement logic
            self.run_systems(dt, input, asset_loader);
            if self.freeze > 0.00 {
                // making sure that the player sees the fatal projectile
                self.freeze = (self.freeze - dt).max(0.00);
                continue;
            }

            // special update behaviour
            {
                // burger
                let burger = &mut self.burger;
                burger.vel = input.dir().normal() * (0.55) * dt + burger.vel * 0.675f64.powf(dt);
                burger.invuln = (burger.invuln - dt).max(0.00);
                burger.dash_charge = (burger.dash_charge + 0.01 * dt).min(1.00);
                burger.hp = burger.hp.min(burger.max_hp());
                if input.space.is_pressed() && burger.can_dash() && input.dir().len() > 0.00 {
                    burger.dash(input, asset_loader);
                }
            };

            // up difficulty
            self.difficulty += 0.10 * dt;
        }
    }
    fn draw(&self, asset_loader: &AssetLoader) {
        // burger
        let b_sprite = if self.burger.invuln > 0.00 {
            asset_loader.texture("burger_invuln")
        } else {
            asset_loader.texture("burger")
        };
        copy_texture(b_sprite, self.burger.pos);
        // cheese
        copy_texture(asset_loader.texture("cheese"), self.cheese.pos);
        let cpos = self.cheese.pos;
        let to_next = self.cheese.next_pos - cpos;
        draw::rec(
            cpos + (to_next.normal() * 10.00),
            2,
            2,
            *asset_loader.color("cheese")
        );

        // particles
        for particle in &self.particles {
            let (w, h) = (2, 2);
            draw::rec(particle.pos, w, h, particle.color);
        }

        for e in &self.entities {
            if let Some(ref draw_mode) = e.draw {
                match *draw_mode {
                    component::Draw::Sprite { name, rotate } => {
                        if rotate {
                            copy_with_rotation(
                                asset_loader.texture(name),
                                e.pos,
                                e.vel.angle() + PI * 0.50
                            );
                        } else {
                            copy_texture(asset_loader.texture(name), e.pos);
                        }
                    }
                    component::Draw::Warning { delay } => {
                        if e.age >= delay {
                            let dur = 6.00;
                            let clr = if e.age % dur < dur * 0.50 {
                                Color::from_rgba(255, 55, 55, 255)
                            } else {
                                Color::from_rgba(255, 255, 55, 255)
                            };
                            draw::rec(e.pos, 10, 10, clr);
                        }
                    }
                    component::Draw::Laser => {
                        let (w, h) = if e.vel.x().abs() > e.vel.y().abs() {
                            (36, 6)
                        } else {
                            (6, 36)
                        };
                        draw::rec(e.pos, w, h, Color::from_rgba(255, 55, 55, 255));
                    }
                }
            }
        }

        // health bar
        let h = 4;
        let mhp = self.burger.max_hp();
        let w = self.burger.hp * 8;
        let from_bot = h + 2;
        let mw = mhp * 8;
        let window_height = CENTER_Y * 2.00;
        let hp_pos = Vector2(2.00, window_height - f64::from(from_bot));
        draw::rec_top_left(hp_pos, mw, h, Color::from_rgba(155, 155, 155, 255));
        draw::rec_top_left(
            hp_pos,
            w.max(0) as _,
            h,
            Color::from_rgba(255, 105, 105, 255)
        );
        // dash bar
        let h = 2;
        let w = self.burger.dash_charge * 8.00 * 8.00;
        let dash_from_bot = from_bot + h;
        let clr = if self.burger.can_dash() {
            Color::from_rgba(255, 255, 255, 255)
        } else {
            Color::from_rgba(55, 155, 255, 255)
        };
        draw::rec_top_left(
            Vector2(2.00, window_height - f64::from(dash_from_bot)),
            w as _,
            h,
            clr
        );
    }
    fn game_is_over(&self) -> bool {
        !self.burger.is_alive() && self.freeze.abs() < 1e-10
    }
    fn reset() -> State {
        let burger_start = CENTER + Vector2(0.00, 12.00);

        State {
            difficulty: 100.00,
            score:      0,
            freeze:     0.00,
            burger:     Player::new(burger_start),
            cheese:     Cheese::new(CENTER - Vector2(0.00, 12.00), burger_start),
            particles:  Vec::new(),
            counters:   Counters::default(),
            entities:   Vec::new()
        }
    }
}
