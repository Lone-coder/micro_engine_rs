// //=============================================================
// //      INVALIDATED BASED ON RECENT DEVELOPMENTS
// //=============================================================
//
//
//
//
// use crate::entity::dynamicEntity::Entity;
// use super::prelude::Messagable;
// use super::prelude::SendValue;
// use std::ops::Index;
// use std::collections::VecDeque;
// use crate::world::World;
//
// //=========================================================
// //              MOST IMPORTANT TODO
// //  create a function bank and associate it with states
// //========================================================
// //========================================================
// //      create/use  a sorting algorithm to sort stuff
// //
// //========================================================
// //                  ANOTHER IMPORTANT TODO
// //                     manage states
// //========================================================
//
//
// // The asstd_fns now provides only one function per entity
// // This might have to change in the future
// // Also making multithread scalable in the future
// pub struct Manager{
//     game_state:usize,
//     game_states:Vec<(usize,usize)>,
//     render_pipeline:VecDeque<(usize,usize)>,
//     function_bank:Vec<Vec<fn(&mut Entity,SendValue)->SendValue>>,
//     entity_bank:Vec<Vec<Entity>>,
//     free_indices:Vec<usize>,
//     entities:VecDeque<Entity>,
//     messages:VecDeque<SendValue>,
//     asstd_fns:VecDeque<Option<fn(&mut Entity,SendValue)->SendValue>>,
//     world:World
//
// }
//
//
//
// impl Manager{
//     pub fn new()->Manager{
//
//         //implement method to load from the map editor
//         // Just dummy variables  for now
//         let n=10usize;
//         let m=10usize;
//
//         Manager{
//             game_state:0,
//             game_states:Vec::new(),
//             render_pipeline:VecDeque::new(),
//             function_bank:Vec::new(),
//             entity_bank:Vec::new(),
//             free_indices:Vec::new(),
//             entities:VecDeque::new(),
//             messages:VecDeque::new(),
//             asstd_fns:VecDeque::new(),
//             world:World::new(n,m)
//         }
//     }
//
//     fn load_entity(&mut self,ent:Entity,func:fn(&mut Entity,SendValue)->SendValue){
//         self.entities.push_back(ent);
//         self.asstd_fns.push_back(Some(func));
//     }
//
//
//     fn destroy(&mut self, index:usize){
//         self.entities[index]=Self::get_dummy_entity();
//         self.asstd_fns[index]=None;
//         self.free_indices.push(index)
//     }
//
//     //                          TASK
//     //                  manage this effectively
//     //  Right now the entities have corresponding functions mapped on an index basis
//     // ie   fn for entity of index i is of index i
//     //  or              entities[i]=>asstd_fns[i]
//     // This can lead to issues because when an entity is popped its corresponding function has to be popped
//     // The current implementation is in the resolving of Destroy message
//     //  can lead to undefined behaviour so check
//
//     fn process_message(&mut self){
//         for m in self.messages.pop_front(){
//         match m {
//
//             SendValue::Number{
//                 index,val
//             }=>{
//                 //let out=self.asstd_fns.index(index).unwrap()(&mut self.entities[index]);
//                 //self.messages.push_back(out)
//             },
//
//             SendValue::CreateEntity{
//                 entity,
//                 func
//             }=>{
//                 if let Some(index)=self.free_indices.pop(){
//                     self.entities[index]=entity;
//                     self.asstd_fns[index]=Some(func);
//                 }
//                 else
//                 {
//                     self.entities.push_back(entity);
//                     self.asstd_fns.push_back(Some(func))
//                 }
//             }
//
//
//
//             //biggest problem , is to change parameters of
//             // functions
//             SendValue::ChangeParams{
//                 index,param
//             }=>{
//
//             },
//
//             SendValue::Destroy{
//                 index
//             }=>{
//                 self.entities[index]=Self::get_dummy_entity();
//                 self.asstd_fns[index]=None;
//                 self.free_indices.push(index)
//             }
//
//             _=>return
//             }
//         }
//     }
//
//     pub fn attach_world(&mut self,x:usize,y:usize){
//         self.world=World::new(x,y)
//     }
//
//
//     // testing
//     pub fn collision_check(&self,index:usize){
//         let val=self.world.get_adj_objs(self.entities.index(index));
//         // do collision checking for each element here
//
//     }
//
//
//     pub fn init_all(&mut self){
//         unimplemented!()
//     }
//
//
//     // Used to run the functions common for all in each cycle
//     fn run_common(&mut self){
//         unimplemented!();
//
//         // idle dummy entity for tests
//         (0..self.asstd_fns.len()).for_each(|v|{
//             let out=self.asstd_fns[v].unwrap_or(Self::dummy_function)(&mut self.entities[v],SendValue::Idle);
//             if out.is_not_idle(){
//                 self.messages.push_back(out)
//             }
//         })
//     }
//
//
//
//     // Test implemenation , change later
//     // This should data for the next levels during the load process
//     // In the background
//     fn load_from_bank(&mut self,index:usize){
//         self.asstd_fns[index]=Some(self.function_bank[index][index]);
//         //Implement Copy trait for entity
//         //self.entities[index]=self.entity_bank[index][index];
//     }
//
//
//
//
//     // loads assets from an external file
//     fn load_ext_assets(&mut self){
//         unimplemented!()
//     }
//
//
//
//     // Option 1 to clear everything
//     fn clear_all(&mut self){
//         self.asstd_fns.clear();
//         self.entities.clear();
//         self.free_indices.clear();
//         self.messages.clear();
//     }
//
//
//     // option 2 to clear up everything
//     fn clean_all(&mut self){
//         self.asstd_fns.iter_mut().for_each(|x|{
//             *x=None;
//         });
//
//         // create code to clean others too
//     }
//
//
//
//     //===========================================================
//     // highly beta stuff
//     // for getting the objects in proxmity for world
//     // for collision check, and those little indicator boxes
//
//     pub fn get_objects(&mut self,index:usize){
//         use std::ops::IndexMut;
//         self.world.get_adj_objs(&self.entities[index]);
//         //self.entities[index];
//         for m in 0..self.entities.len(){
//             if m==index{
//                 continue
//             }
//             else{
//             //    physics_problem(&mut self.entities[index],&mut self.entities[m]);
//             }
//         }
//     }
//
//
//
//     //  todo : asynchronous loading unloading
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//     //Dummy stuff
//     //  Returns a dummy entity (not really, it probably might panic)
//     fn get_dummy_entity()->Entity{
//         Entity::new("dummy".to_owned(),0.0,0.0,0.0,0.0,0.0)
//     }
//
//     // returns a function that does nothing
//     fn dummy_function(_:&mut Entity,_val:SendValue)->SendValue{
//         SendValue::Idle
//     }
// }
//
//
//
// //create Loader
// // create means to load / destroy entities on the fly
// #[cfg(test)]
// fn test(){
//
// }
//
//
// // for tests
//
//
// //
