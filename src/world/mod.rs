// create subspaces
// create a data structure that makes it easy for stuff


//temporarily testing stuff
use crate::entity::staticEntity;
use crate::entity::dynamicEntity;
use crate::physics::collision_rect;

pub struct World{
    layout:Vec<Vec<Vec<staticEntity::StaticEntity>>>,
    block_width:usize,
    block_height:usize
}


impl World{
    // #todo
    //  Automatically creates the block size based on calculations
    //  Inorder to make it feel less weird and get lots of time to load
    pub fn create_new(x_blocks:usize,y_blocks:usize)->World{

        let mut layout=Vec::new();
        (0..y_blocks).for_each(|x|{
                layout.push(Vec::new());
                (0..x_blocks).for_each(|_|{ layout[x].push(Vec::new())})
            });

        World{
            layout:layout,
            block_width:0,
            block_height:0
        }
        }


    //================================WARNING!!==================================================
    // =======================NEVER EVER EVER USE THIS !!!======================================
    // todo
    // Game specific things
    // The following function is probably gonna be used for procedurally generated games
    // or huge open world games where rapid loading/unloading is required
    pub fn extend(&mut self,x_blocks:Option<usize>,y_blocks:Option<usize>){
        //extends only linearly
        // and removes other parts
        // remove it when ready
        unimplemented!();


        (0..y_blocks.unwrap_or(0usize)).for_each(|y|{
            (0..x_blocks.unwrap_or(0usize)).for_each(|_|{ self.layout[y].push(Vec::new())})
        })
    }
    //==========================================================================================


    // Loads Entities into the world
    pub fn loader(&mut self,_entity:staticEntity::StaticEntity){
        let x=_entity.get_x()/self.layout[0].len();
        let y=_entity.get_y()/self.layout.len();
        self.layout[y][x].push(_entity);
    }

    //  Gets the current block the entity is in
    pub fn get_block(&self,x:usize,y:usize)->(usize,usize){
        (x/self.block_height,y/self.block_height)
    }



    //==========================NOTE======================================
    // This method is probably deprecated because physics engine handles it
    // Todo
    // Create a collisions component struct to check for collisions
    // This is probably inefficient as it copies the values during each
    // execution cycle
    // probably have to change to i64
    // gets the objects in the current block
    fn get_objs_in_blk(&self,x:i32,y:i32)->Vec<collision_rect::CollisionRect>{

        unimplemented!();
        let mut out=Vec::new();
        if x<0||y<0||(x>(self.block_width*self.layout[0].len()) as i32)||
            (y>(self.block_height*self.layout.len()) as i32){
            return out
        }
        self.layout[y as usize][x as usize].iter().for_each(|x|{
            //out.push(x.get_components())
            // resolve this issue
        });
        out
    }

//=========================================


    // Redundant code !!!
    // change to incorporate messaging
    // get adjacent blocks
    pub fn get_adj_indices(&self,ent:&dynamicEntity::Entity)->[(i32,i32);4]{
        let v=self.get_block(ent.x,ent.y);
        let out_x=v.0 as i32;
        let out_y=v.1 as i32;
        let bf_x=out_x  + ((self.block_width/2 - ent.x%self.block_width) as i32).signum()  ;
        let bf_y=out_y  + ((self.block_height/2 - ent.y%self.block_height) as i32).signum() ;
        [(out_x,out_y),(out_x,bf_y),(bf_x,out_y),(bf_x,bf_y)]
    }


    //  Gets objects adjacent to a dynamic entity
    pub fn get_adj_objs(&self,_ent:&dynamicEntity::Entity)->Vec<Vec<collision_rect::CollisionRect>>{
        let x=_ent.x;
        let y=_ent.y;
        // Have to change this to return fewer parameters
        let mut out:Vec<Vec<collision_rect::CollisionRect>>=Vec::new();
        let v=self.get_block(x,y);
        let out_x=v.0 as i32;
        let out_y=v.1 as i32;

        //chek if positive or negative here
        let bf_x=v.0 as i32  + ((self.block_width/2 - _ent.x%self.block_width) as i32).signum()  ;
        let bf_y=v.1  as i32 + ((self.block_height/2 - _ent.y%self.block_height) as i32).signum() ;
                    out.push(self.get_objs_in_blk(out_x,out_y));
                    out.push(self.get_objs_in_blk(out_x, bf_y));
                    out.push(self.get_objs_in_blk(bf_x,out_y));
                    out.push(self.get_objs_in_blk(bf_x,bf_y));
            out
        }


        pub fn change_state(&mut self,state:usize,x:usize,y:usize,obj:usize){
            self.layout[y][x][obj].set_state(state)
        }

}


#[test]
fn create_world() {

}
