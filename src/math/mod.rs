//Vector2 module to maipulate points in 2d

use std::ops::{Add, Sub};
use core::cmp::Ordering::Equal;

#[derive(Debug,Copy,Clone)]
pub struct  Vector2 {
    pub x : f32,
    pub y : f32,
}

impl Vector2 {
    pub fn new(x : f32, y : f32) -> Vector2 {
        Vector2 {
            x : x,
            y : y,
        }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    pub fn up() -> Vector2 {
        Vector2::new(0.0, 1.0)
    }

    pub fn right() -> Vector2 {
        Vector2::new(1.0, 0.0)
    }

    pub fn scale(&self, scalar : f32) -> Vector2 {
        Vector2{ x : self.x * scalar, y : self.y * scalar}
    }

    pub fn dot(&self, other : &Vector2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    //cross product in the k direction
    pub fn cross_self(&self, other : &Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn modulus(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let mag = self.modulus();
        Vector2::new(self.x / mag, self.y / mag)
    }

    pub fn find_angle_rel(&self, other : &Self) -> f32 {
        (self.dot(other) / (self.modulus() * other.modulus())).acos()
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Self {
        Self{
            x:self.x+other.x,
            y:self.y+other.y
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x : self.x - other.x,
            y : self.y - other.y
        }
    }
}

//checks whether two vectors are equal
impl PartialEq for Vector2{
    fn eq(&self, other : &Vector2) -> bool {
        self.x == other.x && self.y == other.y
     }
}
impl Eq for Vector2{ }

impl Ord for Vector2{
    fn cmp(&self, other : &Self) -> std::cmp::Ordering {
        ((self.x * self.x + self.y * self.y)).partial_cmp(&((other.x * other.x + other.y * other.y))).unwrap_or(Equal)
    }
}

//checks if one is greater than the other
impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}