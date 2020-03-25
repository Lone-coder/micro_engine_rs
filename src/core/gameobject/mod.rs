//all objects of u8 type are to be redefined
pub mod object_stack;
use crate::core::Translate;
use super::components::{physics, sprite, transform};

pub struct GameObject {
    transform: Option<transform::Transform>,
    physics: Option<physics::Physics>,
    pub render: Option<sprite::Sprite>,
}

impl GameObject {
    pub fn new() -> GameObject {
        GameObject {
            transform: None,
            physics: None,
            render: None,
        }
    }
}

impl Translate for GameObject{

fn translate(&mut self, val: (i32, i32)) {


 }
}
