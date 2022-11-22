use std::{collections::HashMap, rc::Rc};

use sdl2::render::Texture;



pub struct AssetStore<'a> {
    textures: HashMap<String, Texture<'a>>
}

impl<'a> AssetStore<'a> {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn clear_assets(&mut self) {
        todo!()   
    }

    pub fn add_texture(&mut self, asset_id: String, asset_path: String) {
        todo!()
    }

    pub fn get_texture(&self, asset_id: String) -> Texture<'a> {
        todo!()
    }
}
