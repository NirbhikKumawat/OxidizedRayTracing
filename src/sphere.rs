use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self{
            center,
            radius,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t:Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (h-sqrt_d) / a;
        if !t.surrounds(root) {
            root = (h+sqrt_d) / a;
            if !t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = ray.point_at(t);
        let mut normal = (p - self.center) / self.radius;
        if normal.dot(ray.direction()) >= 0.0 {
            normal = -normal;
        }
        Some(HitRecord{
            t,p,normal
        })

    }
}