use super::math;

pub struct Camera {
    pub position : math::Vector2,
    pub width : u32,
    pub height : u32
}

impl Camera {

    pub fn create(_width : u32, _height : u32) -> Camera {
        Camera {
            position : math::Vector2::new(0.0, 0.0),
            width : _width,
            height : _height,
        }
    }

}
