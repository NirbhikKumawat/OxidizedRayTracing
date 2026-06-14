use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utility::random_f64;
use crate::vec3::Vec3;

pub struct ScatteredRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}
pub trait Material: Send + Sync {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatteredRecord> {
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
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<ScatteredRecord> {
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
        if scattered.direction().dot(record.normal) > 0.0 {
            Some(ScatteredRecord {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
pub struct Dielectric {
    pub refractive_index: f64,
}
impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self {
            refractive_index: index,
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(&self, rin: &Ray, record: &HitRecord) -> Option<ScatteredRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let cos_theta = rin.direction().dot(-record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let unit_direction = rin.direction().unit_vector();
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_f64() {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, ri)
        };
        let scattered = Ray::new(record.p, direction);
        Some(ScatteredRecord {
            attenuation,
            scattered,
        })
    }
}
