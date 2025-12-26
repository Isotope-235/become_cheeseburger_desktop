pub enum Draw {
    Sprite { name: &'static str, rotate: bool },
    Warning { delay: f64 },
    Laser,
}
