use std::fmt::{Display, Formatter};

use crate::{ray::Ray, hitable::HitRecord};
use crate::vec3::{Color, random_in_unit_sphere};

pub trait Material : Display{
    fn scatter(&self, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct DiffuseMaterial {
    reflectance: f64,
    color: Color,
}

impl DiffuseMaterial {
    pub fn new(reflectance: f64, color: Color) -> Self {
        Self {
            reflectance,
            color
        }
    }
}

impl Material for DiffuseMaterial {
    fn scatter(&self, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        assert!(hit_record.normal.is_some() && hit_record.point.is_some());
        let mut d = hit_record.normal.unwrap() + random_in_unit_sphere().unit_vec3();
        if d.near_zero() {
            d = hit_record.normal.unwrap();
        }
        scattered.change(&hit_record.point.unwrap(), &(d - hit_record.point.unwrap()));
        attenuation.copy_other(&self.color);
        true
    }
}
impl Display for DiffuseMaterial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("DiffuseMaterial {} {}", self.reflectance, self.color))
    }
}

pub struct SmoothMaterial {
    color: Color,
}

impl SmoothMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}

impl Material for SmoothMaterial {
    fn scatter(&self, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        assert!(hit_record.normal.is_some() && hit_record.point.is_some());
        let d = hit_record.normal.unwrap();
        scattered.change(&hit_record.point.unwrap(), &d);
        attenuation.copy_other(&self.color);
        true
    }
}

impl Display for SmoothMaterial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("SmoothMaterial {}",  self.color))
    }
}