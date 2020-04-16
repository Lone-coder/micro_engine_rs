
// Imports
use crate::entity::staticEntity;
use crate::world::camera::Camera;
use crate::RenderBuff;

// TODO : create condition for  render check



// Direction definitions

const TOP:u8=1;
const TOP_RIGHT:u8=2;
const RIGHT:u8=3;
const BOTTOM_RIGHT:u8=4;
const BOTTOM:u8=5;
const BOTTOM_LEFT:u8=6;
const LEFT:u8=7;
const TOP_LEFT:u8=8;
//Result/errors Definitions





//  Trait definitions
pub trait Tileable{
    fn tile(&self,tiles:&TileTypes,camera:&Camera)->Vec<RenderBuff>{Vec::new()}
    fn interior_check(&self,x:i32,y:i32)->bool;
    fn group_tile(&self,tiles:&TileTypes,camera:&Camera,group:&GroupList)->Vec<RenderBuff>{Vec::new()}
}

/// Basic constructor for a tile type,
/// Determines what type of tile is
pub struct TileParams{
    pub texture_id:usize,
    pub texture_name:String,
    pub sheet_coords:(i32,i32),
    pub width:i32,
    pub height:i32,
}
// vector of all tile types
// this is stored in world and looked up when needed
pub struct TileTypes{
    pub types:Vec<TileParams>
}

//  Basic block unit in the world
pub struct Block{
    pub generic_tile:usize,
    pub tiles:Vec<Box<dyn Tileable>>,
    pub static_entities:Vec<staticEntity::StaticEntity>
}
impl Block{
    pub fn new()->Block{
        Block{
            generic_tile:0,
            tiles:Vec::new(),
            static_entities:Vec::new()
        }
    }
}



// for groups of tiles that need to be combined to form a single structure
pub struct GroupDefinition{
    pub tile_type:usize,
    pub off_x:i32,
    pub off_y:i32
}
pub struct DefinedGroup{
    pub name:String,
    pub groups:Vec<GroupDefinition>,
    pub off:(i32,i32)
}
// List of groups
pub struct GroupList{
    pub groups:Vec<DefinedGroup>
}




pub struct GroupTile{
    defined_group_index:usize,
    x:i32,
    y:i32
}
impl Tileable for GroupTile{
    fn group_tile(&self,tile_types: &TileTypes, camera: &Camera, groups: &GroupList)->Vec<RenderBuff>{

        let mut buff=Vec::new();
        let group=&groups.groups[self.defined_group_index];
        for m in &group.groups{
            buff.push(get_tile_rects(tile_types,m.tile_type,(self.x+m.off_x,self.y+m.off_y),camera));
        }
        buff
    }
    fn interior_check(&self, _: i32, _: i32) -> bool { todo!() }

    }


pub struct GroupTileRoute{
    pub start_coord:(i32,i32),
    pub defined_group_index:usize,
    pub numbers:i32,
    pub direction:u8,
}

impl Tileable for GroupTileRoute{
    fn group_tile(&self,tile_types: &TileTypes, camera: &Camera, groups: &GroupList)->Vec<RenderBuff>{
        let mut buff =Vec::new();
        let dir=get_direction(self.direction);
        let group=&groups.groups[self.defined_group_index];
        (0..self.numbers).for_each(|number|{

            for m in &group.groups{
                //println!("//Debug  : world/tile.rs :121 Tile_coords {:?}",
                //    (self.start_coord.0 +   number*group.off.0*dir.0  +  m.off_x
                //    ,self.start_coord.1 +   number*group.off.1*dir.1  +  m.off_y ));
            }



            for m in &group.groups{
                buff.push(get_tile_rects(tile_types
                                    ,m.tile_type
                                    ,(   self.start_coord.0 +   number*group.off.0*dir.0  +  m.off_x
                                        ,self.start_coord.1 +   number*group.off.1*dir.1  +  m.off_y )
                                    ,camera));
                                }
                            });
        buff
    }
    fn interior_check(&self, _: i32, _: i32) -> bool { todo!() }
    }
// groupings
pub struct Clump {
    tile_type:usize,
    start_coords:(i32,i32),
    end_coords:(i32,i32)
}

impl Tileable for Clump{
    fn tile(&self,tile_types:&TileTypes,cam:&Camera) -> Vec<RenderBuff> {
        let x_len       = self.end_coords.0-self.start_coords.0;
        let y_len       = self.end_coords.1-self.end_coords.0;
        let nums_x      = x_len/tile_types.types[self.tile_type].width;
        let nums_y      = y_len/tile_types.types[self.tile_type].height;
        let tile_width  = tile_types.types[self.tile_type].width;
        let tile_height = tile_types.types[self.tile_type].height;
        let mut buff:Vec<RenderBuff>=Vec::new();

        (0..nums_y).for_each(|row|{
            (0..nums_x).for_each(
                |tile|{
                buff.push(get_tile_rects( tile_types,
                                self.tile_type,
                                (self.start_coords.0 + tile*tile_width,self.start_coords.0+row*tile_height),
                                cam));
                })
        });
        buff
    }

    fn interior_check(&self,x:i32,y:i32) -> bool {
        if (x<self.end_coords.0 && x>self.start_coords.0)&&
            (y<self.end_coords.1 && y>self.start_coords.1){     true    }
            else{   false   }
    }
}



pub struct Route{
    tile_type_id:usize,
    start_coord:(i32,i32),
    direction:u8,
    num_tiles:i32
}
impl Tileable for Route{
    fn tile(&self,tile_types:&TileTypes,cam:&Camera) -> Vec<RenderBuff> {
        let dir=get_direction(self.direction);
        let mut buff:Vec<RenderBuff>=Vec::new();
        let width=tile_types.types[self.tile_type_id].width;
        let height=tile_types.types[self.tile_type_id].height;
        (0..self.num_tiles).for_each(|m|{
            buff.push(get_tile_rects(
                    tile_types,
                    self.tile_type_id,
                    (self.start_coord.0 +m*width*dir.0,
                    self.start_coord.1 +m*height*dir.1),
                    cam));
                });
        buff
    }

    fn interior_check(&self,x:i32,y:i32) -> bool {
        ((x-self.start_coord.0),(y-self.start_coord.1));
        todo!()
    }
}


pub struct Individual{
    pub tile_type:usize,
    pub coords:(i32,i32)
}
impl Tileable for Individual{
    fn tile(&self,tiles:&TileTypes,cam:&Camera) -> Vec<RenderBuff> {
        vec![get_tile_rects(tiles,self.tile_type,self.coords, cam)]
        }
    fn interior_check(&self,_x:i32,_y:i32) -> bool { todo!() }
}

use sdl2::rect::Rect;

pub fn get_tile_rects(  tile_types:&TileTypes,tile_type_id:usize,
                        coords:(i32,i32),cam:&Camera)->RenderBuff{
    let tile_attr=&tile_types.types[tile_type_id];
    let w=tile_attr.width;
    let h=tile_attr.height;
    RenderBuff{
        texture_id:tile_attr.texture_id,
        src_rect:Rect::new(tile_attr.sheet_coords.0,tile_attr.sheet_coords.1
                            ,tile_attr.width as u32,tile_attr.height as u32),
        dst_rect:Rect::new(  cam.get_x_m()-(coords.0-w/2)
                            ,cam.get_y_m()-(coords.1-h/2)
                            ,w as u32
                            ,h as u32    )
    }
}



pub fn get_direction(dir:u8)->(i32,i32){
    match dir{
        TOP         => (0,-1),
        TOP_RIGHT   => (1,-1),
        TOP_LEFT    => (-1,-1),
        BOTTOM_RIGHT=> (1,1),
        BOTTOM      => (0,1),
        BOTTOM_LEFT => (-1,1),
        LEFT        => (-1,0),
        RIGHT       => (1,0),
        _=>(0,0)
    }
}
