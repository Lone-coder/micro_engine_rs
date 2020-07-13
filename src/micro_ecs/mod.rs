pub mod system;
pub mod component;

use system;
use component::{transform::TransformComponent, sprite::SpriteComponent};

type Entity usize;

pub struct MicroECS {
    transforms : Vec<TransformComponent>,   
    sprites : Vec<SpriteComponent>,
}

impl MicroECS {
    pub fn create_new_entity(&mut self) -> Entity {
        0
    }

    pub fn get_component(&mut self, Entity : entity) {

    }
}