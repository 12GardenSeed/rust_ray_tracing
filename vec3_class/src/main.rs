use std::io;
use std::{fs, env};

struct Vec3H {
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

impl std::ops::Index<usize> for Vec3H {
    type Output = f64;
    fn index(&self,index: usize) -> &Self::Output {
        match index {
            x if x >= 0 && x <= 2 => { &self.values[x] }
            _ => {panic!("try index vec3 index more than 2" );}
        }
    } 
}

impl std::ops::IndexMut<usize> for Vec3H {
    fn index_mut(&mut self, index:usize) -> &mut f64 {
        match index {
            x if x >= 0 && x <= 2 => { &mut self.values[x] }
            _ => {panic!("try mut index vec3 index more than 2" );}
        }
    }
}

impl std::ops::Mul<f64> for Vec3H {
    type Output = Vec3H;
    fn mul(self, _rsh: f64) -> Self::Output {
        Vec3H{
            values:vec![
                self.x() * _rsh,
                self.y() * _rsh,
                self.z() * _rsh,
            ]
        }
    }
}

impl std::ops::MulAssign<f64> for Vec3H {
    fn mul_assign(&mut self, _rsh:f64) {
        self[0] = self[0] * _rsh;
        self[1] = self[1] * _rsh;
        self[2] = self[2] * _rsh
    }
}


impl std::ops::Div<f64> for Vec3H {
    type Output = Vec3H;
    fn div(self, _rsh) -> Self::Output {
        Vec3H {
            values:vec![
                self[0] / _rsh,
                self[1] / _rsh,
                self[2] / _rsh
            ]
        }
    }
}

fn main() {
    
}
