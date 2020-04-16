//  GOING ALL OUT BATSHIT CRAZY
//  COMPLETELY UNOPTIMIZED MAY FRY YOUR COMPUTER AND YOU
//  PROCEED WITH CAUTION !!!!!!!!!!!1

// camera module for rendering
pub mod camera;

//  Additional modules for tile generation
pub mod tile;
pub mod naive_tile;
// Newworld module for testing new features
pub mod new_world;

// create a data structure that makes it easy for stuff


//temporarily testing stuff
use crate::entity::staticEntity;
use crate::entity::dynamicEntity;
use naive_tile::{Blocks,Tiletypes};
use crate::RenderBuff;













pub struct World{
    pub tile_types:Tiletypes,
    pub layout:Vec<Vec<Blocks>>,
    pub block_width:usize,
    pub block_height:usize,
    pub x_blocks:usize,
    pub y_blocks:usize,
    pub world_width:usize,
    pub world_height:usize,
    pub camera:(i32,i32)
}

impl World{
    pub fn new(x_blocks:usize,y_blocks:usize)->World{

        let  mut layout:Vec<Vec<Blocks>>=Vec::new();
        (0..y_blocks).for_each(|row|{
            layout.push(Vec::new());
            (0..x_blocks).for_each(|_block|{
                layout[row].push(Blocks::new())
            });
        });


        World{
            tile_types:Tiletypes{tile_type:Vec::new()},
            layout:layout,
            block_width:1600,
            block_height:1200,
            x_blocks:x_blocks,
            y_blocks:y_blocks,
            world_width:x_blocks*1600,
            world_height:y_blocks*1200,
            camera:(0,0)
        }
        }


    // Loads Entities into the world
    pub fn loader(&mut self,_entity:staticEntity::StaticEntity){
        println!("Debug : world/mod.Result:39 loader() entity.get_x()={:?}",_entity.get_x() );
        let x=_entity.get_x()/self.block_width;
        let y=_entity.get_y()/self.block_height;
        if !(x>self.x_blocks||y>self.y_blocks){
            self.layout[y][x].static_entities.push(_entity);
        }
    }


    //  Gets the current block the entity is in
    pub fn get_block(&self,x:i32,y:i32)->(i32,i32){
        (x/(self.block_width as i32),y/(self.block_height as i32))
    }



    //  Gets the blocks adjacent to the entity
    //  [x] | [x][x] | [ ]
    //  ---------------
    //  [x] | [0][x] | [ ]
    //  [x] | [x][x] | [ ]
    //  When the gameobject is in the position marked [0],
    //  The blocks [0] and [x] are returned
    pub fn get_adj_indices(&self,ent:&dynamicEntity::Entity)->[(i32,i32);4]{
        let v=self.get_block(ent.x as i32,ent.y as i32);
        let out_x=v.0 as i32;
        let out_y=v.1 as i32;
        let bf_x=out_x  + ((self.block_width/2 - ent.x%self.block_width) as i32).signum()  ;
        let bf_y=out_y  + ((self.block_height/2 - ent.y%self.block_height) as i32).signum() ;
        [(out_x,out_y),(out_x,bf_y),(bf_x,out_y),(bf_x,bf_y)]
    }



    //  Gets objects adjacent to a dynamic entity
    pub fn get_adj_objs(&self,ent:&dynamicEntity::Entity)->Vec<&staticEntity::StaticEntity>{
        let mut ents:Vec<&staticEntity::StaticEntity>=Vec::new();
        let v=self.get_adj_indices(ent);
        v.iter().for_each(|block|{
            if (block.1>=0 && block.0>=0)&&
            ((block.0 as usize)<self.x_blocks && (block.1 as usize )< self.y_blocks){
            self.layout[block.1 as usize ][block.0 as usize].static_entities.iter().for_each(|object| ents.push(object))
        }
    });
        ents
    }

    // loads static entities from a file
    pub fn load_static_entities(&mut self,file:&str){
        use crate::asset_loader::load_world_static_entities;
        let out=load_world_static_entities(file);
        for m in out{
            println!("// Debug : /world/mod.rs:88 load_static_entities() m = {:?}",m );
            self.loader(m);
        }
    }

    // Private for now
    //  Because In-game changing of block parameters is illegal
    fn set_block_params(&mut self,block_width:usize,block_height:usize){
        self.block_width=block_width;
        self.block_height=block_height;
        self.world_width=self.x_blocks*block_width;
        self.world_height=self.y_blocks*block_height;
    }


    //  Bottleneck
    pub fn get_tiles_in_9_blk_set(&self,cam_x:i32,cam_y:i32)->Vec<Vec<RenderBuff>>{
        let out=self.get_block(cam_x ,cam_y );
        let y =out.1;
        let x =out.0;
        //println!("// Debug : src/world/mod.rs:140 out(0,1) are {:?}",(out.0,out.1) );
        let mut buf:Vec<Vec<RenderBuff>>=Vec::new();
        ((y-1)..(y+2)).for_each(|y_val|{
            ((x-1)..(x+2)).for_each(|x_val|{
            if self.render_condition(x_val,y_val){
                buf.push(self.layout[y_val as usize][x_val as usize].render_tiles(&self.tile_types,cam_x,cam_y));
            }
        })
        });
        //  println!("// Debug : src/world/mod.rs:149 buf.len()={:?}",buf.len());

        buf
    }


    pub fn render_condition(&self,x:i32,y:i32)->bool{
        if (x>=0&&y>=0)&&(x<(self.x_blocks as i32))&&(y<(self.y_blocks as i32)){
            true
        }
        else{
            false
        }
    }

}


impl std::fmt::Debug for World{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    self.layout.iter().for_each(|row|{
            println!("");
            row.iter().for_each(|column|{
                if column.static_entities.is_empty(){
                    print!("[ ]");
                }else{
                    print!("[{}]",column.static_entities.len());
                }
            })
        });

        Ok(())
    }
}












#[cfg(test)]mod tests{
#[test]
fn create_world() {

}
}
