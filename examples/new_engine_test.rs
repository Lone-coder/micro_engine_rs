extern crate sdl2;

use std::env;
use std::path::Path;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use micro_engine_rs::test_engine;

pub fn run(png: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut engine=test_engine::Engine::load_engine(canvas,&texture_creator,sdl_context.event_pump().unwrap());


    engine.load_textures("assets/npc.png");
    engine.load_textures("assets/hero.png");
    engine.load_textures("assets/grass-tile-2.png");

    engine.texture_list.iter().for_each(|x|{
        
    });


    engine.canvas.present();




    'mainloop: loop {
        for event in engine.event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {

    let args: Vec<_> = env::args().collect();

    run(Path::new("assets/hero.png"))?;

    Ok(())
}
