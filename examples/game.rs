// use micro_engine_rs::Engine;
// use micro_engine_rs::entity::dynamicEntity;
// use micro_engine_rs::world::World;
// use std::path::Path;
// use micro_engine_rs::entity::camera::Camera;
//
//
// fn main(){
//
//     let mut engine=Engine::init_engine(800,600,"test");
//     engine.load_texture("assets/hero.png");
//     engine.load_texture("assets/transparent-bg-tiles.png");
//     let mut hero=dynamicEntity::Entity::new("hero".to_owned(),200.0,200.0,5.0,16.0,16.0);
//     let mut npc=dynamicEntity::Entity::new("boy_npc".to_owned(),300.0,300.0,5.0,16.0,16.0);
//     let mut cam=Camera::new();
//     let mut world=World::new(6,6);
//     world.load_static_entities("assets/examples/static_entities.json");
//     println!("{:?}",world );
// }
