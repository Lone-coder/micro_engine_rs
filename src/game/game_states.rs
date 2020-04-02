use crate::entity::dynamicEntity::Entity;
use crate::math::Vector2;
use std::collections::HashMap;

pub struct State{
    coordinates:Vector2,
    entities:HashMap<String,Entity>,
    behaviour:HashMap<String,fn(&mut State)>
}
