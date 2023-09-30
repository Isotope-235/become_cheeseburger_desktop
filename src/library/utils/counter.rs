pub trait Counter {
    fn revolve(&mut self, delta: f64, dt: f64) -> i32;
}

impl Counter for f64 {
    fn revolve(&mut self, delta: f64, dt: f64) -> i32 {
        *self += delta * dt;
        let times = *self as i32 / 100;
        *self %= 100.00;
        times
    }
}
