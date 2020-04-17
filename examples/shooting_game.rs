// trying to make a shooting game
// turned ought to be a particle simluation

 use micro_engine_rs;
use micro_engine_rs::{math, test_engine};
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::env;
use std::path::Path;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;


struct Bullet{
    position : math::Vector2,
    direction : math::Vector2,
    speed : f32
}

struct Player {
    position : math::Vector2,
    size : u32,
    bullets : Vec<Bullet>,
}

impl Player{
    fn new(pos : math::Vector2, s : u32) -> Player {
        Player{
            position : pos,
            size : s,
            bullets : Vec::new(),
        }
    }

    fn update(&mut self, dt : f32) {
        //input handling and other updates

        //physics update for bullets
        for i in 0..self.bullets.len() {
            let x = self.bullets[i].position.x;
            let y = self.bullets[i].position.y;

        if x < -1.0 || x > 800.0 {
            self.bullets[i].position.x = self.position.x;
        }

        if y < -1.0 || y > 600.0 {
            self.bullets[i].position.y = self.position.y;
        }

        self.bullets[i].position.x = self.bullets[i].position.x +
                                        (self.bullets[i].speed * self.bullets[i].direction.x * dt);
        self.bullets[i].position.y = self.bullets[i].position.y +
                                        (self.bullets[i].speed * self.bullets[i].direction.y * dt);
        }

    }

    fn render(&mut self, canvas : &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 210, 0));

        let x = self.position.x - (self.size / 2) as f32;
        let y = self.position.y - (self.size / 2) as f32;

        canvas.draw_rect(Rect::new(x as i32, y as i32, self.size, self.size)).unwrap();

        canvas.set_draw_color(Color::RGB(0, 210, 0));
        for i in 0..self.bullets.len() {

            let x = self.bullets[i].position.x;
            let y = self.bullets[i].position.y;

            canvas.draw_rect(Rect::new(x as i32, y as i32, 20, 20)).unwrap();
        }
    }

    fn create_bullets(&mut self, num_of_bullets : u32) {

        let mut seed = 789;

        for i in 0..num_of_bullets {

            //Custom linear random genetrator
            seed = (1234 * seed + 4321) % 5678;

            let angle = (seed % 360) as f32;

            //calculating direction from angle
            let dir = math::Vector2{x : angle.cos(), y : angle.sin()};
            let pos = self.position;
            let _speed = (10 + (seed % 90)) as f32;

            let bullet = Bullet{
                position : pos,
                direction : dir,
                speed : _speed,
            };

            self.bullets.push(bullet);
        }
    }
}


fn main() {

    //All sdl intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();

    //Create an engine instance
    let mut engine = test_engine::Engine::load_engine(canvas,&texture_creator,sdl_context.event_pump().unwrap());

    engine.running = true;

    let mut delta_time = 0.016;

    let mut player = Player::new(math::Vector2{x : 400.0, y : 300.0}, 64);
    player.create_bullets(100);

    while engine.is_running() {
        let instant = std::time::Instant::now();

        //INPUT
        let key = engine.input_handle();

        //UPDATE
        player.update(delta_time);

        //RENDERING
        engine.canvas.set_draw_color(Color::RGB(0, 0, 0));
        engine.canvas.clear();
        //rendering code here
        player.render(&mut engine.canvas);
        engine.canvas.present();

         delta_time = instant.elapsed().as_secs_f32();

    }

}
