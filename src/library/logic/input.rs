use macroquad::prelude::*;

use crate::vector::Vector2;

pub struct Input {
    pub w: Button,
    pub a: Button,
    pub s: Button,
    pub d: Button,
    pub space: Button,
}
impl Input {
    pub fn get() -> Self {
        Self {
            w: is_key_down(KeyCode::W).into(),
            a: is_key_down(KeyCode::A).into(),
            s: is_key_down(KeyCode::S).into(),
            d: is_key_down(KeyCode::D).into(),
            space: is_key_down(KeyCode::Space).into(),
        }
    }
    pub fn dir(&self) -> Vector2 {

        let Self { w, a, s, d, .. } = *self;
        Vector2(
            f64::from(d) - f64::from(a),
            f64::from(s) - f64::from(w),
        )
    }
}

#[derive(Clone, Copy)]
pub enum Button {
    Pressed,
    Released
}
impl Button {
    pub fn is_pressed(&self) -> bool {
        match self {
            Self::Pressed => true,
            Self::Released => false,
        }
    }
}

impl From<bool> for Button {
    fn from(b: bool) -> Self {
        if b { Self::Pressed } else { Self::Released }
    }
}

impl From<Button> for f64 {
    fn from(b: Button) -> Self {
        match b {
            Button::Pressed => 1.00,
            Button::Released => 0.00,
        }
    }
}