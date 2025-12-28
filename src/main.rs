//#![windows_subsystem = "windows"]
#![allow(clippy::missing_panics_doc)]

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
    let freeze_decay = 10000 / fps;

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

        if ended.not() && state.frozen_time == 0 {
            state.run_systems(dt, &input, &asset_loader);
        }
        state.frozen_time -= std::cmp::min(freeze_decay, state.frozen_time);

        // draw calls
        set_camera(&canvas.camera);
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
    frozen_time: u32,

    difficulty: f64,
    score:      i32,
    counters:   Counters,

    entities:  Vec<Entity>,
    // instances
    burger:    Player,
    cheese:    Cheese,
    particles: Vec<Particle>
}

impl State {
    fn run_systems(&mut self, dt: f64, input: &Input, assets: &AssetLoader) {
        sys::spawn::run(self, dt);

        sys::dash::run(self, dt, input, assets);
        sys::pos::run(self, dt);
        sys::age::run(self, dt);
        sys::player_collide::run(self, assets);
        sys::cheese::run(self, assets);
        sys::friction::run(self, dt);
        sys::bound_burger::run(self);
        sys::bound_stats::run(self);

        sys::destroy_old::run(self, assets);
        sys::destroy_dead::run(&mut self.entities);

        sys::difficulty_up::run(self, dt);
    }

    fn draw(&self, asset_loader: &AssetLoader) {
        let bg = if self.frozen_time == 0 {
            BG
        } else {
            BG_ON_DAMAGE
        };
        clear_background(bg);
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
            match e.class {
                Class::Slug => {
                    copy_with_rotation(
                        asset_loader.texture("slug"),
                        e.pos,
                        e.vel.angle() + PI * 0.50
                    );
                }
                Class::Warning { delay, dir: _ } => {
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
                Class::Laser => {
                    let (w, h) = if e.vel.x().abs() > e.vel.y().abs() {
                        (36, 6)
                    } else {
                        (6, 36)
                    };
                    draw::rec(e.pos, w, h, Color::from_rgba(255, 55, 55, 255));
                }
                Class::Bullet => copy_texture(asset_loader.texture("bullet"), e.pos),
                Class::HealthPack => copy_texture(asset_loader.texture("heart"), e.pos),
                Class::Flak => copy_texture(asset_loader.texture("flak"), e.pos),
                Class::FlakChild => copy_texture(asset_loader.texture("flak_child"), e.pos),
                Class::None => ()
            }
        }

        // health bar
        let h = 4;
        let mhp = Player::max_hp();
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
        !self.burger.is_alive()
    }
    fn reset() -> State {
        let burger_start = CENTER + Vector2(0.00, 12.00);

        State {
            frozen_time: 0,
            difficulty:  100.00,
            score:       0,
            burger:      Player::new(burger_start),
            cheese:      Cheese::new(CENTER - Vector2(0.00, 12.00), burger_start),
            particles:   Vec::new(),
            counters:    Counters::default(),
            entities:    Vec::new()
        }
    }
}
