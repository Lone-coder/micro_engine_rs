use crate::entity::dynamicEntity::Entity;
use super::prelude::Messagable;
use super::prelude::SendValue;
use std::ops::Index;
use std::collections::VecDeque;
use crate::world::World;



// The asstd_fns now provides only one function per entity
// This might have to change in the future
// Also making multithread scalable in the future 
pub struct Manager{

    free_indices:Vec<usize>,
    entities:VecDeque<Entity>,
    messages:VecDeque<SendValue>,
    asstd_fns:Vec<fn(&mut Entity)->SendValue>,
    world:World

}



impl Manager{

    fn load(&mut self,ent:Entity){
        self.entities.push_back(ent)
    }

    fn destroy(&mut self, index:usize){
        self.entities.remove(index);
    }




    //                          TASK
    //                  manage this effectively
    //  Right now the entities have corresponding functions mapped on an index basis
    // ie   fn for entity of index i is of index i
    //  or              entities[i]=>asstd_fns[i]
    // This can lead to issues because when an entity is popped its corresponding function has to be popped
    // The current implementation is in the resolving of Destroy message
    //  can lead to undefines behaviour so check

    fn process_message(&mut self){

        for m in self.messages.pop_front(){
        match m {

            SendValue::Number{
                index,val
            }=>{
                let out=self.asstd_fns.index(index)(&mut self.entities[index]);
                self.messages.push_back(out)
            },

            SendValue::EntityLoad{
                val
            }=>{
                if let Some(val)=self.free_indices.pop(){
                    self.entities[val]=Entity::new("Test".to_owned())
                }
                else
                {
                self.entities.push_back(Entity::new("A".to_owned()))
                }
            }

            SendValue::ChangeParams{
                index,param
            }=>{
                self.entities[index].change_state("Hello".to_string())
            },
            SendValue::Destroy{
                index
            }=>{
                self.entities[index]=Self::get_dummy_entity();
                self.asstd_fns[index]=Self::get_dummmy_function();
                self.free_indices.push(index)
            }

            _=>return
            }
        }
    }

    pub fn attach_world(&mut self,x:usize,y:usize){
        self.world=World::create_new(x,y)
    }


    // testing
    pub fn collision_check(&self,index:usize){
        let val=self.world.get_adj_objs(self.entities.index(index));
        // do collision checking for each element here

    }


    //======================WARNING!!=========================================
    // ==================DIRTY IMPLEMENTATION==================================
    // not so sure of this implementation
    //  Returns a dummy entity
    //  has to change
    fn get_dummy_entity()->Entity{
        Entity::new("Dummy".to_owned())
    }

    // Returns a dummy functions
    // Not so sure of this implementation
    fn get_dummmy_function()->fn(&mut Entity)->SendValue{
        fn do_nothing(ent:&mut Entity)->SendValue{
            SendValue::Destroy{
                index:0
            }
        }

        do_nothing
    }
    //==========================END OF THE DIRT==========================================
}





//create Loader
// create means to load / destroy entities on the fly


#[cfg(test)]
fn test(){

}
