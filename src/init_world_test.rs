use super::core::{physicshandle,physics_engine,scene};
use physicshandle::PhysicsObject;
use scene::Scene;
use physics_engine::Collision_Engine;

pub fn begin(){
    let mut main_obj=PhysicsObject::new(100,100);
    let main_scene=Scene::new(100,100);
    let mut Engine=Collision_Engine::new(& mut main_obj);

    let mut sub_obj_1=PhysicsObject::new(100,100);
    sub_obj_1.init_pos(100,100);

    let mut sub_obj_2=PhysicsObject::new(100,100);
    sub_obj_2.init_pos(200,200);

    let mut sub_obj_3=PhysicsObject::new(100,100);
    sub_obj_3.init_pos(300,300);

    Engine.coll_immin_objs.push_back(&sub_obj_1);
    Engine.coll_immin_objs.push_back(&sub_obj_2);
    Engine.coll_immin_objs.push_back(&sub_obj_3);



}
