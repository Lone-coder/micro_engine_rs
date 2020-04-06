use serde_json;
use std::path::Path;
use std::fs::File;
use crate::entity::{staticEntity::StaticEntity,dynamicEntity::Entity};
use crate::game::prelude::SendValue;

pub struct ReturnFormat{
    dynamic_entities:Vec<Entity>,
    static_entities:Vec<StaticEntity>,
    fns:Vec<fn(&mut Entity)->SendValue>
}

pub fn loader(path_to_file:&Path){


}
