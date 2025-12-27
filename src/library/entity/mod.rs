pub mod player;
pub use player::*;
pub mod consumable;
pub use consumable::*;
pub mod particle;
pub use particle::*;

pub mod component;
pub mod sys;

use crate::library::Vector2;

pub struct Entity {
    pub class:    component::Class,
    pub pos:      Vector2,
    pub vel:      Vector2,
    pub acc:      Vector2,
    pub age:      f64,
    pub lifespan: f64,
    pub alive:    bool
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            class:    component::Class::None,
            pos:      Vector2::ZERO,
            vel:      Vector2::ZERO,
            acc:      Vector2::ZERO,
            age:      0.00,
            lifespan: 0.00,
            alive:    true
        }
    }
}
