use crate::math::Vector2;

#[derive(Debug)]
pub struct TransformComponent {
    pub position : Vector2,
    pub size : Vector2,
    pub angle : f32,
}

//getters and setters
impl TransformComponent {
    pub fn new(position  : Vector2, size : Vector2, angle : f32) -> TransformComponent{
        TransformComponent {
            position : position,
            size : size,
            angle : angle,
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn set_position(&mut self, position : Vector2) {
        self.position = position;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, angle : f32) {
        self.angle = angle;
    }
}
