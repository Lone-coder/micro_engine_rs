//default template for micro engine
use micro_engine_rs;
use micro_engine_rs::core::{gameobject, camera};

fn main()
{
	//let player = game_object::GameObject::new(0.0, 0.0);
	let mut cam = camera::Camera::create(800, 600, None);

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

	while engine.is_running() {
		//used to calculate delta time
        let instant = std::time::Instant::now();

		let keys = engine.input_handle();
		//put game update code here

		engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        engine.canvas.clear();
        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 210, 0));
        //put rendering code here

        engine.canvas.present();

        engine.update(instant);
    }



	let oreo="frar".to_string();

	match oreo.as_str(){
		"walking_up" =>{
			//val.physics.y-=20;
		}

		"walking_right" =>{
			//val.physics.x+=20;
		}

		"walking_left"=>{
			//val.physics.x-=20
		}

		"walking_down"=>{
		//val.physics.y+=20
	}
	_=>()

	}
}
