use crate::utility::{random_double, random_f64};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}
pub type Point3 = Vec3;
impl Vec3 {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }
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
        self / self.length()
    }
    pub fn random_vector() -> Self {
        Self {
            e: [random_f64(), random_f64(), random_f64()],
        }
    }
    pub fn random_vector3(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_double(min, max),
                random_double(min, max),
                random_double(min, max),
            ],
        }
    }
    #[inline]
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_vector3(-1.0, 1.0);
            let lensq = p.length_squared();
            if lensq <= 1.0 && 1e-160 < lensq {
                return p / lensq.sqrt();
            }
        }
    }
    #[inline]
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
    #[inline]
    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }
    #[inline]
    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_parallel = (1.0 - r_perp.length_squared()).abs().sqrt();
        let r_parallel = -r_parallel * normal;
        r_parallel + r_perp
    }
    #[inline]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}
impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, other: Vec3) -> Self::Output {
        Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
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
        Self::new(
            self.e[0] - other.x(),
            self.e[1] - other.y(),
            self.e[2] - other.z(),
        )
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
        Self::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
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
