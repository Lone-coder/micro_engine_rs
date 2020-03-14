use super::core::physicshandle::PhysicsObject;
use std::collections::HashSet;
use sdl2::keyboard::Keycode;

pub fn resolve(obj:&mut PhysicsObject,input:&HashSet<Keycode>){

    if input.contains(&Keycode::Up){
        println!("Go up" );
        if input.contains(&Keycode::LShift){
            println!("Go up fast");
            obj.y-=10
        }
        else{
        obj.y-=4;
    }
}

    if input.contains(&Keycode::Down){
        println!("Go down");
        if input.contains(&Keycode::LShift){
            println!("Go down fast");
            obj.y+=10;
        }
        else{
        obj.y+=4;
    }
}


if input.contains(&Keycode::Left){
    println!("Go Left");
    if input.contains(&Keycode::LShift){
        println!("Go Left");
        obj.x-=10;
    }
    else{
    obj.x-=4;
    }
}


if input.contains(&Keycode::Right){
    println!("Go Right");
    if input.contains(&Keycode::LShift){
        println!("Go Right fast ");
        obj.x+=10;
    }
    else{
    obj.x+=4;
    }
}


}
