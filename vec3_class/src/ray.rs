// mod vec3;
use std::io;
use crate::vec3::Vec3H;

pub struct Ray {
    pub direction: Vec3H,
    pub origin: Vec3H
}

impl Ray {
    pub fn new(origin: &Vec3H, direction: &Vec3H) -> Ray {
        Ray{
            origin :origin.clone(),
            direction: direction.clone()
        }
    }
}