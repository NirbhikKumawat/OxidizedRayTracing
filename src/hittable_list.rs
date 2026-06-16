use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval, EMPTY};
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox:Aabb
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![],bbox: Aabb::new(EMPTY,EMPTY,EMPTY) }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = Aabb::new_from_boxes(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = t.max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, Interval::new(t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }
    fn bounding_box(&self) -> Aabb {
       self.bbox
    }
}
