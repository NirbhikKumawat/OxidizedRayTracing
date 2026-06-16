use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone,Copy)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}
impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn new_from_points(a:&Point3,b:&Point3) -> Self {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        }else{
            Interval::new(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        }else {
            Interval::new(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        }else{
            Interval::new(b[2], a[2])
        };
        Self { x, y, z }
    }
    pub fn new_from_boxes(box1:&Self, box2:&Self) -> Self {
        Self{
            x:Interval::new_from_intervals(&box1.x, &box2.x),
            y:Interval::new_from_intervals(&box1.y, &box2.y),
            z:Interval::new_from_intervals(&box1.z, &box2.z)
        }
    }
    pub fn axis_interval(&self,axis:u8)->Interval{
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Wrong axis"),
        }
    }
    pub fn hit(&self, ray:&Ray, t: &mut Interval) ->bool{
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0/ray_direction[axis as usize];

            let t0 = (ax.min-ray_origin[axis as usize])*adinv;
            let t1 = (ax.max-ray_origin[axis as usize])*adinv;

            if t0<t1 {
                if t0>t.min{
                    t.min=t0;
                }
                if t1<t.max{
                    t.max=t1;
                }
            }else{
                if t1>t.min{
                    t.min=t1;
                }
                if t0<t.max{
                    t.max=t0;
                }
            }
            if t.max<=t.min{
                return false;
            }
        }
        true
    }
}