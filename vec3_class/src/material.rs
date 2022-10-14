use std::fmt::{Display, Formatter};

use crate::utility::Utility;
use crate::{ray::Ray, hitable::HitRecord};
use crate::vec3::{Color, random_in_unit_sphere, Vec3H, dot};

pub trait Material : Display{
    fn scatter(&self, ray:&Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

fn reflect(v: &Vec3H, n: & Vec3H) -> Vec3H {
    *v -  *n * (2f64 * dot(v, n))
}

fn refract(uv: &Vec3H, n: &Vec3H, etai_over_etat: f64) -> Vec3H {
    let cos_theta = if dot(&(*uv * -1.0), n) > 1f64 {dot(&(*uv * -1.0), n)} else {1f64};
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_paralel =  *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_paralel + r_out_perp
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
        f.write_str(&format!("SmoothMaterial color:{}",  self.color))
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
        f.write_str(&format!("FuzzyMetal color: {} fuzz:{}",  self.color, self.fuzz))
    }
}

pub struct Dielectric {
    ir:f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1f64 - ref_idx) / (1f64 + ref_idx);
        r0 = r0 * r0;
        r0 + (1f64 - r0) * (1f64 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray:&Ray, hit_record: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        attenuation.copy_other(&Color::new(1f64, 1f64, 1f64));
        let refraction_ratio = if hit_record.front_face { 1.0 / self.ir } else { self.ir };
        let normal = hit_record.normal.unwrap();
        let unit_direction = ray.direction.unit_vec3();
        let cos_theta = if dot(&unit_direction, &unit_direction) > 1f64 { 
            dot(&(unit_direction), &unit_direction) 
        } else {
            1f64
        };
        let refracted;
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > Utility::get_random_range_f64(0.0, 1.0) {
            refracted = reflect(&unit_direction, &normal);
        } else {
            refracted = refract(&unit_direction, &hit_record.normal.unwrap(), refraction_ratio);
        }
        scattered.change(&hit_record.point.unwrap(), &refracted);
        return true
    }
}

impl Display for Dielectric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Dielectric ir: {}", self.ir))
    }
}