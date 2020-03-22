use micro_engine_rs;
use micro_engine_rs::{world,camera};

use sdl2;

fn main()
{
    //Initialization{:?}
    let mut _world = world::World::generate(6, 6, 800, 600);
    _world.load_objs_from_json();

    let mut _cam = camera::Camera::create(800, 600, Some(&_world));

    _cam.x = (800 * 3);
    _cam.y = (600*3);

    println!("{:?}",() );

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

	while engine.is_running() {

        let instant = std::time::Instant::now();

        //_cam.x += (1000.0 * engine.delta_time() ) as i32;

        let dt = engine.delta_time();

        _cam.camera_event(&mut engine.event_pump,&dt);

        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        engine.canvas.clear();
        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 210, 0));
        //put rendering code here
        _cam.render_scene_objects(&mut engine.canvas);

        engine.canvas.present();

        engine.update(instant);
    }
}
