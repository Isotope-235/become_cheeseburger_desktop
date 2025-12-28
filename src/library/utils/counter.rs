pub trait Counter {
    fn revolve(&mut self, delta: f64, dt: f64) -> i32;
    fn run<F: FnMut()>(&mut self, delta: f64, dt: f64, f: F);
}

impl Counter for f64 {
    fn revolve(&mut self, delta: f64, dt: f64) -> i32 {
        *self += delta * dt;
        let times = *self as i32 / 100;
        *self %= 100.00;
        times
    }

    fn run<F: FnMut()>(&mut self, delta: f64, dt: f64, mut f: F) {
        let times = self.revolve(delta, dt);

        for _ in 0..times {
            f();
        }
    }
}
