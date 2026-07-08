use OxidisedRayTracing::bvh::BvhNode;
use OxidisedRayTracing::camera::Camera;
use OxidisedRayTracing::color::Color;
use OxidisedRayTracing::constant_medium::ConstantMedium;
use OxidisedRayTracing::hittable_list::HittableList;
use OxidisedRayTracing::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use OxidisedRayTracing::quad::Quad;
use OxidisedRayTracing::rotate::{RotateY, RotateZ};
use OxidisedRayTracing::sphere::Sphere;
use OxidisedRayTracing::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use OxidisedRayTracing::translate::Translate;
use OxidisedRayTracing::utility::{random_double, random_f64};
use OxidisedRayTracing::vec3::{Point3, Vec3};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use std::time::Instant;

fn _bouncing_spheres() -> HittableList {
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
fn _earth() -> HittableList {
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
fn _checkered_spheres() -> HittableList {
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
fn _perlin_spheres() -> HittableList {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::<256>::new(4.0));
    let pertext = Arc::new(Lambertian::new_from_texture(pertext));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        pertext.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        pertext,
    )));
    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn _quads() -> HittableList {
    let mut world = HittableList::new();
    let left_red = Arc::new(Lambertian::new(&Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(&Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(&Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn _simple_light() -> HittableList {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::<256>::new(4.0));
    let pertext = Arc::new(Lambertian::new_from_texture(pertext));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        pertext.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        pertext,
    )));
    let diff_light = Arc::new(DiffuseLight::new_from_color(&Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diff_light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light,
    )));
    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn cornell_box() -> HittableList {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(&Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    let box1 = Arc::new(Quad::cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Arc::new(Quad::cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateZ::new(box2, -45.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn _cornell_smoke() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Arc::new(Quad::cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstantMedium::new_from_color(
        box1,
        0.01,
        &Color::new(0.0, 0.0, 0.0),
    )));
    let box2 = Arc::new(Quad::cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstantMedium::new_from_color(
        box2,
        0.01,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let bvh = BvhNode::new_from_hittable(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));
    world
}
fn _final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(&Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            // Assuming random_double(min, max) utility exists in your Rust codebase
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Quad::cuboid(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut world = HittableList::new();

    world.add(Arc::new(BvhNode::new_from_hittable(boxes1)));

    let light = Arc::new(DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(&Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_from_color(
        boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new_from_texture(Arc::new(ImageTexture::new(
        "./textures/earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Arc::new(NoiseTexture::<256>::new(0.2));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_from_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_vector3(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    let boxes2_bvh = Arc::new(BvhNode::new_from_hittable(boxes2));
    let boxes2_rotated = Arc::new(RotateY::new(boxes2_bvh, 15.0));
    let boxes2_translated = Arc::new(Translate::new(
        boxes2_rotated,
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    world.add(boxes2_translated);

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

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 40.0;
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    //let defocus_angle = 0.6;
    //let focus_dist = 10.0;
    let background = Color::new(0.0, 0.0, 0.0);

    let world = _final_scene();

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        background,
    );
    let start_time = Instant::now();
    camera.render(&world, &mut writer)?;
    let duration = start_time.elapsed();
    println!("Render finished in: {:?}", duration);
    Ok(())
}
