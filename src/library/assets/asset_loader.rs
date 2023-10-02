use macroquad::{
    color::Color,
    texture::Texture2D,
};

use super::sound::*;
use super::sprites::*;

/// Helps loading all assets into the game with some handy util functions
///
/// Load sprites or sounds using the [load_sprites](AssetLoader::load_sprites) and [load_sounds](AssetLoader::load_sounds) functions.
#[derive(Debug)]
pub struct AssetLoader {
    sprites: SpriteLoader,
    sounds: SoundLoader,
}

impl Default for AssetLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetLoader {
    pub fn new() -> Self {
        AssetLoader {
            sprites: SpriteLoader::new(),
            sounds: SoundLoader::new(),
        }
    }

    /// Loads a vector of sprite paths into this [AssetLoader],
    /// making the sprites loaded available for rendering the [Texture2D]s
    ///
    /// This function also allows assigning a color for each sprite, facilitating particle
    /// effects and such
    ///
    /// ## Example
    /// ```
    /// let mut asset_loader = AssetLoader::new();
    ///
    /// asset_loader.load_sprites(vec![
    ///   ("sprite_id", Color::RED), // Loads assets/sprites/sprite_id.png
    ///   ("sprite_id2", Color::BLUE), // Loads assets/sprites/sprite_id2.png
    /// ]);
    ///
    /// asset_loader.texture("sprite_id"); // Returns a Texture2D
    /// ```
    pub async fn load_sprites<T: Into<PathColor>>(&mut self, sprite_paths: Vec<T>) {
        self.sprites.load_many(sprite_paths).await;
    }

    /// Loads a vector of sound configurations into this [AssetLoader],
    /// making the sounds loaded available for playing.
    ///
    /// Reference a sound by its ID, which is the name of the folder within assets/sound
    /// that contains the sound variations. Sound loader picks up all sound files within.
    ///
    /// **Supported file types: .wav, .ogg, .flac, .mp3**
    ///
    /// # Example
    /// ## Folder structure
    /// ```text
    /// 📂assets
    ///  ↳ sounds
    ///    ↳ explosion
    ///      ↳ 🔈explosion1.wav
    ///      ↳ 🔈explosion2.wav
    /// ```
    /// ## Code
    /// ```
    /// let mut asset_loader = AssetLoader::new();
    ///
    /// asset_loader.load_sounds(vec![
    ///    SoundConfig {
    ///       id: "explosion"
    ///       volume: 1.0,
    ///       looped: false,
    ///    },
    /// ]);
    ///
    /// // Plays explosion1.wav or explosion2.wav randomly
    /// asset_loader.play_sound("explosion");
    ///
    /// ```
    pub async fn load_sounds<T: Into<SoundConfig>>(&mut self, sound_configs: Vec<T>) {
        self.sounds.load_many(sound_configs).await;
    }

    /// Plays a sound with a given ID.
    /// This function picks a random sound file from the folder with the given ID,
    /// and plays it
    ///
    /// ## Example
    /// ```
    /// // ... load sounds using AssetLoader::load_sounds() ...
    /// asset_loader.play_sound("sound_id");
    /// ```
    pub fn play_sound(&self, id: &str) {
        self.sounds.play(id);
    }

    /// Returns a reference to the [Texture2D] for the given sprite ID.
    ///
    /// ## Example
    /// ```
    /// // ... load sprites using AssetLoader::load_sprites() ...
    /// let texture = asset_loader.texture("sprite_id");
    /// ```
    pub fn texture(&self, path: &str) -> &Texture2D {
        self.sprites.texture(path)
    }

    /// Returns a reference to the [Color] for the given sprite ID.
    ///
    /// ## Example
    /// ```
    /// // ... load sprites using AssetLoader::load_sprites() ...
    /// let color = asset_loader.color("sprite_id");
    /// ```
    pub fn color(&self, path: &str) -> &Color {
        self.sprites.color(path)
    }
}
