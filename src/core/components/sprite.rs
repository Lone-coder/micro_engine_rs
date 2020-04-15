// Data structures
use std::collections::HashMap;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::math::Vector2;

pub struct Animation {
    pub frame_width : i32,
    pub frame_delay : f32,
    pub frame_coords : Vec<(i32, i32)>,
    pub flip_horizontal : bool
}

pub struct TransformComponent {
    position : Vector2,
    size : Vector2,
    angle : f32,
}

pub struct SpriteRect {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32,
}

pub struct SpriteComponent {
    pub sprite_sheet : Texture,
    pub src_rect : SpriteRect,
    pub animations : HashMap<String, Animation>,
    pub current_animation : String,
    pub flip_horizontal : bool,
    pub flip_vertical : bool
}


//converts sprite rect to sdl rect
pub fn spriterect_to_sdlrect(srect : &SpriteRect) -> Rect {

    Rect::new(srect.x, srect.y, srect.width as u32, srect.height as u32)
}

pub struct Sprite {
    pub src_rect : SpriteRect,
    pub dest_rect : SpriteRect,
    pub animations : HashMap<String, Animation>,
    pub current_animation : String,
    pub flip : bool,
    pub texture : Texture
}

impl Sprite {

    pub fn update(&mut self, time : f32) {
        self.flip = false;
        let animation = self.animations.get(&self.current_animation).unwrap();
        self.flip = animation.flip_horizontal;

        let num_of_frames = animation.frame_coords.len() as i32;

        //calculating frame index
        let index = ((time / animation.frame_delay) as i32 % num_of_frames) as usize;

        //println!("frame : {:?}", index);

        //updating dest rect by taking coord from cuurent frame
        self.src_rect.x = animation.frame_coords[index].0;
        self.src_rect.y = animation.frame_coords[index].1;
    }

    pub fn render(&mut self, canvas : &mut Canvas<Window>) {

        let src_rect = spriterect_to_sdlrect(&self.src_rect);
        let mut dest_rect = spriterect_to_sdlrect(&self.dest_rect);

        //placing at the center
        dest_rect.x -= (dest_rect.width() / 2) as i32;
        dest_rect.y -= (dest_rect.height() / 2) as i32;

        canvas.copy_ex(&self.texture, Some(src_rect), Some(dest_rect), 0.0, None, self.flip, false).unwrap();
    }

}
