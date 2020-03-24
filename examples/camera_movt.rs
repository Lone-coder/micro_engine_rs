use micro_engine_rs::HashSet;
use micro_engine_rs::Keycode;
use micro_engine_rs::core::{world,camera};
use micro_engine_rs::Engine;

use sdl2;

fn main()
{
    //Initialization{:?}
    let mut _world = world::World::create_new(6, 6, 20 * 32, 15 * 32);  //block should be multiple of 32

    let mut _cam = camera::Camera::create();

	let mut engine = Engine::init_engine(800, 600, "Camera movement test");

    //game loop
	while engine.is_running() {

        let instant = std::time::Instant::now();

        let dt = engine.delta_time();

        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        engine.canvas.clear();

        _world.render(&mut engine.canvas, &mut _cam);
        engine.canvas.present();

        engine.update(instant);

        move_cam(& mut _cam,engine.input_handle(), dt);
    }
}


fn move_cam(cam:&mut camera::Camera,keys:HashSet<Keycode>,dt:f32) {

        if keys.contains(&Keycode::Up) {
            cam.position.y = cam.position.y - (100.0 * dt);
        }
        if keys.contains(&Keycode::Down) {
            cam.position.y = cam.position.y + (100.0 * dt);
        }

        if keys.contains(&Keycode::Left) {
            cam.position.x = cam.position.x - (100.0 * dt);
        }

        if keys.contains(&Keycode::Right) {
            cam.position.x = cam.position.x + (100.0 * dt);
        }
}
