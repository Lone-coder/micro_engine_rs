use super::math;

pub struct Camera {
    pub position : math::Vector2,
}

impl Camera {

    pub fn create() -> Camera {
        Camera{
            position : math::Vector2::new(0.0, 0.0),
        }
    }

}
