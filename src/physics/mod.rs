use crate::math::Vector2;

pub mod collision_rect;
pub mod physics_component;

//  kind of like another git
// to suggest changes
pub mod changes;

//for visual rendering
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

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
        let mut i = 0;
        while i < self.phys_components.len() {

            let mut j = i + 1;

            while j < self.phys_components.len() {

                // println!("xa {:?} xb {:?} ",self.phys_components[i].position, self.phys_components[j].position);
                let coll_info = self.phys_components[i].check_collision(&self.phys_components[j]);
                let collided = coll_info.collided;

                if collided {

                let n = coll_info.normal;
                
                let impulse_mag = resolve_collisions(&self.phys_components[i], &self.phys_components[j], &n);
                let impulse = n.scale(impulse_mag);

                //updating velocity of objects after collision
                self.phys_components[i].velocity = self.phys_components[i].velocity - impulse.scale(self.phys_components[i].inverse_mass);
                self.phys_components[j].velocity = self.phys_components[j].velocity + impulse.scale(self.phys_components[j].inverse_mass);
                
                //positional correctnesss
                let percent = 0.2; // usually 20% to 80%
                let slope = 0.01;
                let mut p = 0.0;

                if coll_info.penetration_depth - slope > 0.0 {
                    p = coll_info.penetration_depth - slope;              
                }

                let inverse_mass_sum = self.phys_components[i].inverse_mass + self.phys_components[j].inverse_mass;
                let correction = n.scale((p / inverse_mass_sum) * percent);
                self.phys_components[i].position = self.phys_components[i].position - correction.scale(self.phys_components[i].inverse_mass);
                self.phys_components[j].position = self.phys_components[j].position + correction.scale(self.phys_components[j].inverse_mass);
                }
                j += 1;
            }
            i += 1;
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

        canvas.set_draw_color(Color::RGB(255, 255, 0));
        for i in 0..self.phys_components.len() {
            self.phys_components[i].draw_phy_object(canvas);
        }
    }
}

//calculates impusle magnitude for a collision
pub fn resolve_collisions(A : &PhysicsComponent, B : &PhysicsComponent, collision_normal : &Vector2) -> f32 {

    // println!("a rest co{:?}, b rest co {:?} ",A.restitution_coeff,B.restitution_coeff );
    // println!("collision normal= {:?}",collision_normal);
    let rv = B.velocity - A.velocity;
    // println!("rv={:?}", rv);
    let vel_along_normal = rv.dot(collision_normal);
    // println!("vel_normal = {:?}",vel_along_normal );

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

    return -(1.0 + e) * vel_along_normal / (A.inverse_mass + B.inverse_mass) //impulse magnitude
}
