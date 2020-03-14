
use std::fmt;


/// This is used to implement a basic physics object
/// For collisions and other physics stuff.
#[derive(Copy,Clone)]
pub struct PhysicsObject{
    pub x:i32,
    pub y:i32,
    pub prev_coords:(i32,i32),
    pub width:u32,
    pub height:u32,
    pub tile_coords:(u32,u32),
    pub prev_tile_coords:(u32,u32),
    pub tile_params:(u32,u32)
}

impl PhysicsObject{
    pub fn new(x_len:u32,y_len:u32)->PhysicsObject{
        PhysicsObject{
            x:0,
            y:0,
            width:x_len,
            height:y_len,
            tile_coords:(0,0),
            prev_tile_coords:(0,0),
            tile_params:(0,0),
            prev_coords:(0,0),
        }
    }

    pub fn update_tile(&mut self){
        self.tile_coords.0=self.x as u32/self.tile_params.0;
        self.tile_coords.1=self.y as u32/self.tile_params.1;
    }

    //set world tile width and height
    pub fn init_world_params(&mut self, width:u32,height:u32){
        self.tile_params.0=width;
        self.tile_params.1=height;
    }

    pub fn update(&mut self){
        self.prev_tile_coords=self.tile_coords;
        self.prev_coords.0=self.x;
        self.prev_coords.1=self.y;
        self.update_tile();
    }

    pub fn cllsn_immnt_chk(&mut self,_scene:&super::scene::Scene){
        self.tile_coords.0;
        self.tile_coords.1;
    }

    pub fn init_pos(&mut self,x:i32,y:i32){
        self.x=x;
        self.y=y;
        self.update_tile();
    }
}






#[derive(Clone,Copy)]
pub struct StaticObject{

    pub x:i32,
    pub y:i32,
    pub width:u32,
    pub height:u32,

}


impl StaticObject{
    pub fn new(width:u32,height:u32)->StaticObject{
        StaticObject{
            x:0,
            y:0,
            width:width,
            height:height,
        }
    }
}
impl fmt::Debug for StaticObject {

    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"Static")

    }

}
