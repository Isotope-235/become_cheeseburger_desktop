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

impl From<&'static str> for SoundConfig {
    fn from(value: &'static str) -> Self {
        SoundConfig::from_name(value)
    }
}

impl SoundConfig {
    pub fn from_name(name: &'static str) -> Self {
        Self {
            id: name,
            ..Default::default()
        }
    }
}

impl Default for SoundConfig {
    fn default() -> Self {
        Self {
            volume: 1.0,
            looped: false,
            id: "",
        }
    }
}

#[derive(Debug)]
pub struct SoundLoader(HashMap<String, SoundConfig>, HashMap<String, Vec<Sound>>);

impl SoundLoader {
    pub fn new() -> Self {
        SoundLoader(HashMap::new(),
                    HashMap::new())
    }

    pub async fn load_many<T: Into<SoundConfig>>(&mut self, sound_configs: Vec<T>) {
        for sc in sound_configs {
            let sound_config = sc.into();
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
