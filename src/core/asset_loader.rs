//Asset loader handles all assets used by game engine
//it loads assets into memory and return an index rather than a reference
//this index can be storeed for future indexing of assets(for example , while rendering)

use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use std::path::Path;

pub struct AssetLoader<'a> {
    pub textures : Vec<Texture<'a>>,
    //pub texture_creator : &'a mut TextureCreator<WindowContext>,
}

impl <'a> AssetLoader<'a> {

    pub fn new(tex_creator : &'a mut TextureCreator<WindowContext>) -> AssetLoader<'a>{
        AssetLoader {
            textures : Vec::new(),
            //texture_creator : tex_creator,
        }
    }

    pub fn load_texture(&mut self, file_name : &str) -> usize {
        let path = Path::new(file_name);

        //let texture : Texture<'a> = self.texture_creator.load_texture(path).unwrap();
        //self.textures.push(texture);

        self.textures.len() - 1

    }

    pub fn load_settings(&mut self, file_name : String) -> u32
    {
        0
    }
}
