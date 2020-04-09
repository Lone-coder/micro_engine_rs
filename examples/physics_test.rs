// test for physics system
use micro_engine_rs;
use micro_engine_rs::test_engine;
use micro_engine_rs::physics::physics_component::PhysicsComponent;
use micro_engine_rs::physics::PhysicsWorld;
use micro_engine_rs::math::Vector2;

use sdl2::image::InitFlag;
use sdl2::pixels::Color;

fn main() {
    //All SDL intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem.window("Physics engine test", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap();
    let canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();

    //Create an engine instance
    let mut engine = test_engine::Engine::load_engine(canvas,&texture_creator,sdl_context.event_pump().unwrap());
    engine.running = true;

    let mut delta_time = 0.016;

    //Initializing physics components
    let mut A = PhysicsComponent::new(Vector2::new(150.0, 300.0), 1.0, 40.0, 40.0);
    let mut B = PhysicsComponent::new(Vector2::new(500.0, 300.0), 0.0 , 40.0, 40.0);
    let mut floor = PhysicsComponent::new(Vector2::new(400.0, 500.0), 0.0, 800.0, 20.0);
    A.affected_by_gravity = false;
    B.affected_by_gravity = false;
    floor.affected_by_gravity = false;

    A.restitution_coeff = 0.0;
    B.restitution_coeff = 0.0;
    floor.restitution_coeff = 0.0;

    A.velocity = Vector2::new(50.0, 0.0);
    let mut phy_world = PhysicsWorld::create_physics_world(Vector2::new(0.0, 40.0));

    phy_world.add_phys_component(A);
    phy_world.add_phys_component(B);
    phy_world.add_phys_component(floor);
    //phy_world.add_phys_component(PhysicsComponent::new(Vector2::new(500.0, 20.0), 1.0 , 40.0, 40.0));

    // let mut seed1 = 789;
    // let mut seed2 = 456;
    //
    // for i in 0..40 {
    //
    //     seed1 = (1234 * seed1 + 4321) % 5678;
    //     seed2 = (4545 * seed2 + 4534) % 1234;
    //     let x = seed1 % 800;
    //     let y = seed2 % 400;
    //     phy_world.add_phys_component(PhysicsComponent::new(Vector2::new(x as f32, y as f32), 1.0 , 40.0, 40.0));
    // }

    while engine.is_running() {
        let instant = std::time::Instant::now();

        //INPUT
        let key = engine.input_handle();

        //UPDATE
        phy_world.detect_and_resolve_collisions();
        phy_world.update_physics_world(delta_time);

        //RENDERING
        engine.canvas.set_draw_color(Color::RGB(0, 0, 0));
        engine.canvas.clear();
        //rendering code here
        engine.canvas.set_draw_color(Color::RGB(255, 0, 0));
        phy_world.render_world_for_debug(&mut engine.canvas);

        engine.canvas.present();

        delta_time = instant.elapsed().as_secs_f32();

    }

}
