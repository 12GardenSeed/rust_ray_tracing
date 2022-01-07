use crate::{ray::Ray, hitable::HitRecord};
use crate::vec3::{Color};


pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool;
}

struct MaterialDiffuse {}

impl Material for MaterialDiffuse {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool {
        false
    }
}