//Going to become more complicated because lifetime specifiers
// Game object for test
use std::fmt::Debug;

pub struct TestObject{
    pub props:u32,   //u32 for now , contains properties and behaviour
    x:i32,
    y:i32,
}

//Objects in a block
pub struct BlockObjects{
    object:Vec<TestObject>,
}


pub struct World{
    block_map:Vec<Vec<BlockObjects>>,
    block_width:u32,
    block_height:u32,
}

impl World {

    //generates a new world

    pub fn generate(blocks_x:usize,blocks_y:usize)->World{
        let mut map:Vec<Vec<BlockObjects>>=Vec::new();

        for m in 0..blocks_y{
            map.push(Vec::new());

            for n in 0..blocks_x{
                    map[m].push(

                        BlockObjects{
                            object:vec![
                            TestObject{
                                props:20,x:10,y:10
                            }
                            ]
                        })
                    }
                }

    World{
        block_map:map,
        block_width:blocks_x as u32,
        block_height:blocks_y as u32
        }
    }


pub fn display(& mut self){
    for m in 0..self.block_height as usize{
        for n in 0..self.block_width as usize{
            print!("[]" );
        }
        println!();

    }
    println!("--------------------");
}


pub fn test_display(&mut self){

    self.block_map.iter().for_each(|x| {
                                        x.iter().for_each(|_y| print!("[]"));
                                        println!();
                                });
                println!("==========================");
            }


}
