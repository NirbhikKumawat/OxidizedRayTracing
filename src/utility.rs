pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
#[inline]
pub fn random_f64() -> f64 {
    rand::random::<f64>()
}
#[inline]
pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
#[inline]
pub fn random_int(min: i32, max: i32) -> i32 {
    random_double(min as f64, max as f64+1.0) as i32
}
