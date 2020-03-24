//all objects of u8 type are to be redefined
use super::components::{physics, render, transform};

pub struct GameObject {
    transform: Option<transform::Transform>,
    physics: Option<physics::Physics>,
    render: Option<render::Render>,
}

impl GameObject {
    pub fn new() -> GameObject {
        GameObject {
            transform: None,
            physics: None,
            render: None,
        }
    }

    pub fn render() {

    }
}
