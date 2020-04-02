//Probably scehduled for deletion
// Goodbye!


use std::cell::RefCell;
use crate::entity::dynamicEntity;
pub mod game_states;
pub mod entity_manager;
pub mod prelude;

pub struct Game{
    exec_queue:Vec<usize>,
    state:String,
    pub entities:Vec<RefCell<dynamicEntity::Entity>>,
    states:Vec<String>
}

impl Game{
    pub fn new ()->Game{
        Game{
        exec_queue:Vec::new(),
        state:"zero".to_owned(),
        entities:Vec::new(),
        states:Vec::new()
    }
    }

    pub fn load_entity(&mut self,ent:dynamicEntity::Entity){
        self.entities.push(RefCell::new(ent))
    }

    pub fn exec_all(&self){
        for ent in &self.entities{
            ent.borrow_mut().execute()
        }
    }

    pub fn transfer(&mut self,index:usize){//->Box<entity::Entity>{

    }

    pub fn remove_ent(&mut self,index:usize){
        self.entities.remove(index);
    }


}
