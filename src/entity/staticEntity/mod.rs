use crate::physics::collision_rect::CollisionRect;

#[derive(Debug)]
pub struct StaticEntity{
    pub collision_rect:CollisionRect,
    pub state:usize,
    pub texture_id:usize

}

impl StaticEntity{
    // for testing
    pub fn new(x:usize,y:usize,width:usize,height:usize) -> StaticEntity{
        StaticEntity{
            collision_rect:CollisionRect::new(x as f32, y as f32, width  as f32, height as f32),
            state:0,
            texture_id:0
        }
    }

    pub fn get_x(&self) -> usize {
        self.collision_rect.x as usize
    }

    pub fn get_y(&self)->usize{
        self.collision_rect.y as usize
    }

    pub fn get_components(&self){//->CollisionRect{
        // need to resolve this
        //self.collision_rect
    }

    pub fn set_state(&mut self,state:usize){
        self.state=state
    }
}
