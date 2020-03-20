use micro_engine_rs;
use micro_engine_rs::{world,camera};


fn camera_movt_test(){
        let mut _world = world::World::generate(6,6,1600,1200);
        let mut _cam = camera::Camera::create(800,600,Some(&_world));

    for _ in 0..6{
        println!("The coordinates are : ({:?},{:?})",_cam.x,_cam.y );
        _cam.pan_x(400);
        _cam.pan_y(400);
        println!("The objects are :");
        println!("{:?}",_cam.get_objs_in_scene().iter().map(|x| {(x.1,x.2)}).collect::<Vec<(i32,i32)>>());
        println!("The block coordinates are : ({:?})",_cam.get_block() );
        println!("The world width and height are: {:?}",(_cam.world.unwrap().block_width,_cam.world.unwrap().block_height) );

    }

}

fn main()
{
    //Initialization
    let mut _world = world::World::generate(6,6,1600,1200);
    let mut _cam = camera::Camera::create(800,600,Some(&_world));

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");

	fn handle_events(event_pump : &mut sdl2::EventPump)
	{

	}

	fn update(delta_time :f32)
	{

	}

	fn render(canvas : &mut sdl2::render::Canvas<sdl2::video::Window>)
	{
	}


	engine.run(update, handle_events, render, &mut _cam);
}
