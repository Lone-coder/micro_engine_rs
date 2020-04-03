use crate::math::Vector2;

pub mod collision_rect;
pub mod physics_component;

//for visual rendering
use sdl2::video::Window;
use sdl2::render::Canvas;

use crate::physics::physics_component::PhysicsComponent;

//struct that holds all data for physics computation
pub struct PhysicsWorld {
    phys_components : Vec<PhysicsComponent>, //holds all the physics components in the scene
    gravity : Vector2, //acceleration due to gravity
}

impl PhysicsWorld {

    pub fn create_physics_world(gravity : Vector2) -> PhysicsWorld {
        PhysicsWorld {
            phys_components : Vec::new(),
            gravity : gravity,
        }
    }

    //adds a physics component to the physics world
    pub fn add_phys_component(&mut self, component : PhysicsComponent) {
        self.phys_components.push(component);
    }


    //detects collision and resolves it
    pub fn detect_and_resolve_collisions(&mut self) {
        // looping through all physics components and checking for collision
        for i in 0..self.phys_components.len() {
            for j in 0..self.phys_components.len() {
                //avoiding collision check with itself
                if i != j {
                     let coll_info = self.phys_components[i].check_collision2(&self.phys_components[j]);
                     let collided = coll_info;

                    //let coll_info = self.phys_components[i].check_collision(&self.phys_components[j]);
                    //let collided = coll_info.collided;

                    if collided {
                        //let n = coll_info.normal;
                        let n = Vector2::new(1.0, 0.0);
                        //scaling collision normal by impule value to get impulse vector
                        let impulse = n.scale(resolve_collisions(&self.phys_components[i], &self.phys_components[j], &n));

                        //calculating mass ratios for correct collision resoluton
                        let mass_sum = self.phys_components[i].mass + self.phys_components[j].mass;
                        let ratio_a = self.phys_components[i].mass / mass_sum;
                        let ratio_b = self.phys_components[j].mass / mass_sum;

                        //updating velocity of objects after collision
                        self.phys_components[i].velocity = self.phys_components[i].velocity + impulse.scale(ratio_a);
                        self.phys_components[j].velocity = self.phys_components[j].velocity - impulse.scale(ratio_b);
                    }
                }
            }
        }
    }

    pub fn update_physics_world(&mut self, delta_time : f32) {

        for i in 0..self.phys_components.len() {
            //adding gravity effect to all physics body
            if self.phys_components[i].affected_by_gravity {
                self.phys_components[i].velocity = self.phys_components[i].velocity + self.gravity.scale(delta_time);
            }
            self.phys_components[i].update(delta_time);
        }
    }

    //renders all the physics_components for debugging
    pub fn render_world_for_debug(&mut self, canvas : &mut Canvas<Window>) {
        for i in 0..self.phys_components.len() {
            self.phys_components[i].draw_phy_object(canvas);
        }
    }
}

//calculates impusle magnitude for a collision
pub fn resolve_collisions(A : &PhysicsComponent, B : &PhysicsComponent, collision_normal : &Vector2) -> f32 {

    let rv = B.velocity - A.velocity;

    let vel_along_normal = rv.dot(collision_normal);

    if vel_along_normal > 0.0 {
        return 0.0
    }

    //getting minimum coeff of restitution from both components
    let mut e = 0.0;
    if A.restitution_coeff > B.restitution_coeff {
        e = B.restitution_coeff;
    }
    else {
        e = A.restitution_coeff;
    }

    (-(1.0 + e) * vel_along_normal) / (A.inverse_mass + B.inverse_mass) //impulse magnitude
}
