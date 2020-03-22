/*
Just a note here:
    All values in the engine will be represented as (x,y) and not (y,x)
*/


//Going to become more complicated because lifetime specifiers
// Game object for test
pub mod object_loader;

use super::gameobject;
//use rand::prelude::*;

//Block data
pub struct BlockObjects{
    pub object:Vec<(gameobject::GameObject,i32,i32)>,
}

pub struct World{
    pub block_map:Vec<Vec<BlockObjects>>,
    pub block_width:i32,
    pub block_height:i32,
}

impl World {

    // generates a new world
    // The initial objects are placed for tests
    pub fn generate(blocks_x:usize,blocks_y:usize,_block_width:i32,_block_height:i32) -> World {

        if blocks_x < 6 || blocks_y < 6{
             panic!("There must be atleast 6 x and 6 y blocks")
        }

        //let mut rng = rand::thread_rng();

        let mut map:Vec<Vec<BlockObjects>> = Vec::new();

        for m in 0..blocks_y{
            map.push(Vec::new());
            for n in 0..blocks_x {
                map[m].push(
                    BlockObjects {
                        //one game object at center of each block
                        object :  vec![(gameobject::GameObject::new(),
                                        //just to clear rls
                                        // if working comment _block_width/2 and uncomment all rng thingies
                                        _block_width * ((n) as i32) + _block_width/2 /*rng.gen_range(0, _block_width)*/ ,
                                        _block_height * ((m) as i32) + _block_height/2 )/*rng.gen_range(0, _block_height) )*/]
                    }
                )
            }
        }

    World {
        block_map: map,
        block_width: _block_width  as i32,
        block_height: _block_height as i32
    }
}

    //for tests
    pub fn overview_display(&mut self){

        self.block_map.iter().for_each(|x| {
                x.iter().for_each(|_y| print!("[]"));
                println!();
                });

    }


    //for tests
    pub fn obj_disp_per_block(&self){
        for m in self.block_map.iter(){
            for n in m.iter(){
                print!("|{:?}|",n.object.iter().map(|x| {
                    (x.1,x.2)
                }).collect::<Vec<(i32,i32)>>());
            }
            println!("");
        }
    }

    //for tests
    pub fn obj_display(&mut self){
            self.block_map.iter().for_each(|x|{
                x.iter().for_each(|y| {
                    y.object.iter().for_each(|m| {
                        print!("obj (x,y)=({:?},{:?})",m.1,m.2)});
                });
                println!("")
            })
    }


    pub fn load_object(&mut self,coord:(i32,i32),_value:super::gameobject::GameObject){
        self.block_map[(coord.1/self.block_height) as usize ]
                      [(coord.0/self.block_width) as usize ]
                      .object.push((_value,coord.0,coord.1));

    }

    // loads object from Json
    // Gameobject could be made different
    // or not
    pub fn load_objs_from_json(&mut self){
        object_loader::loader().iter().for_each(|v|{
            self.load_object((v.1,v.2),gameobject::GameObject::new());
        });

    }

    pub fn get_max_blocks(&self)->(i32,i32){
        ((self.block_map[0].len()-1) as i32,(self.block_map.len()-1) as i32)
    }


}
