use crate::{Ray, material::{Material, self}};
// use crate::hitable::HitRecord;
// use crate::ray::Ray;
// use std::rc::Rc;
use std::{env, fs, rc::Rc};
use crate::game_objects::{GameObject, GameObjectTrait, Sphere};
use crate::vec3::{Color, Point3, Vec3H, write_color, dot, random_in_unit_sphere};
use crate::utility::Utility;
use material::{DiffuseMaterial, SmoothMaterial, FuzzyMetal, Dielectric};


pub trait Hitable {
    fn hit(&self, hit_record:&mut HitRecord, ray: &Ray,t_min: f64, t_max: f64) -> f64;
}


pub struct HitRecord {
    pub point: Option<Point3>,
    pub normal: Option<Vec3H>,
    pub material: Option<Rc<dyn Material>>,
    pub t: Option<f64>,
    pub front_face:bool,
}

impl HitRecord {
    pub fn new(p:Point3, n: Vec3H, t: f64,material:Rc<dyn Material>) -> HitRecord {
        HitRecord {
            point: Some(p),
            normal:Some(n),
            t:Some(t),
            material: Some(material),
            front_face:false
        }
    }
    pub fn new_default() -> HitRecord {
        HitRecord {
            point: None::<Point3>,
            normal:None::<Vec3H>,
            t:None::<f64>,
            material: None::<Rc<dyn Material>>,
            front_face:false
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3H) {
        self.front_face = dot(&r.direction, outward_normal) < 0f64;
        self.normal = if self.front_face { Some(*outward_normal) } else { Some(*outward_normal * -1.0) }
    }
}


fn unit_random_sphere() -> Vec3H {
    random_in_unit_sphere().unit_vec3()
}

pub fn ray_color(ray:&Ray, objects:&Vec::<Rc<dyn GameObjectTrait>>, depth:usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut min_t = f64::MAX;
    let mut min_index = 0;
    for i in  0..objects.len() {
        let game_obj = objects[i].as_ref();
        let mut hit_record = HitRecord::new_default();
        game_obj.hit(&mut hit_record, &ray, 0.001, 99999.0);
        if let Some(t) = hit_record.t {
            if min_t > t {
                min_t = t;
                min_index = i;
            }
        }
    }
    if min_t < f64::MAX {
        let game_obj = objects[min_index].as_ref();
        let mut hit_record = HitRecord::new_default();
        let t = game_obj.hit(&mut hit_record, &ray, 0.001, 99999.0);
        // let normal = hit_record.normal.unwrap_or(Vec3H::new(0.0, 0.0, 0.0));
        // let hit_point = hit_record.point.unwrap_or(Vec3H::new(0.0, 0.0, 0.0));
        let normal = hit_record.normal.unwrap();
        let hit_point = hit_record.point.unwrap();
        if t > 0f64 {
            let mut target = Ray::default();
            let mut attenuation = Color::default();
            if let Some(material_rc) = game_obj.get_bind_material() {
                let material = material_rc.as_ref();
                material.scatter(&ray, &mut hit_record, &mut attenuation, &mut target);
            } else {
                target.change(&hit_point, &(normal + unit_random_sphere()));
            }
            // assert!(!v.is_in_object(&(target.origin +  target.direction)));
            return ray_color(&target, objects, depth - 1) * attenuation;
        }
    }
    let unit = ray.direction.unit_vec3();
    let t = (unit.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) *  (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
