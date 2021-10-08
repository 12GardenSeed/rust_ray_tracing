mod vec3;
use vec3::{Color, Point3, write_color};
use std::{fs, env};
use std::io;
static image_width:i32 = 256i32;
static image_height:i32 = 256;
fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("test.ppm");
    let mut ss = format!("P3\n{} {} \n255\n", image_width, image_height);
    println!("{:?}", current_dir);
    for i in 0..image_width as usize {
        for j in 0..image_height as usize {
            let r = i as f32 / (image_width - 1) as f32;
            let g = 0.25;
            let b = j as f32 / (image_height - 1) as f32;
            let ir = (255.999 * r ) as i32;
            let ig = (255.999 * g ) as i32;
            let ib = (255.999 * b ) as i32;
            let ss2 = format!("{} \n", write_color(Color::new(i as f64 / (image_width - 1) as f64, 0.25, j as f64 / (image_height - 1) as f64)));
            ss.push_str(&ss2);
            // fs::write(current_dir.to_owned(), ss).unwrap();
        }
    }
    fs::write(current_dir.to_owned(), ss).unwrap();
}