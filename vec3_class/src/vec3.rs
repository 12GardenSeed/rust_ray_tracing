use std::{fs, env};
use crate::utility::utility;
#[derive(Clone, Debug)]
pub struct Vec3H {
    values:Vec<f64>,
}

impl Vec3H {
    #[inline]
    pub fn x(&self) -> f64 {
        self.values[0]
    }

    #[inline]
    pub fn new(x: f64, y:f64, z:f64) -> Self {
        Vec3H {
            values: vec![x,y,z]
        }
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.values[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.values[2]
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]  
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn unit_vec3(&self) -> Vec3H {
        self.clone() / self.length()
    }
}

impl std::ops::Add<Vec3H> for Vec3H {
    type Output = Vec3H;
    #[inline]
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

impl std::ops::Sub<Vec3H> for Vec3H {
    type Output = Vec3H;
    #[inline]
    fn sub(self, _rhs: Vec3H) -> Vec3H {
        let mut v = vec![];
        v.push(self.x() - _rhs.x());
        v.push(self.y() - _rhs.y());
        v.push(self.z() - _rhs.z());
        Vec3H {
            values:v
        }
    }
}


impl std::ops::AddAssign for Vec3H {
    #[inline]
    fn add_assign(&mut self, _rhs: Vec3H) {
        self.values[0] = self.x() + _rhs.x();
        self.values[1] = self.y() + _rhs.y();
        self.values[2] = self.z() + _rhs.z();
    }
}

impl std::ops::Index<usize> for Vec3H {
    type Output = f64;
    #[inline]
    fn index(&self,index: usize) -> &Self::Output {
        match index {
            x if x <= 2 => { &self.values[x] }
            _ => {panic!("try index vec3 index more than 2" );}
        }
    } 
}

impl std::ops::IndexMut<usize> for Vec3H {
    #[inline]
    fn index_mut(&mut self, index:usize) -> &mut f64 {
        match index {
            x if x <= 2 => { &mut self.values[x] }
            _ => {panic!("try mut index vec3 index more than 2" );}
        }
    }
}

impl std::ops::Mul<f64> for Vec3H {
    type Output = Vec3H;
    #[inline]
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
    #[inline]
    fn mul_assign(&mut self, _rsh:f64) {
        self[0] = self[0] * _rsh;
        self[1] = self[1] * _rsh;
        self[2] = self[2] * _rsh
    }
}

impl std::ops::Div<f64> for Vec3H {
    type Output = Vec3H;
    #[inline]
    fn div(self, _rsh: f64) -> Self::Output {
        Vec3H {
            values:vec![
                self[0] / _rsh,
                self[1] / _rsh,
                self[2] / _rsh
            ]
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3H {
    #[inline]
    fn div_assign(&mut self, _rsh: f64) {
        self.values = self.values.iter().map(|&x| x /  _rsh).collect::<Vec<f64>>()
    }
}

impl std::fmt::Display for Vec3H {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

pub fn cross(u: &Vec3H, v: &Vec3H) -> Vec3H {
    Vec3H::new(
        u[1] * v[2] - u[2] - v[1],
        u[2] * v[0] - u[0] - v[2],
        u[0] * v[1] - u[1] - v[0]
    )
}

pub fn dot(u: &Vec3H, v: &Vec3H) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

#[inline]
pub fn random() -> Vec3H {
    Vec3H::new(utility::get_random_range_f64(-1.0, 1.0), utility::get_random_range_f64(-1.0, 1.0), utility::get_random_range_f64(-1.0, 1.0))
}

pub fn random_in_unit_sphere() -> Vec3H {
    loop {
        let r = random();
        if r.length() < 1.0 {
            return r
        }
    }
}


fn to_255(f: f64) -> i32 {
    if f > 1.0 {
        return 255
    }
    (255.999 * f) as i32
}

pub fn write_color(c: Color) -> String {
    let cc = Vec3H::new(c[0] * c[0],c[1] * c[1], c[2] * c[2]);
    format!("{} {} {}", to_255(cc[0]), to_255(cc[1]), to_255(cc[2]))
}

pub type Color = Vec3H;
pub type Point3 = Vec3H;