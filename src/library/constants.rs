use crate::*;

pub const TITLE: &str = "Limited Alpha v0.2.0 - Become Cheeseburger: Desktop Edition";
pub const ITERATIONS: i32 = 5;
pub const DT: f64 = 1.00 / ITERATIONS as f64;
pub const SCORE_TEXT_PARAMS: TextParams = TextParams {
    font: None,
    font_size: 80,
    font_scale: 0.125,
    font_scale_aspect: 1.00,
    color: YELLOW,
    rotation: 0.00
};

// Colors
pub const BG: Color = color_u8!(55, 55, 55, 255);
pub const BG_ON_DAMAGE: Color = color_u8!(255, 55, 55, 255);

// transform
pub const CENTER_X: f64 = 80.00;
pub const CENTER_Y: f64 = 60.00;
pub const CENTER: Vector2 = Vector2(CENTER_X, CENTER_Y);
pub const SCREEN: Vector2 = Vector2(CENTER_X * 2.00, CENTER_Y * 2.00);
pub const SCALE: f64 = 8.00;
