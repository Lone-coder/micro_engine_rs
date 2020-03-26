//Asset loader handles all assets used by game engine
//it loads assets into memory and return an index rather than a reference
//this index can be storeed for futute indexing (for example , while rendering)

use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::WindowContext;

use std::path::Path;

pub struct AssetLoader<'a> {
    pub textures : Vec<Texture<'a>>,
}

impl <'a> AssetLoader<'a> {

    pub fn new() -> AssetLoader<'a>{
        AssetLoader {
            textures : Vec::new(),
        }
    }
    pub fn load_texture(&mut self, file_name : &str, tex_creator : &'a sdl2::render::TextureCreator<WindowContext>) -> usize
    {
        let path = Path::new(file_name);
        let texture = tex_creator.load_texture(&path).unwrap();
        self.textures.push(texture);

        self.textures.len() - 1
    }

    pub fn load_settings(&mut self, file_name : String) -> u32
    {
        0
    }
}
