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
        Vector2(
            d as i32 as f64 - a as i32 as f64,
            s as i32 as f64 - w as i32 as f64,
        )
    }
}
