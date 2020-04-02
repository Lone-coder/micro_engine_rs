// test for physics system
use micro_engine_rs;
use micro_engine_rs::test_engine;
use micro_engine_rs::physics::{collision_rect::CollisionRect,PhysicsComponent};
use micro_engine_rs::physics::resolve_collisions;
use micro_engine_rs::math::Vector2;

use sdl2::render::Canvas;
use sdl2::video::Window;

use std::env;
use std::path::Path;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {

    //All sdl intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem.window("Physics engine test", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();

    //Create an engine instance
    let mut engine = test_engine::Engine::load_engine(canvas,&texture_creator,sdl_context.event_pump().unwrap());

    engine.running = true;

    let mut delta_time = 0.016;


    //Initializing physics components
    let mut A = PhysicsComponent::new(Vector2::new(40.0, 300.0), 5.0, 40.0, 40.0);
    let mut B = PhysicsComponent::new(Vector2::new(500.0, 300.0), 0.0 , 40.0, 40.0);

    A.restitution_coeff = 0.0005;
    B.restitution_coeff = 0.0025;

    A.velocity = Vector2::new(100.0, 0.0);

    while engine.is_running() {
        let instant = std::time::Instant::now();

        //INPUT
        let key = engine.input_handle();

        //UPDATE
        if A.check_collision(&mut B) {
            let n =  Vector2::new(-1.0, 0.0);
            let j = resolve_collisions(&mut A, &mut B, &n);
            let impulse = n.scale(j);
            let mass_sum = A.mass + B.mass;
            let ratio_a = A.mass / mass_sum;
            let ratio_b = B.mass / mass_sum;

            A.velocity = A.velocity + impulse.scale(ratio_a);
            B.velocity = B.velocity - impulse.scale(ratio_b);
        }

        A.update(delta_time);
        B.update(delta_time);

        //RENDERING
        engine.canvas.set_draw_color(Color::RGB(0, 0, 0));
        engine.canvas.clear();
        //rendering code here
        engine.canvas.set_draw_color(Color::RGB(255, 0, 0));
        A.draw_phy_object(&mut engine.canvas);

        engine.canvas.set_draw_color(Color::RGB(0, 255, 255));
        B.draw_phy_object(&mut engine.canvas);

        engine.canvas.present();

        delta_time = instant.elapsed().as_secs_f32();

    }

}
