
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

pub trait IntoPathColor : Clone {
    fn as_path_color(self) -> (String, Color);
}

impl IntoPathColor for (&str, Color) {
    fn as_path_color(self) -> (String, Color) {
        (self.0.to_string(), self.1)
    }
}

impl IntoPathColor for &str {
    fn as_path_color(self) -> (String, Color) {
        (self.to_string(), Color::default())
    }
}

impl Default for SpriteLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteLoader {
    pub fn new() -> Self {
        SpriteLoader(HashMap::new())
    }

    pub async fn load_many(&mut self, paths: Vec<impl IntoPathColor>) {
        for pc in paths {
            let (path, color) = pc.as_path_color();
            let full_path = format!("assets/sprites/{}.png", path);

            let texture = load_texture(&full_path).await.unwrap_or_else(|_err| panic!("Invalid sprite name argument! Path: {}", full_path));

            self.0.insert(
                path,
                Sprite {
                    color,
                    texture,
                },
            );
        }
    }

    pub fn texture(&self, path: &str) -> &Texture2D {
        &self
            .0
            .get(path)
            .unwrap_or_else(|| panic!("Invalid path '{}' for texture", path))
            .texture
    }

    pub fn color(&self, path: &str) -> &Color {
        &self
            .0
            .get(path)
            .unwrap_or_else(|| panic!("Invalid path '{}' for color", path))
            .color
    }
}
