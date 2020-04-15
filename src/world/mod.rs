// // create subspaces
// // create a data structure that makes it easy for stuff
//
//
// //temporarily testing stuff
// use crate::entity::staticEntity;
// use crate::entity::dynamicEntity;
// use crate::physics::collision_rect;
//
// pub struct World{
//     layout:Vec<Vec<Vec<staticEntity::StaticEntity>>>,
//     block_width:usize,
//     block_height:usize,
//     x_blocks:usize,
//     y_blocks:usize,
//     world_width:usize,
//     world_height:usize
// }
//
// impl World{
//     pub fn new(x_blocks:usize,y_blocks:usize)->World{
//
//         let mut layout=Vec::new();
//         (0..y_blocks).for_each(|x|{
//                 layout.push(Vec::new());
//                 (0..x_blocks).for_each(|_|{ layout[x].push(Vec::new())})
//             });
//
//         World{
//             layout:layout,
//             block_width:1600,
//             block_height:1200,
//             x_blocks:x_blocks,
//             y_blocks:y_blocks,
//             world_width:x_blocks*1600,
//             world_height:y_blocks*1200
//         }
//         }
//
//
//     // Loads Entities into the world
//     pub fn loader(&mut self,_entity:staticEntity::StaticEntity){
//         println!("Debug : world/mod.Result:39 loader() entity.get_x()={:?}",_entity.get_x() );
//         let x=_entity.get_x()/self.block_width;
//         let y=_entity.get_y()/self.block_height;
//         if !(x>self.x_blocks||y>self.y_blocks){
//             self.layout[y][x].push(_entity);
//         }
//     }
//
//
//     //  Gets the current block the entity is in
//     pub fn get_block(&self,x:usize,y:usize)->(usize,usize){
//         (x/self.block_height,y/self.block_height)
//     }
//
//
//
//     //  Gets the blocks adjacent to the entity
//     //  [x] | [x][x] | [ ]
//     //  ---------------
//     //  [x] | [0][x] | [ ]
//     //  [x] | [x][x] | [ ]
//     //  When the gameobject is in the position marked [0],
//     //  The blocks [0] and [x] are returned
//     pub fn get_adj_indices(&self,ent:&dynamicEntity::Entity)->[(i32,i32);4]{
//         let v=self.get_block(ent.x,ent.y);
//         let out_x=v.0 as i32;
//         let out_y=v.1 as i32;
//         let bf_x=out_x  + ((self.block_width/2 - ent.x%self.block_width) as i32).signum()  ;
//         let bf_y=out_y  + ((self.block_height/2 - ent.y%self.block_height) as i32).signum() ;
//         [(out_x,out_y),(out_x,bf_y),(bf_x,out_y),(bf_x,bf_y)]
//     }
//
//
//
//     //  Gets objects adjacent to a dynamic entity
//     pub fn get_adj_objs(&self,ent:&dynamicEntity::Entity)->Vec<&staticEntity::StaticEntity>{
//         let mut ents:Vec<&staticEntity::StaticEntity>=Vec::new();
//         let v=self.get_adj_indices(ent);
//         v.iter().for_each(|block|{
//             if (block.1>=0 && block.0>=0)&&
//             ((block.0 as usize)<self.x_blocks && (block.1 as usize )< self.y_blocks){
//             self.layout[block.1 as usize ][block.0 as usize].iter().for_each(|object| ents.push(object))
//         }
//     });
//         ents
//     }
//
//     // loads static entities from a file
//     pub fn load_static_entities(&mut self,file:&str){
//         use crate::asset_loader::load_world_static_entities;
//         let out=load_world_static_entities(file);
//         for m in out{
//             println!("Debug : /world/mod.rs:88 load_static_entities() m = {:?}",m );
//             self.loader(m);
//         }
//     }
//
//     // Private for now
//     //  Because In-game changing of block parameters is illegal
//     fn set_block_params(&mut self,block_width:usize,block_height:usize){
//         self.block_width=block_width;
//         self.block_height=block_height;
//         self.world_width=self.x_blocks*block_width;
//         self.world_height=self.y_blocks*block_height;
//     }
//
// }
//
//
// impl std::fmt::Debug for World{
//     fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//     self.layout.iter().for_each(|row|{
//             println!("");
//             row.iter().for_each(|column|{
//                 if column.is_empty(){
//                     print!("[ ]");
//                 }else{
//                     print!("[{}]",column.len());
//                 }
//             })
//         });
//
//         Ok(())
//     }
// }
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
// #[cfg(test)]mod tests{
// #[test]
// fn create_world() {
//
// }
// }
