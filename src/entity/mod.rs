use crate::core::components::{ Component, ComponentType};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use std::collections::{HashMap,HashSet};

// pub mod dynamicEntity;
// pub mod staticEntity;
// pub mod camera;

pub enum EntityType {
    Player,
    Enemy,
    Empty,
}

pub struct Entity{
    pub components : Vec<Component>,
    pub entity_type : EntityType,
}
//manages all the entities

pub struct EntityManager {
    pub entities : Vec<Entity>,
    pub entity_index_lookup : HashMap<String, usize>,// mapping entity name with index(or id) of the entity
}

impl EntityManager {

    pub fn new() -> EntityManager {
        EntityManager {
            entities : Vec::new(),
            entity_index_lookup : HashMap::new(),
        }
    }

    pub fn create_entity(&mut self, entity_name : &str, type_of_entity : EntityType) {

        self.entities.push( Entity {
            components : Vec::new(),
            entity_type : type_of_entity,
        });

        self.entity_index_lookup.insert(entity_name.to_string(), self.entities.len() - 1 as usize);
    }

    pub fn get_entity_index_with_name(&mut self, entity_name : &str) -> Option<usize> {

        match self.entity_index_lookup.get(&entity_name.to_string()) {
            Some(index) => {
                return Some(*index);
            },
            None => {
                return None;
            }
        }
    }

    //getting component with entity name
    pub fn get_component_index_of_entity(&mut self, entity_name : &str, component_type : ComponentType) -> Option<usize> {

        match self.get_entity_index_with_name(entity_name) {
            Some(index) => {
                for component in self.entities[index].components.iter() {
                    if component.type_of_component == component_type {
                        return Some(component.index);
                    }
                }
                return None;
            },
            None => {
                return None;
            }
        }

    }

    //getting component index with entity id(or index)
    pub fn get_component_index_of_entity_id(&mut self, entity_id : usize, component_type : ComponentType) -> Option<usize> {

        for component in self.entities[entity_id].components.iter() {
            if component.type_of_component == component_type {
                return Some(component.index);
            }
        }
        return None;
    }

    pub fn attach_component(&mut self, entity_name : &str, component : Component) {

        match self.get_entity_index_with_name(entity_name) {
            Some(index) => {
                self.entities[index].components.push(component);
            },
            None => ()
        }
    }

}
