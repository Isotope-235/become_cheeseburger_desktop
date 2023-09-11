//#![windows_subsystem = "windows"]
pub mod input; use input::Input;
mod player; use crate::player::*;
mod bullet; use crate::bullet::*;
mod cheese; use crate::cheese::*;
mod slug; use crate::slug::*;
mod laser; use crate::laser::*;
mod warning; use crate::warning::*;
mod health_pack; use crate::health_pack::*;
mod pos; use crate::pos::*;
mod vector; use crate::vector::*;

use sdl2::rect::Point;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use rand::prelude::*;

use sdl2::{self, pixels::Color, rect::Rect, render::Canvas, video::Window};

const BG: Color = Color::RGB(55, 55, 55);
const TITLE: &'static str = "Limited Alpha v0.2.0 - Become Cheeseburger: Desktop Edition";
const ITERATIONS: i32 = 10;
const DT: f64 = 1.00 / ITERATIONS as f64;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_micros(16_667);

fn feq(x: f64, y: f64) -> bool {
    (x - y).abs() < 1e-10
}
fn rand(x: f64) -> f64 {
    rand::thread_rng().gen::<f64>() * x
}
fn rrange(x: i32) -> i32 {
    rand::thread_rng().gen_range(1..=x)
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
fn render_score<'a, 'b>(score: i32, font: &Font<'a, 'b>, texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a> {
    font.render(&fill_leading_zeroes(score)).solid(Color::YELLOW).unwrap().as_texture(&texture_creator).unwrap()
}

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let fontext = sdl2::ttf::init().unwrap();
    let joystix = fontext.load_font("joystix.otf", 10).unwrap();

    let V2(w, h) = center();
    let window = video
        .window(
            TITLE,
            (w * 2.00 * scale()) as u32,
            (h * 2.00 * scale()) as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut score_txt = render_score(0, &joystix, &texture_creator);

    // ready canvas for first frame
    canvas.set_integer_scale(true).unwrap();
    canvas.set_scale(8.00, 8.00).unwrap();
    canvas.set_draw_color(BG);
    canvas.clear();
    canvas.present();

    // state init
    let mut input = Input::init();
    let mut state = State::reset();

    // once-tests

    // we do a little bit of trolling
    let mut event_pump = context.event_pump().unwrap();

    // main game loop
    'game: loop {
        // first: take the time
        let start_of_frame = std::time::Instant::now();
        // get inputs for this frame
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => {
                    break 'game;
                }
                Event::KeyDown { keycode, repeat, .. } => {
                        if !repeat {
                            if let Some(key) = keycode {
                                use sdl2::keyboard::Keycode;
                                match key {
                                    Keycode::Escape => {
                                        break 'game;
                                    }
                                    Keycode::W => input.w = true,
                                    Keycode::A => input.a = true,
                                    Keycode::S => input.s = true,
                                    Keycode::D => input.d = true,
                                    Keycode::Space => input.space = true,
                                    _ => {}
                                }
                            }
                        }
                    }   
                Event::KeyUp { keycode, repeat, .. } => {
                    if !repeat {
                        if let Some(key) = keycode {
                            use sdl2::keyboard::Keycode;
                            match key {
                                Keycode::W => input.w = false,
                                Keycode::A => input.a = false,
                                Keycode::S => input.s = false,
                                Keycode::D => input.d = false,
                                Keycode::Space => input.space = false,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        // canvas clear
        canvas.set_draw_color(BG);
        canvas.clear();
        // core update
        let score_change = state.progress(&input);
        if score_change > 0 {
            state.score += score_change;
            score_txt = render_score(state.score, &joystix, &texture_creator);
        }
        if state.game_is_over() {
            score_txt = render_score(0, &joystix, &texture_creator);
            state = State::reset()
        };

        // draw calls
        state.draw(&mut canvas);
        let TextureQuery { width, height, .. } = score_txt.query();
        canvas.copy(&score_txt, None, Rect::from_center(Point::new(width as i32 / 2, height as i32 / 2), width, height)).unwrap();

        // present
        canvas.present();

        // wait for the frame timer
        let frame_time = start_of_frame.elapsed();
        std::thread::sleep(if FRAME_DURATION > frame_time {FRAME_DURATION - frame_time} else {std::time::Duration::ZERO})
    }
}

struct State {
    difficulty: f64,
    score: i32,
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
}
impl State {
    fn progress(&mut self, input: &Input) -> i32 {
        let mut score = 0;
        for _ in 0..ITERATIONS as usize {
            // data saved for perf
            let diffscale = self.difficulty * 0.01;
            //

            // spawn_logic

            // bullets
            let times = self.bullet_counter.revolve(1.00);

            for _ in 0..times {
                let side = rrange(4);
                let snake_ch = diffscale * 0.40;
                if chance(snake_ch / (1.00 + snake_ch)) {
                    let direction = num_to_side(side);
                    let shift = get_shift(direction, 4.00);
                    for i in 0..((diffscale * 3.00) as i32) {
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
            let times = self.slug_counter.revolve(0.20 * diffscale);

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
            let times = self.health_packs_counter.revolve(0.10 * self.burger.missing_hp().max(0.00).min(8.00));
            
            for _ in 0..times {
                let (pos, vel) = spawn_posvel(10.00, 12.00);
                let health_pack = HealthPack::new(
                    pos,
                    vel * 0.30
                );
                self.health_packs.push(health_pack);
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
                self.burger.takes_effect(&burger_effect);
            }
            let StateEffect { score: added_score } = state_effect;
            score += added_score;
            

            // special update behaviour
            self.burger.update_bhv(input);
            self.cheese.update_bhv(input);
            update_all_bhv(&mut self.bullets, input);
            update_all_bhv(&mut self.slugs, input);
            update_all_bhv(&mut self.warnings, input);
            update_all_bhv(&mut self.lasers, input);
            update_all_bhv(&mut self.health_packs, input);

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

            // up difficulty
            self.difficulty += 0.10 * DT;
        };
        score
    }
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // burger
        canvas.set_draw_color(self.burger.color());
        draw::rect(canvas, self.burger.pos, 8, 8);
        // cheese
        canvas.set_draw_color(Color::YELLOW);
        draw::rect(canvas, self.cheese.pos, 6, 6);
        // health packs
        canvas.set_draw_color(Color::RGB(55, 255, 55));
        for health_pack in &self.health_packs {
            draw::rect(canvas, health_pack.pos, 6, 6)
        }
        // bullets
        canvas.set_draw_color(Color::RGB(255, 155, 155));
        for bullet in &self.bullets {
            draw::rect(canvas, bullet.pos, 6, 6)
        }
        // slugs
        for slug in &self.slugs {
            draw::rect(canvas, slug.pos, 20, 20)
        }
        // warnings
        for warning in &self.warnings {
            if warning.is_visible() {
                let clr = match warning.age % 6.00 < 3.00 {
                    true => Color::RGB(255, 55, 55),
                    false => Color::RGB(255, 255, 55),
                };
                canvas.set_draw_color(clr);
                draw::rect(canvas, warning.pos, 10, 10)
            }
        }
        // lasers
        canvas.set_draw_color(Color::RGB(255, 55, 55));
        for laser in &self.lasers {
            let (w, h) = match laser.vel.x().abs() > laser.vel.y().abs() {
                true => (36, 6),
                false => (6, 36),
            };
            draw::rect(canvas, laser.pos, w, h)
        }
        // health bar
        let h = 4;
        let halfh = h / 2;
        let mhp = self.burger.max_hp();
        let w = self.burger.bhv.hp * 8.00;
        let from_left = w * 0.50 + 2.00;
        let from_bot = halfh + 2;
        let mw = mhp * 8.00;
        let window_height = center().y() * 2.00;
        canvas.set_draw_color(Color::RGB(155, 155, 155));
        draw::rect(canvas, V2(mw * 0.50 + 2.00, window_height - from_bot as f64), mw as u32, h);
        canvas.set_draw_color(Color::RGB(255, 55, 55));
        draw::rect(canvas, V2(from_left as f64, window_height - from_bot as f64), w as u32, h);
        // dash bar
        let h = 2;
        let halfh = h / 2;
        let mdc = 1.00;
        let w = self.burger.bhv.dash_charge * 8.00 * 8.00;
        let from_left = w * 0.50 + 2.00;
        let from_bot = halfh + 6;
        let mw = mdc * 8.00;
        let clr = match self.burger.can_dash() {
            true => Color::RGB(255, 255, 255),
            false => Color::RGB(55, 155, 255),
        };
        canvas.set_draw_color(clr);
        draw::rect(canvas, V2(from_left as f64, window_height - from_bot as f64), w as u32, h);
    }
    fn game_is_over(&self) -> bool {
        !self.burger.is_alive()
    }
    fn reset() -> State {
        State {
            difficulty: 100.00,
            score: 0,
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
    match num {
        1 => V2(1.00, 0.00),
        2 => V2(-1.00, 0.00),
        3 => V2(0.00, 1.00),
        4 => V2(0.00, -1.00),
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
pub mod draw {
    use sdl2::{rect::Rect, render::Canvas, video::Window};

    use crate::vector::V2;

    pub fn rect(canvas: &mut Canvas<Window>, pos: V2, w: u32 , h: u32) {
        canvas.fill_rect(Rect::from_center(pos, w, h)).unwrap()
    }
}