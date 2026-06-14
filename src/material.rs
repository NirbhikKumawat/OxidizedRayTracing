use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatteredRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}
pub trait Material: Send + Sync {
    fn scatter(&self, rin: &Ray, record: &HitRecord) -> Option<ScatteredRecord> {
        None
    }
}
pub struct Lambertian {
    pub albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, rin: &Ray, record: &HitRecord) -> Option<ScatteredRecord> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        let scattered = Ray::new(record.p, scatter_direction);
        let attenuation = self.albedo;
        Some(ScatteredRecord {
            attenuation,
            scattered,
        })
    }
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, rin: &Ray, record: &HitRecord) -> Option<ScatteredRecord> {
        let reflected = rin.direction().reflect(record.normal);
        let reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(record.p, reflected);
        let attenuation = self.albedo;
        Some(ScatteredRecord {
            attenuation,
            scattered,
        })
    }
}
