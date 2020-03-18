//all objects of u8 type are to be redefined

pub struct GameObject{
    actions:Option<u8>,
    physics_handle:Option<u8>,      // this could be an option
    animation_handle:Option<u8>,
    pub props:Option<u8>
}
impl GameObject{
    pub fn new()->GameObject{
        GameObject{
            actions:None,
            physics_handle:None,
            animation_handle:None,
            props:None
        }
    }
}
