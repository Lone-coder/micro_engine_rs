use micro_engine_rs;
use micro_engine_rs::{world,camera};


fn main()
{
    //Initialization
    let mut _world = world::World::generate(6, 6, 800, 600);
    _world.obj_display();
    let mut _cam = camera::Camera::create(800, 600, Some(&_world));
    _cam.x = (800 * 3);
    _cam.y = (600 * 3);

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

	fn handle_events(event_pump : &mut sdl2::EventPump)
	{

	}

	fn update(delta_time :f32)
	{

	}

	fn render(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>)
	{
	}

	engine.run(update, handle_events, render, &mut _cam);
}
