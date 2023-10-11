use crate::vector::Vector2;

pub struct Input {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}
impl Input {
    pub fn init() -> Input {
        Input {
            w: false,
            a: false,
            s: false,
            d: false,
            space: false,
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
