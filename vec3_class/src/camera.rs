use crate::{ray::Ray, vec3::{Point3, Vec3H, cross}, utility::Utility::degrees_to_radians};

pub struct Camera {
    pub origin: Point3,
    pub lookat:Point3,
    pub vup:Vec3H,
    pub fov:f64, // aspect of view
    pub aspect_ratio: f32,
    pub view_length: f64,
    pub screen_width: i32,
    pub distance: f64,
    pub horizontal: Vec3H,
    pub vertical: Vec3H,
    pub left_lower_corner: Point3,
}

impl Camera {
    pub fn new(origin: Point3,lookat:Point3, vup: Vec3H, fov:f64, aspect_ratio: f32, view_length: f64, screen_width: i32, distance: f64) -> Camera {
        let theta = degrees_to_radians(fov);
        let h = theta.tan();
        let horizontal =   Vec3H::new(view_length * aspect_ratio as f64, 0.0, 0.0);
        let vertical = Vec3H::new(0.0, view_length, 0.0);
        let left_lower_corner = origin - horizontal * 0.5 - vertical * 0.5 - Point3::new(0.0, 0.0, distance);
        Camera {
            origin,
            fov,
            lookat,
            vup,
            aspect_ratio,
            view_length,
            screen_width,
            distance,
            horizontal,
            vertical,
            left_lower_corner 
        }
    }

    pub fn new_fov(origin: Point3,lookat:Point3, vup: Vec3H, fov:f64, aspect_ratio: f32) -> Camera {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let view_length = viewport_height;
        let viewport_width = viewport_height * aspect_ratio as f64;
        let w = (origin - lookat).unit_vec3();
        let u = cross(&vup, &w);
        let v = cross(&u, &w);
        let screen_width = 400;
        let horizontal = v * viewport_width;
        let vertical = u * viewport_height;
        let left_lower_corner = origin - horizontal * 0.5 - vertical * 0.5 - w;
        Camera {
            origin,
            fov,
            lookat,
            vup,
            aspect_ratio,
            view_length,
            screen_width,
            distance: 1.0,
            horizontal,
            vertical,
            left_lower_corner 
        }

    }

    pub fn new_default() -> Camera {
        Camera::new(Point3::new(0.0, 0.0, 0.0),Point3::new(0.0, 0.0, 1.0),Vec3H::new(0.0,1.0,0.0),90.0, 16.0 / 9.0, 2.0, 256, 1.0)
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin, &(self.left_lower_corner + self.horizontal * u + self.vertical * v - self.origin))
    }
}