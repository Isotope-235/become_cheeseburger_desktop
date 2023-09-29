use macroquad::prelude::*;

pub const TITLE: &'static str = "Limited Alpha v0.2.0 - Become Cheeseburger: Desktop Edition";
pub const ITERATIONS: i32 = 5;
pub const DT: f64 = 1.00 / ITERATIONS as f64;

// farger
pub const HEART_RED: Color = color_u8!(221, 16, 85, 255);
pub const BG: Color = color_u8!(55, 55, 55, 255);
pub const BG_ON_DAMAGE: Color = color_u8!(255, 55, 55, 255);
pub const CHEESE_YELLOW: Color = color_u8!(255, 221, 86, 255);