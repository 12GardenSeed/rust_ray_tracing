mod vec3;
mod ray;
mod hitable;
mod game_objects;
mod utility;
mod camera;
use game_objects::{GameObject, GameObjectTrait, Sphere};
use hitable::HitRecord;
use vec3::{Color, Point3, Vec3H, write_color, dot};
use std::{env, fs, rc::Rc};
// use crate::utility::Utility;
use ray::Ray;

use crate::camera::Camera;
// use std::io;
static FILE_NAME:&str = "ray.ppm";
static RADIUS_PIEXL: usize = 1;

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

fn ray_color(ray:&Ray, objects:&Vec::<Rc<dyn GameObjectTrait>>) -> Color {
    for v in  objects {
        let game_obj = v.as_ref();
        let mut hit_record = HitRecord::new_default();
        let t = game_obj.hit(&mut hit_record, &ray, 0.0, 99999.0);
        let normal = hit_record.normal.unwrap_or(Vec3H::new(0.0, 0.0, 0.0));
        if t > 0.0 {
            return Color::new(normal.x() + 1.0, normal.y() + 1.0,normal.z() + 1.0) * 0.5
        }
    }
    let unit = ray.direction.clone().unit_vec3();
    let t = (unit.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) *  (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    //  World
    let camera = Camera::default_new();
    let mut objects = Vec::<Rc<dyn GameObjectTrait>>::new();
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, -100.5, - camera.distance), 100.0)
        )
    );
    let IMAGE_HEIGHT:i32 = f64::floor(camera.image_width as f64 / camera.aspect_ratio as f64) as i32;
    println!("output image\nname:{}\nwidth:{}\nheight:{} ",FILE_NAME, camera.image_width, IMAGE_HEIGHT);
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(FILE_NAME);

    // Image
    let mut first_iter_data = vec![Vec::<Point3>::new(); camera.image_width as usize];
    let mut ss = format!("P3\n{} {} \n255\n", camera.image_width, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT as usize {
        for i in 0..camera.image_width as usize {
            let j = IMAGE_HEIGHT as usize - j - 1;
            let u = i as f64 / (camera.image_width - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = camera.get_ray(u, v);
            first_iter_data[i].push(ray_color(&ray, &objects));
        }
    }
    for j in 0..IMAGE_HEIGHT as usize {
        for i in 0..camera.image_width as usize {
            // let j = IMAGE_HEIGHT as usize - j- 1;
            let mut sum_vec = Vec3H::new(0.0, 0.0, 0.0);
            for dx in 0.. 2 * RADIUS_PIEXL + 1 {
                for dy in 0..2 * RADIUS_PIEXL + 1{
                    sum_vec += first_iter_data[clamp_i32(i as i32 + dx as i32 - RADIUS_PIEXL as i32, 0 , camera.image_width - 1)][clamp_i32(j as i32 + dy as i32 - RADIUS_PIEXL as i32, 0 , IMAGE_HEIGHT - 1)].clone() /  ((2 * RADIUS_PIEXL + 1) as f64).powi(2);
                }
            }
            let ss2 = format!("{}\n", write_color(sum_vec));
            ss.push_str(&ss2);
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();

}