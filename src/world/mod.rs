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



    //  Gets the blocks adjacent to te entity
    //  [x] | [x][x] | [ ]
    //  ---------------
    //  [x] | [0][x] | [ ]
    //  [x] | [x][x] | [ ]
    //  When the gameobject is in the position marked [0],
    //  The blocks [0] and [x] are returned
    pub fn get_adj_indices(&self,ent:&dynamicEntity::Entity)->[(i32,i32);4]{
        let v=self.get_block(ent.x,ent.y);
        let out_x=v.0 as i32;
        let out_y=v.1 as i32;
        let bf_x=out_x  + ((self.block_width/2 - ent.x%self.block_width) as i32).signum()  ;
        let bf_y=out_y  + ((self.block_height/2 - ent.y%self.block_height) as i32).signum() ;
        [(out_x,out_y),(out_x,bf_y),(bf_x,out_y),(bf_x,bf_y)]
    }



    //  Gets objects adjacent to a dynamic entity
    pub fn get_adj_objs(&self,ent:&dynamicEntity::Entity)->Vec<&staticEntity::StaticEntity>{
        let mut out:Vec<&staticEntity::StaticEntity>=Vec::new();
        let v=self.get_adj_indices(ent);
        v.iter().for_each(|block|{
            self.layout[block.1 as usize ][block.0 as usize].iter().for_each(|object| out.push(object))
        });
        out
    }
}



#[cfg(test)]mod tests{
#[test]
fn create_world() {

}
}
