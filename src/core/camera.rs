/*

    Scheduled for refactoring

    Not deleted because looking up

*/




// Useful info:
// Here &'a world implies that the Camera lives atleast for as long as the world lives
// or any reference to 'a musn't live longer than it



    // Returns all objects in proximity for collisions
    // and rendering

    // to get all game objects loop over the value returned

    /*
            [B][B][B][B]
            [X][X][X][B]
            [X][C][X][B]
            [X][X][X][B]
            [B][B][B][B]

            in the above example:
            * C is the block containing camera
            * All objects in the units marked X will be rendered
            * and used for collisions

    */




/*
    pub fn get_objs_in_scene(&mut self) -> Vec<(&super::gameobject::GameObject,i32,i32)> {

        let mut proximity_blocks:Vec<(&super::gameobject::GameObject,i32,i32)> = Vec::new();

        let world = self.world.unwrap();
        let block = self.get_block().unwrap();

        //debug
        // returns x,y
        //println!("");
        //println!("Current block is {:?}",(block.1,block.0));

        for m in 0..3 {
            for n in 0..3 {
                //debug test
                if self.is_not_within_bounds((block.1 + 1- m,block.0+1-n)){
                    continue
                }
                println!(" block  map : ({:?},{:?})",block.1 + 1- m,block.0+1-n );
                                //y                         //x
                world.block_map[(block.1 + 1- m) as usize][(block.0+1-n) as usize].
                object.iter()
                .for_each(|x| {
                    proximity_blocks.push((&x.0,x.1-self.x,x.2-self.y));
                    //print!("blocks are :{:?}",(x.1,x.2));
                    //println!("");
                    //println!(" relative values are {:?}", (x.1-self.x,x.2-self.y));
                });
            }
        }
            proximity_blocks
    }


*/


/*
    pub fn is_not_within_bounds(&self,block:(i32,i32))->bool{
            let max_blocks=self.world.unwrap().get_max_blocks();
            if ((block.0<0)||(block.0>max_blocks.0))||((block.1<0)||(block.1>max_blocks.1)){
                true
            }else{
                false
            }
    }

*/
