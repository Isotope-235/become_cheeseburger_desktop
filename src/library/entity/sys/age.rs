use crate::State;

const RATE: f64 = 1.00;

pub fn run(state: &mut State, dt: f64) {
    for e in &mut state.entities {
        e.age = apply(e.age, dt);
    }
    for p in &mut state.particles {
        p.age = apply(p.age, dt);
    }
}

fn apply(age: f64, dt: f64) -> f64 {
    age + RATE * dt
}
