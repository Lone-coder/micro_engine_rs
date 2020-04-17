use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};

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

#[derive(Debug)]
pub struct Animation {
    pub frame_width : u32,
    pub frame_delay : f32,
    pub frame_coords : Vec<(i32, i32)>,
    pub flip_horizontal : bool
}

impl Animation {
    pub fn new(frame_width : u32, frame_delay : f32, frame_coords : Vec<(i32, i32)>, flip_horizontal : bool) -> Animation {
        Animation {
            frame_width : frame_width,
            frame_delay : frame_delay,
            frame_coords : frame_coords,
            flip_horizontal : flip_horizontal
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
