use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Translate {
    pub offset: Vec3,
    object: Arc<dyn Hittable>,
    bbox: Aabb,
}
impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            bbox,
            offset,
            object,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let offset_ray =
            Ray::new_with_time(ray.origin() - self.offset, ray.direction(), ray.time());
        if let Some(record) = self.object.hit(&offset_ray, t) {
            return Some(HitRecord {
                p: record.p + self.offset,
                normal: record.normal,
                front_face: record.front_face,
                t: record.t,
                mat: record.mat,
                u: record.u,
                v: record.v,
            });
        }
        None
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
