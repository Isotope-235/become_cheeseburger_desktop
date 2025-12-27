use crate::library::Vector2;

pub struct Lifespan {
    pub time:     f64,
    pub on_ended: Option<EndedEffect>
}

pub enum EndedEffect {
    Flak,
    Warning { dir: Vector2 }
}
