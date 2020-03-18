/// Useful info:
/// Here &'a world implies that the Camera lives atleast for as long as the world lives
/// or any reference to 'a musn't live longer than it


pub struct Camera<'a,'b>{
    pub x:i32,
    pub y:i32,
    pub width:i32,
    pub height:i32,
    scene_objects:Vec<& 'b super::game_object::GameObject>,
    pub world:Option<&'a super::world::World>,
}


impl<'a,'b> Camera<'a,'b>{
    pub fn create(width:i32,height:i32,_world:Option<&'a super::world::World>)->Camera<'a,'b>{
        Camera{
            x:width/2,
            y:height/2,
            width:width,
            height:height,
            scene_objects:Vec::new(),
            world:_world
        }
    }

    pub fn display_scene_objects(& mut self){
        self.scene_objects.iter().for_each(|x| println!("{:?}",x.props ));
    }


    pub fn attach_world(&mut self, world: &'a super::world::World){
            self.world=Some(world)
    }


    pub fn pan_x(& mut self,val:i32){
        self.x+=val
    }


    pub fn pan_y(&mut self,val:i32){
        self.y+=val
    }


    pub fn get_coord(&mut self,val:i32)->(i32,i32){
        (self.x,self.y)
    }


    pub fn add_obj(&mut self,obj:&'b super::game_object::GameObject){
        self.scene_objects.push(obj)
    }


    pub fn remove_obj(&mut self, obj: &'b super::game_object::GameObject){
        //todo
        self.scene_objects.remove(0);
    }

}
