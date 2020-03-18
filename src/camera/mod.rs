pub struct Camera<'a>{
    pub width:i32,
    pub height:i32,
    scene_objects:Vec<& 'a super::world::TestObject>
}

impl<'a> Camera<'a>{
    pub fn create(width:i32,height:i32)->Camera<'a>{
        Camera{
            width:width,
            height:height,
            scene_objects:Vec::new()
        }
    }

    pub fn display_scene_objects(& mut self){
        self.scene_objects.iter().for_each(|x| println!("{:?}",x.props ));
    }
}
