use std::collections::HashMap;
use super::super::core::physicshandle::StaticObject;

pub struct ObjectList{

    pub static_object_list:HashMap<String,StaticObject>,

}

impl ObjectList{
    pub fn test_new()->ObjectList{
        let mut object_list=ObjectList{
            static_object_list:HashMap::new()
        };
        for m in 0..10{
        let mut statobj=StaticObject::new(100,100);
        statobj.x=m*800;
        statobj.y=m*600;

        object_list.static_object_list.insert(("Stone".to_owned()+&m.to_string()).to_string(),statobj);

        }

        object_list

    }

    pub fn insert_static_objs(&mut self,K:String,V:StaticObject){
        self.static_object_list.insert(K,V);
    }
}
