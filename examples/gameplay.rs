//micro engine modules
use micro_engine_rs::{math::Vector2, test_engine};
use micro_engine_rs::core::components::{ComponentManager, ComponentType, Component};
use micro_engine_rs::core::components::sprite::{SpriteComponent, SpriteRect, Animation};
use micro_engine_rs::core::components::transform::TransformComponent;
use micro_engine_rs::core::components::state::{State, StateComponent, PlayerState, EnemyState};
use micro_engine_rs::system::{RenderSystem, AnimationSystem, InputHandlingSystem, MovementSystem, AISystem};
use micro_engine_rs::entity::{EntityManager, EntityType};


//standard imports
use std::env;
use std::path::Path;
use std::collections::{HashMap,HashSet};

//sdl modules
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn main() {
    //All sdl intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem.window("Game play test", 800, 600)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap();

    let canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();

    //Create an engine instance
    let mut engine = test_engine::Engine::load_engine(canvas,&texture_creator,sdl_context.event_pump().unwrap());
    engine.running = true;
    let mut delta_time = 1.0 / 60.0;

    //used by animation system
    let global_instant = std::time::Instant::now();

    let mut component_manager = ComponentManager::new();
    let mut entity_manager = EntityManager::new();

    //animation data
    let idle = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(16, 32)],
        flip_horizontal : false,
        flip_vertical : false,
    };
    let walk_left = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 0), (16, 0), (32, 0)],
        flip_horizontal : false,
        flip_vertical : false,
    };
    let walk_right = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 0), (16, 0), (32, 0)],
        flip_horizontal : true,
        flip_vertical : false,
    };
    let walk_up = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 16), (16, 16), (32, 16)],
        flip_horizontal : false,
        flip_vertical : false,
    };
    let walk_down = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 32), (16, 32), (32, 32)],
        flip_horizontal : false,
        flip_vertical : false,
    };

    let mut sprite = SpriteComponent {
        src_rect : SpriteRect{ x : 0, y : 0, width : 16, height : 16},
        animations : HashMap::new(),
        current_animation : "walk_left".to_string(),
        flip_horizontal : false,
        flip_vertical : false,
        texture : engine.texture_creator.unwrap().load_texture(std::path::Path::new("assets/hero.png")).unwrap()
    };

    sprite.animations.insert("walk_left".to_string(), walk_left);
    sprite.animations.insert("walk_up".to_string(), walk_up);
    sprite.animations.insert("walk_right".to_string(), walk_right);
    sprite.animations.insert("walk_down".to_string(), walk_down);
    sprite.animations.insert("idle".to_string(), idle);

    let transform = TransformComponent::new(Vector2::new(400.0, 300.0), Vector2::new(64.0, 64.0), 0.0);
    let player_state = StateComponent { current_state : State::PlayerState(PlayerState::Idle) };

    entity_manager.create_entity("player", EntityType::Player);
    entity_manager.attach_component("player", component_manager.create_sprite_component(sprite));
    entity_manager.attach_component("player", component_manager.create_transform_component(transform));
    entity_manager.attach_component("player", component_manager.create_state_component(player_state));


    let mut enemy_sprite = SpriteComponent {
        src_rect : SpriteRect{ x : 0, y : 0, width : 16, height : 16},
        animations : HashMap::new(),
        current_animation : "idle".to_string(),
        flip_horizontal : false,
        flip_vertical : false,
        texture : engine.texture_creator.unwrap().load_texture(std::path::Path::new("assets/hero.png")).unwrap(),
    };

    let enemy_idle = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(16, 32)],
        flip_horizontal : false,
        flip_vertical : false,
    };
    let enemy_walk_left = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 0), (16, 0), (32, 0)],
        flip_horizontal : false,
        flip_vertical : false,
    };
    let enemy_walk_right = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 0), (16, 0), (32, 0)],
        flip_horizontal : true,
        flip_vertical : false,
    };
    let enemy_walk_up = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 16), (16, 16), (32, 16)],
        flip_horizontal : false,
        flip_vertical : false,
    };
    let enemy_walk_down = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 32), (16, 32), (32, 32)],
        flip_horizontal : false,
        flip_vertical : false,
    };

    enemy_sprite.animations.insert("walk_left".to_string(), enemy_walk_left);
    enemy_sprite.animations.insert("walk_up".to_string(), enemy_walk_up);
    enemy_sprite.animations.insert("walk_right".to_string(), enemy_walk_right);
    enemy_sprite.animations.insert("walk_down".to_string(), enemy_walk_down);
    enemy_sprite.animations.insert("idle".to_string(), enemy_idle);

    let enemy_state = StateComponent { current_state : State::EnemyState(EnemyState::Idle) };

    let enemy  = entity_manager.create_entity("enemy", EntityType::Enemy);
    let enemy_transform = TransformComponent::new(Vector2::new(500.0, 500.0), Vector2::new(64.0, 64.0), 0.0);
    entity_manager.attach_component("enemy", component_manager.create_transform_component(enemy_transform));
    entity_manager.attach_component("enemy", component_manager.create_sprite_component(enemy_sprite));
    entity_manager.attach_component("enemy", component_manager.create_state_component(enemy_state));

    let mut old_keyset : HashSet<Keycode> = HashSet::new();

    while engine.is_running() {
        let instant = std::time::Instant::now();

        //INPUT
        let new_keyset = engine.input_handle();
        InputHandlingSystem::PlayerInput(&new_keyset, &old_keyset, &mut component_manager
                                .states[entity_manager
                                .get_component_index_of_entity("player", ComponentType::StateComponent).unwrap()]);
        old_keyset = new_keyset;

        //GAME UPDATE
        //AI update
        update_ai(&mut entity_manager, &mut component_manager, delta_time);

        for entity_index in 0..entity_manager.entities.len() {

            let sprite_index = entity_manager.get_component_index_of_entity_id(entity_index, ComponentType::SpriteComponent);
            let state_index =  entity_manager.get_component_index_of_entity_id(entity_index, ComponentType::StateComponent);
            let transform_index = entity_manager.get_component_index_of_entity_id(entity_index, ComponentType::TransformComponent);

            match (state_index, sprite_index)  {
                (Some(s_id), Some(sprt_id)) => {
                    AnimationSystem::UpdateAnimationState(&mut component_manager.states[s_id], &mut component_manager.sprites[sprt_id]);
                },
                (_,_) => ()
            }
            match (state_index, transform_index) {
                (Some(s_id), Some(t_id)) => {
                    MovementSystem::move_entity(&mut component_manager.transforms[t_id], &mut component_manager.states[s_id], delta_time);
                },
                (_,_) =>()
            }
            match state_index  {
                Some(s_id) => {
                    AnimationSystem::UpdateAnimation(&mut component_manager
                                        .sprites[s_id], global_instant.elapsed().as_millis() as f32);
                }
                _ => ()
            }
        }

        //RENDER
        engine.canvas.set_draw_color(Color::RGB(20, 20, 20));
        engine.canvas.clear();

        //rendering code here
        for index in 0..entity_manager.entities.len() {
            let transform_index = entity_manager.get_component_index_of_entity_id(index, ComponentType::TransformComponent);
            let sprite_index = entity_manager.get_component_index_of_entity_id(index, ComponentType::SpriteComponent);

            match (transform_index, sprite_index) {
                (Some(t_id), Some(sprt_id)) => {
                    RenderSystem::RenderSprite(&mut component_manager.transforms[t_id], &mut component_manager.sprites[sprt_id], &mut engine.canvas);
                },
                (_,_) => (),
            }
        }
        engine.canvas.present();
        
        delta_time = instant.elapsed().as_secs_f32();
        //println!("{:?}", delta_time);
    }
}

//custom update system
fn update_ai(entity_manager : &mut EntityManager, component_manager : &mut ComponentManager, dt : f32) {
    
    let target_position = component_manager.transforms[entity_manager
                        .get_component_index_of_entity("player",  ComponentType::TransformComponent).unwrap()].position;
    
    let follower_position = &mut component_manager.transforms[entity_manager
                        .get_component_index_of_entity("enemy",  ComponentType::TransformComponent).unwrap()];

    let t = target_position;
    let f = follower_position.position;
    let dir = (t - f).normalize();
    let speed = (t - f).modulus() * 0.5;

    follower_position.position = follower_position.position + dir.scale(dt * speed);
}