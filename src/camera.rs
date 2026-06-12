use std::io::Write;
use crate::color::{print_color, Color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::INFINITY;
use crate::vec3::{Point3, Vec3};

pub struct Camera{
    aspect_ratio : f64,
    image_width : u32,
    image_height : u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_data_u: Vec3,
    pixel_data_v: Vec3,
}
impl Camera {
    pub fn new(aspect_ratio:f64,image_width:u32,center:Point3,viewport_height:f64,focal_length:f64) -> Self{
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_data_u = viewport_u/image_width as f64;
        let pixel_data_v = viewport_v/image_height as f64;

        let viewport_upper_left = center - Vec3::new(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + 0.5* (pixel_data_u + pixel_data_v);

        Self{
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_data_u,
            pixel_data_v,
            pixel00_loc,
        }
    }
    pub fn render(&self,world:&HittableList,writer:&mut impl Write)->std::io::Result<()> {
        writeln!(writer,"P3\n{} {}\n255", self.image_width, self.image_height)?;
        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64*self.pixel_data_u)+(j as f64*self.pixel_data_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = ray_color(&r,world);
                print_color(&pixel_color,writer);
            }
        }
        writer.flush()?;
        Ok(())
    }
}
fn ray_color(ray:&Ray,world:&HittableList)->Color{
    if let Some(record) = world.hit(ray, Interval::new(0.0,INFINITY)) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5*(unit_direction.y()+1.0);
    (1.0-a)*Color::new(1f64,1f64,1f64)+a*Color::new(0.5,0.7,1f64)
}