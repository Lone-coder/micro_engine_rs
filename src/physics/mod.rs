use crate::math::Vector2;

pub mod collision_rect;

pub use crate::physics::collision_rect::CollisionRect;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

pub struct PhysicsComponent {
    pub position : Vector2,
    pub velocity : Vector2,
    pub mass : f32,
    pub invers_mass : f32,
    collision_rect : CollisionRect,
    pub restitution_coeff : f32
}

impl PhysicsComponent {

    pub fn new(position : Vector2, mass : f32, width : f32, height : f32) -> PhysicsComponent {

        let mut invers_mass = 0.0;

        if mass == 0.0 {
            invers_mass == 0.0;
        }
        else {
            invers_mass = 1.0 / mass;
        }

        PhysicsComponent {
            position : position,
            velocity : Vector2::new(0.0, 0.0),
            mass : mass,
            invers_mass : invers_mass,
            collision_rect : CollisionRect::new(position.x, position.y, width, height),
            restitution_coeff : 0.0
        }
    }

    pub fn check_collision(&mut self, other : &mut Self) -> bool {

        self.collision_rect.x = self.position.x;
        self.collision_rect.y = self.position.y;

        self.collision_rect.is_colliding_with(&mut other.collision_rect)
    }

    pub fn update(&mut self, dt : f32) {
        self.position.x = self.position.x + (self.velocity.x * dt);
        self.position.y = self.position.y + (self.velocity.y * dt);
    }

    //draws collision rect for visual debugging
    pub fn draw_phy_object(&mut self, canvas : &mut Canvas<Window>) {

        let x = self.position.x as i32;
        let y = self.position.y as i32;
        let width = self.collision_rect.width as u32;
        let height = self.collision_rect.height as u32;

        canvas.draw_rect(Rect::new(x, y, width, height)).unwrap();
    }
}


pub fn resolve_collisions(A : &mut PhysicsComponent, B : &mut PhysicsComponent, collision_normal : &Vector2) -> f32 {
    let mut impulse = 0.0;

    let rv = A.velocity - B.velocity;

    let vel_along_normal = Vector2::dot_2(&rv, &collision_normal);

    if vel_along_normal > 0.0 {
        return 0.0
    }

    let mut e = 0.0;

    if A.restitution_coeff > B.restitution_coeff {
        e = B.restitution_coeff;
    }
    else {
        e = A.restitution_coeff;
    }

    let j = -(1.0 + e) * vel_along_normal;
    impulse = j / (A.invers_mass + B.invers_mass);

    impulse
}
