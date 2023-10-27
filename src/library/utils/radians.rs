pub trait Radians {
    fn as_radians(&self) -> f64;
}
impl Radians for f64 {
    fn as_radians(&self) -> f64 {
        *self * std::f64::consts::PI * 2.00
    }
}
impl Radians for i32 {
    fn as_radians(&self) -> f64 {
        f64::from(*self).as_radians()
    }
}
