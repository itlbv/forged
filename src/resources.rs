use std::collections::HashMap;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct ResourceManager<'l> {
    texture_creator: &'l TextureCreator<WindowContext>,
    pub textures: HashMap<String, Texture<'l>>,
}

impl<'l> ResourceManager<'l> {
    pub fn new(texture_creator: &'l TextureCreator<WindowContext>) -> Self {
        Self { texture_creator, textures: HashMap::new() }
    }

    pub fn load_texture(&mut self, path: &Path) {
        let t = self.texture_creator.load_texture(path).expect("Can't load texture");
        self.textures.insert(String::from("1"), t);
    }
}