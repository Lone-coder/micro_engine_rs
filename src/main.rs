//default template for micro engine
use micro_engine_rs;
use micro_engine_rs::{game_object, camera};

fn main()
{
	//let player = game_object::GameObject::new(0.0, 0.0);
	let mut cam = camera::Camera::create(800, 600, None);

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Micro engine test");

	fn handle_events(event_pump : &mut sdl2::EventPump)
	{

	}

	fn update(delta_time :f32)
	{

	}

	fn render(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>)
	{
		canvas.set_draw_color(micro_engine_rs::Color::RGB(255, 210, 0));
		canvas.fill_rect(micro_engine_rs::Rect::new(0, 0, 20,20)).unwrap();
	}

	engine.run(update, handle_events, render, &mut cam);
}
