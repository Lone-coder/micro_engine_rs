use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::collections::HashSet;

use crate::core::components::state::{StateComponent, State, PlayerState};

pub fn PlayerInput(new_keys : &HashSet<Keycode>, old_keys : &HashSet<Keycode>, state_component : &mut StateComponent) {

    let keys_not_pressed = old_keys - new_keys;

    for key in new_keys.iter() {
        match key {
            Keycode::Up => {
                state_component.current_state = State::PlayerState(PlayerState::WalkingUp);
            },
            Keycode::Down => {
                state_component.current_state = State::PlayerState(PlayerState::WalkingDown);
            },
            Keycode::Right => {
                state_component.current_state = State::PlayerState(PlayerState::WalkingRight);
            },
            Keycode::Left => {
                state_component.current_state = State::PlayerState(PlayerState::WalkingLeft);
            },
            _=> (),
        }
    }

    for key in keys_not_pressed.iter() {
        match key {
            Keycode::Up | Keycode::Down |
            Keycode::Right | Keycode::Left => {
                state_component.current_state = State::PlayerState(PlayerState::Idle);
            },
            _=> (),
        }
    }
}
