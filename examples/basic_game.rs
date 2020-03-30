pub mod character_functions;
extern crate sdl2;

use std::env;
use std::path::Path;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use micro_engine_rs::test_engine;

pub fn run(png: &Path) -> Result<(), String> {
    //                                  init
    //=============================================================================================
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    //-----------------------------------------------------------------------------------
    //========================================================================================


    //Create the engine
    let mut engine=test_engine::Engine::load_engine(canvas,&texture_creator,sdl_context.event_pump().unwrap());
    //------------------------------------------------------------------------------------------------
    //===============================================================================================


    // load the textures
    engine.load_textures("assets/hero.png");
    let mut game=micro_engine_rs::game::Game::new();

    game.entities=vec![
            micro_engine_rs::test_object::Entity::new("Soldier".to_string()),
            micro_engine_rs::test_object::Entity::new("Enemy".to_string()),
            micro_engine_rs::test_object::Entity::new("Enemy".to_string()),
            micro_engine_rs::test_object::Entity::new("Enemy".to_string()),
            micro_engine_rs::test_object::Entity::new("JavidsDumbBrain".to_string()),
    ];

    for m in &mut game.entities{
        m.load_interactive(character_functions::follow);
        m.load(character_functions::circle)
    }

    let mut ent=micro_engine_rs::test_object::Entity::new("Soldier".to_string());
    ent.load_many(vec![character_functions::init,
                       character_functions::walk_left,
                       character_functions::walk_up,
                       character_functions::walk_down,
                       character_functions::panic,
                       character_functions::circle
                   ]);
    ent.exec_one(0);
    //game_loop
    for mut m in game.entities{
        m.inter_exec(&mut ent);
    }









    //load the level

    //adjust the game states








    //create the event loop and the logic

    println!("physics is x:  {:?}  y: {:?}",ent.physics.x,ent.physics.y);
    ent.exec_one(0);
    println!("animation is{:?}",ent.animation.sprite_coords);
    println!("physics is x:  {:?}  y: {:?}",ent.physics.x,ent.physics.y);
    println!("animation is{:?}",ent.animation.state);





    engine.running=false;


    while engine.is_running(){
        engine.input_handle();
        engine.canvas.clear();
        engine.canvas.copy(&engine.texture_list[0],ent.animation.get_frame_inc(),Rect::new(ent.physics.x,ent.physics.y, 128,128));
        engine.canvas.present();
        ent.exec_one(5);
        std::thread::sleep_ms(100);


    }






    Ok(())
}



fn main() -> Result<(), String> {

    let args: Vec<_> = env::args().collect();

    run(Path::new("assets/hero.png"))?;

    Ok(())
}
