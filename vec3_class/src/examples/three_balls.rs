mod vec3;
mod ray;
mod hitable;
mod game_objects;
mod utility;
mod camera;
mod material;
use game_objects::{GameObject, GameObjectTrait, Sphere};
use hitable::{HitRecord, ray_color};
use vec3::{Color, Point3, Vec3H, write_color, dot, random_in_unit_sphere};
use std::{env, fs, rc::Rc};
use utility::Utility;
use ray::Ray;
use material::{DiffuseMaterial, SmoothMaterial, FuzzyMetal, Dielectric};

use crate::camera::Camera;
// use std::io;
static FILE_NAME:&str = "ray.ppm";
static RADIUS_PIEXL: usize = 2;
static SAMPLING_COUNT: usize = 500;
static MAX_DEPTH:usize = 50;

fn main() {
    //  World
    // let camera = Camera::new(Point3::new(0.0, 0.0, 0.0),Point3::new(0.0, 0.0, -1.0), 16.0 / 9.0,2.0,400, 1.0);
    // let camera = Camera::new_fov(Point3::new(-2.0, 2.0, 1.0), Point3::new(0.0, 0.0, -1.0), Point3::new(0.0, 1.0, 0.0), 90.0, 16.0 / 9.0);
    let camera = Camera::new_fov(Point3::new(-2.0, 2.0, 1.0), Point3::new(0.0, 0.0, -1.0), Point3::new(-1.0, 1.0, 0.0), 90.0, 16.0 / 9.0);
    let mut objects = Vec::<Rc<dyn GameObjectTrait>>::new();
    let material_center = Rc::new(DiffuseMaterial::new(0.1, Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(SmoothMaterial::new(Color::new(0.8, 0.8, 0.8)));
    let material_left2 = Rc::new(DiffuseMaterial::new(1.0, Color::new(0.8, 0.8, 0.8)));
    let material_left3 = Rc::new(Dielectric::new(8.0));
    let material_right = Rc::new(SmoothMaterial::new(Color::new(0.8, 0.6, 0.2)));
    let material_right2 = Rc::new( FuzzyMetal::new(0.5, Color::new(0.8, 0.6, 0.2)));
    let material_ground = Rc::new(DiffuseMaterial::new(1.0, Color::new(0.8, 0.8, 0.)));
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, -100.5, -camera.distance), 100.0, Some(material_ground))
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Some(material_center))
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(-1.0, -0.0, -1.0), -0.4, Some(material_left3.clone()))
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(-1.0, -0.0, -1.0), 0.5, Some(material_left3))
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
                let rand_x = (Utility::get_random_range_f64(-1.0, 1.0) * (RADIUS_PIEXL + 1) as f64)  + i as f64;
                let rand_y = (Utility::get_random_range_f64(-1.0, 1.0) * (RADIUS_PIEXL + 1) as f64)  + j as f64;
                let u = Utility::clamp_f64(rand_x, 0f64, (camera.screen_width - 1) as f64) / (camera.screen_width - 1) as f64;
                let v = Utility::clamp_f64(rand_y, 0f64 , (image_height - 1) as f64) / (image_height - 1) as f64;
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