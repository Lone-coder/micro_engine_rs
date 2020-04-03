use crate::math::Vector2;

pub mod collision_rect;
pub mod physics_component;

//for visual rendering
use sdl2::video::Window;
use sdl2::render::Canvas;

use crate::physics::physics_component::PhysicsComponent;

//struct that holds all data for physics computation
pub struct PhysicsWorld {
    phys_components : Vec<PhysicsComponent>,
    gravity : Vector2,
}

impl PhysicsWorld {

    pub fn create_physics_world(gravity : Vector2) -> PhysicsWorld {
        PhysicsWorld {
            phys_components : Vec::new(),
            gravity : gravity,
        }
    }

    pub fn add_phys_component(&mut self, component : PhysicsComponent) {
        self.phys_components.push(component);
    }

    pub fn detect_and_resolve_collisions(&mut self) {
        // looping through all physics components and checking for collision
        for i in 0..self.phys_components.len() {
            for j in 0..self.phys_components.len() {
                //avoiding collision check with itself
                if i != j {
                     let coll_info = self.phys_components[i].check_collision(&self.phys_components[j]);

                    if coll_info.collided {
                        let n = coll_info.normal;

                        //scaling collision normal by impule value to get impulse vector
                        let impulse = n.scale(resolve_collisions(&self.phys_components[i], &self.phys_components[j], &n));

                        //calculating mass ratios for correct collision resoluton
                        let mass_sum = self.phys_components[i].mass + self.phys_components[j].mass;
                        let ratio_a = self.phys_components[i].mass / mass_sum;
                        let ratio_b = self.phys_components[j].mass / mass_sum;

                        self.phys_components[i].velocity = self.phys_components[i].velocity - impulse.scale(ratio_a);
                        self.phys_components[j].velocity = self.phys_components[j].velocity + impulse.scale(ratio_b);
                    }
                }
            }
        }
    }

    pub fn update_physics_world(&mut self, delta_time : f32) {
        //adding gravity effect to all physics body
        for i in 0..self.phys_components.len() {
            if self.phys_components[i].affected_by_gravity {
                self.phys_components[i].velocity = self.phys_components[i].velocity + self.gravity.scale(delta_time);
            }
            self.phys_components[i].update(delta_time);
        }
    }

    pub fn render_world_for_debug(&mut self, canvas : &mut Canvas<Window>) {
        for i in 0..self.phys_components.len() {
            self.phys_components[i].draw_phy_object(canvas);
        }
    }
}

pub fn resolve_collisions(A : &PhysicsComponent, B : &PhysicsComponent, collision_normal : &Vector2) -> f32 {

    let mut impulse = 0.0;

    let rv = B.velocity - A.velocity;

    let vel_along_normal = Vector2::dot_2(&rv, collision_normal);

    if vel_along_normal > 0.0 {
        return 0.0
    }

    let mut e = 0.0;

    //getting minimum coeff of restitution from both components
    if A.restitution_coeff > B.restitution_coeff {
        e = B.restitution_coeff;
    }
    else {
        e = A.restitution_coeff;
    }

    let j = -(1.0 + e) * vel_along_normal;
    impulse = j / (A.inverse_mass + B.inverse_mass);

    impulse
}
