use crate::aabb::Aabb;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval, UNIVERSE};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utility::{INFINITY, random_f64};
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }
    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_color(albedo)),
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(ray, UNIVERSE)?;
        let mut rec2 = self
            .boundary
            .hit(ray, Interval::new(rec1.t + 0.0001, INFINITY))?;

        if rec1.t < t.min {
            rec1.t = t.min;
        }
        if rec2.t > t.max {
            rec2.t = t.max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let at = rec1.t + hit_distance / ray_length;
        Some(HitRecord {
            t: at,
            p: ray.point_at(at),
            mat: self.phase_function.clone(),
            normal: Vec3::new(1.0, 0.0, 0.0),
            front_face: true,
            u: 0.0,
            v: 0.0,
        })
    }
    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
