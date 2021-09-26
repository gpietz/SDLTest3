use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use std::collections::HashMap;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TextureId {
    Spaceship,
    Bjarne,
}

pub struct TextureMap<'a> {
    textures: HashMap<usize, Texture<'a>>
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TextureInfo(pub Rect);

impl<'a> TextureMap<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new()
        }
    }

    fn add_texture(&mut self
                   texture_creator: &'a TextureCreator<WindowContext>,
                   texture_id: TextureId,
                   name: &str) {
        let texture = texture_creator.load_texture(name).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        println!("Loaded texture #{}: {} ({}x{})", texture_id as usize, name, width, height);
        self.textures.insert(texture_id as usize, texture);
    }

    pub fn load_textures(&mut self, texture_creator: &'a TextureCreator<WindowContext>) {
        self.add_texture(texture_creator, TextureId::Spaceship, "resources/gfx/f1.png");
        self.add_texture(texture_creator, TextureId::Bjarne, "resources/gfx/bjarne.jpg");
    }

    pub fn get_texture(&self, texture_id: TextureId) -> Option<&Texture<'a>> {
        let texture_key = texture_id as usize;
        self.textures.get(&texture_key)
    }

    pub fn get_texture_rect(&self, texture_id: TextureId) -> Rect {
        let texture = self.get_texture(texture_id).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        Rect::new(0, 0, width, height)
    }

    pub fn submit(&mut self, texture_id: TextureId, texture: Texture<'a>) {
        let TextureQuery { width, height, .. } = texture.query();
        let texture_key = texture_id as usize;
        let status = if self.textures.contains_key(&texture_key) {
            "Replaced texture"
        } else {
            "Added texture"
        };
        println!("{} #{} ({}x{})", status, texture_key, width, height);
        self.textures.insert(texture_key, texture);
    }
}
