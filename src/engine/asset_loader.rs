use std::collections::HashMap;

use macroquad::{
    audio::{load_sound, Sound},
    file::load_file,
    text::{load_ttf_font_from_bytes, Font},
    texture::{load_texture, FilterMode, Texture2D},
};

pub struct AssetLoader {
    root_path: String,
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
    fonts: HashMap<String, Font>,
}

impl AssetLoader {
    pub fn new(root_path: &str) -> Self {
        AssetLoader {
            root_path: root_path.to_string(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    // Load a texture and store it in the HashMap
    pub async fn load_texture(&mut self, path: &str, key: &str) -> Result<(), macroquad::Error> {
        let full_path = format!("{}/{}", self.root_path, path);
        let texture = load_texture(&full_path).await?;
        texture.set_filter(FilterMode::Nearest);
        self.textures.insert(key.to_string(), texture);
        Ok(())
    }

    // Load a sound and store it in the HashMap
    pub async fn load_sound(&mut self, path: &str, key: &str) -> Result<(), macroquad::Error> {
        let full_path = format!("{}/{}", self.root_path, path);
        let sound = load_sound(&full_path).await?;
        self.sounds.insert(key.to_string(), sound);
        Ok(())
    }

    // Load a font and store it in the HashMap
    pub async fn load_font(&mut self, path: &str, key: &str) -> Result<(), macroquad::Error> {
        let full_path = format!("{}/{}", self.root_path, path);
        let bytes = load_file(&full_path).await?;
        let font = load_ttf_font_from_bytes(&bytes)?;
        self.fonts.insert(key.to_string(), font);
        Ok(())
    }

    // Get a texture reference
    pub fn texture(&self, key: &str) -> Option<&Texture2D> {
        self.textures.get(key)
    }

    // Get a sound reference
    pub fn sound(&self, key: &str) -> Option<&Sound> {
        self.sounds.get(key)
    }

    // Get a font reference
    pub fn font(&self, key: &str) -> Option<&Font> {
        self.fonts.get(key)
    }
}
