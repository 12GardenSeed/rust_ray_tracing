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
    // let camera = Point3::new(0.0, 0.0, 0.0);
    // let light = Point3::new(0.0 , 0.0, 0.0);
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(FILE_NAME);

    // Image
    let mut ss = format!("P3\n{} {} \n255\n", camera.image_width, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT as usize {
        for i in 0..camera.image_width as usize {
            let j = IMAGE_HEIGHT as usize - j - 1;
            let u = i as f64 / (camera.image_width - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = camera.get_ray(u, v);
            let ss2 = format!("{}\n", write_color(ray_color(&ray, &objects)));
            ss.push_str(&ss2);
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();

    // test 
    // let r = Ray{
    //     origin: Point3::new(0.0, 0.0, 0.0),
    //     direction: Vec3H::new(0.0, 1.0, -1.0)
    // };
    // println!("{}", ray_color(&r, &objects));

}