use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,
    material: Arc<dyn Material>,
    bbox: Aabb,
}
impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Arc<dyn Material>) -> Self {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(e2);
        let normal = normal.unit_vector();

        let mut min = Point3::new(
            v0.x().min(v1.x()).min(v2.x()),
            v0.y().min(v1.y()).min(v2.y()),
            v0.z().min(v1.z()).min(v2.z()),
        );
        let mut max = Point3::new(
            v0.x().max(v1.x()).max(v2.x()),
            v0.y().max(v1.y()).max(v2.y()),
            v0.z().max(v1.z()).max(v2.z()),
        );

        let delta = 0.0001;
        for c in 0..3 {
            if max[c] - min[c] < delta {
                min[c] -= delta / 2.0;
                max[c] += delta / 2.0;
            }
        }
        let bbox = Aabb::new_from_points(&min, &max);

        Self {
            v0,
            v1,
            v2,
            normal,
            material,
            bbox,
        }
    }
}
impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_int: Interval) -> Option<HitRecord> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let p = ray.direction().cross(e2);
        let det = e1.dot(p);

        let epsilon = 1e-8;
        if det > -epsilon && det < epsilon {
            return None;
        }

        let inv_det = 1.0 / det;
        let s1 = ray.origin() - self.v0;
        let u = inv_det * s1.dot(p);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s2 = s1.cross(e1);
        let v = inv_det * ray.direction().dot(s2);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * e2.dot(s2);
        if !t_int.surrounds(t) {
            return None;
        }
        let p = ray.point_at(t);
        let mut front_face = true;
        let mut normal = self.normal;
        if normal.dot(ray.direction()) >= 0.0 {
            normal = -normal;
            front_face = false;
        }
        Some(HitRecord {
            p,
            normal,
            front_face,
            t,
            u,
            v,
            mat: Arc::clone(&self.material),
        })
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
