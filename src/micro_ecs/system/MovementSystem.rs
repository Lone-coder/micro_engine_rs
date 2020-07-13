use crate::core::components::transform::TransformComponent;
use crate::core::components::state::{StateComponent, State, PlayerState ,EnemyState};
use crate::math::Vector2;

pub fn move_entity(transform : &mut TransformComponent, state : &mut StateComponent, delta_time : f32) {

    match state.current_state {
        State::PlayerState(PlayerState::WalkingUp) | State::EnemyState(EnemyState::WalkingUp) => {
            transform.position = transform.position + Vector2::new(0.0, -1.0).scale(delta_time * 80.0);
        },
        State::PlayerState(PlayerState::WalkingDown) | State::EnemyState(EnemyState::WalkingDown) => {
            transform.position = transform.position + Vector2::new(0.0, 1.0).scale(delta_time * 80.0);
        },
        State::PlayerState(PlayerState::WalkingRight) | State::EnemyState(EnemyState::WalkingRight) => {
            transform.position = transform.position + Vector2::new(1.0, 0.0).scale(delta_time * 80.0);
        },
        State::PlayerState(PlayerState::WalkingLeft) | State::EnemyState(EnemyState::WalkingLeft) => {
            transform.position = transform.position + Vector2::new(-1.0, 0.0).scale(delta_time * 80.0);
        },
        _ => (),
    }
}
