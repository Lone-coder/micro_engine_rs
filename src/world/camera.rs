pub struct Camera{
    pub width:i32,
    pub height:i32,
    pub x:i32,
    pub y:i32,
}

impl Camera{
    pub fn new(width:i32,height:i32,x:i32,y:i32)->Camera{
        Camera{
            width:width,
            height:height,
            x:x,
            y:y
        }
    }
    pub fn get_x_m(&self)->i32{
        self.x+self.width/2
    }
    pub fn get_y_m(&self)->i32{
        self.y+self.height/2
    }

    pub fn get_tile_rect(&self){

    }

    pub fn render(){
        unimplemented!()
    }

}
