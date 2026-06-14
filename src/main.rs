use OxidisedRayTracing::camera::Camera;
use OxidisedRayTracing::color::Color;
use OxidisedRayTracing::hittable_list::HittableList;
use OxidisedRayTracing::material::{Dielectric, Lambertian, Metal};
use OxidisedRayTracing::sphere::Sphere;
use OxidisedRayTracing::vec3::Point3;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        Point3::new(0.0, 0.0, 0.0),
        viewport_height,
        focal_length,
        samples_per_pixel,
        max_depth,
    );
    camera.render(&world, &mut writer)?;
    Ok(())
}
