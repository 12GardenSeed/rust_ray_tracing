use std::{fs, env};
use std::io;

static image_width:i32 = 256i32;
static image_height:i32 = 256;
macro_rules! ff {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        format!($($arg)*)
    })
}
fn main() {
    ff!("P3\n \{} {} \n255", image_width, image_height);
    for i in 0..image_width as usize {
        for j in 0..image_height as usize {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;
            let ir = (255.999 * r ) as i32;
            let ig = (255.999 * g ) as i32;
            let ib = (255.999 * b ) as i32;
            ff!("{} {} {} ", ir, ig, ib);
        }
    }
}