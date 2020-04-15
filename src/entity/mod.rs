use crate::core::components::{ Component, ComponentType};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use std::collections::{HashMap,HashSet};

// pub mod dynamicEntity;
// pub mod staticEntity;
// pub mod camera;

pub struct Entity{
    pub components : Vec<Component>
}
//manages all the entities

pub struct EntityManager {
    pub entities : Vec<Entity>,
}

impl EntityManager {

    pub fn new() -> EntityManager {
        EntityManager {
            entities : Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entities.push( Entity{ components : Vec::new()});
        self.entities.len() - 1 as usize
    }

    pub fn get_component_index(&mut self, entity_id : usize, component_type : ComponentType) -> Option<usize> {
        for component in self.entities[entity_id].components.iter() {
            if component.type_of_component == component_type {
                return Some(component.index);
            }
        }

        return None;
    }

    pub fn attach_component(&mut self, entity_id : usize, component : Component) {

        if entity_id < self.entities.len() {
            self.entities[entity_id].components.push(component);
        }
        else {
            println!("Could not find entity with id : {:?}", entity_id);
        }
    }

}
