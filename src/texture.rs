use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Vec3 {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct SolidColor {
    albedo: Color,
}
impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::new(Color::new(red, green, blue))
    }
}
impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Vec3 {
        self.albedo
    }
}
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        let inv_scale = 1.0 / scale;
        let even = even.clone();
        let odd = odd.clone();
        Self {
            inv_scale,
            even,
            odd,
        }
    }
    pub fn from_colors(scale: f64, even: Color, odd: Color) -> Self {
        let c1 = Arc::new(SolidColor::new(even));
        let c2 = Arc::new(SolidColor::new(odd));
        Self::new(scale, c1, c2)
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Vec3 {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;
        let is_even = (x + y + z) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
