use std::rc::Rc;

use crate::hitable::{HitRecord, hitable};
use crate::{Point3, dot};
pub trait Component {
    fn get_name(&mut self) -> String;
}

pub trait GameObjectTrait:hitable {
    fn is_game_object(&mut self) -> bool {
        true
    }
}

pub struct GameObject {
    components: Vec<Rc<dyn Component>>,
}

impl GameObject {
    fn new() -> GameObject {
        GameObject {
            components:vec![]
        }
    }
}

pub struct Sphere {
    game_object:GameObject,
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center:Point3, radius:f64) -> Sphere {
        Sphere {
            game_object:GameObject::new(),
            center:center,
            radius:radius
        }
    }
}

impl hitable for Sphere {
    fn hit(&self, hit_record:&mut HitRecord, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> f64 {
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&ray.direction, &(ray.origin.clone() - self.center.clone()));
        let c = dot(&(ray.origin.clone() - self.center.clone()), &(ray.origin.clone() - self.center.clone())) - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        let res =  (- half_b + delta.sqrt()) / a; 
        if delta < 0.0 || res < t_min || res > t_max {
            return -1.0
        }

        // println!("debug hittable {} {} {} {} {}",a,half_b,c,delta,res);
        // println!("VECS: oo1 {}", ray.origin.clone() - self.center.clone());
        let p = ray.at(res);
        let mut normal = (p.clone() - self.center.clone()).unit_vec3();
        if dot( &normal, &ray.direction) < 0.0  {
            normal = normal * -1.0
        }
        hit_record.point = Some(p);
        hit_record.normal = Some(normal);
        hit_record.t = Some(delta);
        delta
    }
}

impl GameObjectTrait for Sphere {}