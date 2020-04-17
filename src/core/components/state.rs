#[derive(Debug)]
pub enum PlayerState {
    WalkingLeft,
    WalkingRight,
    WalkingUp,
    WalkingDown,
    Idle,
    Shoot,
}

#[derive(Debug)]
pub enum EnemyState {
    WalkingLeft,
    WalkingRight,
    WalkingUp,
    WalkingDown,
    Idle,
    Attack,
}

#[derive(Debug)]
pub enum State {
    PlayerState(PlayerState),
    EnemyState(EnemyState),
}

#[derive(Debug)]
pub struct StateComponent {
    pub current_state : State,
}
