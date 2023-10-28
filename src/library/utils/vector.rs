use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;

/// A 2D vector with `f64` components.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2(pub f64, pub f64);

impl From<f64> for Vector2 {
    fn from(value: f64) -> Self {
        Self(value.cos(), value.sin())
    }
}
impl From<(f64, f64)> for Vector2 {
    fn from(value: (f64, f64)) -> Self {
        Self(value.0, value.1)
    }
}

impl Vector2 {
    /// The null vector.
    ///
    /// ## Example
    ///
    /// ```
    /// assert_eq!(Vector2::ZERO, Vector2(0.00, 0.00));
    /// ```
    pub const ZERO: Self = Self(0.00, 0.00);
    /// Returns the length of the vector *before* applying the square root.
    ///
    /// ## Example
    ///
    /// ```
    /// let v = Vector2(1.00, 1.00);
    /// assert_eq!(v.square_len(), 2.00);
    /// ```
    #[must_use]
    pub fn square_len(self) -> f64 {
        let Self(x, y) = self;
        x.mul_add(y, y.powi(2))
    }
    /// Returns the length of the vector. Equivalent to `self.square_len().sqrt()`.
    ///
    /// ## Example
    ///
    /// ```
    /// let v = Vector2(1.00, 1.00);
    /// assert_eq!(v.len(), 2.00f64.sqrt());
    /// ```
    #[must_use]
    pub fn len(self) -> f64 {
        self.square_len().sqrt()
    }
    /// Returns the equivalent vector with a length of 1. Essentially performs `self / self.len()`.
    #[must_use]
    pub fn normal(self) -> Self {
        let length = self.len();
        let Self(x, y) = self;
        if length > 1e-10 {
            Self(x / length, y / length)
        } else {
            Self::ZERO
        }
    }
    /// Returns the angle of the vector in radians.
    ///
    /// This method uses the `atan2` implementation, and will therefore always respect the signs of the components.
    #[must_use]
    pub fn angle(self) -> f64 {
        let Self(x, y) = self;
        y.atan2(x)
    }
    /// Returns the negated vector. Equivalent to `self * -1.00`.
    #[must_use]
    pub fn negate(self) -> Self {
        let Self(x, y) = self;
        Self(-x, -y)
    }
    /// Returns the vector with its component values swapped.
    #[must_use]
    pub fn invert(self) -> Self {
        let Self(x, y) = self;
        Self(y, x)
    }
    /// Returns the vector rotated 90 degrees counter-clockwise.
    ///
    /// ## Example
    ///
    /// ```
    /// let v = Vector2(0.00, 1.00);
    /// assert_eq!(v.rotate_once(), Vector2(-1.00, 0.00));
    /// ```
    #[must_use]
    pub fn rotate_once(self) -> Self {
        let Self(x, y) = self;
        Self(-y, x)
    }
    /// Returns the x component of the vector.
    ///
    /// ## Example
    ///
    /// ```
    /// let v = Vector2(1.00, 2.00);
    /// assert_eq!(v.x(), 1.00);
    /// ```
    pub fn x(self) -> f64 {
        self.0
    }
    /// Returns the y component of the vector.
    ///
    /// ## Example
    ///
    /// ```
    /// let v = Vector2(1.00, 2.00);
    /// assert_eq!(v.y(), 2.00);
    /// ```
    pub const fn y(self) -> f64 {
        self.1
    }
    /// Returns the vector with its x component set to the given value.
    #[must_use]
    pub fn aligned(self) -> Self {
        let Self(x, y) = self;
        if x.abs() > y.abs() {
            Self(x, 0.00)
        } else {
            Self(0.00, y)
        }
    }
    /// Returns the vector multiplied component-wise by the given vector.
    ///
    /// Should not be confused with the dot product.
    ///
    /// ## Example
    ///
    /// ```
    /// let v1 = Vector2(1.00, 2.00);
    /// let v2 = Vector2(-1.00, 2.00);
    /// assert_eq!(v1.mul_per(v2), Vector2(-1.00, 4.00));
    /// ```
    #[must_use]
    pub fn mul_per(self, rhs: Self) -> Self {
        let Self(x, y) = self;
        let Self(ox, oy) = rhs;
        Self(x * ox, y * oy)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        let Self(ox, oy) = rhs;
        self.0 += ox;
        self.1 += oy;
    }
}
impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Self(sx, sy) = self;
        let Self(ox, oy) = rhs;
        Self(sx - ox, sy - oy)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2() {
        let v = Vector2(1.00, 2.00);
        let cmp = |a: f64, b: f64| (a - b).abs() < 1e-10;
        assert!(cmp(v.square_len(), 5.00));
        assert!(cmp(v.len(), 5.00f64.sqrt()));
        assert!(cmp(v.angle(), 1.107_148_717_794_090_4));
        assert!(cmp(v.x(), 1.00));
        assert!(cmp(v.y(), 2.00));
        let mut v = Vector2(1.00, 2.00);
        v += Vector2(3.00, 4.00);
    }
}
