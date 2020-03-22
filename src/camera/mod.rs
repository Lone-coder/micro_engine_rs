/// Useful info:
/// Here &'a world implies that the Camera lives atleast for as long as the world lives
/// or any reference to 'a musn't live longer than it
use super::Rect;
use super::Color;

use sdl2 ::event::Event;
use sdl2 ::keyboard::Keycode;

pub struct Camera<'a,'b>{
    pub x:i32,
    pub y:i32,
    pub width:i32,
    pub height:i32,
    scene_objects:Vec<& 'b super::game_object::GameObject>,
    pub world:Option<&'a super::world::World>,
}

impl<'a,'b> Camera<'a,'b>{
    pub fn create(width:i32, height:i32, _world:Option<&'a super::world::World>) -> Camera<'a,'b> {
        Camera{
            x : width / 2,
            y : height / 2,
            width : width,
            height : height,
            scene_objects : Vec::new(),
            world: _world
        }
    }

    pub fn display_scene_objects(&mut self){
        self.scene_objects.iter().for_each(|x| println!("{:?}",x.props ));
    }

    pub fn attach_world(&mut self, world: &'a super::world::World){
        self.world = Some(world)
    }

    pub fn pan_x(& mut self,val:i32){
        self.x+=val
    }


    pub fn pan_y(&mut self,val:i32){
        self.y+=val
    }


    pub fn get_coord(&mut self) -> (i32,i32) {
        (self.x,self.y)
    }


    pub fn add_obj(&mut self,obj:&'b super::game_object::GameObject) {
        self.scene_objects.push(obj);
    }

    pub fn get_block(&mut self) -> Option<(i32,i32)> {
        if let Some(world)=self.world {
            Some((self.x / world.block_width, self.y / world.block_height))
        }else{
            None
        }
    }

    //camera movement test
    pub fn camera_event(&mut self,event_pump : &mut sdl2::EventPump, dt : &f32) {

        for event in event_pump.poll_iter() {
	        match event {
	            Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                        self.y -= (500.0 * (*dt)) as i32;
	                },
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                        self.y += (500.0 * (*dt)) as i32;
                    },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                        self.x += (500.0 * (*dt)) as i32;
                    },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                        self.x -= (500.0 * (*dt)) as i32;
                    },
	            _=>(),
	        }
	    }
    }

    // Returns all objects in proximity for collisions
    // and rendering

    // to get all game objects loop over the value returned

    /*
            [B][B][B][B]
            [X][X][X][B]
            [X][C][X][B]
            [X][X][X][B]
            [B][B][B][B]

            in the above example:
            * C is the block containing camera
            * All objects in the units marked X will be rendered
            * and used for collisions

    */

    pub fn get_objs_in_scene(&mut self) -> Vec<(&super::game_object::GameObject,i32,i32)> {

        let mut proximity_blocks:Vec<(&super::game_object::GameObject,i32,i32)> = Vec::new();

        let world = self.world.unwrap();
        let block = self.get_block().unwrap();
        if self.is_edge_condition(&block){
            return Vec::new()
        }

        for m in 0..3 {
            for n in 0..3 {
                world.block_map[(block.1 + 1- m) as usize][(block.0+1-n) as usize].
                object.iter()
                .for_each(|x| {
                    proximity_blocks.push((&x.0,x.1-self.x,x.2-self.y))
                });
            }
        }
            proximity_blocks
    }


    //  Checks if the given block is in the edge
    pub fn is_edge_condition(&mut self,val:&(i32,i32))->bool{
        if (val.0==0 ||val.0 == self.world.unwrap().block_map[0].len() as i32)||
        (val.1==0 || val.1 ==self.world.unwrap().block_map.len() as i32){
            return true
        }else{
            return false
        }
    }


    // scheduled for testing
    // inefficient need to be faster but will suffice for now
    pub fn get_objs_in_scene_test(&mut self)->Vec<(&super::game_object::GameObject,i32,i32)>{
            let mut out:Vec<(&super::game_object::GameObject,i32,i32)>=Vec::new();
            let blocks_max=self.world.unwrap().get_max_blocks();
            if let Some(val)=self.get_adj_blocks()
            {
            val.iter().for_each(|x|{
                self.world.unwrap().block_map[x.0 as usize ][x.1 as usize]
                .object
                .iter()
                .for_each(|y|{
                    out.push((&y.0,self.x-y.1,self.y-y.2))
                });
            });
            }
            else{
                if self.x<0{
                    self.x=0
                }
                if self.x>blocks_max.0{
                    self.x=blocks_max.0
                }
                if self.y<0{
                    self.y=0
                }
                if self.y>blocks_max.1{
                    self.y=blocks_max.1
                }
            }
            out
    }


    //scheduled for testing
    pub fn get_adj_blocks(&mut self )->Option<Vec<(i32,i32)>>{
        let max_block=self.world.unwrap().get_max_blocks();
        let mut out:Vec<(i32,i32)>=Vec::new();
        let block=self.get_block().unwrap();
        for m in 0..3{
                for  n in 0..3{
                    if (block.1+1-m,block.0+1-n) >= (0,0) && (max_block.0-(block.1+1-m),max_block.1-(block.0+1-n)) >= (0,0){
                        out.push((block.1+1-m,block.0+1-m))
                    }
                }
        }


        if out.is_empty(){
            None
        }
        else{
        Some(out)
        }
    }





    pub fn render_scene_objects(&mut self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.get_objs_in_scene().iter().for_each( |vec_obj| {
            //rendering filled rects in place of static gameobjects

            canvas.set_draw_color(Color::RGB(255, 210, 0));
            canvas.fill_rect(Rect::new(vec_obj.1, vec_obj.2, 20, 20)).unwrap();
        });
    }


    pub fn print_params(&mut self){

                println!("The coordinates are : ({:?},{:?})",self.x,self.y );
                println!("The objects are :");
                println!("{:?}",self.get_objs_in_scene().iter().map(|x| {(x.1,x.2)}).collect::<Vec<(i32,i32)>>());
                println!("The block coordinates are : ({:?})",self.get_block() );
                println!("The world width and height are: {:?}",(self.world.unwrap().block_width,self.world.unwrap().block_height) );

    }
}
