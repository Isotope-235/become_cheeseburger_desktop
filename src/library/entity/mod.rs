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
    pub lifespan: Option<component::Lifespan>,
    pub hp:       Option<f64>,

    pub draw: Option<component::Draw>
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            class:    component::Class::None,
            pos:      Vector2::ZERO,
            vel:      Vector2::ZERO,
            acc:      Vector2::ZERO,
            age:      0.00,
            lifespan: None,
            hp:       None,

            draw: None
        }
    }
}
