use super::collision_rect::CollisionInfo;
use super::collision_rect::CollisionRect;
use crate::math::Vector2;



fn abs(x : f32) -> f32 {
    if x >= 0.0 {
        return x;
    }
    else {
        return -x;
    }
}


// change detect collision
pub fn debug_detect_collison(a : &CollisionRect, b : &CollisionRect) -> CollisionInfo{
    let mut coll_info = CollisionInfo {
            normal : Vector2::new(0.0, 0.0),
            penetration_depth : 0.0,
            collided : false
        };

        let x_diff=a.x-b.x;     //n.x
        let y_diff=a.y-b.y;     //n.y
        let x_penetration=x_diff-(a.width+b.width)/2.0;
        let y_penetration=y_diff-(a.width+b.width)/2.0;

        if x_penetration<0.0 && y_penetration<0.0{
            coll_info.collided=true;
            if x_penetration<y_penetration{
                coll_info.normal=Vector2::new(-1.0*x_diff.signum(),0.0);
            }else{
                coll_info.normal=Vector2::new(0.0,-1.0*y_diff.signum());
            }
        }
        coll_info
}








pub fn detect_collison(a : &CollisionRect, b : &CollisionRect) -> CollisionInfo {
    let a_top = a.y + (a.height / 2.0);
    let a_down = a.y - (a.height / 2.0);
    let a_left = a.x - (a.width / 2.0);
    let a_right = a.x + (a.width / 2.0);

    let b_top = b.y + (b.height / 2.0);
    let b_down = b.y - (b.height / 2.0);
    let b_left = b.x - (b.width / 2.0);
    let b_right = b.x + (b.width / 2.0);

    let mut coll_info = CollisionInfo {
                            normal : Vector2::new(0.0, 0.0),
                            penetration_depth : 0.0,
                            collided : false
                        };

    let n = Vector2::new(b.x, b.y) - Vector2::new(a.x, a.y);

    let ax_extent = (a_right - a_left) / 2.0;
    let bx_extent = (b_right - b_left) / 2.0;

    let x_overlap = ax_extent + bx_extent - abs(n.x);
    //println!("x_overlap : {:?}", x_overlap);

    if x_overlap > 0.0 {
        let ay_extent = (a_top - a_down) / 2.0;
        let by_extent = (b_top - b_down) / 2.0;

        let y_overlap = ay_extent + by_extent - abs(n.y);
        //println!("y_overlap : {:?}", y_overlap);

        if y_overlap > 0.0 {
            if x_overlap > y_overlap {
                if n.x > 0.0 {
                    coll_info.normal = Vector2::new(-1.0, 0.0);
                }
                else {
                    coll_info.normal = Vector2::new(1.0, 0.0);
                }

                coll_info.penetration_depth = x_overlap;
                coll_info.collided = true;
            }
            else {
                if n.y < 0.0 {
                    coll_info.normal = Vector2::new(0.0, -1.0);
                }
                else {
                    coll_info.normal = Vector2::new(0.0, 1.0);
                }

                coll_info.penetration_depth = y_overlap;
                coll_info.collided = true;
            }

        }

    }
    //println!("{:?}", coll_info.collided);

    coll_info
}
