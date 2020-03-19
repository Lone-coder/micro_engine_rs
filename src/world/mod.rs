//Going to become more complicated because lifetime specifiers
// Game object for test




//Objects in a block
pub struct BlockObjects{
    object:Vec<super::game_object::GameObject>,
}


pub struct World{
    pub block_map:Vec<Vec<BlockObjects>>,
    pub block_width:u32,
    pub block_height:u32,
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
                        object : vec![super::game_object::GameObject::new()]
                    }
                )
            }
        }

    World{
        block_map:map,
        block_width:blocks_x as u32,
        block_height:blocks_y as u32
        }
    }




    pub fn test_display(&mut self){

        self.block_map.iter().for_each(|x| {
                x.iter().for_each(|_y| print!("[]"));
                println!();
                });

        println!("===================================");

    }

    pub fn load_objects(&mut self){
        
    }


}
