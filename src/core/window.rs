//Handles window creation
pub struct Window{
    width : u32,
    height : u32,
    title : String,
}

impl Window{
    pub fn create_new_window(_width : u32, _height : u32, _title : String) -> Window
    {
        Window{
            width : _width,
            height : _height,
            title : _title,
        }
    }

    pub fn get_width(self) -> u32{
        self.width
    }

    pub fn get_height(self) -> u32{
        self.height
    }

    pub fn get_title(self) -> String{
        self.title
    }
}
