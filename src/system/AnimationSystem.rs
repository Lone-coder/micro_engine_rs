use crate::core::components::{SpriteComponent, StateComponent, PlayerState, State};

pub fn UpdateAnimationState(state : &mut StateComponent, sprite : &mut SpriteComponent) {

    match state.current_state {
        State::PlayerState(PlayerState::WalkingUp) => {
            sprite.current_animation = "walk_up".to_string();
        },
        State::PlayerState(PlayerState::WalkingDown) => {
            sprite.current_animation = "walk_down".to_string();
        },
        State::PlayerState(PlayerState::WalkingRight) => {
            sprite.current_animation = "walk_right".to_string();
        },
        State::PlayerState(PlayerState::WalkingLeft) => {
            sprite.current_animation = "walk_left".to_string();
        },
        State::PlayerState(PlayerState::Idle) => {
            sprite.current_animation = "idle".to_string();
        },
        _ => (),
    }

}

pub fn UpdateAnimation(sprite : &mut SpriteComponent, time_elasped : f32) {

    let animation = &sprite.animations.get(&sprite.current_animation).unwrap();
    let num_of_frames = animation.frame_coords.len() as i32;
    let frame_delay = animation.frame_delay;

    //calculating frame index
    let index = ((time_elasped / frame_delay) as i32 % num_of_frames) as usize;

    //println!("frame : {:?}", index);

    //updating dest rect by taking coord from cuurent frame
    sprite.src_rect.x = animation.frame_coords[index].0;
    sprite.src_rect.y = animation.frame_coords[index].1;

    sprite.flip_horizontal = animation.flip_horizontal;
}
