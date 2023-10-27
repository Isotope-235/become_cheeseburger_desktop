//#![windows_subsystem = "windows"]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::wildcard_imports, clippy::must_use_candidate, clippy::too_many_lines)]

use std::f64::consts::PI;

use macroquad::{prelude::*, rand::ChooseRandom};
use macroquad_canvas::Canvas2D;

use library::*;

pub mod library;

#[macroquad::main(window())]
async fn main() {
    
    // get fps via cringe
    let fps = fps::find().await;
    println!("Targeted framerate: {fps}");

    // fps-based timestep
    let dt = DT * 60.00 / f64::from(fps);
    
    // pixel perfection
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.00,
        0.00,
        SCREEN_X as f32,
        SCREEN_Y as f32,
    ));
    camera.zoom = vec2(camera.zoom.x, camera.zoom.y * -1.00);
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
        ]).await
        .load_sprites(vec![
            ("cheese", Color::from_rgba(255, 221, 86, 255)),
            ("heart", Color::from_rgba(221, 16, 85, 255)),
        ]).await
        .load_sounds(vec!["explosion", "heal", "laser", "damage", "dash"])
        .await;

    let joystix = load_ttf_font("joystix.otf").await.unwrap();

    // state init
    let mut state = State::reset();

    // tests
    let text_params = TextParams {
        font: Some(&joystix),
        ..SCORE_TEXT_PARAMS
    };

    // main game loop
    loop {
        // get inputs for this frame
        let input = Input::get();
        state.progress(&input, dt, &asset_loader);
        state.score += state.score_last_frame;

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
                &c.to_owned().to_string(),
                1.00 + i as f32 * 8.00,
                9.00,
                text_params.clone(),
            );
        }
        set_default_camera();
        canvas.draw();

        // game should only end after freeze frames are rendered, so this goes after draw calls
        if state.game_is_over() {
            state = State::reset();
        };

        // present

        // wait for the frame timer
        next_frame().await;
    }
}

pub struct Units<T> {
    pub s: Vec<T>,
    pub counter: f64,
}
impl<T> Units<T> {
    fn new() -> Self {
        Units {
            s: Vec::new(),
            counter: 0.00,
        }
    }
}
struct State {
    difficulty: f64,
    score: i32,
    score_last_frame: i32,
    freeze: f64,
    burger: Pos<Player>,
    cheese: Pos<Cheese>,
    bullet: Units<Pos<Bullet>>,
    slug: Units<Pos<Slug>>,
    warning: Units<Pos<Warning>>,
    lasers: Vec<Pos<Laser>>,
    health_pack: Units<Pos<HealthPack>>,
    frag: Units<Pos<Frag>>,
    frag_children: Vec<Pos<frag::Child>>,
    particles: Vec<Pos<Particle>>,
    cross_counter: f64,
}

impl State {
    fn progress(&mut self, input: &Input, dt: f64, asset_loader: &AssetLoader) {
        let mut score = 0;
        for _ in 0..ITERATIONS {
            if self.freeze > 0.00 {
                self.freeze = (self.freeze - dt).max(0.00);
                continue;
            }
            // data saved for perf
            let diff_scale = self.difficulty * 0.01;
            //

            // spawn_logic

            // bullets
            let times = self.bullet.counter.revolve(1.10 + 0.25 * diff_scale, dt);

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
                        let bullet = Bullet::new(pos, vel * 1.25, delay);
                        self.bullet.s.push(bullet);
                    }
                } else {
                    for i in 0..((1.00 + diff_scale * 2.00) as i32) {
                        let delay = f64::from(i) * 10.00;
                        let (pos, vel) = spawn_pos_vel_from(side, 4.00 + delay, 4.00);
                        let bullet = Bullet::new(pos, vel * 1.25, delay);
                        self.bullet.s.push(bullet);
                    }
                }
            }

            // cheeses

            // slugs
            let times = self.slug.counter.revolve(0.125 + 0.025 * diff_scale, dt);

            for _ in 0..times {
                let (pos, vel) = spawn_pos_vel(10.00, 10.00);
                let slug = Slug::new(pos, vel * 0.50);
                self.slug.s.push(slug);
            }

            // warnings
            let times = self.warning.counter.revolve(0.15 + 0.10 * diff_scale, dt);

            for i in 0..(times * diff_scale as i32) {
                let (mut pos, dir) = spawn_pos_vel(-12.00, 12.00);
                // move laser so it targets player
                let shift = rand(30.00) - 15.00;
                if dir.x().abs() < 1e-10 {
                    pos.0 = self.burger.pos.x() + shift;
                } else {
                    pos.1 = self.burger.pos.y() + shift;
                }
                self.warning
                    .s
                    .push(Warning::new(pos, dir, f64::from(i) * (15.00)));
            }

            // health packs
            let times = self.health_pack.counter.revolve(
                0.10 * (self.burger.missing_hp() - (self.health_pack.s.len() * 2) as f64)
                    .max(0.00)
                    .min(8.00),
                dt,
            );

            for _ in 0..times {
                let (pos, vel) = spawn_pos_vel(10.00, 12.00);
                let health_pack = HealthPack::new(pos, vel * 0.30);
                self.health_pack.s.push(health_pack);
            }

            // frag
            let times = self.frag.counter.revolve(0.10 + 0.02 * diff_scale, dt);

            for _ in 0..times {
                let (pos, vel) = spawn_pos_vel(4.00, 4.00);
                let frag = Frag::new(pos, vel * 0.50);
                self.frag.s.push(frag);
            }

            let times = self
                .cross_counter
                .revolve((-0.25 + 0.135 * diff_scale).max(0.00), dt);

            for _ in 0..times {
                for i in 0..4 {
                    let starting_point = SCREEN.mul_per(num_to_corner(i));
                    let direction = CENTER - starting_point;
                    let vel = direction.normal();
                    for ii in 0..3 {
                        self.bullet.s.push(Bullet::new(
                            starting_point - vel * 10.00 * f64::from(ii),
                            vel * 1.75,
                            0.00,
                        ));
                    }
                }
            }

            // movement logic
            self.burger.update_pos(dt);
            self.burger.stays_in_bounds();
            self.cheese.update_pos(dt);
            pos::update_all(&mut self.bullet.s, dt);
            pos::update_all(&mut self.slug.s, dt);
            pos::update_all(&mut self.warning.s, dt);
            pos::update_all(&mut self.lasers, dt);
            pos::update_all(&mut self.health_pack.s, dt);
            pos::update_all(&mut self.frag.s, dt);
            pos::update_all(&mut self.frag_children, dt);
            pos::update_all(&mut self.particles, dt);

            // inter-unitary logic
            let burger_circle = self.burger.hit_circle();
            let mut state_effect = StateEffect::default();

            if self.cheese.hit_circle().is_hitting(&burger_circle) {
                self.cheese.takes_effect(&self.cheese.self_effect_on_hit());
                state_effect += self.cheese.effect_on_hit(asset_loader);
            };
            let hit_info = &mut HitInfo {
                state_effect_accumulator: &mut state_effect,
                burger_circle: &burger_circle,
                asset_loader,
            };
            do_all_hits(&mut self.health_pack.s, hit_info);
            if self.burger.is_targetable() {
                do_all_hits(&mut self.bullet.s, hit_info);
                do_all_hits(&mut self.slug.s, hit_info);
                do_all_hits(&mut self.lasers, hit_info);
                do_all_hits(&mut self.frag.s, hit_info);
                do_all_hits(&mut self.frag_children, hit_info);
            }
            if state_effect.burger_damage > 0.00 {
                asset_loader.play_sound("damage");
            }
            self.takes_effect(state_effect, &mut score);
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
                burger.bhv.invuln = (burger.bhv.invuln - dt).max(0.00);
                burger.bhv.dash_charge = (burger.bhv.dash_charge + 0.01 * dt).min(1.00);
                burger.bhv.hp = burger.bhv.hp.min(burger.max_hp());
                if input.space.is_pressed() && burger.can_dash() && input.dir().len() > 0.00 {
                    burger.dash(input, asset_loader);
                }
            };
            {
                // cheese
                let cheese = &mut self.cheese;
                if cheese.bhv.hp < 1e-10 {
                    let Vector2(x, y) = CENTER;
                    let new_pos = loop {
                        let maybe_pos = Vector2(rand(x), rand(y)) + CENTER * 0.50;
                        if (self.burger.pos - maybe_pos).len() > 16.00 {
                            break maybe_pos;
                        }
                    };
                    cheese.pos = new_pos;
                    cheese.bhv.hp = 1.00;
                }
            };
            { // bullets
                 // nothing for now
            };
            { // slugs
                 // nothing for now
            };
            {
                // warning.s
                for warning in &self.warning.s {
                    if !warning.will_live() {
                        let dir = warning.dir();
                        let laser = Laser::new(warning.pos - dir * 40.00, dir * 7.00);
                        asset_loader.play_sound("laser");
                        self.lasers.push(laser);
                    }
                }
            };
            { // lasers
                 // nothing for now
            };
            { // health packs
                 // nothing for now
            };
            {
                // frags
                for frak in &self.frag.s {
                    if !frak.will_live() {
                        let number = 8;
                        for i in 0..number {
                            let dir = f64::from(i).as_radians() / f64::from(number);
                            let child =
                                frag::Child::new(frak.pos, Vector2::ZERO, Vector2::from(dir) * 0.01);
                            self.frag_children.push(child);
                        }
                    }
                }
            };
            { // frag children
                self.frag_children.shuffle();
            };
            {
                // particles
                for particle in &mut self.particles {
                    particle.vel *= (1.00 - particle.bhv.friction).powf(dt);
                }
            };

            // remove elements
            self.bullet.s.retain(|b| b.age < 750.00 && b.bhv.hp > 1e-10);
            self.slug.s.retain(|s| s.age < 1500.00 && s.bhv.hp > 1e-10);
            self.warning.s.retain(Pos::<Warning>::will_live);
            self.lasers.retain(|l| l.age < 500.00 && l.bhv.hp > 1e-10);
            self.health_pack
                .s
                .retain(|hp| hp.age < 500.00 && hp.bhv.hp > 1e-10);
            self.frag.s.retain(Pos::<Frag>::will_live);
            self.frag_children
                .retain(|c| c.age < 300.00 && c.bhv.hp > 1e-10);
            self.particles.retain(|p| p.age <= p.bhv.lifetime);

            // up difficulty
            self.difficulty += 0.10 * dt;
        }
        self.score_last_frame = score;
    }
    fn draw(&self, asset_loader: &AssetLoader) {
        // burger
        let b_sprite = if self.burger.bhv.invuln > 0.00 {
            asset_loader.texture("burger_invuln")
        } else {
            asset_loader.texture("burger")
        };
        copy_texture(b_sprite, self.burger.pos);
        // cheese
        copy_texture(asset_loader.texture("cheese"), self.cheese.pos);
        // health packs
        for health_pack in &self.health_pack.s {
            copy_texture(asset_loader.texture("heart"), health_pack.pos);
        }
        // bullets
        for bullet in &self.bullet.s {
            copy_texture(asset_loader.texture("bullet"), bullet.pos);
        }
        // slugs
        for slug in &self.slug.s {
            copy_with_rotation(
                asset_loader.texture("slug"),
                slug.pos,
                slug.vel.angle() + PI * 0.50,
            );
        }
        // warning.s
        for warning in &self.warning.s {
            if warning.is_visible() {
                let dur = 6.00;
                let clr = if warning.age % dur < dur * 0.50 {
                    Color::from_rgba(255, 55, 55, 255)
                } else {
                    Color::from_rgba(255, 255, 55, 255)
                };
                draw::rec(warning.pos, 10, 10, clr);
            }
        }
        // lasers
        for laser in &self.lasers {
            let (w, h) = if laser.vel.x().abs() > laser.vel.y().abs() {
                (36, 6)
            } else {
                (6, 36)
            };
            draw::rec(laser.pos, w, h, Color::from_rgba(255, 55, 55, 255));
        }
        // flak
        for flak in &self.frag.s {
            copy_texture(asset_loader.texture("flak"), flak.pos);
        }
        // flak children
        for flak_child in &self.frag_children {
            copy_texture(asset_loader.texture("flak_child"), flak_child.pos);
        }
        // particles
        for particle in &self.particles {
            let (w, h) = (2, 2);
            draw::rec(particle.pos, w, h, particle.bhv.color);
        }
        // health bar
        let h = 4;
        let mhp = self.burger.max_hp();
        let w = self.burger.bhv.hp * 8.00;
        let from_bot = h + 2;
        let mw = mhp * 8.00;
        let window_height = CENTER_Y * 2.00;
        let hp_pos = Vector2(2.00, window_height - f64::from(from_bot));
        draw::rec_top_left(hp_pos, mw as i32, h, Color::from_rgba(155, 155, 155, 255));
        draw::rec_top_left(
            hp_pos,
            w.max(0.00) as i32,
            h,
            Color::from_rgba(255, 105, 105, 255),
        );
        // dash bar
        let h = 2;
        let w = self.burger.bhv.dash_charge * 8.00 * 8.00;
        let dash_from_bot = from_bot + h;
        let clr = if self.burger.can_dash() {
            Color::from_rgba(255, 255, 255, 255)
        } else {
            Color::from_rgba(55, 155, 255, 255)
        };
        draw::rec_top_left(
            Vector2(2.00, window_height - f64::from(dash_from_bot)),
            w as i32,
            h,
            clr,
        );
    }
    fn game_is_over(&self) -> bool {
        !self.burger.is_alive() && self.freeze.abs() < 1e-10
    }
    fn reset() -> State {
        State {
            difficulty: 100.00,
            score: 0,
            score_last_frame: 0,
            freeze: 0.00,
            burger: Player::new(CENTER + Vector2(0.00, 12.00)),
            cheese: Cheese::new(CENTER - Vector2(0.00, 12.00)),
            bullet: Units::new(),
            slug: Units::new(),
            warning: Units::new(),
            lasers: Vec::new(),
            health_pack: Units::new(),
            frag: Units::new(),
            frag_children: Vec::new(),
            particles: Vec::new(),
            cross_counter: 0.00,
        }
    }
    fn takes_effect(&mut self, effect: StateEffect, score_accumulator: &mut i32) {
        *score_accumulator += effect.score;
        self.freeze += effect.freeze + effect.burger_damage.max(0.00);
        self.burger.bhv.hp -= effect.burger_damage;
        self.particles.extend(effect.particles);
    }
}




