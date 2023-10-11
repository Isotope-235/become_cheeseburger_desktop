use crate::*;
use macroquad::prelude::*;

pub const TITLE: &str = "Limited Alpha v0.2.0 - Become Cheeseburger: Desktop Edition";
pub const ITERATIONS: i32 = 5;
pub const DT: f64 = 1.00 / ITERATIONS as f64;

// Colors
pub const BG: Color = color_u8!(55, 55, 55, 255);
pub const BG_ON_DAMAGE: Color = color_u8!(255, 55, 55, 255);

pub fn center() -> Vector2 {
    Vector2(80.00, 60.00)
}

pub fn scale() -> f64 {
    8.00
}
