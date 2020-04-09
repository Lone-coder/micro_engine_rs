use crate::core::components;
use crate::physics::physics_component::PhysicsComponent;
use crate::math::Vector2;

//#[derive(Clone)]
pub struct Entity {
    pub x:usize,
    pub y:usize,
    pub name:String,
    pub state:usize,
    pub physics:PhysicsComponent,
    pub animation:crate::core::components::sprite::Sprite,
}

impl Entity{

    // Todo  enter correct required parameters
    pub fn new(name:String,x:f32,y:f32,mass:f32,width:f32,height:f32)->Entity{
        Entity{
            x:x as usize,
            y:y as usize,
            name:name,
            state:0,
            physics:PhysicsComponent::new(Vector2::new(x,y),mass,width,height),
            animation:components::sprite::Sprite::new(),
        }
    }
}
