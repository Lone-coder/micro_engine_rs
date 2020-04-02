// create subspaces
// create a data structure that makes it easy for stuff


//temporarily testing stuff
use crate::entity::staticEntity;
use crate::entity::dynamicEntity;


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


    // todo
    // Game specific things
    // The following function is probably gonna be used for procedurally generated games
    // or huge open world games where rapid loading/unloading is required
    pub fn extend(&mut self,x_blocks:Option<usize>,y_blocks:Option<usize>){
        //extends only linearly
        // and removes other parts
        (0..y_blocks.unwrap_or(0usize)).for_each(|y|{
            (0..x_blocks.unwrap_or(0usize)).for_each(|_|{ self.layout[y].push(Vec::new())})
        })
    }


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


    // Todo
    // Create a collisions component struct to check for collisions
    fn get_objs_in_blk(&self,x:usize,y:usize)->Vec<crate::core::components::physics::Physics>{
        let out=Vec::new();
        if x<0||y<0||x>self.block_width*self.layout[0].len()||
            y>self.block_height*self.layout.len(){
            return out
        }
        self.layout[y][x].iter().for_each(|x|{
            unimplemented!()
        });
        out
    }

    // get adjacent blocks
    pub fn get_adj_indices(){
        unimplemented!()
    }


    //  Gets objects adjacent to a dynamic entity
    pub fn get_adj_objs(&self,_ent:&dynamicEntity::Entity){
        let x=_ent.x;
        let y=_ent.y;
        // Have to change this to return fewer parameters
        let mut out:Vec<Vec<crate::core::components::physics::Physics>>=Vec::new();
        let v=self.get_block(x,y);

        //chek if positive or negative here
        let bf_x=v.0 + ((self.block_width/2 - _ent.x%self.block_width) as i32).signum() as usize ;
        let bf_y=v.1 + ((self.block_height/2 - _ent.y%self.block_height) as i32).signum() as usize ;
                    out.push(self.get_objs_in_blk(v.0,v.1));
                    out.push(self.get_objs_in_blk(v.0, bf_y));
                    out.push(self.get_objs_in_blk(bf_x,v.1));
                    out.push(self.get_objs_in_blk(bf_x,bf_y));
        }
}


#[test]
fn create_world() {
    let entity=staticEntity::StaticEntity::new(1500,1500);
    let mut world=World::create_new(20,20);
    world.loader(entity);
}
