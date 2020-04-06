use micro_engine_rs::entity::dynamicEntity::Entity;
use micro_engine_rs::game::prelude::*;

pub fn translate(ent:&mut Entity,_msg:SendValue)->SendValue{

    if let SendValue::Number{
        index: _,val
    }=_msg{
        ent.x+=val[0] as usize;
        ent.y+=val[0] as usize;
        return SendValue::Number{
            index:0,
            val:vec![ent.x as f32,ent.y as f32]
        }
    }
    // do a collision check
    // the physics engine should take care of this
    SendValue::Idle
}



pub fn bullet_move(_ent:&mut Entity,_msg:SendValue)->SendValue{

    _ent.x+=_ent.x+(_ent.physics.velocity.x as usize);
    _ent.x+=_ent.x+(_ent.physics.velocity.x as usize);
    // this is how it should go
    //          ||
    //          ||
    //          \/
    // if _ent.physics.collided_things_has_some(index)
    //{     return SendValue::Destroy{index:index}}
    //

    SendValue::Idle
}



/*
:Random things because of forgetting my own api :)


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
Idle
*/
