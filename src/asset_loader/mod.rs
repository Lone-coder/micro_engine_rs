use serde_json::{Value};
use std::env;
use std::fs;
use std::path::Path;

use crate::entity::staticEntity::StaticEntity;
use crate::world::World;

//static entity JSON format
/*
{
    "x" : f32
    "y" : f32
    "width" : f32,
    "height" : f32,
}

*/
pub fn load_static_entity(filename : &str) -> StaticEntity {

    let content = fs::read_to_string(Path::new(filename))
                    .expect("Unable to load entity file");

    let v : Value = serde_json::from_str(&content).unwrap();

    let x = v["x"].as_u64().unwrap() as usize;
    let y = v["y"].as_u64().unwrap() as usize;
    let width = v["width"].as_u64().unwrap() as usize;
    let height = v["height"].as_u64().unwrap() as usize;

    StaticEntity::new(x, y, width, height)

}

//world description
pub fn load_world(filename : &str) -> World {

    let content = fs::read_to_string(Path::new(filename))
                    .expect("Unable to load world description file");

    let v : Value = serde_json::from_str(&content).unwrap();

    World::create_new(0,0)

}
