use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t:Interval) -> Option<HitRecord> {
        let mut closest_so_far = t.max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray,Interval::new(t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }
}