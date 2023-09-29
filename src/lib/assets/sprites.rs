use std::collections::HashMap;

use macroquad::color::Color;
use macroquad::prelude::{load_texture, Texture2D};

#[derive(Debug)]
pub struct Sprite {
    pub color: Color,
    pub texture: Texture2D,
}

#[derive(Debug)]
pub struct SpriteLoader(HashMap<String, Sprite>);

pub struct PathColor(pub String, pub Color);

impl From<(&str, Color)> for PathColor {
    fn from(value: (&str, Color)) -> Self {
        PathColor(value.0.to_string(), value.1)
    }
}


impl SpriteLoader {
    pub fn new() -> Self {
        SpriteLoader(HashMap::new())
    }

    pub async fn load_many<T: Into<PathColor>>(&mut self, paths: Vec<T>) {
        for pc in paths {
            let path_color = pc.into();
            let full_path = format!("assets/sprites/{}.png", path_color.0);
            let texture = load_texture(&full_path).await;

            let t = texture.unwrap_or_else(
                |_err| panic!("Invalid sprite name argument! Path: {}", full_path)
            );

            self.0.insert(path_color.0, Sprite {
                color: path_color.1,
                texture: t,
            });
        }
    }

    pub fn texture(&self, path: &str) -> &Texture2D {
        &self.0.get(path).unwrap_or_else(|| panic!("Invalid path '{}' for texture", path)).texture
    }

    pub fn color(&self, path: &str) -> &Color {
        &self.0.get(path).unwrap_or_else(|| panic!("Invalid path '{}' for color", path)).color
    }
}
