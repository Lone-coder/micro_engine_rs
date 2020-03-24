use super::tile;
use super::math;
use super::camera;
//All units are in pixels
use sdl2::render::Canvas;
use sdl2::video::Window;

//A block of tiles
// have origin at top left corner
pub struct TileBlock {
    pub tiles : Vec<tile::Tile>,
    pub position : math::Vector2,
}

//world is made of tile blocks
pub struct World{
    pub blocks : Vec<Vec<TileBlock>>,
    pub block_width : u32,
    pub block_height : u32,
    pub tile_width : u32,
    pub tile_height : u32,
}

impl World {
    //creates a new world with 0 id for all tiles
    pub fn create_new(num_blk_row : usize, num_blk_col : usize, _block_width : u32, _block_height : u32) -> World {

        let mut _blocks : Vec<Vec<TileBlock>> = Vec::new();

        let world_width = num_blk_col as u32 * _block_width;
        let world_height = num_blk_row as u32 * _block_height;

        let num_tiles_inrow = world_width / 32; //since each tiles is 32x32 pixels
        let num_tiles_incol = world_height / 32;

        for m in 0..num_blk_col {

            _blocks.push(Vec::new());

            for n in 0..num_blk_row {

                _blocks[m].push(
                    TileBlock {
                        tiles :  vec![tile::Tile::new(0, 0.0, 0.0)],
                        position : math::Vector2::new( (m as u32 * _block_width) as f32, (n as u32 * _block_height) as f32)
                    }
                )
            }
        }

        World {
            blocks: _blocks,
            block_width : _block_width,
            block_height : _block_height,
            tile_width : 32,
            tile_height : 32,
        }
    }

    pub fn render(&mut self, canvas : &mut Canvas<Window>, cam : &mut camera::Camera) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(210, 10, 0));

        for a in &self.blocks{
            for b in a{
                let x = b.position.x - cam.position.x;
                let y = b.position.y - cam.position.y;

                canvas.draw_rect(sdl2::rect::Rect::new(x as i32,
                    y as i32, self.block_width, self.block_height)).unwrap();
            }
        }
    }


}
