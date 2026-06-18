use crate::color::Color;
use crate::img::RtwImage;
use crate::interval::Interval;
use crate::vec3::{Point3};
use std::sync::Arc;
use crate::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
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
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
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
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
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
pub struct ImageTexture {
    image: RtwImage,
}
impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;

        let pixel = self.image.pixel_data(i, j);
        let color_scale = 1.0 / 255.0;
        color_scale * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}
pub struct NoiseTexture<const N: usize> {
    noise: Perlin<N>,
}
impl<const N: usize> NoiseTexture<N> {
    pub fn new() -> Self {
        let noise = Perlin::new();
        Self { noise }
    }
}
impl<const N: usize> Texture for NoiseTexture<N> {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)*self.noise.noise(p)
    }
}
