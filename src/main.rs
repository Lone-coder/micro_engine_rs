//main_player is the real player
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use micro_engine_rs::objects::{world, objects,camera,generator};
use micro_engine_rs::core::physicshandle::PhysicsObject;
use sdl2::rect::Rect;
use std::collections::{HashMap,HashSet};
use micro_engine_rs::controller;
extern crate sdl2;
fn main(){

    let _out=main_loop();
    //println!("testing" );
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

        let objectList=objects::ObjectList::test_new();
        let main_obj=PhysicsObject::new(100,100);
        let new_world=generator::generate_world(1600,1200,(6,6),&objectList);
        let mut cam=camera::Camera::new(&new_world);
        cam.move_cam_rel(900,300);
        cam.find_objects_test();
        cam.set_to_origin();
        cam.find_objects_test();
        println!("{:?}",cam.center);

        //new_world.display();



        let mut running=true;
        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },


                    Event::KeyDown{keycode:Some(Keycode::Right),..}=>{
                        cam.move_cam_rel(100,0)
                    },

                    Event::KeyDown{keycode:Some(Keycode::Left),..}=>{
                        cam.move_cam_rel(-100,0)
                    },

                    Event::KeyDown{keycode:Some(Keycode::Up),..}=>{
                        cam.move_cam_rel(0,-100)
                    },
                    Event::KeyDown{keycode:Some(Keycode::Down),..}=>{
                        cam.move_cam_rel(0,100)
                    },

                    _=>(),
                }
            }

            cam.find_objects_test();
            println!("{:?}",cam.center);
            cam.obj_in_focus();
            //new_world.display();

            std::thread::sleep(std::time::Duration::from_millis(300));

        }









        Ok(())

}
