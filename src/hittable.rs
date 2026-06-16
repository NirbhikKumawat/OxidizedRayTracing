use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
use crate::aabb::Aabb;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Arc<dyn Material>,
    pub front_face: bool,
}
pub trait Hittable: Send + Sync {
    fn hit(&self, _: &Ray, _: Interval) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self)->Aabb;
}
