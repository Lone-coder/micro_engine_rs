use crate::math::Vector2;
use crate::core::components::state::{StateComponent, State, EnemyState};

pub fn follow(target : &Vector2, follower_position : &Vector2, follower_state : &mut StateComponent) {

    if target.x < follower_position.x {
        follower_state.current_state = State::EnemyState(EnemyState::WalkingLeft);
    }

    if target.x > follower_position.x {
        follower_state.current_state = State::EnemyState(EnemyState::WalkingRight);
    }

    if target.y < follower_position.y {
        follower_state.current_state = State::EnemyState(EnemyState::WalkingUp);
    }

    if target.y > follower_position.y {
        follower_state.current_state = State::EnemyState(EnemyState::WalkingDown);
    }
}
