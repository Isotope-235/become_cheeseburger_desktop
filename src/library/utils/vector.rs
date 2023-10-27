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
    pub const ZERO: Vector2 = Vector2(0.00, 0.00);
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
        let Vector2(x, y) = self;
        x.powi(2) + y.powi(2)
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
    pub fn normal(self) -> Vector2 {
        let length = self.len();
        let Vector2(x, y) = self;
        if length > 1e-10 {
            Vector2(x / length, y / length)
        } else {
            Vector2::ZERO
        }
    }
    /// Returns the angle of the vector in radians.
    ///
    /// This method uses the `atan2` implementation, and will therefore always respect the signs of the components.
    #[must_use]
    pub fn angle(self) -> f64 {
        let Vector2(x, y) = self;
        y.atan2(x)
    }
    /// Returns the negated vector. Equivalent to `self * -1.00`.
    #[must_use]
    pub fn negate(self) -> Vector2 {
        let Vector2(x, y) = self;
        Vector2(-x, -y)
    }
    /// Returns the vector with its component values swapped.
    #[must_use]
    pub fn invert(self) -> Vector2 {
        let Vector2(x, y) = self;
        Vector2(y, x)
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
    pub fn rotate_once(self) -> Vector2 {
        let Vector2(x, y) = self;
        Vector2(-y, x)
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
    pub fn y(self) -> f64 {
        self.1
    }
    /// Returns the vector with its x component set to the given value.
    #[must_use]
    pub fn aligned(self) -> Vector2 {
        let Vector2(x, y) = self;
        if x.abs() > y.abs() {
            Vector2(x, 0.00)
        } else {
            Vector2(0.00, y)
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
    pub fn mul_per(self, rhs: Vector2) -> Vector2 {
        let Vector2(x, y) = self;
        let Vector2(ox, oy) = rhs;
        Vector2(x * ox, y * oy)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        let Vector2(ox, oy) = rhs;
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

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        let Vector2(sx, sy) = self;
        let Vector2(ox, oy) = rhs;
        Vector2(sx - ox, sy - oy)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2() {
        let v = Vector2(1.00, 2.00);
        assert_eq!(v.square_len(), 5.00);
        assert_eq!(v.len(), 5.00f64.sqrt());
        assert_eq!(v.angle(), 1.1071487177940904);
        assert_eq!(v.x(), 1.00);
        assert_eq!(v.y(), 2.00);
        let mut v = Vector2(1.00, 2.00);
        v += Vector2(3.00, 4.00);
    }
}
