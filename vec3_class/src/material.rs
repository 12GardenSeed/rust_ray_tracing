use std::fmt::{Display, Formatter};

use crate::{ray::Ray, hitable::HitRecord};
use crate::vec3::{Color, random_in_unit_sphere, Vec3H, dot};

pub trait Material : Display{
    fn scatter(&self, ray:&Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

fn reflect(v: &Vec3H, n: & Vec3H) -> Vec3H {
    *v -  *n * (2f64 * dot(v, n))
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
    fn scatter(&self, ray:&Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
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
    fn scatter(&self, ray:&Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
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

pub struct FuzzyMetal {
    color :Color,
    fuzz :f64,
}

impl Material for FuzzyMetal {
    fn scatter(&self, ray:&Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        assert!(hit_record.normal.is_some() && hit_record.point.is_some());
        let n = hit_record.normal.unwrap();
        let d = reflect(&ray.direction, &n);
        scattered.change(&hit_record.point.unwrap(), &(d + random_in_unit_sphere() * self.fuzz));
        attenuation.copy_other(&self.color);
        true
    }
}

impl FuzzyMetal {
    pub fn new( fuzz: f64, color: Color) -> Self {
        Self { color, fuzz }
    }
}

impl Display for FuzzyMetal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FuzzyMetal {} {}",  self.color, self.fuzz))
    }
}