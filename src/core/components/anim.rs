pub struct Anim{
    pub name:String,
    pub height:i32,
    pub width:i32,
    pub frame_order:Vec<(i32,i32)>,
    pub current_frame:usize

}

impl Anim{

    pub fn load(name:String,frame_order:Vec<(i32,i32)>)->Anim{
        Anim{
            name:name,
            frame_order:frame_order,
            height:0,
            width:0,
            current_frame:0,
        }
    }

    pub fn next_frame(&self)
    {
            
    }
}
