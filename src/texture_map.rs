use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TextureId {
    Spaceship = 0,
    Bjarne = 1,
}

pub struct TextureMap<'a> {
    textures: Vec<Texture<'a>>
}

impl<'a> TextureMap<'a> {
    pub fn new() -> Self {
        Self {
            textures: Vec::new()
        }
    }

    fn add_texture(&mut self,
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
        self.textures.get(texture_id as usize)
    }

    pub fn get_texture_rect(&self, texture_id: TextureId) -> Rect {
        let texture = self.textures.get(texture_id as usize).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        Rect::new(0, 0, width, height)
    }
}
