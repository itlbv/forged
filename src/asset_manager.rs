use std::collections::HashMap;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::WindowContext;

pub struct AssetManager<'l> {
    texture_creator: &'l TextureCreator<WindowContext>,
    textures: HashMap<String, Texture<'l>>,

    ttf_context: &'l Sdl2TtfContext,
    font: Font<'l, 'l>,
}

impl<'l> AssetManager<'l> {
    pub fn new(texture_creator: &'l TextureCreator<WindowContext>, ttf_context: &'l Sdl2TtfContext) -> Self {
        Self { texture_creator,
            textures: HashMap::new(),
            ttf_context,
            font: ttf_context.load_font(Path::new("assets/clacon2.ttf"), 128).unwrap(),
        }
    }

    pub fn load_texture(&mut self, texture_id: &str, path: &Path) {
        let t = self.texture_creator.load_texture(path).expect("Can't load texture");
        self.textures.insert(String::from(texture_id), t);
    }

    pub fn borrow_texture(&self, texture_id: &str) -> &Texture<'l> {
        self.textures.get(texture_id).unwrap()
    }
}