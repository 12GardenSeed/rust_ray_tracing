mod vec3h;
use std::io;
use crate::vec3h::Vec3H;

pub struct Ray {
    direction: Vec3H,
    origin: Vec3H
}

impl Ray {
    pub fn new(origin: &Vec3H, direction: &Vec3H) -> Ray {
        Ray{
            origin :origin.clone(),
            direction: direction.clone()
        }
    }
}