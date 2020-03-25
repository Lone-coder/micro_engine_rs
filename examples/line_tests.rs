//just for fun !
//:}
use micro_engine_rs::HashSet;
use micro_engine_rs;
use micro_engine_rs::Keycode;
use micro_engine_rs::core::{world,camera};
use camera::Camera;
use sdl2::rect::Rect;
use sdl2::pixels::Color;


use sdl2;

fn main()
{
    //Initialization{:?}
    let mut _world = world::World::generate(6, 6, 8000, 6000);
    _world.load_objs_from_json();
    _world.obj_disp_per_block();

    let mut _cam = camera::Camera::create(800, 600, Some(&_world));

    _cam.x = 800 * 3;
    _cam.y = 600 * 3;

    println!("{:?}",() );

	let mut engine = micro_engine_rs::Engine::init_engine(800, 600, "Camera movement test");



    // instance of engine running

	while engine.is_running() {

        let instant = std::time::Instant::now();

        let dt = engine.delta_time();

        //_cam.x += (1000.0 * dt ) as i32;

        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        engine.canvas.clear();

        engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 210, 0));

        render_cam(&mut _cam,&mut engine.canvas);
        //_cam.render_scene_objects(&mut engine.canvas);

        engine.canvas.present();

        engine.update(instant);

        move_cam(5000.0,& mut _cam,engine.input_handle(),dt);
    }
}



fn move_cam(param:f32,cam:&mut camera::Camera,keys:HashSet<Keycode>,dt:f32){

        if keys.contains(&Keycode::Up){
            cam.y-=(param*dt) as i32
        }
        if keys.contains(&Keycode::Down){
            cam.y+=(param*dt) as i32
        }

        if keys.contains(&Keycode::Left){
            cam.x-=(param*dt) as i32
        }

        if keys.contains(&Keycode::Right){
            cam.x+=(param*dt) as i32
        }
}

fn render_cam(cam:&mut Camera,canvas:&mut sdl2::render::Canvas<sdl2::video::Window>){
    let (x,y)=(cam.width/2,cam.height/2);

    cam.get_objs_in_scene().iter().for_each( |vec_obj| {
        //rendering filled rects in place of static gameobjects
        canvas.draw_line(sdl2::rect::Point::new(x, y),sdl2::rect::Point::new(vec_obj.1, vec_obj.2));
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas.fill_rect(Rect::new(vec_obj.1, vec_obj.2, 20, 20)).unwrap();
    });

}
