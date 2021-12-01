use crate::{ray::Ray, hitable::HitRecord};
use crate::vec3::{Color};


pub trait material {
    fn scatter(ray: &Ray, hit_record: &mut HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool;
}

struct MaterialDiffuse {}

impl material for MaterialDiffuse {
    fn scatter(ray: &Ray, hit_record: &mut HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool {
        false
    }
}