// create subspaces
// create a data structure that makes it easy for stuff


//temporarily testing stuff
use crate::entity::staticEntity;


pub struct World{
    layout:Vec<Vec<Vec<staticEntity::StaticEntity>>>,
}


impl World{
    // #todo
    //  Automatically creates the block size based on calculations
    //  Inorder to make it feel less weird and get lots of time to load
    pub fn create_new(&mut self,x_blocks:usize,y_blocks:usize){
        (0..y_blocks).for_each(|x|{
                self.layout.push(Vec::new());
                (0..x_blocks).for_each(|_|{ self.layout[x].push(Vec::new())})
            })
        }



    // Game specific things
    // The following function is probably gonna be used for procedurally generated games
    // or huge open world games where rapid loading/unloading is required
    pub fn extend(&mut self,x_blocks:Option<usize>,y_blocks:Option<usize>){
        //extends only linearly
        // and removes other parts
        (0..y_blocks.unwrap_or(0usize)).for_each(|y|{
            (0..x_blocks.unwrap_or(0usize)).for_each(|_|{})
        })
    }




    pub fn loader(&mut self,_entity:staticEntity::StaticEntity){

    }
}
