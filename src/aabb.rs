use crate::interval::{EMPTY, Interval};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}
impl Default for Aabb {
    fn default() -> Self {
        Self {
            x: EMPTY,
            y: EMPTY,
            z: EMPTY,
        }
    }
}
impl Aabb {
    pub fn new(mut x: Interval, mut y: Interval, mut z: Interval) -> Self {
        Self::pad_to_minimums(&mut x);
        Self::pad_to_minimums(&mut y);
        Self::pad_to_minimums(&mut z);
        Self { x, y, z }
    }
    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        let mut x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let mut y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let mut z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };
        Self::pad_to_minimums(&mut x);
        Self::pad_to_minimums(&mut y);
        Self::pad_to_minimums(&mut z);
        Self { x, y, z }
    }
    pub fn new_from_boxes(box1: &Self, box2: &Self) -> Self {
        let mut x = Interval::new_from_intervals(&box1.x, &box2.x);
        let mut y = Interval::new_from_intervals(&box1.y, &box2.y);
        let mut z = Interval::new_from_intervals(&box1.z, &box2.z);

        Self::pad_to_minimums(&mut x);
        Self::pad_to_minimums(&mut y);
        Self::pad_to_minimums(&mut z);

        Self { x, y, z }
    }
    pub fn axis_interval(&self, axis: u8) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Wrong axis"),
        }
    }
    pub fn hit(&self, ray: &Ray, t: &mut Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_direction[axis as usize];

            let t0 = (ax.min - ray_origin[axis as usize]) * adinv;
            let t1 = (ax.max - ray_origin[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > t.min {
                    t.min = t0;
                }
                if t1 < t.max {
                    t.max = t1;
                }
            } else {
                if t1 > t.min {
                    t.min = t1;
                }
                if t0 < t.max {
                    t.max = t0;
                }
            }
            if t.max <= t.min {
                return false;
            }
        }
        true
    }
    pub fn longest_axis(&self) -> u8 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
    fn pad_to_minimums(interval: &mut Interval) {
        let delta = 0.0001;
        if interval.size() < delta {
            *interval = interval.expand(delta);
        }
    }
}
impl Add<Vec3> for Aabb {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Aabb::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}
