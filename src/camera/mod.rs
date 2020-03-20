/// Useful info:
/// Here &'a world implies that the Camera lives atleast for as long as the world lives
/// or any reference to 'a musn't live longer than it
use super::Rect;
use super::Color;

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
            x : width + width/2,
            y : height + height/2,
            width : width,
            height : height,
            scene_objects : Vec::new(),
            world: _world
        }
    }

    pub fn display_scene_objects(&mut self){
        self.scene_objects.iter().for_each(|x| println!("{:?}",x.props ));
    }

    pub fn render_scene_objects(&mut self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.get_objs_in_scene().iter().for_each( |vec_obj| {
            //rendering filled rects in place of static gameobjects
            canvas.set_draw_color(Color::RGB(255, 210, 0));
            canvas.fill_rect(Rect::new(vec_obj.1, vec_obj.2, 20, 20));
        });
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


    pub fn get_coord(&mut self)->(i32,i32){
        (self.x,self.y)
    }


    pub fn add_obj(&mut self,obj:&'b super::game_object::GameObject){
        self.scene_objects.push(obj)
    }

    pub fn get_block(&mut self)->Option<(i32,i32)>{
        if let Some(world)=self.world{
            Some((self.x/world.block_width,self.y/world.block_height))
        }else{
            None
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
    pub fn get_objs_in_scene(&mut self)->Vec<(&super::game_object::GameObject,i32,i32)>{
        let mut proximity_blocks:Vec<(&super::game_object::GameObject,i32,i32)> = Vec::new();
        let world=self.world.unwrap();
        let block=self.get_block().unwrap();

        for m in 0..3 {

            for n in 0..3 {
                //debug
                println!("{:?}",block.1+1-m);
                world.block_map[(block.1+ 1 -m) as usize][(block.0+1-n) as usize].

                object.iter()
                .for_each(|x| {
                    proximity_blocks.push((&x.0,x.1-self.x,x.2-self.y))
                });
            }
        }
            proximity_blocks
    }
}
