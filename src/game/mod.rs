use super::test_object;
pub mod game_states;

pub struct Game{
    exec_queue:Vec<usize>,
    state:String,
    pub entities:Vec<test_object::Entity>,
    states:Vec<String>
}

impl Game{
    pub fn new ()->Game{
        Game{
        exec_queue:Vec::new(),
        state:"zero".to_owned(),
        entities:Vec::new(),
        states:Vec::new()
    }
    }

    pub fn load_entity(&mut self,ent:test_object::Entity){
        self.entities.push(ent)
    }

    pub fn exec_all(&mut self){
        for ent in &mut self.entities{
            ent.execute()
        }
    }


}
