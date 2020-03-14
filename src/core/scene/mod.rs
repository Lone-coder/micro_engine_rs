use super::physicshandle::PhysicsObject;

pub struct Scene{
    pub tile_width:u32,
    pub tile_height:u32,
    pub scene_array:[[u32;10];10],
}

impl Scene{
    pub fn new(tile_width:u32,tile_height:u32)->Scene{
        Scene{
            tile_width:tile_width,
            tile_height:tile_height,
            scene_array:[[0;10];10]
        }
    }

    pub fn display(&mut self){
        for m in self.scene_array.iter(){
            println!("{:?}",m );
        }
    }

    pub fn update_object(&mut self,obj:&PhysicsObject,val:u32){
        if obj.prev_tile_coords!=obj.tile_coords{
            self.scene_array[obj.prev_tile_coords.1 as usize][obj.prev_tile_coords.0 as usize]=0;
        }
        self.scene_array[obj.tile_coords.1 as usize][obj.tile_coords.0 as usize]=val;
    }
}
