use micro_engine_rs::test_object::Entity;

fn main(){

}

pub fn init(val:&mut Entity){
    let vals_up=vec![(16,0),(16,16),(16,32)];
    let vals_down=vec![(32,0),(32,16),(32,32)];
    let vals_left=vec![(0,0),(0,16),(0,32)];
    let panic=vec![(48,0),(48,16),(48,32)];
    val.animation.height=16;
    val.animation.width=16;
    val.animation.load_states("walking_up".to_string(),vals_up);
    val.animation.load_states("walking_down".to_string(),vals_down);
    val.animation.load_states("walking_left".to_string(),vals_left);
    val.animation.load_states("panic".to_string(),panic);
}

pub fn walk_left(val:&mut Entity){
    val.animation.change_state("walking_left".to_string());
    val.physics.x-=10;
}
pub fn walk_up(val:&mut Entity){
    val.animation.change_state("walking_up".to_string());
    val.physics.y-=10;
}

pub fn walk_down(val:&mut Entity){
    val.animation.change_state("walking_down".to_string());
    val.physics.y+=10;
}

pub fn panic(val:&mut Entity){
    val.animation.change_state("panic".to_string());
}


pub fn circle(val:&mut Entity){
    if val.physics.x<=100 && val.physics.y<=100{
        val.animation.change_state("walking_down".to_string());
        val.state="walking_down".to_string();
        val.physics.x=100;
        val.physics.y+=20;
    }

    else if val.physics.x<=100 && val.physics.y>=400{
        val.animation.change_state("walking_left".to_string());
        val.state="walking_right".to_string();
        val.physics.x+=20;
        val.physics.y=400;
    }

    else if val.physics.x>=400 && val.physics.y<=100{
        val.animation.change_state("walking_left".to_string());
        val.state="walking_left".to_string();
        val.physics.y=100;
        val.physics.x-=20;
    }

    else if val.physics.x>=400 && val.physics.y>=400{
        val.animation.change_state("walking_up".to_string());
        val.state="walking_up".to_string();
        val.physics.y=400;
        val.physics.y-=20;
        val.physics.x=400;
    }

    else{
        match val.state.as_str(){
            "walking_up" =>{
                println!("walking up");
                val.physics.y-=20;
            }

            "walking_right" =>{
                println!("walking right");
                val.physics.x+=20;
            }

            "walking_left"=>{
                println!("walking left");
                val.physics.x-=20
            }

            "walking_down"=>{
                println!("walking down");
                val.physics.y+=20
            }
        _=>()

        }
    }


}
