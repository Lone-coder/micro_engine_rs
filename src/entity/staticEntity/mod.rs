use crate::physics::collision_rect::CollisionRect;
use crate::core::components::sprite::Sprite;

pub struct StaticEntity{
    pub name:String,
    pub collision_rect:CollisionRect,
    pub state:String,
    pub texture_id:usize,
    pub animation:Sprite
}

impl StaticEntity{
    // for testing
    pub fn new(name:String,x:f32,y:f32,width:f32,height:f32) -> StaticEntity{
        StaticEntity{
            name:name,
            collision_rect:CollisionRect::new(x,y,width, height),
            state:"Dead".to_owned(),
            texture_id:0,
            animation:Sprite::new()
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

    pub fn set_state(&mut self,state:String){
        self.state=state;
        self.animation.change_state(self.state.clone());
    }
}


impl std::fmt::Debug for StaticEntity{
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f,"name:{} x: {} y: {}",self.name, self.get_x(),self.get_y())
    }
}
