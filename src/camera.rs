use crate::color::{Color, print_color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{INFINITY, random_f64, degrees_to_radians};
use crate::vec3::{Point3, Vec3};
use std::io::Write;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_data_u: Vec3,
    pixel_data_v: Vec3,
    pixel_samples_scale: f64,
    samples_per_pixel: u32,
    max_depth: u32,
}
impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let center = look_from;

        let focal_length = (look_from-look_at).length();

        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h *focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from-look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_data_u = viewport_u / image_width as f64;
        let pixel_data_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (focal_length*w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_data_u + pixel_data_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_data_u,
            pixel_data_v,
            pixel00_loc,
            pixel_samples_scale,
            samples_per_pixel,
            max_depth,
        }
    }
    pub fn render(&self, world: &HittableList, writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(
            writer,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;
        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, world, self.max_depth);
                }
                pixel_color *= self.pixel_samples_scale;
                print_color(&pixel_color, writer);
            }
        }
        writer.flush()?;
        Ok(())
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_data_u)
            + ((j as f64 + offset.y()) * self.pixel_data_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
}
fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(record) = world.hit(ray, Interval::new(0.001, INFINITY)) {
        if let Some(scat) = record.mat.scatter(ray, &record) {
            return scat.attenuation * ray_color(&scat.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1f64, 1f64, 1f64) + a * Color::new(0.5, 0.7, 1f64)
}
fn sample_square() -> Vec3 {
    Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}
