use super::world;
use super::camera;
use super::gameobject;

struct Level<'b,'a>
{
    world : world::World,
    main_camera : camera::Camera<'a, 'b>,
    game_objects : Vec<gameobject::GameObject>,
}

impl Level<'_, '_> {

    //level data parsed from .meLevel file
    pub fn load_level()
    {

    }
}
