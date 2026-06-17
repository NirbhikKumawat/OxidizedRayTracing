use std::sync::Arc;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BvhNode {
    bbox: Aabb,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}
impl BvhNode {
    pub fn new_from_hittable(list: HittableList)->Self{
        Self::new(&mut list.objects.clone(), 0, list.objects.len() as u32)
    }
    pub fn new(objects:& mut Vec<Arc<dyn Hittable>>, start: u32, end: u32) -> Self {
        let mut bbox = Aabb::default();
        for object_index in start..end {
            bbox = Aabb::new_from_boxes(&bbox,&objects[object_index as usize].bounding_box());
        }
        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!(),
        };
        let object_span = end - start;
        let (left,right) = match object_span{
            1 => (objects[start as usize].clone(), objects[start as usize].clone()),
            2 => (objects[start as usize].clone(), objects[start as usize + 1].clone()),
            _ => {
                let slice = &mut objects[start as usize..end as usize];
                slice.sort_by(|a,b|{
                    if comparator(a.clone(), b.clone()){
                        std::cmp::Ordering::Less
                    }else{
                        std::cmp::Ordering::Greater
                    }
                });
                let mid = start + (object_span/2);
                (Arc::new(Self::new(objects,start,mid)) as Arc<dyn Hittable>, Arc::new(Self::new(objects,mid,end)) as Arc<dyn Hittable>)
            }
        };
        Self{
            left,
            right,
            bbox,
        }
    }
}
impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, mut t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray,&mut t){
            return None;
        }
        let hit_left = self.left.hit(ray, t);
        let (right,hit_left) = if let Some(left) = hit_left {
             (left.t,Some(left))
        }else{
            (t.max,None)
        };
        let hit_right = self.right.hit(ray,Interval::new(t.min,right));
        hit_right.or(hit_left)
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
fn box_compare (a:Arc<dyn Hittable> ,b:Arc<dyn Hittable>, axis_index:u8)->bool{
    let a_axis = a.bounding_box().axis_interval(axis_index);
    let b_axis = b.bounding_box().axis_interval(axis_index);
    a_axis.min<b_axis.min
}
fn box_x_compare(a:Arc<dyn Hittable>,b:Arc<dyn Hittable>)->bool{
    box_compare(a,b,0)
}
fn box_y_compare(a:Arc<dyn Hittable>,b:Arc<dyn Hittable>)->bool{
    box_compare(a,b,1)
}
fn box_z_compare(a:Arc<dyn Hittable>,b:Arc<dyn Hittable>)->bool{
    box_compare(a,b,2)
}
