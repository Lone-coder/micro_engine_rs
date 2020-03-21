use micro_engine_rs;
use micro_engine_rs::{world,camera};


fn main()
{
    //Initialization{:?}
    let mut _world = world::World::generate(6, 6, 800, 600);
    _world.load_objs_from_json();
    _world.obj_disp_per_block();

    let mut _cam = camera::Camera::create(800, 600, Some(&_world));

    _cam.x = (800 * 3);
    _cam.y = (600*3);

    println!("{:?}",() );

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

	while engine.is_running() {

        let instant = std::time::Instant::now();

        _cam.x += (1000.0 * engine.delta_time() ) as i32;

        engine.update(instant);
    }
}
