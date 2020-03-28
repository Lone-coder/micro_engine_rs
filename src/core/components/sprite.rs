// Data structures
use  std::collections::VecDeque;
use std::collections::HashMap;

//

//Rect for rects
use sdl2::rect::Rect;

pub struct Sprite {
    pub height:i32,
    pub width:i32,
    pub sprite_coords:VecDeque<(i32,i32)>,
    pub index:usize,
    pub state_map:HashMap<String,(usize,usize)>,
    pub state:String,
    change_time:i32,
}

impl Sprite{
    pub fn new()->Sprite{
        let mut sprite=Sprite{
            height:0,
            width:0,
            sprite_coords:VecDeque::new(),
            index:0,
            state_map:HashMap::new(),
            state:"Dead".to_owned(),
            change_time:0,
        };

         sprite.state_map.insert(sprite.state.to_owned(),(0,0));
         sprite
    }

    pub fn get_coords_inc(&mut self)->(i32,i32){

        self.index=self.index+1;
        println!("self.index is {:?}",self.index );
        self.sprite_coords[self.index-1]
    }


    // load a sequence into sprite_coords
    pub fn load_sequence(&mut self,val:Vec<(i32,i32)>){
        val.iter().for_each(|x|{
            self.sprite_coords.push_back(*x)
        })
    }

    //debug
    pub fn disp_sequence(&self){
        self.sprite_coords.iter().for_each(|x|{
            println!("{:?}",x );
        })
    }

    // load a single value to sprite coords
    pub fn load_individual(&mut self,val:(i32,i32)){
        self.sprite_coords.push_back(val)
    }



    //SDL2 only !
    // get rect of current value
    // Returns sdl2::rect::Rect
    fn get_rect(&mut self)->Rect{
        let val=self.get_coords_inc();
        Rect::new(val.0,val.1,self.width as u32,self.height as u32)

    }


    pub fn set_states(&mut self,val:String,index1:usize,index2:usize){
        self.state_map.insert(val,(index1,index2)).unwrap_or_default();
    }



    // important user functions
    // changes the state to a value
    pub fn change_state(&mut self,val:String){
        match self.state_map.get(&val){
            Some(value) =>{
                self.state=val
            }
            _=>()
        }
    }


    // loads a sequence of animation frames based on a type of
    // behaviour
    pub fn load_states(&mut self, state:String,val:Vec<(i32,i32)>){
        let start=self.sprite_coords.len();
        self.load_sequence(val);
        let end=self.sprite_coords.len()-1;

        self.state_map.insert(state,(start,end));
    }




    //SDL2 only
    // returns  the animation frames it has to go through
    // Returns an sdl2::rect::Rect
    // in the current state
    // gets the current frame and increments
    pub fn get_frame_inc(&mut self)->Rect{
        let val=self.state_map[&self.state.to_owned()];

        //debug
        if self.index>val.1||self.index<val.0{
            self.index=val.0;
            self.get_rect()
        }else{
            self.get_rect()
        }

    }


    //Non SDL function
    // General , will return a tuple (a,b)
    // containig coordinates of the next frames
    pub fn get_next_frame(&mut self)->(i32,i32){
        let val=self.state_map[&self.state.to_owned()];
        if self.index>val.1||self.index<val.0{
            self.index=val.0;
            self.get_coords_inc()
        }else{
            self.get_coords_inc()
        }

    }


    pub fn set_change_interval(&mut self, val:i32){
        self.change_time=val
    }

}
