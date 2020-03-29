use crate::core::components;

//#[derive(Clone)]
pub struct Entity{
    pub name:String,
    pub state:String,
    pub physics:super::core::components::physics::Physics,
    pub animation:components::sprite::Sprite,
    //behaviour that entity does that depends/ affects only it
    pub behaviour:Vec<fn(&mut Entity)>,

    //behaviour that entity does that affects it and other objects
    pub interactive_behaviour:Vec<fn(&mut Entity,Vec<&mut Entity>)>

}

impl Entity{
    pub fn new(name:String)->Entity{
        Entity{
            name:name,
            state:"".to_owned(),
            physics:components::physics::Physics{
                x:500,
                y:500
            },
            animation:components::sprite::Sprite::new(),

            //change to hashmap
            behaviour:Vec::new(),
            interactive_behaviour:Vec::new()
        }
    }

    pub fn load_many(&mut self,f:Vec<fn(&mut Entity)>){
        for m in f.iter(){
            self.behaviour.push(*m)
        }
    }

    pub fn load(&mut self,f:fn(&mut Entity)){
        self.behaviour.push(f)
    }

    pub fn pop_val(&mut self){
        self.behaviour.pop();
    }

    pub fn execute(&mut self){
        for m in 0..self.behaviour.len(){
            self.behaviour[m](self)
        }
    }

    pub fn exec_one(&mut self,num:usize){
        self.behaviour[num](self)
    }

    pub fn change_state(&mut self,state:String){
        self.animation.change_state(state.to_owned());
        self.state=state;
    }


    pub fn load_interactive(&mut self,f:fn(&mut Entity,Vec<&mut Entity>)){
        self.interactive_behaviour.push(f)
    }


    //This is to execute instructions based on things in the outside world
    pub fn inter_exec(&mut self,val:&mut Entity){
        self.interactive_behaviour[0](self,vec![val])
    }


}
