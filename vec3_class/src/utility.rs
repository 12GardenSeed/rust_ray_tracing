extern crate rand;
use rand::Rng;

pub mod Utility {
    use rand::Rng;

    #[inline]
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }
    #[inline]
    pub fn get_pi() -> f64 { std::f64::consts::PI }

    pub fn get_random_f64() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen::<f64>()
    }
    
    pub fn get_random_range_f64(min:f64, max:f64) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
    }
}