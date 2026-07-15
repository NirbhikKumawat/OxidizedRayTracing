use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{INFINITY, degrees_to_radians};
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub enum Axis {
    X,
    Y,
    Z,
}

pub struct Rotate {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
    axis: Axis,
}
impl Rotate {
    pub fn new(object: Arc<dyn Hittable>, angle: f64, axis: Axis) -> Rotate {
        let radians = degrees_to_radians(angle);
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let mut bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let tester = match axis {
                        Axis::X => {
                            let newy = cos_theta * y - sin_theta * z;
                            let newz = sin_theta * y + cos_theta * z;
                            Vec3::new(x, newy, newz)
                        }
                        Axis::Y => {
                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;
                            Vec3::new(newx, y, newz)
                        }
                        Axis::Z => {
                            let newx = cos_theta * x - sin_theta * y;
                            let newy = sin_theta * x + cos_theta * y;
                            Vec3::new(newx, newy, z)
                        }
                    };

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        bbox = Aabb::new_from_points(&min, &max);
        Self {
            object,
            bbox,
            sin_theta,
            cos_theta,
            axis,
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let (origin, direction) = match self.axis {
            Axis::X => (
                Point3::new(
                    ray.origin().x(),
                    self.cos_theta * ray.origin().y() + self.sin_theta * ray.origin().z(),
                    -self.sin_theta * ray.origin().y() + self.cos_theta * ray.origin().z(),
                ),
                Vec3::new(
                    ray.direction().x(),
                    self.cos_theta * ray.direction().y() + self.sin_theta * ray.direction().z(),
                    -self.sin_theta * ray.direction().y() + self.cos_theta * ray.direction().z(),
                ),
            ),
            Axis::Y => (
                Point3::new(
                    self.cos_theta * ray.origin().x() - self.sin_theta * ray.origin().z(),
                    ray.origin().y(),
                    self.sin_theta * ray.origin().x() + self.cos_theta * ray.origin().z(),
                ),
                Vec3::new(
                    self.cos_theta * ray.direction().x() - self.sin_theta * ray.direction().z(),
                    ray.direction().y(),
                    self.sin_theta * ray.direction().x() + self.cos_theta * ray.direction().z(),
                ),
            ),
            Axis::Z => (
                Point3::new(
                    self.cos_theta * ray.origin().x() + self.sin_theta * ray.origin().y(),
                    -self.sin_theta * ray.origin().x() + self.cos_theta * ray.origin().y(),
                    ray.origin().z(),
                ),
                Vec3::new(
                    self.cos_theta * ray.direction().x() + self.sin_theta * ray.direction().y(),
                    -self.sin_theta * ray.direction().x() + self.cos_theta * ray.direction().y(),
                    ray.direction().z(),
                ),
            ),
        };

        let rotated_ray = Ray::new_with_time(origin, direction, ray.time());

        if let Some(record) = self.object.hit(&rotated_ray, t) {
            let (p, normal) = match self.axis {
                Axis::X => (
                    Point3::new(
                        record.p.x(),
                        self.cos_theta * record.p.y() - self.sin_theta * record.p.z(),
                        self.sin_theta * record.p.y() + self.cos_theta * record.p.z(),
                    ),
                    Vec3::new(
                        record.normal.x(),
                        self.cos_theta * record.normal.y() - self.sin_theta * record.normal.z(),
                        self.sin_theta * record.normal.y() + self.cos_theta * record.normal.z(),
                    ),
                ),
                Axis::Y => (
                    Point3::new(
                        self.cos_theta * record.p.x() + self.sin_theta * record.p.z(),
                        record.p.y(),
                        -self.sin_theta * record.p.x() + self.cos_theta * record.p.z(),
                    ),
                    Vec3::new(
                        self.cos_theta * record.normal.x() + self.sin_theta * record.normal.z(),
                        record.normal.y(),
                        -self.sin_theta * record.normal.x() + self.cos_theta * record.normal.z(),
                    ),
                ),
                Axis::Z => (
                    Point3::new(
                        self.cos_theta * record.p.x() - self.sin_theta * record.p.y(),
                        self.sin_theta * record.p.x() + self.cos_theta * record.p.y(),
                        record.p.z(),
                    ),
                    Vec3::new(
                        self.cos_theta * record.normal.x() - self.sin_theta * record.normal.y(),
                        self.sin_theta * record.normal.x() + self.cos_theta * record.normal.y(),
                        record.normal.z(),
                    ),
                ),
            };

            return Some(HitRecord {
                p,
                normal,
                t: record.t,
                mat: record.mat,
                front_face: record.front_face,
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
