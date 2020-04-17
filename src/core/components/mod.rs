pub mod sprite;
pub mod transform;
pub mod state;

use sprite::SpriteComponent;
use transform::TransformComponent;
use state::StateComponent;

//enum used to check type of component
#[derive(Debug, Eq, PartialEq)]
pub enum ComponentType {
    TransformComponent,
    SpriteComponent,
    StateComponent,
}

//used by entity to refer the components
#[derive(Debug)]
pub struct Component {
    pub type_of_component : ComponentType, //component type for comparison
    pub index : usize //index of component in the component manager array
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

    pub fn create_state_component(&mut self, state : StateComponent) -> Component {

        self.states.push(state);

        Component {
            type_of_component : ComponentType::StateComponent,
            index : self.states.len() - 1 as usize
        }
    }
}
