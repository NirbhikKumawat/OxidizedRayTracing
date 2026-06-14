use crate::interval::Interval;
use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;
pub fn print_color(color: &Color, buffer: &mut impl Write) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);

    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    writeln!(buffer, "{} {} {}", ir, ig, ib).unwrap();
}
#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0f64
}
