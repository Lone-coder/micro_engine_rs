extern crate sdl2;
//main_player is the real player
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::collections::{HashMap,HashSet};




fn main(){

    let _out=main_loop();
}


fn main_loop()->Result<(),String>{

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


        //new_world.display();



        let mut running=true;
        while running {
            for event in event_pump.poll_iter() {
                match event {

                    _=>(),
                }
            }


            std::thread::sleep(std::time::Duration::from_millis(300));

        }









        Ok(())

}
