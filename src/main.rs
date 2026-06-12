use std::env;
use std::fs::File;
use std::io::{BufWriter,Write};
use OxidisedRayTracing::color::{print_color, Color};
use OxidisedRayTracing::ray::Ray;
use OxidisedRayTracing::vec3::{Point3, Vec3};

fn ray_color(ray:&Ray)->Color{
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t>0.0{
        let n = (ray.point_at(t)-Vec3::new(0.0,0.0,-1.0)).unit_vector();
        return 0.5*Color::new(n.x()+1f64,n.y()+1f64,n.z()+1f64);
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5*(unit_direction.y()+1.0);
    (1.0-a)*Color::new(1f64,1f64,1f64)+a*Color::new(0.5,0.7,1f64)
}
fn hit_sphere(center:&Point3, radius:f64, ray: &Ray) ->f64{
    let oc = *center-ray.origin();
    let a = ray.direction().length_squared();
    let h = oc.dot(ray.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h-a*c;
    if discriminant < 0.0 {
        -1.0
    }else{
        (h-discriminant.sqrt())/(a)
    }

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
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);

    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_data_u = viewport_u/image_width as f64;
    let pixel_data_v = viewport_v/image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5* (pixel_data_u + pixel_data_v);


    writeln!(writer,"P3\n{} {}\n255", image_width, image_height)?;
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64*pixel_data_u)+(j as f64*pixel_data_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            print_color(&pixel_color, &mut writer);
        }
    }
    writer.flush()?;
    Ok(())
}
