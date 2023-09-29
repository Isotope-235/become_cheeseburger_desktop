use std::collections::HashMap;
use macroquad::prelude::{load_texture, Texture2D};

#[derive(Debug)]
pub struct SpriteLoader(HashMap<String, Texture2D>);


impl SpriteLoader {
    pub fn new() -> Self {
        SpriteLoader(HashMap::new())
    }

    pub async fn load_many(&mut self, paths: Vec<&str>) {
        for path in paths {
            let full_path = path.to_string() + ".png";
            let texture = load_texture(&full_path).await;

            match texture {
                Ok(t) => { self.0.insert(full_path, t); }
                Err(_) => { panic!("Invalid sprite name argument!") }
            }
        }
    }

    pub fn get_texture(&self, path: &str) -> Option<&Texture2D> {
        self.0.get(path)
    }
}
