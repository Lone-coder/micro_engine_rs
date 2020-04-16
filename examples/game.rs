use micro_engine_rs::Engine;
use micro_engine_rs::entity::dynamicEntity;
use micro_engine_rs::world::new_world;
use new_world::World;
use micro_engine_rs::world::tile::{Route,Clump,GroupTileRoute};
use std::path::Path;
use std::collections::HashSet;
use sdl2::keyboard::Keycode;
use std::thread;
type KeyPresses=HashSet<sdl2::keyboard::Keycode>;


fn main(){
    let dt=0.016;
    let mut world=World::new(4096,4096,3,3);
    load_tiles(&mut world);
    let mut engine=Engine::init_engine(800,600,"test");
    engine.load_texture("assets/pathsheet.png");
    world.load_defined_group(unitcreator::group_creator_for_tests());
    world.load(unitcreator::create_group_route_for_tests(),(0,0));

    while engine.is_running(){
        let start_time=std::time::Instant::now();
        engine.canvas.clear();
        resolve(engine.input_handle(),&mut world.camera,dt);

        for m in world.render_tiles(){
            engine.render_from_buff(m)
        }
        println!("//Debug : examples/game.rs : 31 camera(x,y):{:?}",(world.camera.x,world.camera.y) );
        engine.canvas.present();
        let end_time=std::time::Instant::now();
        println!("frame time in ms {:?}",end_time-start_time);
        //thread::sleep_ms(100)
    }
}

fn load_tiles(world:&mut World){
    world.load_tile_types(String::from("pathtile"),0,32,32,208,48);
    world.load_tile_types(String::from("Grass"),0,16,16,96,32);
    world.load_tile_types(String::from("up_path_tile"),0,32,32,256,32);
    world.load_tile_types(String::from("down_path_tile"),0,32,32,256,80);

}

mod unitcreator{
    use micro_engine_rs::world;
    use world::tile;
    use tile::{GroupDefinition,DefinedGroup};
    use tile::GroupTileRoute;
    pub fn group_creator_for_tests()->DefinedGroup{
            DefinedGroup{
                name:String::from("Road"),
                groups:vec![GroupDefinition{tile_type:0,off_x:0,off_y:0},
                            GroupDefinition{tile_type:2,off_x:0,off_y:-32},
                            GroupDefinition{tile_type:3,off_x:0,off_y:32},
                            ],
                off:(32,64)
            }
    }

    pub fn create_group_route_for_tests()->GroupTileRoute{
        GroupTileRoute{
            start_coord:(32,32),
            defined_group_index:0,
            numbers:100,
            direction:micro_engine_rs::definitions::directions::RIGHT,
        }
    }



}

fn resolve(keys:KeyPresses,cam:&mut micro_engine_rs::world::camera::Camera,dt:f32){
    if keys.contains(&Keycode::Up){
        cam.y-=(200.0*dt) as i32
    }
    if keys.contains(&Keycode::Down){
        cam.y+=(200.0*dt) as i32
    }
    if keys.contains(&Keycode::Left){
        cam.x-=(200.0*dt) as i32
    }
    if keys.contains(&Keycode::Right){
        cam.x+=(200.0 *dt) as i32
    }
}
