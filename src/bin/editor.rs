//TODO(Lone-coder) : implement tile editing, tile selection from palatte, tilemap saving tilemap loading

use sdl2;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::video::WindowContext;

struct Tile {
    d_rect: EditorRect,
    s_rect: EditorRect,
}

struct TilePalette {
    texture: Texture,
    tile_width: u32,
    tile_height: u32,
    width: u32,
    height: u32,
}

impl TilePalette {
    fn load_from_file(
        file_path: &str,
        tile_width: u32,
        tile_height: u32,
        tex_creator: &mut TextureCreator<WindowContext>,
    ) -> TilePalette {
        let texture = tex_creator
            .load_texture(std::path::Path::new(file_path))
            .unwrap();
        let tex_query = texture.query();
        TilePalette {
            texture: texture,
            tile_width: tile_width,
            tile_height: tile_height,
            width: tex_query.width,
            height: tex_query.height,
        }
    }
}

#[derive(Copy, Clone)]
struct EditorRect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

type EditorCursor = EditorRect;
type WindowRect = EditorRect;

struct DisplayWindow {
    rect: WindowRect,
    cursor: EditorCursor,
}

struct TileMap {
    tiles: Vec<Tile>,
    width: u32, //unit tiles
    height: u32,
    tile_width: u32,
    tile_height: u32,
}

struct EditorGrid {
    cell_width: u32,
    cell_height: u32,
    rows: u32,
    columns: u32,
}

enum EditorMode {
    EditTileMap, //key : TAB
    SelectTile,  //key : Q
    SaveTileMap, // key : S
}

fn main() {
    let editor_width = 1280;
    let editor_height = 720;

    //All sdl intialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video_subsystem
        .window("MicroEditor(TileMap)", editor_width, editor_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;

    //Editor variables
    let mut editor_mode = EditorMode::EditTileMap;

    let tile_palette = TilePalette::load_from_file(
        "assets\\LPC_city_outside\\city_outside.png",
        32,
        32,
        &mut texture_creator,
    );
    let mut tile_palette_display = DisplayWindow {
        rect: WindowRect {
            x: (editor_width - tile_palette.width / 2) as i32,
            y: 0,
            width: tile_palette.width / 2,
            height: tile_palette.height / 2,
        },
        cursor: EditorCursor {
            x: (editor_width - tile_palette.width / 2) as i32,
            y: 0,
            width: tile_palette.tile_width / 2,
            height: tile_palette.tile_height / 2,
        },
    };

    let mut selection_cursor = EditorRect {
        x: 0,
        y: 0,
        width: tile_palette.tile_width,
        height: tile_palette.tile_height,
    };

    let grid = EditorGrid {
        cell_width: 32,
        cell_height: 32,
        rows: 20,
        columns: 20,
    };

    //Tile map cursor
    let mut cursor = EditorCursor {
        x: 0,
        y: 0,
        width: grid.cell_width,
        height: grid.cell_height,
    };

    let mut tile_map = TileMap {
        tiles: Vec::new(),
        width: grid.columns,
        height: grid.rows,
        tile_width: grid.cell_width,
        tile_height: grid.cell_height,
    };

    for _i in 0..(grid.columns * grid.rows) {
        let rect = EditorRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let tile = Tile {
            d_rect: rect,
            s_rect: rect,
        };
        tile_map.tiles.push(tile);
    }

    let mut dragable = Rect::new(0, 0, 300, 200);
    let mut header = Rect::new(0, 0, 300, 30);

    let mut mouse_left_down = false;
    let mut contains = false;

    while running {
        let state = event_pump.mouse_state();
        let mouse_pos = Point::new(state.x(), state.y());

        if header.contains_point(mouse_pos) && !contains {
            contains = true;
        }

        if mouse_left_down && contains {
            dragable.x = mouse_pos.x - 20;
            dragable.y = mouse_pos.y - 5;
        }

        header.x = dragable.x;
        header.y = dragable.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    mouse_left_down = true;
                    contains = false;
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    mouse_left_down = false;
                    contains = false;
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    editor_mode = EditorMode::SelectTile;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    editor_mode = EditorMode::EditTileMap;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    let m = cursor.x / tile_map.tile_width as i32;
                    let n = cursor.y / tile_map.tile_height as i32;
                    let index = m + n * tile_map.width as i32;

                    if index < (tile_map.width * tile_map.height) as i32
                        && m < tile_map.width as i32
                        && n < tile_map.height as i32
                    {
                        tile_map.tiles[index as usize].s_rect = selection_cursor;
                        tile_map.tiles[index as usize].d_rect = cursor;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Delete),
                    ..
                } => {
                    let m = cursor.x / tile_map.tile_width as i32;
                    let n = cursor.y / tile_map.tile_height as i32;
                    let index = m + n * tile_map.width as i32;

                    if index < (tile_map.width * tile_map.height) as i32
                        && m < tile_map.width as i32
                        && n < tile_map.height as i32
                    {
                        tile_map.tiles[index as usize].s_rect = EditorRect {
                            x: 0,
                            y: 0,
                            width: 0,
                            height: 0,
                        };
                        tile_map.tiles[index as usize].d_rect = EditorRect {
                            x: 0,
                            y: 0,
                            width: 0,
                            height: 0,
                        };
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => match editor_mode {
                    EditorMode::EditTileMap => {
                        cursor.y -= cursor.height as i32;
                    }
                    EditorMode::SelectTile => {
                        tile_palette_display.cursor.y -= tile_palette_display.cursor.height as i32;
                        selection_cursor.y -= selection_cursor.height as i32;
                    }
                    _ => (),
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => match editor_mode {
                    EditorMode::EditTileMap => {
                        cursor.y += cursor.height as i32;
                    }
                    EditorMode::SelectTile => {
                        tile_palette_display.cursor.y += tile_palette_display.cursor.height as i32;
                        selection_cursor.y += selection_cursor.height as i32;
                    }
                    _ => (),
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => match editor_mode {
                    EditorMode::EditTileMap => {
                        cursor.x += cursor.width as i32;
                    }
                    EditorMode::SelectTile => {
                        tile_palette_display.cursor.x += tile_palette_display.cursor.width as i32;
                        selection_cursor.x += selection_cursor.width as i32;
                    }
                    _ => (),
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => match editor_mode {
                    EditorMode::EditTileMap => {
                        cursor.x -= cursor.width as i32;
                    }
                    EditorMode::SelectTile => {
                        tile_palette_display.cursor.x -= tile_palette_display.cursor.width as i32;
                        selection_cursor.x -= selection_cursor.width as i32;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        draw_grids(&grid, &mut canvas);

        //tile map
        draw_tile_map(&tile_map, &tile_palette, &mut canvas);
        canvas
            .copy(
                &tile_palette.texture,
                Some(editorrect_to_sdlrect(&selection_cursor)),
                Some(editorrect_to_sdlrect(&cursor)),
            )
            .unwrap();
        draw_cursor(&cursor, &mut canvas);
        //tile palette
        canvas
            .copy(
                &tile_palette.texture,
                None,
                Some(editorrect_to_sdlrect(&tile_palette_display.rect)),
            )
            .unwrap();
        canvas
            .draw_rect(editorrect_to_sdlrect(&tile_palette_display.rect))
            .unwrap();
        draw_cursor(&tile_palette_display.cursor, &mut canvas);
        canvas.set_draw_color(Color::RGB(18, 18, 18));
        canvas.fill_rect(dragable).unwrap();
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(header).unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.draw_rect(dragable).unwrap();
        canvas.present();
    }
}

fn draw_grids(grid: &EditorGrid, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

    for _j in 0..grid.rows {
        for _i in 0..grid.columns {
            let x = _i as u32 * grid.cell_width;
            let y = _j as u32 * grid.cell_height;
            canvas
                .draw_rect(Rect::new(
                    x as i32,
                    y as i32,
                    grid.cell_width,
                    grid.cell_height,
                ))
                .unwrap();
        }
    }
}

fn draw_tile_map(tile_map: &TileMap, tile_palatte: &TilePalette, canvas: &mut Canvas<Window>) {
    for _i in 0..tile_map.tiles.len() {
        canvas
            .copy(
                &tile_palatte.texture,
                Some(editorrect_to_sdlrect(&tile_map.tiles[_i].s_rect)),
                Some(editorrect_to_sdlrect(&tile_map.tiles[_i].d_rect)),
            )
            .unwrap();
    }
}

fn draw_cursor(cursor: &EditorCursor, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGBA(255, 255, 0, 255));
    canvas
        .draw_rect(Rect::new(cursor.x, cursor.y, cursor.width, cursor.height))
        .unwrap();
    canvas
        .draw_rect(Rect::new(
            cursor.x + 1,
            cursor.y + 1,
            cursor.width - 1,
            cursor.height - 1,
        ))
        .unwrap();
    canvas
        .draw_rect(Rect::new(
            cursor.x + 2,
            cursor.y + 2,
            cursor.width - 2,
            cursor.height - 2,
        ))
        .unwrap();
}

fn editorrect_to_sdlrect(cursor: &EditorRect) -> Rect {
    Rect::new(cursor.x, cursor.y, cursor.width, cursor.height)
}
