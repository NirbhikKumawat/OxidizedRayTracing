use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3{
    e: [f64; 3]
}
pub type Point3 = Vec3;
impl Vec3 {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{e: [x, y, z]}
    }
    #[inline]
    pub fn x(&self) -> f64 { self.e[0] }
    #[inline]
    pub fn y(&self) -> f64 { self.e[1] }
    #[inline]
    pub fn z(&self) -> f64 { self.e[2] }
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    #[inline]
    pub fn dot(self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
    #[inline]
    pub fn cross(self, other: Vec3) -> Self {
        Self::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }
    #[inline]
    pub fn unit_vector(self) -> Self {
        self/self.length()
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, other: Vec3) -> Self::Output {
        Self::new(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2])
    }
}
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, other: Vec3) -> Self::Output {
        Self::new(self.e[0] - other.x(), self.e[1] - other.y(), self.e[2] - other.z())
    }
}
impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        Self::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Self::Output {
        Self::new(self.e[0]*other.e[0], self.e[1]*other.e[1], self.e[2]*other.e[2])
    }
}
impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, other: f64) -> Self::Output {
        let r = 1.0 / other;
        Self::new(self.e[0] * r, self.e[1] * r, self.e[2] * r)
    }
}
impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        let r = 1.0 / other;
        self.e[0] *= r;
        self.e[1] *= r;
        self.e[2] *= r;
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}