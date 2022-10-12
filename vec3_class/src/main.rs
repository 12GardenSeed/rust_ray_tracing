mod vec3;
mod ray;
mod hitable;
mod game_objects;
mod utility;
mod camera;
mod material;
use game_objects::{GameObject, GameObjectTrait, Sphere};
use hitable::HitRecord;
use vec3::{Color, Point3, Vec3H, write_color, dot, random_in_unit_sphere};
use std::{env, fs, rc::Rc};
use utility::Utility;
use ray::Ray;
use material::{DiffuseMaterial, SmoothMaterial, FuzzyMetal};

use crate::camera::Camera;
// use std::io;
static FILE_NAME:&str = "ray.ppm";
static RADIUS_PIEXL: usize = 1;
static SAMPLING_COUNT: usize = 100;
static MAX_DEPTH:usize = 50;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    assert!(min < max);
    if x < min { return min }
    else if x > max { return max }
    x
}

fn clamp_i32(x: i32, min: i32, max: i32) -> usize {
    assert!(min < max && min >= 0);
    if x < min { return min  as usize}
    else if x > max { return max as usize} 
    x as usize
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

fn main() {
    //  World
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0),16.0 / 9.0,2.0,1024, 1.0);
    let mut objects = Vec::<Rc<dyn GameObjectTrait>>::new();
    let material_center = Rc::new(DiffuseMaterial::new(1.0, Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(SmoothMaterial::new(Color::new(0.8, 0.8, 0.8)));
    let material_left2 = Rc::new(DiffuseMaterial::new(1.0, Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(SmoothMaterial::new(Color::new(0.8, 0.6, 0.2)));
    let material_right2 = Rc::new( FuzzyMetal::new(0.5, Color::new(0.8, 0.6, 0.2)));
    let material_ground = Rc::new(DiffuseMaterial::new(1.0, Color::new(0.8, 0.8, 0.)));
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, -100.5, - camera.distance), 100.0, Some(material_ground))
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Some(material_center))
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(-1.0, -0.0, -1.0), 0.5, Some(material_left))
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(material_right2))
        )
    );
    let image_height:i32 = f64::floor(camera.screen_width as f64 / camera.aspect_ratio as f64) as i32;
    println!("output image\nname:{}\nwidth:{}\nheight:{} ",FILE_NAME, camera.screen_width, image_height);
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(FILE_NAME);

    // Image
    let mut ss = format!("P3\n{} {} \n255\n", camera.screen_width, image_height);
    
    for j in 0..image_height as usize {
        println!("ver:{}", j);
        for i in 0..camera.screen_width as usize {
            let mut sum_vec = Vec3H::default();
            for t in 0..SAMPLING_COUNT {
                let j = image_height as usize - j - 1;
                let rand_x = (Utility::get_random_range_f64(-1.0, 1.0) * (RADIUS_PIEXL + 1) as f64).round() as i32  + i as i32;
                let rand_y = (Utility::get_random_range_f64(-1.0, 1.0) * (RADIUS_PIEXL + 1) as f64).round() as i32  + j as i32;
                let u = clamp_i32(rand_x as i32, 0 , camera.screen_width - 1) as f64 / (camera.screen_width - 1) as f64;
                let v = clamp_i32(rand_y as i32, 0 , image_height - 1) as f64 / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                let add = ray_color(&ray, &objects, MAX_DEPTH);
                sum_vec += add;
            }
            sum_vec /= SAMPLING_COUNT as f64;
            let ss2 = format!("{}\n", write_color(sum_vec));
            ss.push_str(&ss2);
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();
}