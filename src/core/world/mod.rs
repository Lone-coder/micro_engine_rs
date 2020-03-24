use super::tile;

//A block of tiles
// have origin at top left corner
pub struct TileBlock {
    pub tiles : Vec<tile::Tile>,
}

//world is made of tile blocks
pub struct World{
    pub blocks : Vec<Vec<TileBlock>>,
    pub block_width : i32,
    pub block_height : i32,
    pub tile_width : u32,
    pub tile_height : u32,
}

impl World {
    //creates a new world with 0 id for all tiles
    pub fn create_new(num_blk_row : usize, num_blk_col : usize, _block_width : i32, _block_height : i32) -> World {

        let mut _blocks : Vec<Vec<TileBlock>> = Vec::new();

        for m in 0..num_blk_col {

            _blocks.push(Vec::new());

            for n in 0..num_blk_row {
                _blocks[m].push(
                    TileBlock {
                        tiles :  vec![tile::Tile::new(0, 0.0, 0.0)],
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

    pub fn render(&mut self) {


    }

}
