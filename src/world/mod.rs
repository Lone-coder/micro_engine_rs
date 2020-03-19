//Going to become more complicated because lifetime specifiers
// Game object for test
pub mod object_loader;



//Objects in a block
pub struct BlockObjects{
    pub object:Vec<(super::game_object::GameObject,i32,i32)>,
}


pub struct World{
    pub block_map:Vec<Vec<BlockObjects>>,
    pub block_width:i32,
    pub block_height:i32,
}

impl World {

    // generates a new world
    // The initial objects are placed for tests
    pub fn generate(blocks_x:usize,blocks_y:usize,_block_width:i32,_block_height:i32)->World{

        if blocks_x < 6||blocks_y < 6{
             panic!("There must be atleast 6 x and 6 y blocks")
        }


        let mut map:Vec<Vec<BlockObjects>>=Vec::new();
        for m in 0..blocks_y{
            map.push(Vec::new());

            for n in 0..blocks_x{
                map[m].push(
                    BlockObjects{
                        object : vec![(super::game_object::GameObject::new(),
                                        _block_width * (n-1) as i32 + _block_width/2 ,
                                        _block_height * (n-1) as i32 +_block_height/2 )]
                    }
                )
            }
        }

    World{
        block_map:map,
        block_width:blocks_x  as i32,
        block_height:blocks_y as i32
        }
    }



    //for tests
    pub fn overview_display(&mut self){

        self.block_map.iter().for_each(|x| {
                x.iter().for_each(|_y| print!("[]"));
                println!();
                });
                super::print_bar();

    }

    //for tests
    pub fn obj_display(&mut self){
            self.block_map.iter().for_each(|x|{
                x.iter().for_each(|y| {
                    y.object.iter().for_each(|m| println!("{:?}",m.1))
                })
            })
    }



    pub fn load_object(&mut self,coord:(i32,i32),_value:super::game_object::GameObject){

        self.block_map[(coord.1/self.block_height) as usize ]
                      [(coord.0/self.block_width) as usize ]
                      .object.push((_value,coord.0,coord.1));

    }


}
