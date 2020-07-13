//TODO(Lone-coder): Implement basic mouse interactions and events

use sdl2;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::render::{TextureCreator, Texture};
use sdl2::render::Canvas;


fn main() {
    
    let width = 1280;
    let height = 720;

     //All sdl intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem.window("MicroUI(immediate mode)", width, height)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap();
    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();
    let mut texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                Event::MouseButtonDown{mouse_btn : sdl2::mouse::MouseButton::Left, ..} => {
                },
                Event::MouseButtonUp{mouse_btn : sdl2::mouse::MouseButton::Left, ..} => {
                },
                _=>(),
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0 , 0, 0));
        canvas.clear();

        canvas.present();
    }
}