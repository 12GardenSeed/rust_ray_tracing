use std::rc::Rc;

use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::{Point3, dot};
pub trait Component {
    fn get_name(&mut self) -> String;
}

pub trait GameObjectTrait:Hitable {
    fn is_game_object(&mut self) -> bool {
        true
    }
    fn is_in_object(&self, p: &Point3) -> bool;
    fn distance(&self, p: &Point3) -> f64;
    fn bind_material(&mut self, material: Rc<dyn Material>);
    fn get_bind_material(&self) -> &Option<Rc<dyn Material>>;
}

pub struct GameObject {
    components: Vec<Box<dyn Component>>,
}

impl GameObject {
    fn new() -> GameObject {
        GameObject {
            components:vec![]
        }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(center:Point3, radius:f64, material: Option<Rc<dyn Material>>) -> Sphere {
        Sphere {
            center:center,
            radius:radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, hit_record:&mut HitRecord, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> f64 {
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&ray.direction, &(ray.origin - self.center));
        let c = dot(&(ray.origin - self.center), &(ray.origin - self.center)) - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        let res =  (- half_b - delta.sqrt()) / a; 
        if delta < 0.0 || res < t_min || res > t_max {
            return -1.0
        }
        let p = ray.at(res);
        let normal = (p - self.center).unit_vec3();
        hit_record.t = Some(res);
        hit_record.point = Some(p);
        // hit_record.normal = Some(normal);
        hit_record.set_face_normal(ray, &normal);
        hit_record.material = self.material.clone();
        delta
    }
}

impl GameObjectTrait for Sphere {
    fn is_in_object(&self, p: &Point3) -> bool {
        if self.distance(p) < 0f64 {
            println!("error inside go d {} r {} ", self.distance(p), self.radius);
        }
        self.distance(p) < 0f64
    }
    fn distance(&self, p: &Point3) -> f64 {
        let v = *p - self.center;
        v.length() - self.radius
    }

    fn is_game_object(&mut self) -> bool {
        true
    }

    fn bind_material(&mut self, material:Rc<dyn Material>) {
        self.material = Some(material)
    }

    fn get_bind_material(&self) -> &Option<Rc<dyn Material>> {
        &self.material
    }
}