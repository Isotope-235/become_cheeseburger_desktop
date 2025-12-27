pub enum Draw {
    Sprite { name: &'static str, rotate: bool },
    Warning,
    Laser
}
