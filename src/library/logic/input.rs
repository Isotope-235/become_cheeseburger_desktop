use macroquad::prelude::*;

use crate::vector::Vector2;

pub struct Input {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}
impl Input {
    pub fn get() -> Input {
        Input {
            w: is_key_down(KeyCode::W),
            a: is_key_down(KeyCode::A),
            s: is_key_down(KeyCode::S),
            d: is_key_down(KeyCode::D),
            space: is_key_down(KeyCode::Space),
        }
    }
    pub fn dir(&self) -> Vector2 {
        let Input { w, a, s, d, .. } = *self;
        fn dir(b: bool) -> f64 {
            if b { 1.00 } else { 0.00 }
        }
        Vector2(
            dir(d) - dir(a),
            dir(s) - dir(w),
        )
    }
}
