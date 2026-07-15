use crate::color::{Color, print_color};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{INFINITY, degrees_to_radians, random_f64};
use crate::vec3::{Point3, Vec3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_data_u: Vec3,
    pixel_data_v: Vec3,
    pixel_samples_scale: f64,
    samples_per_pixel: u32,
    inv_sqrt_samples_per_pixel: f64,
    sqrt_spp: u32,
    max_depth: u32,
    defocus: bool,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: f64,
    background: Color,
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
        background: Color,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let center = look_from;

        let focal_length = (look_from - look_at).length();

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_data_u = viewport_u / image_width as f64;
        let pixel_data_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_data_u + pixel_data_v);

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale = 1.0 / (sqrt_spp as f64 * sqrt_spp as f64);
        let inv_sqrt_samples_per_pixel = 1.0 / (sqrt_spp as f64);
        let defocus = false;
        Self {
            image_width,
            image_height,
            center,
            pixel_data_u,
            pixel_data_v,
            pixel00_loc,
            pixel_samples_scale,
            inv_sqrt_samples_per_pixel,
            sqrt_spp,
            samples_per_pixel,
            max_depth,
            defocus,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
            defocus_angle: 0.0,
            background,
        }
    }
    pub fn new_with_defocus(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        defocus_dist: f64,
        background: Color,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let center = look_from;

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale = 1.0 / (sqrt_spp as f64 * sqrt_spp as f64);
        let inv_sqrt_samples_per_pixel = 1.0 / (sqrt_spp as f64);

        let defocus = true;

        let focal_length = defocus_dist;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_data_u = viewport_u / image_width as f64;
        let pixel_data_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_data_u + pixel_data_v);

        let defocus_radius = focal_length * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_data_u,
            pixel_data_v,
            pixel_samples_scale,
            inv_sqrt_samples_per_pixel,
            samples_per_pixel,
            sqrt_spp,
            max_depth,
            defocus,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
            background,
        }
    }
    fn multi_render(&self, world: &HittableList) -> Vec<Color> {
        let total_pixels = self.image_width * self.image_height;
        eprintln!(
            "Rendering {} pixels across multiple threads...",
            total_pixels
        );

        let pixels_renderd = AtomicUsize::new(0);

        let pixels: Vec<Color> = (0..total_pixels)
            .into_par_iter()
            .map(|idx| {
                let j = idx / self.image_width;
                let i = idx % self.image_width;

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, si, sj);
                        pixel_color += self.ray_color(&r, world, self.max_depth);
                    }
                }
                pixel_color *= self.pixel_samples_scale;
                let completed = pixels_renderd.fetch_add(1, Ordering::Relaxed) + 1;
                if completed.is_multiple_of(self.image_width as usize) {
                    let scanlines_remaining =
                        self.image_height as usize - (completed / self.image_width as usize);

                    eprint!("\rScanlines remaining: {}    ", scanlines_remaining);
                }

                pixel_color
            })
            .collect();

        pixels
    }

    pub fn render(&self, world: &HittableList, writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(
            writer,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;
        let pixels = self.multi_render(world);
        for pixel in pixels {
            print_color(&pixel, writer);
        }
        writer.flush()?;
        Ok(())
    }
    fn get_ray(&self, i: u32, j: u32, si: u32, sj: u32) -> Ray {
        let offset = self.sample_square_stratified(si, sj);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_data_u)
            + ((j as f64 + offset.y()) * self.pixel_data_v);
        let ray_origin = if self.defocus && self.defocus_angle > 0.0 {
            self.defocus_disk_sample()
        } else {
            self.center
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();
        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }
    pub fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(record) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            if let Some(scat) = record.mat.scatter(ray, &record) {
                scat.attenuation * self.ray_color(&scat.scattered, world, depth - 1)
                    + record.mat.emitted(record.u, record.v, &record.p)
            } else {
                record.mat.emitted(record.u, record.v, &record.p)
            }
        } else {
            self.background
        }
    }
    fn sample_square_stratified(&self, si: u32, sj: u32) -> Vec3 {
        let px = ((si as f64 + random_f64()) * self.inv_sqrt_samples_per_pixel) - 0.5;
        let py = ((sj as f64 + random_f64()) * self.inv_sqrt_samples_per_pixel) - 0.5;
        Vec3::new(px, py, 0.0)
    }
}
