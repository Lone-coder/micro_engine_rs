//default template for micro engine
use micro_engine_rs;
use micro_engine_rs::core::{gameobject, camera};

fn main()
{
	//let player = game_object::GameObject::new(0.0, 0.0);
	let mut cam = camera::Camera::create(800, 600);

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Micro Engine test");

	while engine.is_running() {
		//used to calculate delta time
        let instant = std::time::Instant::now();

		//put game update code here

		engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        engine.canvas.clear();
        //put rendering code here

        engine.canvas.present();

        engine.update(instant);
    }
}
