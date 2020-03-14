
use std::fmt;
use super::super::core::physicshandle::StaticObject;

use std ::collections::VecDeque;
use super::camera::Camera;

pub struct ObjBlkCoords<'a>{
    pub st_object:&'a StaticObject,
    pub coords:(i32,i32)        //assert >0
}

impl fmt::Debug for ObjBlkCoords<'_>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"O")
}
}


pub struct Block<'a>{
    pub tile_type:Option<&'a StaticObject>,
    pub static_objects:VecDeque<ObjBlkCoords<'a>>

}

impl<'a> Block<'_>{
    pub fn new()->Block<'a>{
            Block{
                tile_type:None,
                static_objects:VecDeque::new()
            }
    }
}

impl fmt::Debug for Block<'_>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        for _ in self.static_objects.iter(){
        write!(f,"O");
        }
        write!(f,"T")
    }

}



pub struct World<'a>{
    pub max_width:i32,
    pub max_height:i32,
    pub block_width:i32,
    pub block_height:i32,
    pub world_map:Vec<Vec<Block<'a>>>,
}

impl World<'_>{
    pub fn new<'a>(screen_width:i32,screen_height:i32,blocks:(i32,i32))->World<'a>{
        let mut new_world=World{
            max_width:screen_width*blocks.0,
            max_height:screen_height*blocks.1,
            block_width:screen_width,
            block_height:screen_height,
            world_map:Vec::new()
        };
        for m in 0..blocks.1 as usize{
            new_world.world_map.push(Vec::new());
            for _n in 0..blocks.0 as usize{
                new_world.world_map[m].push(Block{
                    tile_type:None,
                    static_objects:VecDeque::new()
                });

            }
        }

        new_world
    }


    pub fn display(&self){
        for m in self.world_map.iter(){
            println!("{:?}",m );
        }
    }
}
