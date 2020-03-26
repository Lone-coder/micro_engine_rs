//Asset loader handles all assets used by game engine
//it loads assets into memory and return an index rather than a reference
//this index can be storeed for future indexing of assets(for example , while rendering)

use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::LoadTexture;

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
    pub fn load_texture(&mut self, file_name : &str, canvas : &mut Canvas<Window>) -> usize
    {
        0
    }

    pub fn load_settings(&mut self, file_name : String) -> u32
    {
        0
    }
}
