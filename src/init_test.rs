use super::camera;
use super::world;

pub fn init_test<'b,'a>()->(world::World,camera::Camera<'a,'b>){

    let mut world=world::World::generate(6,6);
    world.block_width=800;
    world.block_height=600;

    let cam=camera::Camera::create(400,300,None);
    (world,cam)

    //after getting this , attach world to cam
    //using camera.attach_world
}
