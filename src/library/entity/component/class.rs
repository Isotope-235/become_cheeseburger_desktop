use crate::library::Vector2;

#[derive(Clone, Copy)]
pub enum Class {
    None,
    Bullet,
    Slug,
    Laser,
    HealthPack,
    Flak,
    FlakChild,
    Warning { dir: Vector2, delay: f64 }
}
