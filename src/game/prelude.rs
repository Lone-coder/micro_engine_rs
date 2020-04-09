//===========================================================
//                          INVALIDATED
//===========================================================
use crate::entity::dynamicEntity;

pub trait Messagable {
    fn send(&self);
}


pub enum SendValue{
    Number{
        index:usize,
        val:Vec<f32>
    },
    Request{
        val:usize
    },
    CreateEntity{
        entity:dynamicEntity::Entity,
        func:fn(&mut dynamicEntity::Entity,SendValue)->SendValue
    },
    ChangeParams{
        index:usize,
        param:usize
    },
    Destroy{
        index:usize
    },
    Objects{
                //x   y   w   h
        val:Vec<(f32,f32,f32,f32)>
    },
    Idle

}

impl SendValue{
    pub fn is_not_idle(&self)->bool{
        if let(Self::Idle)=self{
            false
        }
        else{
            true
        }
    }
}
