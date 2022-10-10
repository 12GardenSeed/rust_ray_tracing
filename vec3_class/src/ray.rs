// mod vec3;
use crate::game_objects::GameObjectTrait;
use crate::hitable::HitRecord;
use crate::vec3::{Color, Point3, Vec3H, write_color, dot, random_in_unit_sphere};
use std::rc::Rc;

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

    pub fn at(&self, t: f64) -> Vec3H {
        self.origin + self.direction * t
    }

    pub fn change(&mut self, origin: &Vec3H, direction: &Vec3H) {
        self.origin.copy_other(origin);
        self.direction.copy_other(direction);
    }

    pub fn default() -> Self {
        Ray{
            origin : Vec3H::default(),
            direction: Vec3H::default()
        }

    }
}
