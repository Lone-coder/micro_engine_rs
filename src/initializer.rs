use super::core::{physicshandle::PhysicsObject,scene::Scene};
pub fn init(obj:&mut PhysicsObject,scene:&Scene){
    obj.init_world_params(scene.tile_width, scene.tile_height);
}
