use std::{collections::HashMap, rc::Rc};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub type AssetId = String;

pub struct AssetStore {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<Texture>>,
}

impl AssetStore {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            textures: HashMap::new(),
        }
    }

    pub fn clear_assets(&mut self) {
        self.textures.clear();
    }

    pub fn add_texture(&mut self, asset_id: AssetId, asset_path: String) {
        let texture = self.texture_creator.load_texture(asset_path).unwrap();
        self.textures.insert(asset_id, Rc::new(texture));
    }

    pub fn get_texture(&self, asset_id: &AssetId) -> Rc<Texture> {
        self.textures.get(asset_id).unwrap().clone()
    }
}
