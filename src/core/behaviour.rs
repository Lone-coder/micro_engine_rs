//just a example to show behaviour implementation
use std::collections::HashMap;

pub struct Entity {
    behaviours : HashMap<String, fn(&mut Entity)>,
}

impl  Entity {
    pub fn new() -> Entity {
        Entity {
            behaviours : HashMap::new(),
        }
    }

    pub fn add_behaviour(&mut self, behaviuor_name : String, func : fn(&mut Entity))
    {
        self.behaviours.insert(behaviuor_name, func);
    }

    pub fn behave(&mut self, behaviuor_name : String)
    {
        (self.behaviours.get(&behaviuor_name).unwrap())(self);
    }
}
