use super::super::core::physicshandle::{StaticObject,PhysicsObject};
use super::world::World;
use std::collections::HashSet;
#[derive(Debug)]
pub struct InCamObj<'a>{
    pub coords:(i32,i32),
    pub obj:&'a StaticObject,
}

pub struct Camera<'a> {
    cam_width:i32,
    cam_height:i32,
    pub center:(i32,i32),
    world:&'a World<'a>,
    pub cam_objs:Vec<InCamObj<'a>>

}

impl Camera<'_>{
    pub fn new<'a>(w:&'a World<'a>)->Camera<'a>{
        Camera{
            cam_width:w.block_width/2,
            cam_height:w.block_height/2,
            center:(0,0),
            world:w,
            cam_objs:Vec::new(),
        }
    }


    pub fn set_cam_params(&mut self, width:i32,height:i32){
        assert!(width<=(self.world.block_width/2));
        assert!(height<=(self.world.block_width/2));
        self.cam_height=height;
        self.cam_width=width;
        }

    pub fn clear_cam(&mut self){
        self.cam_objs.clear()
    }


    pub fn move_cam_rel(&mut self,x:i32,y:i32){
        if self.center.0+x<=self.cam_width{
            self.center.0=self.cam_width
        }else{
            self.center.0+=x
        }


        if self.center.1+y<=self.cam_height{
            self.center.1=self.cam_height
        }else{
            self.center.1+=y
        }
    }


    pub fn find_objects_test(&mut self){
            self.cam_objs.clear();
            let l_x_blk=(self.center.0-self.cam_width)/self.world.block_width;
            let r_x_blk=(self.center.0+self.cam_width)/self.world.block_width;
            let t_y_blk=(self.center.1-self.cam_height)/self.world.block_height;
            let b_y_blk=(self.center.1+self.cam_height)/self.world.block_height;

            let mut new_hash:HashSet<(i32,i32)>=HashSet::new();
            new_hash.insert((l_x_blk,t_y_blk));
            new_hash.insert((l_x_blk,b_y_blk));
            new_hash.insert((r_x_blk,t_y_blk));
            new_hash.insert((r_x_blk,b_y_blk));

            for m in new_hash.iter(){
                for n in self.world.world_map[m.1 as usize ][m.0 as usize ].static_objects.iter(){
                    self.cam_objs.push(InCamObj{
                        obj:n.st_object,
                        coords:(n.coords.0-self.center.0,n.coords.1-self.center.1)

                    })
                }
            }

        }


        pub fn obj_in_focus(&self){
            for m in self.cam_objs.iter(){
                println!("{:?}",m );
            }
        }

        pub fn display_params(&self){
            println!("x = {:?}",self.center.0);
            println!("y = {:?}",self.center.1);
        }

        pub fn set_to_origin(&mut self){
            self.center=(self.world.block_width/2,self.world.block_height/2);
        }



}
