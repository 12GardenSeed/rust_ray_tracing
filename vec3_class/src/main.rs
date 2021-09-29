use std::io;
use std::{fs, env};

pub struct Vec3H {
    values:Vec<f64>,
}

impl Vec3H {
    fn x(&self) -> f64 {
        self.values[0]
    }
    fn y(&self) -> f64 {
        self.values[1]
    }
    fn z(&self) -> f64 {
        self.values[2]
    }
}

impl std::ops::Add<Vec3H> for Vec3H {
    type Output = Vec3H;
    fn add(self, _rhs: Vec3H) -> Vec3H {
        let mut v = vec![];
        v.push(self.x() + _rhs.x());
        v.push(self.y() + _rhs.y());
        v.push(self.z() + _rhs.z());
        Vec3H {
            values:v
        }
    }
}

impl std::ops::AddAssign for Vec3H {
    fn add_assign(&mut self, _rhs: Vec3H) {
        self.values[0] = self.x() + _rhs.x();
        self.values[1] = self.y() + _rhs.y();
        self.values[2] = self.z() + _rhs.z();
    }
}

impl std::ops::Index for Vec3H {
    type Output = f64;
    fn index(index: usize<0..3>) -> &self::Output {
    } 
}

fn main() {
    
}
