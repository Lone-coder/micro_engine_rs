pub mod sprite;

// Data structures
use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};

use sdl2::render::Texture;

use crate::math::Vector2;

//enum used to check type of component
#[derive(Eq, PartialEq)]
pub enum ComponentType {
    TransformComponent,
    SpriteComponent,
    StateComponent,
}

//used by entity to refer the components
pub struct Component {
    pub type_of_component : ComponentType,
    pub index : usize //index of component in the component manager array
}

pub struct TransformComponent {
    pub position : Vector2,
    pub size : Vector2,
    pub angle : f32,
}

pub struct SpriteRect {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32,
}

pub struct Animation {
    pub frame_width : i32,
    pub frame_delay : f32,
    pub frame_coords : Vec<(i32, i32)>,
    pub flip_horizontal : bool
}

pub struct SpriteComponent {
    pub texture : Texture,
    pub src_rect : SpriteRect,
    pub animations : HashMap<String, Animation>,
    pub current_animation : String,
    pub flip_horizontal : bool,
    pub flip_vertical : bool
}

#[derive(Debug)]
pub enum PlayerState {
    WalkingLeft,
    WalkingRight,
    WalkingUp,
    WalkingDown,
    Idle,
    Shoot,
}

#[derive(Debug)]
pub enum State {
    PlayerState(PlayerState),
}

pub struct StateComponent {
    pub current_state : State,
}

 pub struct ComponentManager {
    pub transforms : Vec<TransformComponent>,
    pub sprites : Vec<SpriteComponent>,
    pub states : Vec<StateComponent>,
}

impl ComponentManager {

    pub fn new() -> ComponentManager {
        ComponentManager {
            transforms : Vec::new(),
            sprites : Vec::new(),
            states : Vec::new(),
        }
    }

    pub fn create_sprite_component(&mut self, sprite : SpriteComponent) -> Component {
        self.sprites.push(sprite);

        Component {
            type_of_component : ComponentType::SpriteComponent,
            index : self.sprites.len() - 1 as usize,
        }
    }

    pub fn create_transform_component(&mut self, transform : TransformComponent) -> Component {
        self.transforms.push(transform);

        Component {
            type_of_component : ComponentType::TransformComponent,
            index : self.transforms.len() - 1 as usize
        }
    }

    pub fn create_state_component(&mut self, state : State) -> Component {

        self.states.push(StateComponent{current_state : state});

        Component {
            type_of_component : ComponentType::StateComponent,
            index : self.states.len() - 1 as usize
        }
    }
}
