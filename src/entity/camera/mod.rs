use crate::math::Vector2;
use crate::world::World;
use crate::entity::{staticEntity::StaticEntity,dynamicEntity::Entity};

//sdl2 only delete later
use sdl2::rect::Rect;


pub struct RenderBuff{
    pub texture_id:usize,
    pub src_rect:Rect,
    pub dst_rect:Rect,
}



pub struct Camera{
    pub x:f32,
    pub y:f32,
    pub width:f32,
    pub height:f32,
    physics:Vector2,
    renderbuff:Vec<RenderBuff>
    }

impl Camera{
    pub fn new()->Camera{
        Camera{x:0.0,y:0.0,physics:Vector2::new(0.0,0.0),renderbuff:Vec::new(),width:0.0,height:0.0}
        }

    pub fn get_render_objects(&mut self,world:&World){
        let ent=Entity::new("dummy".to_owned(),self.x,self.y,0.1,0.0,0.0);
        
    }



}
