use crate::{Ray, Vec3H, game_objects::GameObject, vec3::Point3};

pub trait hitable {
    fn hit(&self, hit_record:&mut HitRecord, ray: &Ray,t_min: f64, t_max: f64) -> f64;
}


pub struct HitRecord{
    pub point: Option<Point3>,
    pub normal: Option<Vec3H>,
    pub t: Option<f64>
}


impl HitRecord {
    pub fn new(p:Point3, n: Vec3H, t: f64) -> HitRecord {
        HitRecord {
            point: Some(p),
            normal:Some(n),
            t:Some(t)
        }
    }
    pub fn new_default() -> HitRecord {
        HitRecord {
            point: None::<Point3>,
            normal:None::<Vec3H>,
            t:None::<f64>
        }
    }
}