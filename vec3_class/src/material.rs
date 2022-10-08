use crate::{ray::Ray, hitable::HitRecord};
use crate::vec3::{Color, random_in_unit_sphere};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

struct MaterialDiffuse {
    reflectance: f64,
    color: Color,
}

impl Material for MaterialDiffuse {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        if let Some(normal) = hit_record.normal.clone() {
            let mut d = normal.clone() + random_in_unit_sphere().unit_vec3();
            if d.near_zero() {
                d = normal;
            }
            if let Some(point) = hit_record.point.clone() {
                scattered.change(point, d);
                attenuation.copy_other(&self.color);
            }
        }
        true
    }
}

struct SmoothMaterial {

}

impl Material for SmoothMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
}