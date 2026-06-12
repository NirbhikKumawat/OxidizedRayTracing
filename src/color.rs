use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;
pub fn print_color(color: &Color,buffer:&mut impl Write) {
    let r = color.x();
    let g = color.y();
    let b = color.z();
    
    let intensity = Interval::new(0.0,0.999);

    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    writeln!(buffer, "{} {} {}", ir, ig, ib).unwrap();
}