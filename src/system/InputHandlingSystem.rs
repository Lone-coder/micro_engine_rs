use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::components::{ StateComponent, State, PlayerState};

pub fn PlayerInput(event_pump : &mut EventPump, state_component : &mut StateComponent) {

    match event_pump.poll_event() {
        Some(event) => {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    state_component.current_state = State::PlayerState(PlayerState::WalkingUp);
                },
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    state_component.current_state = State::PlayerState(PlayerState::WalkingDown);
                },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    state_component.current_state = State::PlayerState(PlayerState::WalkingRight);
                },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    state_component.current_state = State::PlayerState(PlayerState::WalkingLeft);
                },
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    state_component.current_state = State::PlayerState(PlayerState::Shoot);
                },
                Event::KeyUp {keycode: Some(Keycode::Left), ..} |
                Event::KeyUp {keycode: Some(Keycode::Right), ..} |
                Event::KeyUp {keycode: Some(Keycode::Up), ..} |
                Event::KeyUp {keycode: Some(Keycode::Down), ..} => {
                    state_component.current_state = State::PlayerState(PlayerState::Idle);
                },

                _=>(),
            }
        },
        None => {
            return;
        }
    }
}
