pub mod animation;

pub use animation::Animation;

use std::collections::HashMap;

use sdl2::render::Texture;

#[derive(Debug)]
pub struct SpriteRect {
    pub x : i32,
    pub y : i32,
    pub width : u32,
    pub height : u32,
}

impl SpriteRect {
    pub fn new(x : i32, y : i32, width : u32, height : u32) -> SpriteRect {
        SpriteRect {
            x : x,
            y : y,
            width : width,
            height : height,
        }
    }
}

pub struct SpriteComponent {
    pub texture : Texture,
    pub src_rect : SpriteRect,
    pub animations : HashMap<String, Animation>,
    pub current_animation : String,
    pub flip_horizontal : bool,
    pub flip_vertical : bool
}
