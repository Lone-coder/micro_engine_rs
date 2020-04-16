use crate::world::tile::DefinedGroup;
use crate::world::tile;
use tile::{Block,TileTypes,Tileable,TileParams};
use tile::{Clump,Route,Individual,GroupList};
use crate::world::camera::Camera;
use sdl2::rect::Rect;
use crate::RenderBuff;

pub struct World{
    tile_types:TileTypes,
    tile_groups:GroupList,
    blocks:Vec<Vec<Block>>,
    block_width:i32,
    block_height:i32,
    x_blocks:usize,
    y_blocks:usize,
    pub camera:Box<Camera>
}

impl World{

    pub fn new(block_width:i32,block_height:i32,nblocks_x:usize,nblocks_y:usize)->World{

        let mut tempblock:Vec<Vec<Block>>=Vec::new();
        (0..nblocks_y).for_each(|row|{
            tempblock.push(Vec::new());
            (0..nblocks_x).for_each(|_block|{
                tempblock[row].push(Block::new())
            })
        });


        World{
            x_blocks:nblocks_x,
            y_blocks:nblocks_y,
            tile_types:TileTypes{
                types:Vec::new()
            },
            tile_groups:GroupList{groups:Vec::new()},
            blocks:tempblock,
            block_width:block_width,
            block_height:block_height,
            camera:Box::new(Camera::new(0,0,0,0))
        }
    }


    pub fn load_clump(&mut self,clump:Clump,block:(usize,usize)){
        self.blocks[block.1][block.0].tiles.push(Box::new(clump));
    }

    pub fn load_route(&mut self,route:Route,block:(usize,usize)){
        self.blocks[block.1][block.0].tiles.push(Box::new(route))
    }

    pub fn load_individual(&mut self,tile:Individual,block:(usize,usize)){
        self.blocks[block.1][block.0].tiles.push(Box::new(tile))
    }

    // not sure , so left it there
    pub fn load<T: 'static +  Tileable>(&mut self,val:T,block:(usize,usize)){
        self.blocks[block.1][block.0].tiles.push(Box::new(val))
    }


    pub fn get_tile_rects(&self,id:usize,coords:(i32,i32))->RenderBuff{
        let tile_attr=&self.tile_types.types[id];
        let w=tile_attr.width;
        let h=tile_attr.height;
        RenderBuff{
            texture_id  : tile_attr.texture_id,
            src_rect    : Rect::new(tile_attr.sheet_coords.0,tile_attr.sheet_coords.1
                                    ,w as u32 , h as u32),
            dst_rect    : Rect::new(self.camera.get_x_m() - (coords.0-w/2),
                                    self.camera.get_y_m() - (coords.0-h/2),w as u32,h as u32)
        }
    }


    pub fn render_check(&self,x:i32,y:i32,w:i32,h:i32)->bool{
        let cond_x=(self.camera.get_x_m()-(x-w)).abs();
        let cond_y=(self.camera.get_y_m()-(x-h)).abs();
        if cond_x>self.camera.width && cond_y>self.camera.height{
            true
        }
        else{
            false
        }
    }

    pub fn render_tiles(&self)->Vec<Vec<RenderBuff>>{
        let x_blk=self.camera.x/self.block_width ;
        let y_blk=self.camera.y/self.block_height;
        let mut buff:Vec<Vec<RenderBuff>>=Vec::new();
        ((y_blk-1)..(y_blk+2)).for_each(|row|{
            ((x_blk-1)..(x_blk+2)).for_each(|cell|{
                    if self.render_condition(cell,row){
                        for m in &self.blocks[cell as usize][row as usize].tiles{
                            //buff.push(m.tile(&self.tile_types,&self.camera));
                            buff.push(m.group_tile(&self.tile_types,&self.camera,&self.tile_groups));
                    }
                }
            })
        });
        buff
    }


    pub fn load_tile_types(&mut self
                        ,name:String
                        ,texture_id:usize
                        ,width:i32
                        ,height:i32
                        ,sheet_x:i32
                        ,sheet_y:i32){
        self.tile_types.types.push(
            TileParams{
                texture_id:texture_id,
                texture_name:name,
                sheet_coords:(sheet_x,sheet_y),
                width:width,
                height:height
            }
        )
    }



    pub fn render_condition(&self,x:i32,y:i32)->bool{
        if (x>=0&&y>=0)&&(x<(self.x_blocks as i32))&&(y<(self.y_blocks as i32)){
            true
        }
        else{
            false
        }
    }

    pub fn load_defined_group(&mut self,group:DefinedGroup){
        self.tile_groups.groups.push(group)
    }



}


#[test]
fn make_world() {
    let mut world=World::new(2000,2000,20,20);

    let tileobject=tile::Individual{
        tile_type:0,
        coords:(20,20)
    };
}
