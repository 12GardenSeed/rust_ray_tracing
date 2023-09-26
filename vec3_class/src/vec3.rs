use crate::utility::Utility;
const NZ:f64 = 1e-8; 
#[derive(Clone, Copy, Debug)]
pub struct Vec3H {
    x:f64,
    y:f64,
    z:f64,
}

impl Vec3H {
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn new(x: f64, y:f64, z:f64) -> Self {
        Vec3H {
            x,y,z
        }
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.z
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
        *self / self.length()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        self.x() < NZ && self.y() < NZ && self.z() < NZ
    }

    #[inline]
    pub fn copy_other(&mut self, other: &Vec3H) {
        self[0] = other[0];
        self[1] = other[1];
        self[2] = other[2];
    }

    #[inline]
    pub fn default() -> Self {
        Self::new(0f64, 0f64, 0f64)
    }
}

impl std::ops::Add<Vec3H> for Vec3H {
    type Output = Vec3H;
    #[inline]
    fn add(self, _rhs: Vec3H) -> Vec3H {
        Vec3H {
            x: self.x() + _rhs.x(),
            y: self.y() + _rhs.y(),
            z: self.z() + _rhs.z()
        }
    }
}

impl std::ops::Sub<Vec3H> for Vec3H {
    type Output = Vec3H;
    #[inline]
    fn sub(self, _rhs: Vec3H) -> Vec3H {
        Vec3H {
            x: self.x() - _rhs.x(),
            y: self.y() - _rhs.y(),
            z: self.z() - _rhs.z()
        }
    }
}


impl std::ops::AddAssign for Vec3H {
    #[inline]
    fn add_assign(&mut self, _rhs: Vec3H) {
        self[0] = self[0] + _rhs[0];
        self[1] = self[1] + _rhs[1];
        self[2] = self[2] + _rhs[2];
    }
}

impl std::ops::Index<usize> for Vec3H {
    type Output = f64;
    #[inline]
    fn index(&self,index: usize) -> &Self::Output {
        match index {
            0 => { &self.x },
            1 => { &self.y },
            2 => { &self.z }
            _ => { panic!("try index vec3 index more than 2" ) }
        }
    } 
}

impl std::ops::IndexMut<usize> for Vec3H {
    #[inline]
    fn index_mut(&mut self, index:usize) -> &mut f64 {
        match index {
            0 => { &mut self.x },
            1 => { &mut self.y },
            2 => { &mut self.z }
            _ => {panic!("try mut index vec3 index more than 2" );}
        }
    }
}

impl std::ops::Mul<f64> for Vec3H {
    type Output = Vec3H;
    #[inline]
    fn mul(self, _rsh: f64) -> Self::Output {
        Vec3H{
            x: self.x() * _rsh,
            y: self.y() * _rsh,
            z: self.z() * _rsh,
        }
    }
}

impl std::ops::Mul<Vec3H> for Vec3H {
    type Output = Vec3H;
    #[inline]
    fn mul(self, _rsh: Vec3H) -> Self::Output {
        Vec3H{
            x: self.x() * _rsh.x(),
            y: self.y() * _rsh.y(),
            z: self.z() * _rsh.z(),
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
            x: self[0] / _rsh,
            y: self[1] / _rsh,
            z: self[2] / _rsh
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3H {
    #[inline]
    fn div_assign(&mut self, _rsh: f64) {
        //  考虑优化用iter
        self.x = self.x / _rsh;
        self.y = self.y / _rsh;
        self.z = self.z / _rsh;
    }
}

impl std::fmt::Display for Vec3H {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3H: {} {} {}", self[0], self[1], self[2])
    }
}

impl PartialEq for Vec3H {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

pub fn cross(u: &Vec3H, v: &Vec3H) -> Vec3H {
    Vec3H::new(
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0]
    )
}

pub fn dot(u: &Vec3H, v: &Vec3H) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

#[inline]
pub fn random(min:f64, max: f64) -> Vec3H {
    assert!(min < max);
    Vec3H::new(
        Utility::get_random_range_f64(min, max),
        Utility::get_random_range_f64(min, max), 
        Utility::get_random_range_f64(min, max)
    )
}

pub fn random_in_unit_sphere() -> Vec3H {
    loop {
        let r = random(-1., 1.);
        if r.length() < 1.0 && !r.near_zero() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross() {
        assert_eq!(cross(&Vec3H::new(1., 0., 0.), &Vec3H::new(0., 1., 0.)), Vec3H::new(0., 0., 1.));
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&Vec3H::new(1., 0., 0.), &Vec3H::new(0., 1., 0.)), 0.);
        assert_eq!(dot(&Vec3H::new(1., 5., 0.), &Vec3H::new(0., 1., 4.)), 5.);
    }

    #[test]
    fn test_random_in_unit_sphere() {
        for i in 0..100 {
            let random_vec = random_in_unit_sphere();
            assert!(!random_vec.near_zero());
            assert!(random_vec.length() < 1.);
        }
    }

}