use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Arc<dyn Material>,
}
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        None
    }
}
