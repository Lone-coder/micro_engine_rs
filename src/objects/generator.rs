use super::world;
use world::World;
use super::objects::ObjectList;
use std::collections::VecDeque;
use super::world::ObjBlkCoords;


//generate world is to generate from json

pub fn generate_world<'a>
(block_width:i32,block_height:i32,blocks:(usize,usize),objects:&'a ObjectList)->World<'a>{
    let mut new_world = World::new(block_width,
        block_height,(blocks.0 as i32,blocks.1 as i32));
        for m in 0..blocks.1{

                new_world.world_map[m][m].static_objects.push_back(ObjBlkCoords{
                    st_object:&objects.static_object_list["Stone1"],
                    coords:((m as i32)*new_world.block_width+50,(m as i32)*new_world.block_height+50)

                })

        }

    new_world
 }
