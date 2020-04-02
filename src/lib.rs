//Micro Engine core modules
pub mod core;
pub mod physics;
pub mod render;
pub mod math;
pub mod test_engine;

// Testing modules
pub mod entity;
pub mod game;
pub mod world;



// SDL2 crate
extern crate sdl2;
use crate::sdl2::image::LoadTexture;
pub use sdl2::video::Window;
pub use sdl2::video::WindowContext;
pub use sdl2::event::Event;
pub use sdl2::pixels::Color;
pub use sdl2::keyboard::Keycode;
pub use sdl2::rect::Rect;
use sdl2::render::TextureCreator;

// Serde for creating an object loader
extern crate serde_json;

//some std imports
pub use std::collections::{HashMap,HashSet};
pub use std::time::{Duration, Instant};
pub use std::path::Path;

//engine data struct (likely to change)
pub struct Engine<'a>{
    pub canvas : sdl2::render::Canvas<Window>,
    pub event_pump : sdl2::EventPump,
    pub textures : Vec<sdl2::render::Texture<'a>>,
    pub texture_creator : TextureCreator<WindowContext>,
    running : bool,
    delta_time : f32,

}

impl <'a> Engine <'a>{

    pub fn init_engine(screen_width : u32, screen_height : u32, window_title : &str) -> Engine
    {
        let sdl_context = sdl2::init().unwrap();

    	let video_subsystem = sdl_context.video().unwrap();

    	let window = video_subsystem.window(window_title, screen_width, screen_height)
    	.position_centered().build().map_err(|e| e.to_string()).unwrap();

        let mut _canvas = window.into_canvas()
    	.accelerated().build().map_err(|e| e.to_string()).unwrap();

        _canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

        let texture_creator = _canvas.texture_creator();

        let _timer = sdl_context.timer().unwrap();

        let _event_pump = sdl_context.event_pump().unwrap();

        Engine{
            canvas : _canvas,
            event_pump : _event_pump,
            running : true,
            delta_time : 0.005,
            textures : Vec::new(),
            texture_creator : texture_creator,
        }
    }


    pub fn is_running(& self) -> bool {
        self.running
    }

    pub fn delta_time(& self) ->f32 {
        self.delta_time
    }



    pub fn update(&mut self, instant : std::time::Instant) {

        //uncomment below statement to cap FPS at 60
	    std::thread::sleep(std::time::Duration::from_millis(16)); //waiting for 60fps 1 /60 = 0.016 secs

		self.delta_time = instant.elapsed().as_secs_f32();
    }





    // Quits the game or returns all keys pressed
    // as a HashSet
    pub fn input_handle(&mut self)->HashSet<Keycode>{

        for event in self.event_pump.poll_iter() {
	        match event {
	            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
	                    self.running = false;
	                },
	            _=>(),
	        }
	    }

        let key_presses:HashSet<Keycode>=self.event_pump
                        .keyboard_state()
                        .pressed_scancodes()
                        .filter_map(Keycode::from_scancode).collect();

        key_presses
    }

    pub fn load_texture(&mut self, file_name : &str){
        let path = Path::new(file_name);

        //self.textures.push(self.texture_creator.load_texture(path).unwrap());

    }


}
