use std::collections::HashMap;
use super::GameObject;
use super::super::world;

pub fn get_stack_test()->HashMap<String,GameObject>{
    let mut stack=HashMap::with_capacity(20);
    insert_object(&mut stack);
    stack
}

pub fn insert_object(stack:&mut HashMap<String,GameObject>){
    world::object_loader::loader("maps/objects.json".to_string())
    .iter().for_each(|x|{
        stack.insert(x.0.to_owned(),GameObject::new());
    });
}
