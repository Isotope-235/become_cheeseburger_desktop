use std::collections::HashMap;
use std::fs;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct SoundConfig {
    pub volume: f32,
    pub looped: bool,
    pub id: &'static str,
}

#[derive(Debug)]
pub struct SoundLoader(HashMap<String, SoundConfig>, HashMap<String, Vec<Sound>>);

impl SoundLoader {
    pub fn new() -> Self {
        SoundLoader(HashMap::new(),
                    HashMap::new())
    }

    /// Loads a vector of sound configurations into this [SoundLoader],
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
    /// let mut sound_loader = SoundLoader::new();
    ///
    /// sound_loader.load_many(vec![
    ///    SoundConfig {
    ///       id: "explosion"
    ///       volume: 1.0,
    ///       looped: false,
    ///    },
    /// ]);
    ///
    /// // Plays explosion1.wav or explosion2.wav randomly
    /// sound_loader.play("explosion");
    ///
    /// ```
    pub async fn load_many(&mut self, sound_configs: Vec<SoundConfig>) {
        for sound_config in sound_configs {
            let paths = fs::read_dir(format!("assets/sounds/{}", sound_config.id)).unwrap_or_else
            (|err| panic!("Invalid sound id argument! Path: assets/sounds/{}", sound_config.id));

            // Find all sound variations and save them
            let mut sound_variations: Vec<Sound> = Vec::new();

            for sound_path in paths {
                let sound_variation = sound_path.unwrap().file_name().into_string().unwrap();

                let full_path = format!("assets/sounds/{}/{}", sound_config.id, sound_variation);
                let sound_result = load_sound(&full_path).await;

                let sound = sound_result.unwrap_or_else(
                    |_err| panic!("Invalid sound name argument! Path: {}", full_path)
                );

                sound_variations.push(sound);
            }

            // Save sound variations
            self.1.insert(sound_config.id.to_string(), sound_variations);

            // Save sound config
            self.0.insert(sound_config.id.to_string(), sound_config);
        }
    }

    /// Plays a sound with the given ID.
    /// Note that the sound MUST be loaded before playing it.
    ///
    /// ## Example
    /// ```
    /// // ... load sounds using SoundLoader::load_many() ...
    /// sound_loader.play("sound_id");
    pub fn play(&self, id: &str) {
        let sound_config = self.0.get(id).unwrap_or_else(|| panic!("Invalid sound id '{}' for playing", id));

        let sound_variations = self.1.get(id).unwrap_or_else(|| panic!("Invalid \
        sound id '{}' for playing", id));

        let sound = &sound_variations.choose(&mut rand::thread_rng()).unwrap();

        dbg!("Playing sound: {}", sound);

        play_sound(sound, PlaySoundParams {
            looped: sound_config.looped,
            volume: sound_config.volume,
        });
    }
}

impl Default for SoundLoader {
    fn default() -> Self {
        Self::new()
    }
}
