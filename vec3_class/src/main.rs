mod vec3;
mod ray;
use vec3::{Color, Point3, Vec3H, write_color, dot};
use std::{fs, env};
use ray::Ray;
// use std::io;
static IMAGE_WIDTH:i32 = 256i32;
static ASPECT_RATIO:f64 = 1.0;
static FILE_NAME:&str = "ray.ppm";
fn main_1() {
    let IMAGE_HEIGHT = 256;
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("test.ppm");
    let mut ss = format!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{:?}", current_dir);
    for i in 0..IMAGE_WIDTH as usize {
        for j in 0..IMAGE_HEIGHT as usize {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = 0.25;
            let b = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let ir = (255.999 * r ) as i32;
            let ig = (255.999 * g ) as i32;
            let ib = (255.999 * b ) as i32;
            let ss2 = format!("{} \n", write_color(Color::new(i as f64 / (IMAGE_WIDTH - 1) as f64, 0.25, j as f64 / (IMAGE_HEIGHT - 1) as f64)));
            ss.push_str(&ss2);
            // fs::write(current_dir.to_owned(), ss).unwrap();
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();
}

fn ray_color(ray:&Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 1.0)
    }
    let unit = ray.direction.clone().unit_vec3();
    let t = (unit.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) *  (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point3, redius: f64, r: &Ray) -> bool {
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&r.direction, &(r.origin.clone() - center.clone()));
    let c = dot(&(r.origin.clone() - center.clone()), &(r.origin.clone() - center.clone())) - redius * redius;
    let delta = b * b - 4.0 * a * c;
    delta >= 0.0
}

fn main() {
    let IMAGE_HEIGHT:i32 = f64::floor((IMAGE_WIDTH as f64 /ASPECT_RATIO)) as i32;
    println!("output image\nname:{}\nwidth:{}\nheight:{} ",FILE_NAME, IMAGE_WIDTH, IMAGE_HEIGHT);
    let camera = Point3::new(0.0, 0.0, 0.0);
    let light = Point3::new(0.0 , 0.0, 0.0);
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(FILE_NAME);

    // Image
    let horizontal = Vec3H::new(2.0 / ASPECT_RATIO, 0.0, 0.0);
    let vertical = Vec3H::new(0.0, 2.0, 0.0);
    let left_lower_corner =  camera.clone() - horizontal.clone() / 2.0 - vertical.clone() / 2.0 - Point3::new(0.0, 0.0, 1.0);

    let mut ss = String::new();
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
            let ss2 = format!("{}\n", write_color(ray_color(&ray)));
            ss.push_str(&ss2);
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();
}