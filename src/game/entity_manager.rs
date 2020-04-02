use crate::entity::dynamicEntity::Entity;
use super::prelude::Messagable;
use super::prelude::SendValue;
use std::ops::Index;
use std::collections::VecDeque;

pub struct Message{
    index:usize,
    value:Box<dyn Messagable>
}

pub struct Manager{
    entities:VecDeque<Entity>,
    messages:VecDeque<SendValue>,
    asstd_fns:Vec<fn(&mut Entity)->SendValue>
}

impl Manager{
    fn load(&mut self,ent:Entity){
        self.entities.push_back(ent)
    }

    fn destroy(&mut self, index:usize){
        self.entities.remove(index);
    }

    fn process_message(&mut self){

        let out=self.messages.pop_front().unwrap();
        match out {

            SendValue::Number{
                index,val
            }=>{

                self.asstd_fns.index(index)(&mut self.entities[index]);
            }


            SendValue::EntityLoad{
                val
            }=>{
                self.entities.push_back(Entity::new("A".to_owned()))
            }

            _=>return
        }

    }
}





//create Loader
// create means to load / destroy entities on the fly
