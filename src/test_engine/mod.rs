// Testing modules
// SDL2 crate
// This set of implementations is only for SDL2.
// For using other libraries , the implementation details will change

extern crate sdl2;
use sdl2::render::Texture;
pub use sdl2::video::Window;
pub use sdl2::video::WindowContext;
pub use sdl2::event::Event;
pub use sdl2::pixels::Color;
pub use sdl2::keyboard::Keycode;
pub use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;


//some std imports
pub use std::collections::{HashMap,HashSet};
pub use std::time::{Duration, Instant};

//engine data struct (likely to change)
pub struct Engine<'a>{
    pub canvas :sdl2::render::Canvas<Window>,
    pub texture_creator:Option<&'a TextureCreator<sdl2::video::WindowContext>>,
    pub event_pump : sdl2::EventPump,
    pub running : bool,
    delta_time : f32,
    pub texture_list:Vec<Texture<'a>>,
}

impl <'a>Engine<'a>{


    pub fn load_engine(canvas:sdl2::render::Canvas<sdl2::video::Window>,
                    texture_creator:&'a TextureCreator<sdl2::video::WindowContext>,pump:sdl2::EventPump)->Engine<'a>{

        Engine{
            canvas:canvas,
            texture_creator:Some(&texture_creator),
            event_pump:pump,
            running:false,
            delta_time:3.0,
            texture_list:Vec::new(),

        }

    }

    pub fn load_textures(&mut self, p:&str){
        self.texture_list.push(self.texture_creator.unwrap().load_texture(std::path::Path::new(p)).unwrap());
    }


    pub fn clear(&mut self){
        self.canvas.clear()
    }

    pub fn render(&mut self,m:usize,source:sdl2::rect::Rect,dest:sdl2::rect::Rect){
        self.canvas.copy(&self.texture_list[m],source,dest);
    }

    pub fn update_game(&mut self,index:usize){

    }


    pub fn is_running(& self) -> bool {
        self.running
    }

    pub fn delta_time(& self) ->f32 {
        self.delta_time
    }



    pub fn update(&mut self, instant : std::time::Instant) {

        // uncomment below statement to cap FPS at 60
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


}
