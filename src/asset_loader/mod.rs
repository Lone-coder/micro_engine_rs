use serde_json::{Value};
use std::fs;
use std::path::Path;
use crate::entity::staticEntity::StaticEntity;
use crate::entity::dynamicEntity::Entity;
use crate::world::World;

// Static entity JSON format
// is in assets/examples/static_entity.json
pub fn load_static_entity(filename : &str) -> StaticEntity {

    let content = fs::read_to_string(Path::new(filename))
                    .expect("Unable to load entity file");
    let v : Value = serde_json::from_str(&content).unwrap();
    let x = v["x"].as_f64().unwrap() as f32 ;
    let y = v["y"].as_f64().unwrap() as f32 ;
    let width = v["width"].as_f64().unwrap() as f32 ;
    let height = v["height"].as_f64().unwrap() as f32;
    StaticEntity::new("nemo".to_owned(),x, y, width, height)

}








// World static entites format is in the file assets/examples/static_entities.json
// The texture id is not currently loaded because a decision has to be on
// Whether the texture ids have to be a vector or a HashMap
pub fn load_world_static_entities(filename:&str)->Vec<StaticEntity>{
    let content=fs::read_to_string(Path::new(filename)).expect("unable to locate file,please make  sure the path
    you have provided is correct");
    let json_value:serde_json::Value=serde_json::from_str(&content).unwrap();
    let entities=json_value["entities"].as_array().unwrap();
    entities.iter().map(|entity|{
        let mut nent=StaticEntity::new(
            entity["name"].as_str().unwrap().to_string()
            ,entity["x"].as_f64().unwrap() as f32
            ,entity["y"].as_f64().unwrap() as f32
            ,entity["width"].as_f64().unwrap() as f32
            ,entity["height"].as_f64().unwrap() as f32
        );

        entity["anim"].as_array().unwrap().iter().for_each(|anim|{
            let frames=anim["frames_coords"].as_array().unwrap();
            nent.animation.load_states(
                anim["state"].as_str().unwrap().to_string(),
                frames.iter().map(|coords|(coords["x"].as_i64().unwrap() as i32,coords["y"].as_i64().unwrap() as i32)).collect::<Vec<(i32,i32)>>()

            )
        });

        nent
    }).collect::<Vec<StaticEntity>>()
}




//  Loads a list of dynamic entities
//  Format in assets/examples/dynamic_entities.json
//  the current implementation lacks DRY-ness as it is almost similar
//  to the method on top
pub fn load_dynamic_entities(filename:&str)->Vec<Entity>{
    let content=fs::read_to_string(Path::new(filename)).expect("unable to locate file,please make  sure the path
    you have provided is correct");
    let json_value:serde_json::Value=serde_json::from_str(&content).unwrap();
    let entities=json_value["entities"].as_array().unwrap();
    entities.iter().map(|entity|{
        let mut nent=Entity::new(
            entity["name"].as_str().unwrap().to_string()
            ,entity["x"].as_f64().unwrap() as f32
            ,entity["y"].as_f64().unwrap() as f32
            ,entity["width"].as_f64().unwrap() as f32
            ,entity["height"].as_f64().unwrap() as f32
            ,entity["mass"].as_f64().unwrap() as f32
        );

        entity["anim"].as_array().unwrap().iter().for_each(|anim|{
            let frames=anim["frames_coords"].as_array().unwrap();
            nent.animation.load_states(
                anim["state"].as_str().unwrap().to_string(),
                frames.iter().map(|coords|(coords["x"].as_i64().unwrap() as i32,coords["y"].as_i64().unwrap() as i32)).collect::<Vec<(i32,i32)>>()

            )
        });

        nent
    }).collect::<Vec<Entity>>()
}
