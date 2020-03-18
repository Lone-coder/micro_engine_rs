extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::collections::{HashMap,HashSet};
use std::time::{Duration, Instant}; //for calculationg delta time

fn main(){

	println!("testing");
    let _out = main_loop();
}

fn main_loop()->Result<(),String>{

	let mut world = micro_engine_rs::world::World::generate(6,6);


	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;
	let window = video_subsystem.window("SDL2", 800, 600)
	.position_centered().build().map_err(|e| e.to_string())?;
	let mut canvas = window.into_canvas()
	.accelerated().build().map_err(|e| e.to_string())?;
	let texture_creator = canvas.texture_creator();
	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
	let mut timer = sdl_context.timer()?;
	let mut event_pump = sdl_context.event_pump()?;

	let mut running = true;
	let mut delta_time = 0.005;
	let mut min_fps = 10000.0;
	let mut max_fps = 0.0;

	while running {
		let start = Instant::now();

	    for event in event_pump.poll_iter() {
	        match event {
	            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
	                    running = false;
	                },
	            _=>(),
	        }
	    }

		//rendering routine
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		canvas.set_draw_color(Color::RGB(255, 210, 0));
		world.render(&mut canvas);

		canvas.present();

		//world.test_display(
	    //std::thread::sleep(std::time::Duration::from_millis(16)); //waiting for 60fps 1 /60 = 0.016 secs

		delta_time = start.elapsed().as_secs_f32();
		let fps = 1.0 / delta_time;
		println!("{:?} fps , dt = {:?}", fps, delta_time);

		//catching min and max FPS
		if fps > max_fps {
			max_fps = fps;
		}

		if fps < min_fps{
			min_fps = fps;
		}

	}
	//logging fps details
	println!("min_fps = {:?}  , max_fps = {:?}", min_fps, max_fps);
	Ok(())

}
