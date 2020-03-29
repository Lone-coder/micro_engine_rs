//Trait Definitions
pub trait Physics {
    fn modify(&mut self);
    fn get(&self)->crate::core::components::physics::Physics;
}
