use micro_engine_rs::core::behaviour::Entity;

fn read(e : &mut Entity)
{
    println!("reading...");
}

fn attack(e : &mut Entity)
{
    println!("attacking");
}

fn main()
{
    let mut entity = Entity::new();
    entity.add_behaviour("Attack".to_string(), attack);
    entity.behave("Attack".to_string());

    entity.add_behaviour("Read".to_string(), read);
    entity.behave("Read".to_string());
}
