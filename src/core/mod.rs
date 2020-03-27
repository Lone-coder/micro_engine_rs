pub mod gameobject;
pub mod components;
pub mod level;
pub mod tile;
pub mod window;
pub mod world;
pub mod input;
pub mod camera;
pub mod asset_loader;
pub mod behaviour;

// Trait for moving objects
pub trait Translate {
    fn translate (&mut self ,val:(i32,i32));
}
