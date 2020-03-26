use micro_engine_rs::HashSet;
use micro_engine_rs;
use micro_engine_rs::Keycode;
use micro_engine_rs::core::{world,camera};
use micro_engine_rs::core::{gameobject,components};

use sdl2;

fn main()
{
    //Initialization{:?}
    let mut _world = world::World::generate(6, 6, 800, 600);
    _world.load_objs_from_json();
    _world.obj_disp_per_block();

    let mut _cam = camera::Camera::create(800, 600, Some(&_world));

    _cam.x = 800 * 3;
    _cam.y = 600 * 3;

    println!("{:?}",() );

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

    //temporary creation of a small renderer
//    let obj1=GameObject::new();
//    obj1.render=Some(components::Sprite::new())

//    let walk_states=





    // instance of engine running

	while engine.is_running() {

        let instant = std::time::Instant::now();

        let dt = engine.delta_time();

        //_cam.x += (1000.0 * dt ) as i32;

        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        engine.canvas.clear();

        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 210, 0));

        _cam.render_scene_objects(&mut engine.canvas);

        engine.canvas.present();

        engine.update(instant);

        move_cam(& mut _cam,engine.input_handle(),dt);
    }
}



fn move_cam(cam:&mut camera::Camera,keys:HashSet<Keycode>,dt:f32){

        if keys.contains(&Keycode::Up){
            cam.y-=(100.0*dt) as i32
        }
        if keys.contains(&Keycode::Down){
            cam.y+=(100.0*dt) as i32
        }

        if keys.contains(&Keycode::Left){
            cam.x-=(100.0*dt) as i32
        }

        if keys.contains(&Keycode::Right){
            cam.x+=(100.0*dt) as i32
        }
}
