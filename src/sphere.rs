use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
use crate::aabb::Aabb;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = radius*Vec3::new(1.0,1.0,1.0);
        let bbox = Aabb::new_from_points(&(center - rvec), &(center + rvec));
        Self {
            center: Ray::new(center, Vec3::default()),
            radius,
            mat,
            bbox
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        let rvec = radius*Vec3::new(1.0,1.0,1.0);
        let center =  Ray::new(center1, center2 - center1);
        let box1 = Aabb::new_from_points(&(center.point_at(0.0) - rvec), &(center.point_at(0.0) + rvec));
        let box2 = Aabb::new_from_points(&(center.point_at(1.0) - rvec), &(center.point_at(1.0) + rvec));
        let bbox = Aabb::new_from_boxes(&box1, &box2);
        Self {
            center,
            radius,
            mat,
            bbox
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let current_center = self.center.point_at(ray.time());
        let oc = current_center - ray.origin();
        let a = ray.direction().length_squared();
        let h = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = ray.point_at(t);
        let mut normal = (p - current_center) / self.radius;
        let mut front_face = true;
        if normal.dot(ray.direction()) >= 0.0 {
            normal = -normal;
            front_face = false;
        }
        let mat = Arc::clone(&self.mat);
        Some(HitRecord {
            t,
            p,
            normal,
            mat,
            front_face,
        })
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
