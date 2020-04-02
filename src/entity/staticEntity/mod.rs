use crate::core::components::physics::Physics;

pub struct StaticEntity{
    components:Physics,
    coordinates:(usize,usize)
}
impl StaticEntity{

    // for testing
    pub fn new(x:usize,y:usize)->StaticEntity{
        StaticEntity{
            components:Physics{
                x:x as i32,y:y as i32
             },
            coordinates:(100,100)
        }
    }

    pub fn get_x(&self)->usize {
        self.coordinates.0
    }
    pub fn get_y(&self)->usize{
        self.coordinates.1
    }
}
