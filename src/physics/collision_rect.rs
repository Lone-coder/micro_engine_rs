use crate::math::Vector2;

//collision rect to collision detection
#[derive(Debug)]
pub struct CollisionRect {
    pub x : f32,
    pub y : f32,
    pub width : f32,
    pub height : f32,
}

//struct to store info about a AABB collision
pub struct CollisionInfo {
    pub normal: Vector2,
    pub penetration_depth : f32,
    pub collided : bool,
}

impl CollisionRect {

    pub fn new(x : f32, y : f32, width : f32, height : f32) -> CollisionRect{
        CollisionRect {
            x : x,
            y : y,
            width : width,
            height : height,
        }
    }

    //Basic AABB collision test
    pub fn is_colliding_with(&self, other : &CollisionRect) -> bool {

        let self_top = self.y + (self.height / 2.0);
        let self_down = self.y - (self.height / 2.0);
        let self_left = self.x - (self.width / 2.0);
        let self_right = self.x + (self.width / 2.0);

        let other_top = other.y + (other.height / 2.0);
        let other_down = other.y - (other.height / 2.0);
        let other_left = other.x - (other.width / 2.0);
        let other_right = other.x + (other.width / 2.0);

        if self_right < other_left || self_left > other_right {
            return false;
        }

        if self_top < other_down || self_down > other_top {
            return false;
        }

        return true;
    }

}

fn abs(x : f32) -> f32 {
    if x >= 0.0 {
        return x;
    }
    else {
        return -x;
    }
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
