use micro_engine_rs::asset_loader;
use micro_engine_rs::entity::staticEntity::StaticEntity;

fn main() {
    //let static_entity : StaticEntity = asset_loader::load_static_entity("assets/static_entity.json");
    println!("{:?}", asset_loader::load_world_static_entities("assets/examples/static_entities.json"));
}
