use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}
impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox = Self::set_bbox(q, u, v);
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        Self {
            q,
            u,
            v,
            bbox,
            mat,
            normal,
            d,
            w,
        }
    }
    fn set_bbox(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let b1 = Aabb::new_from_points(&q, &(q + u + v));
        let b2 = Aabb::new_from_points(&(q + u), &(q + v));
        Aabb::new_from_boxes(&b1, &b2)
    }
}
impl Hittable for Quad {
    fn hit(&self, ray: &Ray, rt: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction());
        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot(ray.origin())) / denom;
        if !rt.contains(t) {
            return None;
        }
        let p = ray.point_at(t);
        let planar_hitpt_vector = p - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));
        if !is_interior(alpha, beta) {
            return None;
        }
        let mat = self.mat.clone();
        let mut front_face = true;
        let mut normal = self.normal;
        if normal.dot(ray.direction()) >= 0.0 {
            normal = -normal;
            front_face = false;
        }
        Some(HitRecord {
            t,
            p,
            mat,
            normal,
            front_face,
            u: alpha,
            v: beta,
        })
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
fn is_interior(a: f64, b: f64) -> bool {
    let unit_interval = Interval::new(0.0, 1.0);
    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }
    true
}
