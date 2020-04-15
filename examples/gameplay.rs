use micro_engine_rs::{math::Vector2, test_engine};
use micro_engine_rs::core::components::{ComponentManager, TransformComponent,ComponentType,
                                        StateComponent ,Component, SpriteComponent, Animation, SpriteRect,
                                        PlayerState, State};
use micro_engine_rs::system::{RenderSystem, AnimationSystem, InputHandlingSystem, MovementSystem};
use micro_engine_rs::entity::EntityManager;

use std::env;
use std::path::Path;
use std::collections::{HashMap,HashSet};

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

    let idle = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(16, 32)],
        flip_horizontal : false,
    };

    let walk_left = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 0), (16, 0), (32, 0)],
        flip_horizontal : false,
    };

    let walk_right = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 0), (16, 0), (32, 0)],
        flip_horizontal : true,
    };

    let walk_up = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 16), (16, 16), (32, 16)],
        flip_horizontal : false,
    };

    let walk_down = Animation {
        frame_width : 16,
        frame_delay : 100.0,
        frame_coords : vec![(0, 32), (16, 32), (32, 32)],
        flip_horizontal : false,
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

    let transform = TransformComponent {
        position : Vector2::new(400.0, 300.0),
        size : Vector2::new(64.0, 64.0),
        angle : 0.0
    };

    let mut component_manager = ComponentManager::new();
    let mut entity_manager = EntityManager::new();

    let player = entity_manager.create_entity();

    entity_manager.attach_component(player, component_manager.create_sprite_component(sprite));
    entity_manager.attach_component(player, component_manager.create_transform_component(transform));
    entity_manager.attach_component(player, component_manager.create_state_component(State::PlayerState(PlayerState::Idle)));

    let enemy  = entity_manager.create_entity();

    while engine.is_running() {
        let instant = std::time::Instant::now();

        //INPUT
        for entity_index in 0..entity_manager.entities.len() {
            let state_index =  entity_manager.get_component_index(entity_index, ComponentType::StateComponent);

            match state_index  {
                Some(s_id) => {
                    InputHandlingSystem::PlayerInput(&mut engine.event_pump, &mut component_manager.states[s_id]);
                }
                _ => ()
            }
        }


        let key = engine.input_handle();


        for entity_index in 0..entity_manager.entities.len() {

            let sprite_index = entity_manager.get_component_index(entity_index, ComponentType::SpriteComponent);
            let state_index =  entity_manager.get_component_index(entity_index, ComponentType::StateComponent);
            let transform_index = entity_manager.get_component_index(entity_index, ComponentType::TransformComponent);


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

        //RENDERING
        engine.canvas.set_draw_color(Color::RGB(18, 18, 18));
        //engine.canvas.clear();
        //rendering code here

        for index in 0..entity_manager.entities.len() {

            let transform_index = entity_manager.get_component_index(index, ComponentType::TransformComponent);
            let sprite_index = entity_manager.get_component_index(index, ComponentType::SpriteComponent);

            match (transform_index, sprite_index) {
                (Some(t_id), Some(sprt_id)) => {
                    RenderSystem::RenderSprite(&mut component_manager.transforms[t_id], &mut component_manager.sprites[sprt_id], &mut engine.canvas);
                },
                (_,_) => (),
            }

        }


        engine.canvas.present();

        delta_time = instant.elapsed().as_secs_f32();
    }
}
