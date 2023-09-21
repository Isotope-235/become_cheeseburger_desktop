//#![windows_subsystem = "windows"]
pub mod input;
use std::f64::consts::PI;

use input::Input;
mod player; use crate::player::*;
mod bullet; use crate::bullet::*;
mod cheese; use crate::cheese::*;
mod slug; use crate::slug::*;
mod laser; use crate::laser::*;
mod warning; use crate::warning::*;
mod health_pack; use crate::health_pack::*;
mod flak; use crate::flak::*;
mod pos; use crate::pos::*;
mod vector; use crate::vector::*;

use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

const BG: Color = color_u8!(55, 55, 55, 255);
const TITLE: &'static str = "Limited Alpha v0.2.0 - Become Cheeseburger: Desktop Edition";
const ITERATIONS: i32 = 10;
const DT: f64 = 1.00 / ITERATIONS as f64;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_micros(16_667 / 2);

fn rand(x: f64) -> f64 {
    rand::gen_range(0.00, x)
}
fn rrange(x: i32) -> i32 {
    rand::gen_range(0, x + 1)
}
fn chance(x: f64) -> bool {
    rand(1.00) < x
}
fn center() -> vector::V2 {
    vector::V2(80.00, 60.00)
}
fn scale() -> f64 {
    8.00
}
fn fill_leading_zeroes(num: i32) -> String {
    let missing_zeroes = 5 - num.checked_ilog10().unwrap_or(0) - 1;
    let lead = "0".repeat(missing_zeroes as usize);
    let mut output = num.to_string();
    output.insert_str(0, &lead);
    output
}

fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (center().x() * 2.00 * scale()) as i32,
        window_height: (center().y() * 2.00 * scale()) as i32,
        sample_count: 0,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf())]
async fn main() {
    let joystix = load_ttf_font("joystix.otf").await.unwrap();
    // camera
    let mut camera = Camera2D::from_display_rect(Rect::new(0.00, 0.00, (center().x() * 2.00) as f32, (center().y() * 2.00) as f32));
    camera.zoom = vec2(camera.zoom.x, camera.zoom.y * -1.00);
    set_camera(&camera);
    let mut canvas = Canvas2D::new((center().x() * 2.00) as f32, (center().y() * 2.00) as f32);
    // state init
    let mut state = State::reset();

    // once-tests
    let text_params = TextParams {
        font: Some(&joystix),
        font_size: 80,
        font_scale: 0.125,
        font_scale_aspect: 1.00,
        color: YELLOW,
        ..Default::default()
    };

    let mut num = 1.00;
    let iters = 13;
    let growth = 2.00f64.powf(1.00 / iters as f64);
    for _ in 0..iters {
        num *= growth
    }

    dbg!(num);
    // we do a little bit of trolling

    // main game loop
    loop {
        // first: take the time
        let start_of_frame = std::time::Instant::now();
        // get inputs for this frame
        // canvas clear
        // core update
        let input = Input {
            w: is_key_down(KeyCode::W),
            a: is_key_down(KeyCode::A),
            s: is_key_down(KeyCode::S),
            d: is_key_down(KeyCode::D),
            space: is_key_down(KeyCode::Space)
        };
        let score_change = state.progress(&input);
        if score_change > 0 {
            state.score += score_change;
            // score_txt = render_score(state.score, &joystix, &texture_creator);
        }

        // draw calls
        set_camera(&canvas.camera);
        clear_background(BG);
        state.draw();
        let score_text = fill_leading_zeroes(state.score);
        let mut score_chars = score_text.chars();
        let mut i = 0;
        while let Some(c) = score_chars.next() {
            draw_text_ex(&c.to_owned().to_string(), 1.00 + i as f32 * 8.00, 9.00, text_params.clone());
            i += 1;
        };
        set_default_camera();
        canvas.get_texture_mut().set_filter(FilterMode::Nearest);
        canvas.draw();

        // game should only end after freeze frames are rendered, so this goes after draw calls
        if state.game_is_over() {
            // score_txt = render_score(0, &joystix, &texture_creator);
            state = State::reset()
        };

        // present

        // wait for the frame timer
        let frame_time = start_of_frame.elapsed();
        if FRAME_DURATION > frame_time {std::thread::sleep(FRAME_DURATION - frame_time)};
        next_frame().await
    }
}

struct State {
    difficulty: f64,
    score: i32,
    freeze: f64,
    burger: Pos<Player>,
    cheese: Pos<Cheese>,
    bullet_counter: f64,
    bullets: Vec<Pos<Bullet>>,
    slug_counter: f64,
    slugs: Vec<Pos<Slug>>,
    warning_counter: f64,
    warnings: Vec<Pos<Warning>>,
    lasers: Vec<Pos<Laser>>,
    health_packs_counter: f64,
    health_packs: Vec<Pos<HealthPack>>,
    flak_counter: f64,
    flaks: Vec<Pos<Flak>>,
    flak_children: Vec<Pos<FlakChild>>,
}
impl State {
    fn progress(&mut self, input: &Input) -> i32 {
        let mut score = 0;
        for _ in 0..ITERATIONS as usize {
            if self.freeze > 0.00 {
                self.freeze = (self.freeze - DT).max(0.00);
                continue;
            }
            // data saved for perf
            let diffscale = self.difficulty * 0.01;
            //

            // spawn_logic

            // bullets
            let times = self.bullet_counter.revolve(0.80 + 0.50 * diffscale);

            for _ in 0..times {
                let side = rrange(4);
                let snake_ch = diffscale * 0.225;
                if chance(snake_ch / (1.00 + snake_ch)) {
                    let direction = num_to_side(side);
                    let shift = get_shift(direction, 4.00);
                    for i in 0..((2.00 + diffscale) as i32) {
                        let delay = i as f64 * 10.00;
                        let (pos, vel) = {
                            let side_buffer = 4.00 + delay;
                            let buffer = direction * side_buffer;
                            let pos = center() + direction.mul_per(center()) + buffer;
                            (pos + shift, direction.negate())
                        };
                        let bullet = Bullet::new(
                            pos,
                            vel * 1.25,
                            delay
                        );
                        self.bullets.push(bullet);
                    }
                } else {
                    for i in 0..((diffscale * 3.00) as i32) {
                        let delay = i as f64 * 10.00;
                        let (pos, vel) = spawn_posvel_from(side, 4.00 + delay, 4.00);
                        let bullet = Bullet::new(
                            pos,
                            vel * 1.25,
                            delay
                        );
                        self.bullets.push(bullet);
                    }

                }
            }

            // cheeses

            // slugs
            let times = self.slug_counter.revolve(0.125 + 0.025 * diffscale);

            for _ in 0..times {
                let (pos, vel) = spawn_posvel(10.00, 10.00);
                let slug = Slug::new(
                    pos,
                    vel * 0.50
                );
                self.slugs.push(slug);
            }

            // warnings
            let times = self.warning_counter.revolve(0.15);

            for i in 0..(times * diffscale as i32) {
                let (mut pos, dir) = spawn_posvel(-12.00, 12.00);
                // move laser so it targets player
                let shift = rand(30.00) - 15.00;
                if dir.x().abs() < 1e-10 {
                    pos.0 = self.burger.pos.x() + shift;
                } else {
                    pos.1 = self.burger.pos.y() + shift;
                }
                self.warnings.push(Warning::new(pos, dir, i as f64 * (15.00)));
            }

            // health packs
            let times = self.health_packs_counter.revolve(0.10 * (self.burger.missing_hp() - (self.health_packs.len() * 2) as f64).max(0.00).min(8.00));

            for _ in 0..times {
                let (pos, vel) = spawn_posvel(10.00, 12.00);
                let health_pack = HealthPack::new(
                    pos,
                    vel * 0.30
                );
                self.health_packs.push(health_pack);
            }

            let times = self.flak_counter.revolve(0.10 + 0.02 * diffscale);

            for _ in 0..times {
                let (pos, vel) = spawn_posvel(4.00, 4.00);
                let flak = Flak::new(
                    pos,
                    vel * 0.50
                );
                self.flaks.push(flak);
            }

            // movement logic
            self.burger.update_pos();
            self.burger.stays_in_bounds();
            self.cheese.update_pos();
            update_all_pos(&mut self.bullets);
            update_all_pos(&mut self.slugs);
            update_all_pos(&mut self.warnings);
            update_all_pos(&mut self.lasers);
            update_all_pos(&mut self.health_packs);
            update_all_pos(&mut self.flaks);
            update_all_pos(&mut self.flak_children);

            // inter-unitary logic
            let burger_circle = self.burger.hitcircle();
            let mut state_effect = StateEffect::default();
            let mut burger_effect = Effect::default();

            if self.cheese.hitcircle().is_hitting(&burger_circle) {
                burger_effect += self.cheese.target_effect_onhit();
                self.cheese.takes_effect(&self.cheese.self_effect_onhit());
                state_effect += self.cheese.state_effect_onhit();
            };
            do_all_hits(&mut self.health_packs, &mut state_effect, &burger_circle, &mut burger_effect);

            if self.burger.is_targetable() {
                do_all_hits(&mut self.bullets, &mut state_effect, &burger_circle, &mut burger_effect);
                do_all_hits(&mut self.slugs, &mut state_effect, &burger_circle, &mut burger_effect);
                do_all_hits(&mut self.lasers, &mut state_effect, &burger_circle, &mut burger_effect);
                do_all_hits(&mut self.flaks, &mut state_effect, &burger_circle, &mut burger_effect);
                do_all_hits(&mut self.flak_children, &mut state_effect, &burger_circle, &mut burger_effect);
                self.burger.takes_effect(&burger_effect);
            }
            state_effect.freeze += burger_effect.damage;
            let StateEffect { score: added_score, freeze } = state_effect;
            score += added_score;
            self.freeze += freeze;
            if self.freeze > 0.00 { // making sure that the player sees the fatal projectile
                self.freeze = (self.freeze - DT).max(0.00);
                continue;
            }


            // special update behaviour
            { // burger
                let ref mut burger = self.burger;
                burger.vel = input.dir().normal() * (0.55) * DT + burger.vel * 0.675f64.powf(DT);
                burger.bhv.invuln = (burger.bhv.invuln - DT).max(0.00);
                burger.bhv.dash_charge = (burger.bhv.dash_charge + 0.01 * DT).min(1.00);
                burger.bhv.hp = burger.bhv.hp.min(burger.max_hp());
                if input.space && burger.can_dash() && input.dir().len() > 0.00 {
                    burger.dash(input);
                }
            };
            
            
            { // cheese
                let ref mut cheese = self.cheese;
                if cheese.bhv.hp < 1e-10 {
                    let V2(x, y) = center();
                    cheese.pos = V2(rand(x), rand(y)) + center() * 0.50;
                    cheese.bhv.hp = 1.00;
                }
            };
            { // bullets
                // nothing for now
            };
            { // slugs
                // nothing for now
            };
            { // warnings
                // nothing for now
            };
            { // lasers
                // nothing for now
            };
            { // health packs
                // nothing for now
            };
            { // flaks
                // nothing for now
            };

            // remove elements
            self.bullets.retain(|b| b.age < 750.00 && !b.should_be_removed());
            self.slugs.retain(|s| s.age < 1500.00 && !s.should_be_removed());
            for warning in &self.warnings {
                if warning.should_be_removed() {
                    let dir = warning.dir();
                    let laser = Laser::new(warning.pos - dir * 40.00, dir * 7.00);
                    self.lasers.push(laser);
                }
            }
            self.warnings.retain(|w| !w.should_be_removed());
            self.lasers.retain(|l| !l.should_be_removed());
            self.health_packs.retain(|hp| !hp.should_be_removed());
            for flak in &self.flaks {
                if flak.should_be_removed() {
                    let number = 8;
                    for i in 0..number {
                        let dir = i as f64 * PI * 2.00 / number as f64;
                        let child = FlakChild::new(flak.pos, V2::ZERO, V2::from(dir) * 0.01);
                        self.flak_children.push(child);
                    }
                }
            }
            self.flaks.retain(|f| !f.should_be_removed());
            self.flak_children.retain(|c| !c.should_be_removed());

            // up difficulty
            self.difficulty += 0.10 * DT;
        };
        score
    }
    fn draw(&self) {
        // burger
        let burger_color = {
            match self.burger.bhv.invuln > 0.00 {
                true => Color::from_rgba(255, 255, 255, 255),
                false => Color::from_rgba(155, 155, 255, 255),
            }
        };
        draw_rec(self.burger.pos, 8, 8, burger_color);
        // cheese
        draw_rec(self.cheese.pos, 6, 6, YELLOW);
        // health packs
        for health_pack in &self.health_packs {
            draw_rec(health_pack.pos, 6, 6, Color::from_rgba(55, 255, 55, 255));
        }
        // bullets
        let bullet_color = Color::from_rgba(255, 155, 155, 255);
        for bullet in &self.bullets {
            draw_rec(bullet.pos, 6, 6, bullet_color);
        }
        // slugs
        for slug in &self.slugs {
            draw_rec(slug.pos, 20, 20, bullet_color);
        }
        // warnings
        for warning in &self.warnings {
            if warning.is_visible() {
                let dur = 6.00;
                let clr = match warning.age % dur < dur * 0.50 {
                    true => Color::from_rgba(255, 55, 55, 255),
                    false => Color::from_rgba(255, 255, 55, 255),
                };
                draw_rec(warning.pos, 10, 10, clr)
            }
        }
        // lasers
        for laser in &self.lasers {
            let (w, h) = match laser.vel.x().abs() > laser.vel.y().abs() {
                true => (36, 6),
                false => (6, 36),
            };
            draw_rec(laser.pos, w, h, Color::from_rgba(255, 55, 55, 255));
        }
        // flak
        for flak in &self.flaks {
            draw_rec(flak.pos, 14, 14, Color::from_rgba(255, 155, 155, 255));
        }
        // flak children
        for flak_child in &self.flak_children {
            draw_rec(flak_child.pos, 4, 4, Color::from_rgba(255, 155, 155, 255));
        }
        // health bar
        let h = 4;
        let mhp = self.burger.max_hp();
        let w = self.burger.bhv.hp * 8.00;
        let from_bot = h + 2;
        let mw = mhp * 8.00;
        let window_height = center().y() * 2.00;
        let hp_pos = V2(2.00, window_height - from_bot as f64);
        draw_rec_top_left(hp_pos, mw as i32, h, Color::from_rgba(155, 155, 155, 255));
        draw_rec_top_left(hp_pos, w.max(0.00) as i32, h, Color::from_rgba(255, 105, 105, 255));
        // dash bar
        let h = 2;
        let w = self.burger.bhv.dash_charge * 8.00 * 8.00;
        let dash_from_bot = from_bot + h;
        let clr = match self.burger.can_dash() {
            true => Color::from_rgba(255, 255, 255, 255),
            false => Color::from_rgba(55, 155, 255, 255),
        };
        draw_rec_top_left(V2(2.00, window_height - dash_from_bot as f64), w as i32, h, clr);
    }
    fn game_is_over(&self) -> bool {
        !self.burger.is_alive() && self.freeze.abs() < 1e-10
    }
    fn reset() -> State {
        State {
            difficulty: 100.00,
            score: 0,
            freeze: 0.00,
            burger: Player::new(center()),
            cheese: Cheese::new(center() - V2(4.00, 4.00)),
            bullet_counter: 0.00,
            bullets: Vec::new(),
            slug_counter: 0.00,
            slugs: Vec::new(),
            warning_counter: 0.00,
            warnings: Vec::new(),
            lasers: Vec::new(),
            health_packs_counter: 0.00,
            health_packs: Vec::new(),
            flak_counter: 0.00,
            flaks: Vec::new(),
            flak_children: Vec::new(),
        }
    }
}
pub trait Counter {
    fn revolve(&mut self, delta: f64) -> i32;
}
impl Counter for f64 {
    fn revolve(&mut self, delta: f64) -> i32 {
        *self = *self + delta * DT;
        let times = *self as i32 / 100;
        *self = *self % 100.00;
        times
    }
}
fn num_to_side(num: i32) -> V2 {
    match num % 4 {
        1 => V2(1.00, 0.00),
        2 => V2(-1.00, 0.00),
        3 => V2(0.00, 1.00),
        0 => V2(0.00, -1.00),
        _ => panic!("dear god")
    }
}
fn get_rand_dir() -> V2 {
    num_to_side(rrange(4))
}
fn spawn_posvel(side_buffer: f64, edge_buffer: f64) -> (V2, V2) {
    let direction = get_rand_dir();
    let shift = get_shift(direction, edge_buffer);
    let buffer = direction * side_buffer;
    let pos = center() + direction.mul_per(center()) + buffer;
    (pos + shift, direction.negate())
}
fn spawn_posvel_from(side: i32, side_buffer: f64, edge_buffer: f64) -> (V2, V2) {
    let direction = num_to_side(side);
    let shift = get_shift(direction, edge_buffer);
    let buffer = direction * side_buffer;
    let pos = center() + direction.mul_per(center()) + buffer;
    (pos + shift, direction.negate())
}
fn get_shift(dir: V2, edge_buffer: f64) -> V2 {
    let rot_dir = dir.rotate_once();
    let shift_range = rot_dir.mul_per(center()).len() - edge_buffer;
    rot_dir * (rand(shift_range * 2.00) - shift_range)
}
fn draw_rec(pos: V2, w: i32, h: i32, color: Color) {
    debug_assert!(w % 2 == 0);
    debug_assert!(h % 2 == 0);
    let (half_w, half_h) = (w / 2, h / 2);
    draw_rectangle(pos.x() as f32 - half_w as f32, pos.y() as f32 - half_h as f32, w as f32, h as f32, color);
}
fn draw_rec_top_left(pos: V2, w: i32, h: i32, color: Color) {
    draw_rectangle(pos.x() as f32, pos.y() as f32, w as f32, h as f32, color);
}