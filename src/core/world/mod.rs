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
pub struct World {
    pub blocks : Vec<Vec<TileBlock>>,
    pub block_width : u32,
    pub block_height : u32,
    pub tile_size : u32,
}

impl World {
    //creates a new world with 0 id for all tiles
    pub fn create_new(num_blk_row : usize, num_blk_col : usize, _block_width : u32,
                    _block_height : u32, _tile_size : u32) -> World {

        let mut _blocks : Vec<Vec<TileBlock>> = Vec::new();

        let n_tile_row = _block_height / _tile_size;
        let n_tile_col = _block_width / _tile_size;

        for m in 0..num_blk_col {

            _blocks.push(Vec::new());

            for n in 0..num_blk_row {

                let block_x = m as u32 * _block_width;
                let block_y = n as u32 * _block_height;

                let mut tile_block  = TileBlock {
                    position : math::Vector2::new( block_x as f32, block_y as f32),
                    tiles : Vec::new(),
                };

                for a in 0..n_tile_row {
                    for b in 0..n_tile_col {
                        //calculatin pos of tile in world spcae
                        let tile_x = (b * _tile_size) + block_x;
                        let tile_y = (a * _tile_size) + block_y;

                        tile_block.tiles.push(tile::Tile::new(0, tile_x as f32, tile_y as f32));
                    }
                }

                _blocks[m].push(tile_block);
            }
        }

        World {
            blocks: _blocks,
            block_width : _block_width,
            block_height : _block_height,
            tile_size : _tile_size
        }
    }

    pub fn render(&mut self, canvas : &mut Canvas<Window>, cam : &mut camera::Camera) {

        let n_tile_row = self.block_height / self.tile_size;
        let n_tile_col = self.block_width / self.tile_size;

        println!("Block : {:?}, {:?}", cam.position.x as u32 / self.block_width,
                        cam.position.y as u32 / self.block_height);

        for vec_of_block in &self.blocks {
            for blk in vec_of_block {

                canvas.set_draw_color(sdl2::pixels::Color::RGB(180, 0, 0));
                //drawing tiles
                for a in 0..n_tile_row {
                    for b in 0..n_tile_col {

                        let tile_x = blk.tiles[(b + (a * n_tile_col)) as usize]
                                                    .position.x - cam.position.x;
                        let tile_y = blk.tiles[(b + (a * n_tile_col)) as usize]
                                                    .position.y - cam.position.y;

                        canvas.draw_rect(sdl2::rect::Rect::new(tile_x as i32, tile_y as i32,
                                                    self.tile_size, self.tile_size)).unwrap();
                    }
                }

                //world to screen space transform (getting coords relative to camera)
                let x = blk.position.x - cam.position.x;
                let y = blk.position.y - cam.position.y;

                //drawing blocks for debug
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                canvas.draw_rect(sdl2::rect::Rect::new(x as i32,
                    y as i32, self.block_width, self.block_height)).unwrap();
            }
        }

    }

}
