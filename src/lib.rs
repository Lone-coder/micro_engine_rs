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

// Serde for creating an object loader
extern crate serde_json;

//some std imports
pub use std::collections::{HashMap,HashSet};
pub use std::time::{Duration, Instant};

//engine data struct (likely to change)
pub struct Engine{
    pub canvas : sdl2::render::Canvas<Window>,
    pub event_pump : sdl2::EventPump,
    running : bool,
    delta_time : f32,
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
            running : true,
            delta_time : 0.005
        }

    }

    pub fn is_running(& self) -> bool {
        self.running
    }

    pub fn delta_time(& self) ->f32 {
        self.delta_time
    }

    pub fn update(&mut self, instant : std::time::Instant) {

        //event handling
	    for event in self.event_pump.poll_iter() {
	        match event {
	            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
	                    self.running = false;
	                },
	            _=>(),
	        }
	    }
        //uncomment below statement to cap FPS at 60
	    std::thread::sleep(std::time::Duration::from_millis(16)); //waiting for 60fps 1 /60 = 0.016 secs

		self.delta_time = instant.elapsed().as_secs_f32();
    }

}

pub fn print_bar(){
    println!("=========================================================================");
}
