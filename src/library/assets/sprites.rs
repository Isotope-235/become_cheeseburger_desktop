use std::collections::HashMap;

use macroquad::{
    color::Color,
    prelude::{Texture2D, load_texture}
};

#[derive(Debug)]
pub struct Sprite {
    pub color:   Color,
    pub texture: Texture2D
}

#[derive(Debug)]
pub struct Loader(HashMap<String, Sprite>);

pub trait IntoPathColor: Clone {
    fn into_path_color(self) -> (String, Color);
}

impl IntoPathColor for (&str, Color) {
    fn into_path_color(self) -> (String, Color) {
        (self.0.to_string(), self.1)
    }
}

impl IntoPathColor for &str {
    fn into_path_color(self) -> (String, Color) {
        (self.to_string(), Color::default())
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}

impl Loader {
    pub fn new() -> Self {
        Loader(HashMap::new())
    }
    /// # Panics
    ///
    /// Panics if no file matches any of the given file names.
    pub async fn load_many(&mut self, paths: Vec<impl IntoPathColor>) {
        for pc in paths {
            let (path, color) = pc.into_path_color();
            let full_path = format!("assets/sprites/{path}.png");

            let texture = load_texture(&full_path)
                .await
                .unwrap_or_else(|_err| panic!("Invalid sprite name argument! Path: {full_path}"));

            self.0.insert(path, Sprite { color, texture });
        }
    }
    /// # Panics
    ///
    /// Panics if the given path is invalid.
    pub fn texture(&self, path: &str) -> &Texture2D {
        &self
            .0
            .get(path)
            .unwrap_or_else(|| panic!("Invalid path '{path}' for texture"))
            .texture
    }
    /// # Panics
    ///
    /// Panics if the given path is invalid.
    pub fn color(&self, path: &str) -> &Color {
        &self
            .0
            .get(path)
            .unwrap_or_else(|| panic!("Invalid path '{path}' for color"))
            .color
    }
}
