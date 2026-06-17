use crate::aabb::Aabb;
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
    pub front_face: bool,
    pub(crate) u: f64,
    pub(crate) v: f64,
}
pub trait Hittable: Send + Sync {
    fn hit(&self, _: &Ray, _: Interval) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self) -> Aabb;
}
