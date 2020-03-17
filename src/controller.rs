
use std::collections::HashSet;
use sdl2::keyboard::Keycode;

pub fn resolve(input:&HashSet<Keycode>){

    if input.contains(&Keycode::Up){
        println!("Go up" );

}

    if input.contains(&Keycode::Down){
        println!("Go down");
}


if input.contains(&Keycode::Left){
    println!("Go Left");

}


if input.contains(&Keycode::Right){
    println!("Go Right");

}


}
