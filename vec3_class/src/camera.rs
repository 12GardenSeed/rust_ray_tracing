use crate::{ray::Ray, vec3::{Point3, Vec3H}};


// static IMAGE_WIDTH:i32 = 256i32;
// static ASPECT_RATIO:f64 = 16.0 / 9.0;
// static FILE_NAME:&str = "ray.ppm";
// static VIEW_LENGTH:f64 = 2.0;
// static DISTANCE:f64 = 1.0;
pub struct Camera {
    pub origin: Point3,
    pub aspect_ratio: f32,
    pub view_length: f64,
    pub image_width: i32,
    pub distance: f64,
    pub horizontal: Vec3H,
    pub vertical: Vec3H,
    pub left_lower_corner: Point3,
}

impl Camera {
    pub fn new(origin: Point3, aspect_ratio: f32, view_length: f64, image_width: i32, distance: f64) -> Camera {
        let horizontal =   Vec3H::new(view_length * aspect_ratio as f64, 0.0, 0.0);
        let vertical = Vec3H::new(0.0, view_length, 0.0);
        let left_lower_corner = origin.clone() - horizontal.clone() * 0.5 - vertical.clone() * 0.5 - Point3::new(0.0, 0.0, distance);
        Camera {
            origin,
            aspect_ratio,
            view_length,
            image_width,
            distance,
            horizontal,
            vertical,
            left_lower_corner 
        }
    }

    pub fn default_new() -> Camera {
        Camera::new(Point3::new(0.0, 0.0, 0.0), 16.0 / 9.0, 2.0, 256, 1.0)
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin, &(self.left_lower_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v - self.origin.clone()))
    }
}