mod vec3;
mod ray;
mod hitable;
mod game_objects;
use game_objects::{GameObject, GameObjectTrait, Sphere};
use hitable::HitRecord;
use vec3::{Color, Point3, Vec3H, write_color, dot};
use std::{env, fs, rc::Rc};
use ray::Ray;
// use std::io;
static IMAGE_WIDTH:i32 = 512i32;
static ASPECT_RATIO:f64 = 16.0 / 9.0;
static FILE_NAME:&str = "ray.ppm";

fn ray_color(ray:&Ray, objects:&Vec::<Rc<dyn GameObjectTrait>>) -> Color {
    for v in  objects {
        let game_obj = v.as_ref();
        let mut hit_record = HitRecord::new_default();
        let t = game_obj.hit(&mut hit_record, &ray, 0.0, 99999.0);
        let normal = hit_record.normal.unwrap_or(Vec3H::new(0.0, 0.0, 0.0));
        if t >= 0.0 {
            return Color::new(normal.x() + 1.0, normal.y() + 1.0,normal.z() + 1.0) * 0.5
        }
    }
    let unit = ray.direction.clone().unit_vec3();
    let t = (unit.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) *  (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    let mut objects = Vec::<Rc<dyn GameObjectTrait>>::new();
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, 0.5, -1.0), 0.5)
        )
    );
    objects.push(
        Rc::new(
            Sphere::new(Point3::new(0.0, -0.4, -1.0), 0.6)
        )
    );
    let IMAGE_HEIGHT:i32 = f64::floor(IMAGE_WIDTH as f64 /ASPECT_RATIO) as i32;
    println!("output image\nname:{}\nwidth:{}\nheight:{} ",FILE_NAME, IMAGE_WIDTH, IMAGE_HEIGHT);
    let camera = Point3::new(0.0, 0.0, 0.0);
    // let light = Point3::new(0.0 , 0.0, 0.0);
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(FILE_NAME);

    // Image
    let horizontal = Vec3H::new(2.0 * ASPECT_RATIO , 0.0, 0.0);
    let vertical = Vec3H::new(0.0, 2.0 , 0.0);
    let left_lower_corner =  camera.clone() - horizontal.clone() / 2.0 - vertical.clone() / 2.0 - Point3::new(0.0, 0.0, 1.0);
    let mut ss = format!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT as usize {
        for i in 0..IMAGE_WIDTH as usize {
            let j = IMAGE_HEIGHT as usize - j - 1;
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                &camera, 
                &(left_lower_corner.clone() + horizontal.clone() * u +  vertical.clone() * v)
            );
            let ss2 = format!("{}\n", write_color(ray_color(&ray, &objects)));
            ss.push_str(&ss2);
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();
}