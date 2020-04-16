use crate::entity::staticEntity;
use crate::RenderBuff;
use sdl2::rect::Rect;

pub struct Tile{
    tile_type:usize,
    x:i32,
    y:i32
}

pub struct TileParams{
    sheet_id:usize,
    width:u32,
    height:u32,
    sprite_x:i32,
    sprite_y:i32
}






pub struct Tiletypes{
    pub tile_type:Vec<TileParams>
}
impl Tiletypes{
    pub fn load(&mut self,sheet_id:usize,width:u32,height:u32,sprite_x:i32,sprite_y:i32){
        self.tile_type.push(
            TileParams{
                sheet_id:sheet_id,
                width:width,
                height:height,
                sprite_x:sprite_x,
                sprite_y:sprite_y,
                })
            }
    }



pub struct Blocks{
    pub static_entities:Vec<staticEntity::StaticEntity>,
    pub tiles:Vec<Vec<Tile>>
}

impl Blocks{

    pub fn new()->Blocks{
        Blocks{
            static_entities:Vec::new(),
            tiles:Vec::new()
        }
    }

    //  bottleneck
    pub fn render_tiles(&self,tile_types:&Tiletypes,cam_x:i32,cam_y:i32)->Vec<RenderBuff>{
        let mut buff:Vec<RenderBuff>=Vec::new();
        self.tiles.iter().for_each(|row|{
                row.iter().for_each(|tile|{
                    buff.push(Self::get_tile_rects(tile_types,&tile, cam_x, cam_y));
                })
        });
        buff
    }


    pub fn get_tile_rects(tile_types:&Tiletypes,tile:&Tile,cam_x:i32,cam_y:i32)->RenderBuff{
        let tileattr=&tile_types.tile_type[tile.tile_type];
        RenderBuff{
            texture_id:tileattr.sheet_id,
            src_rect:Rect::new(tileattr.sprite_x,tileattr.sprite_y,tileattr.width,tileattr.height),
            dst_rect:Rect::new(cam_x-tile.x,cam_y-tile.y,tileattr.width,tileattr.height)
        }
    }

    pub fn load_generic_tiles_testing(&mut self,id:usize,blocks_x:usize,blocks_y:usize,
                                        start_x:i32,start_y:i32,tile_width:u32,tile_height:u32){
        let mut current_x=start_x;
        let mut current_y=start_y;
        (0..blocks_y).for_each(|row|{
            current_x=start_x;
            self.tiles.push(Vec::new());
            (0..blocks_x).for_each(|tile|{
                self.tiles[row].push(
                    Tile{
                    tile_type:id,
                    x:current_x,
                    y:current_y
                });
                current_x+=tile_width as i32;
            });
            current_y+=tile_height as i32;
        })
    }
}
