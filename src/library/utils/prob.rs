use crate::*;

pub fn rand(x: f64) -> f64 {
    rand::gen_range(0.00, x)
}

pub fn rrange(x: i32) -> i32 {
    rand::gen_range(0, x + 1)
}

pub fn chance(x: f64) -> bool {
    rand(1.00) < x
}

pub fn get_rand_dir() -> Vector2 {
    num_to_side(rrange(4))
}
