use OxidisedRayTracing::camera::Camera;
use OxidisedRayTracing::hittable_list::HittableList;
use OxidisedRayTracing::sphere::Sphere;
use OxidisedRayTracing::vec3::Point3;
use std::env;
use std::fs::File;
use std::io::BufWriter;

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
    let samples_per_pixel = 10;
    let max_depth = 50;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0f64, 0f64, -1f64), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0f64, -100.5, -1f64),
        100.0,
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
