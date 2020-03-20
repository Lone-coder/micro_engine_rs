//Micro Engine core modules
pub mod controller;
pub mod window;
pub mod world;
pub mod game_object;
pub mod camera;
pub mod level;
pub mod tile_object;

// SDL2 crate
extern crate sdl2;
pub use sdl2::video::Window;
pub use sdl2::event::Event;
pub use sdl2::pixels::Color;
pub use sdl2::keyboard::Keycode;
pub use sdl2::rect::Rect;

//some std imports
pub use std::collections::{HashMap,HashSet};
pub use std::time::{Duration, Instant};

//engine data struct (likely to change)
pub struct Engine{
    canvas : sdl2::render::Canvas<Window>,
    event_pump : sdl2::EventPump,
}

impl Engine{

    pub fn init_engine(screen_width : u32, screen_height : u32, window_title : &str) -> Engine
    {
        let sdl_context = sdl2::init().unwrap();

    	let video_subsystem = sdl_context.video().unwrap();

    	let window = video_subsystem.window(window_title, screen_width, screen_height)
    	.position_centered().build().map_err(|e| e.to_string()).unwrap();

        let mut _canvas = window.into_canvas()
    	.accelerated().build().map_err(|e| e.to_string()).unwrap();

        let _texture_creator = _canvas.texture_creator();

        _canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

        let timer = sdl_context.timer().unwrap();

        let _event_pump = sdl_context.event_pump().unwrap();

        Engine{
            canvas : _canvas,
            event_pump : _event_pump,
        }

    }

    pub fn run( &mut self, update : fn(delta_time : f32),
                handle_events : fn(event_pump : &mut sdl2::EventPump),
                render : fn(canvas : &mut sdl2::render::Canvas<Window>),cam : &mut camera::Camera)
    {
        let mut running = true;
    	let mut delta_time = 0.005;
    	let mut min_fps = 10000.0;
    	let mut max_fps = 0.0;

        //main game loop
    	while running {
    		let start = Instant::now();

            //event handling
    	    for event in self.event_pump.poll_iter() {
    	        match event {
    	            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
    	                    running = false;
    	                },
    	            _=>(),
    	        }
    	    }

            handle_events(&mut self.event_pump);
            update(delta_time);
            //rendering routine
    		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    		self.canvas.clear();
            cam.render_scene_objects(&mut self.canvas);
    		render(&mut self.canvas);
    		self.canvas.present();

            //uncomment below statement to cap FPS at 60
    	    std::thread::sleep(std::time::Duration::from_millis(16)); //waiting for 60fps 1 /60 = 0.016 secs

    		delta_time = start.elapsed().as_secs_f32();
    		let fps = 1.0 / delta_time;
    		//println!("{:?} fps , dt = {:?}", fps, delta_time);

    		//catching min and max FPS
    		if fps > max_fps {
    			max_fps = fps;
    		}

    		if fps < min_fps{
    			min_fps = fps;
    		}
        }

        println!("minFPS = {:?}, maxFPS = {:?}", min_fps, max_fps);

    }

}

pub fn print_bar(){
    println!("=========================================================================");
}
