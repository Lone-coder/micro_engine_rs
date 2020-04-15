// use crate::math::Vector2;
// use crate::world::World;
// use crate::entity::{staticEntity::StaticEntity,dynamicEntity::Entity};
//
// //sdl2 only delete later
// use sdl2::rect::Rect;
//
//
// pub struct RenderBuff{
//     pub texture_id:usize,
//     pub src_rect:Rect,
//     pub dst_rect:Rect,
// }
//
//
// pub struct Camera{
//     pub x:f32,
//     pub y:f32,
//     pub width:f32,
//     pub height:f32,
//     physics:Vector2,
// }
//
// impl Camera{
//     pub fn new()->Camera{
//         Camera {x:0.0,y:0.0,physics:Vector2::new(0.0,0.0),width:0.0,height:0.0}
//         }
//
//
//
//     pub fn get_render_objects(&mut self,world:&World)->Vec<RenderBuff>{
//         let mut render_val:Vec<RenderBuff>=Vec::new();
//         let ent=Entity::new("dummy".to_owned(),self.x,self.y,0.1,0.0,0.0);
//         world.get_adj_objs(&ent).iter().for_each(|obj|{
//
//             if (obj.collision_rect.x-self.x).abs()<(self.width*0.75)||(obj.collision_rect.x-self.y).abs()<(self.height*0.75){
//                 render_val.push(
//                     RenderBuff{
//                     texture_id:obj.texture_id,
//                     src_rect:obj.animation.get_frame_static(),
//                     dst_rect:Rect::new( (obj.collision_rect.x-self.x) as i32
//                                     ,(obj.collision_rect.y-self.y) as i32
//                                     ,obj.collision_rect.width as u32
//                                     ,obj.collision_rect.height as u32)
//                                 })
//             }
//         });
//         render_val
//
//     }
// }
