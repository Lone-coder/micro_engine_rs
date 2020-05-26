//TODO : implement tile editing, tile selection from palatte 
use micro_engine_rs::math::Vector2;

use sdl2;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render:: {TextureCreator, Texture};
use sdl2::render::Canvas;

use std::collections::HashSet;

struct Tile {
    position : Vector2,
    tile_palatte_index : u32
}

struct TilePalette {
    texture : Texture,
    tile_width : u32,
    tile_height : u32,
}

struct TilePaletteDisplay {

}

struct TileMap {
    tiles : Vec<Tile>,
    width : u32, //unit tiles
    height : u32,
    tile_width : u32,
    tile_height : u32
}

struct EditorGrid {
    cell_width : u32,
    cell_height : u32,
    rows : u32,
    columns : u32,   
}

struct EditorCursor {
    position : Vector2,
    width : u32,
    height : u32,
}

enum EditorMode {
    EDIT_TILEMAP, //key : TAB
    SELECT_TILE, //key : Q
    SAVE_TILEMAP,// key : S
}

fn main() {
    
     //All sdl intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem.window("MicroEditor", 1280, 720)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string()).unwrap();
    let mut texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;

    //Editor variables

    let grid = EditorGrid {
        cell_width : 64,
        cell_height : 64,
        rows : 10,
        columns : 10,   
    };

    let mut cursor = EditorCursor {
        position : Vector2::new(0.0, 0.0),
        width : grid.cell_width,
        height : grid.cell_height,
    };

    let editor_mode = EditorMode::EDIT_TILEMAP;

    let old_keys : HashSet<Keycode> = HashSet::new();
    //let new_keys : HashSet<Keycode> = HashSet::new();


    while(running) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _=>()
            }
        }
        let key_presses : HashSet<Keycode> = event_pump
                        .keyboard_state()
                        .pressed_scancodes()
                        .filter_map(Keycode::from_scancode).collect();

        for key in key_presses.iter() {
            match key {
                Keycode::Up => {
                    cursor.position.y -= grid.cell_height as f32;
                    break;
                },
                Keycode::Down => {
                    cursor.position.y += grid.cell_height as f32;
                    break;
                },
                Keycode::Right => {
                    cursor.position.x += grid.cell_width as f32;
                    break;
                },
                Keycode::Left => {
                    cursor.position.x -= grid.cell_width as f32;
                    break;
                },
                _=> (),
            }
        } 


        canvas.set_draw_color(Color::RGBA(0 ,0 ,0 , 255));
        canvas.clear();
        draw_grids(&grid, &mut canvas);
        draw_cursor(&cursor, &mut canvas);
        canvas.present();
    }

}

fn draw_grids(grid : &EditorGrid, canvas : &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGBA(255 ,255 ,255 , 255));

    for _j in 0..grid.rows {
        for _i in 0..grid.columns {
            let x = _i as u32 * grid.cell_width;
            let y = _j as u32 * grid.cell_height;
            canvas.draw_rect(Rect::new(x as i32, y as i32, grid.cell_width, grid.cell_height,)).unwrap();
        }
    }
}

fn draw_cursor(cursor : &EditorCursor, canvas : &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGBA(255, 255, 0, 255));
    canvas.draw_rect(Rect::new( cursor.position.x as i32,cursor.position.y as i32, cursor.width, cursor.height)).unwrap();
    canvas.draw_rect(Rect::new( cursor.position.x as i32 + 1,cursor.position.y as i32 + 1, cursor.width - 1, cursor.height - 1)).unwrap();
    canvas.draw_rect(Rect::new( cursor.position.x as i32 + 2,cursor.position.y as i32 + 2, cursor.width - 2, cursor.height - 2)).unwrap();
}