use crate::math::Vector2;

#[derive(Debug)]
pub struct TransformComponent {
    pub position : Vector2,
    pub size : Vector2,
    pub angle : f32,
}

impl TransformComponent {
    pub fn new(position  : Vector2, size : Vector2, angle : f32) -> TransformComponent{
        TransformComponent {
            position : position,
            size : size,
            angle : angle,
        }
    }
}
