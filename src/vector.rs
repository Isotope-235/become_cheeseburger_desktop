use std::ops::Mul;

use std::ops::Add;

use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct V2(pub f64, pub f64);

impl From<f64> for V2 {
    fn from(value: f64) -> Self {
        Self(value.cos(), value.sin())
    }
}

impl V2 {
    pub const ZERO: V2 = V2(0.00, 0.00);
    pub fn square_len(self) -> f64 {
        let V2(x, y) = self;
        x.powi(2) + y.powi(2)
    }
    pub fn len(self) -> f64 {
        self.square_len().sqrt()
    }
    pub fn normal(self) -> V2 {
        let length = self.len();
        let V2(x, y) = self;
        if length > 1e-10 {
            V2(x / length, y / length)
        } else {
            V2::ZERO
        }
    }
    pub fn angle(self) -> f64 {
        let V2(x, y) = self;
        y.atan2(x)
    }
    pub fn negate(self) -> V2 {
        let V2(x, y) = self;
        V2(-x, -y)
    }
    pub fn invert(self) -> V2 {
        let V2(x, y) = self;
        V2(y, x)
    }
    pub fn rotate_once(self) -> V2 {
        let V2(x, y) = self;
        V2(-y, x)
    }
    pub fn x(self) -> f64 {
        self.0
    }
    pub fn y(self) -> f64 {
        self.1
    }
    pub fn aligned(self) -> V2 {
        let V2(x, y) = self;
        if x.abs() > y.abs() {
            V2(x, 0.00)
        } else {
            V2(0.00, y)
        }
    }
    pub fn mul_per(self, rhs: V2) -> V2 {
        let V2(x, y) = self;
        let V2(ox, oy) = rhs;
        V2(x * ox, y * oy)
    }
}

impl AddAssign<Self> for V2 {
    fn add_assign(&mut self, rhs: Self) {
        let V2(ox, oy) = rhs;
        self.0 += ox;
        self.1 += oy;
    }
}

impl Add<V2> for V2 {
    type Output = V2;

    fn add(self, rhs: V2) -> Self::Output {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<V2> for V2 {
    type Output = V2;

    fn sub(self, rhs: V2) -> Self::Output {
        let V2(sx, sy) = self;
        let V2(ox, oy) = rhs;
        V2(sx - ox, sy - oy)
    }
}

impl Mul<f64> for V2 {
    type Output = V2;

    fn mul(self, rhs: f64) -> Self::Output {
        V2(self.0 * rhs, self.1 * rhs)
    }
}