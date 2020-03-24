use super::math;

pub struct Tile {
    id : u32, //used to index texture from tile set
    pub position : math::Vector2,
}

impl Tile {
    pub fn new(_id : u32 , _x : f32, _y : f32) -> Tile {
        Tile {
            id : _id,
            position : math::Vector2{x : _x, y : _y},
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.id
    }


}
