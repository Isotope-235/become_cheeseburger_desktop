use crate::*;
use macroquad::prelude::Conf;
pub fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (center().x() * 2.00 * scale()) as i32,
        window_height: (center().y() * 2.00 * scale()) as i32,
        sample_count: 0,
        window_resizable: false,
        ..Default::default()
    }
}