use crate::math::Vector2;
use super::collision_rect::{CollisionRect, CollisionInfo, detect_collison};

//For visual debugging
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

//Component which handles all physics related tasks
pub struct PhysicsComponent {
    pub position : Vector2,
    pub velocity : Vector2,
    pub mass : f32,
    pub inverse_mass : f32,
    collision_rect : CollisionRect,
    pub restitution_coeff : f32,
    pub affected_by_gravity : bool,
}

impl PhysicsComponent {

    pub fn new(position : Vector2, mass : f32, width : f32, height : f32) -> PhysicsComponent {
        let mut inverse_mass = 0.0;

        //checking for 0.0 mass which represent infinite mass in this engine
        if mass == 0.0 {
            inverse_mass = 0.0;
        }
        else {
            inverse_mass = 1.0 / mass;
        }

        PhysicsComponent {
            position : position,
            velocity : Vector2::new(0.0, 0.0),
            mass : mass,
            inverse_mass : inverse_mass, //useful for calculations
            collision_rect : CollisionRect::new(position.x, position.y, width, height),
            restitution_coeff : 0.0,
            affected_by_gravity : true,
        }
    }

    //routine used by physics world to check collision
    pub fn check_collision(&self, other : &Self) -> CollisionInfo {
        detect_collison(&self.collision_rect, &other.collision_rect)
    }

    pub fn update(&mut self, dt : f32) {
        //updating position of collision rect
        self.collision_rect.x = self.position.x;
        self.collision_rect.y = self.position.y;

        //Integrating velocity to get position
        self.position.x = self.position.x + (self.velocity.x * dt);
        self.position.y = self.position.y + (self.velocity.y * dt);
    }

    //draws collision rect for visual debugging
    pub fn draw_phy_object(&mut self, canvas : &mut Canvas<Window>) {

        //calcualting pos and dimensions for rendering
        let x = (self.position.x - (self.collision_rect.width / 2.0)) as i32;
        let y = (self.position.y - (self.collision_rect.height / 2.0)) as i32;
        let width = self.collision_rect.width as u32;
        let height = self.collision_rect.height as u32;

        canvas.draw_rect(Rect::new(x, y, width, height)).unwrap();
    }
}
