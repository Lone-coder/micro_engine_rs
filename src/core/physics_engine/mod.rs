use super::scene::Scene;
use super::physicshandle::{PhysicsObject,StaticObject};
use std::collections::VecDeque;


pub struct Collision_Engine <'a>{
    pub main_obj:&'a mut  PhysicsObject,
    pub coll_immin_objs:VecDeque<&'a PhysicsObject>,
    pub coll_immin_objs_static:VecDeque<StaticObject>,
    pub scene_props:(u32,u32)
}


impl <'a>Collision_Engine<'a>{
    pub fn new(main_obj:&'a mut  PhysicsObject)->Collision_Engine<'a>{
        Collision_Engine{
            main_obj:main_obj,
            coll_immin_objs:VecDeque::new(),
            coll_immin_objs_static:VecDeque::new(),
            scene_props:(0,0)
        }
    }

    pub fn check(&self)->bool{
        let mut flag=false;
        for m in self.coll_immin_objs.iter(){
            if Collison_imminent(self.main_obj,m){
                flag=true;
            }
        }



        flag
    }




}

pub fn Collison_imminent(obj1:&PhysicsObject,obj2:&PhysicsObject)->bool{
    let mut cond_chk=false;
    if ((obj1.x-obj2.x).abs() as u32)<((obj1.width+obj2.width)/2)&&((obj1.y-obj2.y).abs() as u32)<((obj1.height+obj2.height)/2){
        cond_chk=true
    }
    cond_chk
}

pub fn Please_Frikkin_dont(obj1:&mut PhysicsObject){
    obj1.x=obj1.prev_coords.0;
    obj1.y=obj1.prev_coords.1;

}
