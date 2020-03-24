use micro_engine_rs::math::Vector2;

fn main(){
    let a=Vector2{
        x:20.0,
        y:30.0
    };

    let b=Vector2{
        x:10.0,
        y:50.0
    };

    println!("{:?}",a+b );

    println!("{:?}",a<b );

    println!("{:?}",a==b );

    println!("{:?}",a.dot(&b));

    println!("{:?}",a.find_angle_rel(&b));
}
