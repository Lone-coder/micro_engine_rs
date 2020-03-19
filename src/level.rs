use super::world;
use super::camera;
use super::game_object;

struct Level<'b,'a>
{
    world : world::World,
    main_camera : camera::Camera<'a, 'b>,
    game_objects : Vec<game_object::GameObject>,
}

impl Level<'_, '_> {

    //level data parsed from .meLevel file
    pub fn load_level()
    {

    }
}
