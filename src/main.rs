use OxidisedRayTracing::bvh::BvhNode;
use OxidisedRayTracing::camera::Camera;
use OxidisedRayTracing::color::Color;
use OxidisedRayTracing::hittable_list::HittableList;
use OxidisedRayTracing::material::{Dielectric, Lambertian, Metal};
use OxidisedRayTracing::sphere::Sphere;
use OxidisedRayTracing::texture::{CheckerTexture, ImageTexture};
use OxidisedRayTracing::utility::{random_double, random_f64};
use OxidisedRayTracing::vec3::{Point3, Vec3};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

fn bouncing_spheres() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Arc::new(Lambertian::new_from_texture(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let sphere_material = Arc::new(Lambertian::new(
                        &(Color::random_vector() * Color::random_vector()),
                    ));
                    let center2 = center + Vec3::new(0.0, random_f64(), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let sphere_material = Arc::new(Metal::new(
                        Color::random_vector3(0.5, 1.0),
                        random_double(0.0, 0.5),
                    ));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_left = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_left,
    )));

    let material_center = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_center,
    )));

    let material_right = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_right,
    )));

    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn earth() -> HittableList {
    let mut world = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("./textures/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    world.add(globe);
    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn checkered_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material = Arc::new(Lambertian::new_from_texture(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material,
    )));
    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
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
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let world = earth();

    let camera = Camera::new_with_defocus(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_dist,
    );
    camera.render(&world, &mut writer)?;
    Ok(())
}
