#[derive(Debug)]
pub struct Animation {
    pub frame_width : u32,
    pub frame_delay : f32,
    pub frame_coords : Vec<(i32, i32)>,
    pub flip_horizontal : bool,
    pub flip_vertical : bool,
}

impl Animation {
    pub fn new(frame_width : u32, frame_delay : f32, frame_coords : Vec<(i32, i32)>, flip_horizontal : bool, flip_vertical : bool) -> Animation {
        Animation {
            frame_width : frame_width,
            frame_delay : frame_delay,
            frame_coords : frame_coords,
            flip_horizontal : flip_horizontal,
            flip_vertical : flip_vertical,
        }
    }
}
