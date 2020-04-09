use micro_engine_rs::Engine;
use micro_engine_rs::entity::dynamicEntity;
use micro_engine_rs::world::World;
use std::path::Path;



fn main(){

    let mut engine=Engine::init_engine(800,600,"test");
    engine.load_texture("assets/hero.png");
    engine.load_texture("assets/transparent-bg-tiles.png");
    

}
