use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

use std::collections::HashMap;
use super::core::{physicshandle,physics_engine,scene};
use physicshandle::PhysicsObject;
use scene::Scene;
use physics_engine::Collision_Engine;
use super::controller;
use std::collections::HashSet;
pub fn begin()->Result<(),String>{

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
    let mut time=0;



    let mut s1=Scene::new(100,100);
    let mut main_obj=PhysicsObject::new(100,100);
    main_obj.init_world_params(s1.tile_width,s1.tile_height);
    main_obj.init_pos(100,100);

    let mut a1=PhysicsObject::new(50,50);
    a1.init_world_params(s1.tile_width,s1.tile_height);
    a1.init_pos(200,200);

    let mut a2=PhysicsObject::new(50,50);
    a2.init_world_params(s1.tile_width,s1.tile_height);
    a2.init_pos(300,300);

    let mut a3=PhysicsObject::new(50,50);
    a3.init_world_params(s1.tile_width,s1.tile_height);
    a3.init_pos(500,500);
    let mut Engine=Collision_Engine::new(&mut main_obj);


    Engine.coll_immin_objs.push_back(&a1);
    Engine.coll_immin_objs.push_back(&a2);
    Engine.coll_immin_objs.push_back(&a3);


    let mut main_obj_rect=Rect::new(Engine.main_obj.x,Engine.main_obj.y,Engine.main_obj.width, Engine.main_obj.height);
    let a1_rect=Rect::new(a1.x-a1.width as i32/2,a1.y-a1.height as i32/2,a1.width, a1.height);
    let a2_rect=Rect::new(a2.x-a2.width as i32/2,a2.y-a2.height as i32/2,a2.width, a2.height);
    let a3_rect=Rect::new(a3.x-a3.width as i32/2,a3.y-a3.height as i32/2,a3.width, a3.height);
    s1.update_object(&a1,5);
    s1.update_object(&a2,7);
    s1.update_object(&a3,9);







    let mut running=true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _=>(),
            }
        }

    Engine.main_obj.update();
    Engine.main_obj.update_tile();
    s1.update_object(&a1,5);
    s1.update_object(&a2,7);
    s1.update_object(&a3,9);
    s1.update_object(Engine.main_obj,3);
    let keys:HashSet<sdl2::keyboard::Keycode>= event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
    controller::resolve(&mut Engine.main_obj,&keys);
    if Engine.check(){
        Engine.main_obj.x=Engine.main_obj.prev_coords.0;
        Engine.main_obj.y=Engine.main_obj.prev_coords.1;
    }

    main_obj_rect.set_x(Engine.main_obj.x-(Engine.main_obj.width as i32)/2);
    main_obj_rect.set_y(Engine.main_obj.y-(Engine.main_obj.height as i32)/2);


    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
    canvas.clear();
    canvas.set_draw_color(sdl2::pixels::Color::RGBA(255,255,255,255));

    canvas.fill_rect(main_obj_rect)?;



    canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 0, 0,255));
    canvas.fill_rect(a1_rect)?;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 255, 0,255));
    canvas.fill_rect(a2_rect)?;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 255,255));
    canvas.fill_rect(a3_rect)?;


    canvas.present();

    


    if timer.ticks()-time>2000{
        time=timer.ticks();
        s1.display();
    }

    std::thread::sleep(Duration::from_millis(50));
}
    Ok(())
}
