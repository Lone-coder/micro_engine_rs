use sdl2::rect::Rect;
use sdl2::pixels::Color;

use super::math;
use super::world;

pub struct Camera {
    pub position : math::Vector2,
    pub width : i32,
    pub height : i32,
}

impl Camera {

    pub fn create(width : i32, height : i32) -> Camera {
        Camera{
            position : math::Vector2::new(0.0, 0.0),
            width : width,
            height : height,
        }
    }

}
