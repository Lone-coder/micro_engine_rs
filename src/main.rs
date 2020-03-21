//default template for micro engine
use micro_engine_rs;
use micro_engine_rs::{game_object, camera};

fn main()
{
	//let player = game_object::GameObject::new(0.0, 0.0);
	let mut cam = camera::Camera::create(800, 600, None);

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

	while engine.is_running() {
		//used to calculate delta time
        let instant = std::time::Instant::now();

		//put game update code here

        engine.update(instant);
    }
}
