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
// use crate::utility::utility;
use ray::Ray;

use crate::camera::Camera;
// use std::io;
static FILE_NAME:&str = "ray.ppm";
static RADIUS_PIEXL: usize = 1;
static SAMPLING_COUNT: usize = 99;
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

fn ray_color(ray:&Ray, objects:&Vec::<Rc<dyn GameObjectTrait>>, depth:usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut i = 0;
    for v in  objects {
        let game_obj = v.as_ref();
        let mut hit_record = HitRecord::new_default();
        let t = game_obj.hit(&mut hit_record, &ray, 0.001, 99999.0);
        let normal = hit_record.normal.unwrap_or(Vec3H::new(0.0, 0.0, 0.0));
        let hit_point = hit_record.point.unwrap_or(Vec3H::new(0.0, 0.0, 0.0));
        if t > 0.0 {
            let target = hit_point.clone() + normal.clone() + unit_random_sphere();
            assert!(!v.is_in_object(&target));
            return ray_color(&Ray::new(&hit_point, &(target.clone() - hit_point.clone())), objects, depth - 1) * 0.5;
        }
        i += 1;
    }
    let unit = ray.direction.clone().unit_vec3();
    let t = (unit.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) *  (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    //  World
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0),16.0 / 9.0,2.0,400, 1.0);
    let mut objects = Vec::<Rc<dyn GameObjectTrait>>::new();
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, None)
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, -100.5, - camera.distance), 100.0, None)
        )
    );
    let image_height:i32 = f64::floor(camera.image_width as f64 / camera.aspect_ratio as f64) as i32;
    println!("output image\nname:{}\nwidth:{}\nheight:{} ",FILE_NAME, camera.image_width, image_height);
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(FILE_NAME);

    // Image
    // let mut first_iter_data = vec![Vec::<Point3>::new(); camera.image_width as usize];
    let mut ss = format!("P3\n{} {} \n255\n", camera.image_width, image_height);
    // for j in 0..image_height as usize {
    //     for i in 0..camera.image_width as usize {
    //         let j = image_height as usize - j - 1;
    //         let u = i as f64 / (camera.image_width - 1) as f64;
    //         let v = j as f64 / (image_height - 1) as f64;
    //         let ray = camera.get_ray(u, v);
    //         first_iter_data[i].push(ray_color(&ray, &objects, MAX_DEPTH));
    //     }
    // }
    // for j in 0..image_height as usize {
    //     for i in 0..camera.image_width as usize {
    //         // let j = image_height as usize - j- 1;
    //         let mut sum_vec = Vec3H::new(0.0, 0.0, 0.0);
    //         for dx in 0.. 2 * RADIUS_PIEXL + 1 {
    //             for dy in 0..2 * RADIUS_PIEXL + 1{
    //                 sum_vec += first_iter_data[clamp_i32(i as i32 + dx as i32 - RADIUS_PIEXL as i32, 0 , camera.image_width - 1)][clamp_i32(j as i32 + dy as i32 - RADIUS_PIEXL as i32, 0 , image_height - 1)].clone() /  ((2 * RADIUS_PIEXL + 1) as f64).powi(2);
    //             }
    //         }
    //         // for i in 0..SAMPLING_COUNT {
    //         //     let rand_x = (Utility::get_random_range_f64(0.0, 1.0) * RADIUS_PIEXL as f64 - RADIUS_PIEXL as f64 / 2.0).floor() as usize + i;
    //         //     let rand_y = (Utility::get_random_range_f64(0.0, 1.0) * RADIUS_PIEXL as f64 - RADIUS_PIEXL as f64 / 2.0).floor() as usize + j;
    //         //     sum_vec += first_iter_data[clamp_i32(rand_x as i32, 0 , camera.image_width - 1)][clamp_i32(rand_y as i32, 0 , image_height - 1)].clone() / SAMPLING_COUNT as f64;
    //         // }
    //         let ss2 = format!("{}\n", write_color(sum_vec));
    //         ss.push_str(&ss2);
    //     }
    // }
    for j in 0..image_height as usize {
        println!("ver:{}", j);
        for i in 0..camera.image_width as usize {
            let mut sum_vec = Vec3H::new(0.0, 0.0, 0.0);
            for t in 0..SAMPLING_COUNT {
                let j = image_height as usize - j- 1;
                let add_x = t % 3;
                let add_y = (t / 3) % 3; 
                let rand_x = add_x + i;//(Utility::get_random_range_f64(0.0, 1.0) * (RADIUS_PIEXL * 2 + 1) as f64  - RADIUS_PIEXL as f64).floor() as usize + i;
                let rand_y = add_y + j;//(Utility::get_random_range_f64(0.0, 1.0) * (RADIUS_PIEXL * 2 + 1) as f64  - RADIUS_PIEXL as f64).floor() as usize + j;
                let u = clamp_i32(rand_x as i32, 0 , camera.image_width - 1) as f64 / (camera.image_width - 1) as f64;
                let v = clamp_i32(rand_y as i32, 0 , image_height - 1) as f64 / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                let add = ray_color(&ray, &objects, MAX_DEPTH);
                sum_vec += add / SAMPLING_COUNT as f64;
            }
            let ss2 = format!("{}\n", write_color(sum_vec));
            ss.push_str(&ss2);
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();
}