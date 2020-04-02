pub struct CollisionRect {
    pub x : f32,
    pub y : f32,
    pub width : f32,
    pub height : f32,
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

    pub fn is_colliding_with(&mut self, other : &mut CollisionRect) -> bool {

        let self_top = self.y - (self.height / 2.0);
        let self_down = self.y + (self.height / 2.0);
        let self_left = self.x - (self.width / 2.0);
        let self_right = self.x + (self.width / 2.0);

        let other_top = other.y - (other.height / 2.0);
        let other_down = other.y + (other.height / 2.0);
        let other_left = other.x - (other.width / 2.0);
        let other_right = other.x + (other.width / 2.0);

        if self_right < other_left || self_left > other_right {
            return false;
        }

        if self_top > other_down || self_down < other_top {
            return false;
        }

        return true;
    }

}
