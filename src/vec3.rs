use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, vec: Vec3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn length(&self) -> f64 {
        (self.dot(*self)).sqrt()
    }

    pub fn cross(&self, vec: Vec3) -> Vec3 {
        Vec3::new(
            self.y * vec.z - self.z * vec.y,
            self.z * vec.x - self.x * vec.z,
            self.x * vec.y - self.y * vec.x,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}
// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, vec: Vec3) {
        *self = *self + vec;
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x - vec.x, self.y - vec.y, self.z - vec.z)
    }
}
// Vec3 -= Vec3
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, vec: Vec3) {
        *self = *self - vec;
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self.x * vec.x, self.y * vec.y, self.z * vec.z)
    }
}
// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self * vec.x, self * vec.y, self * vec.z)
    }
}
// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3; 

    fn mul(self, val: f64) -> Vec3 {
        Vec3::new(self.x * val, self.y * val, self.z * val)
    }
}
// Vec3 *= f64
// Here you use no Output because it ends up being Vec3 = Vec3 * f64 and Vec3 is already defined as the type of output
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, val: f64) -> Vec3 {
        Vec3::new(self.x / val, self.y / val, self.z / val)
    }
}
// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self * -1.0
    }
}

pub type Point3 = Vec3;