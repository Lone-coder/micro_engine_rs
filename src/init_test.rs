use super::camera;
use super::world;

pub fn init_test<'b,'a>()->(world::World,camera::Camera<'a,'b>){
    let world=world::World::generate(6,6);
    world.block_width=800;
    world.block_height=600;
}
