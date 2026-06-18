use crate::utility::{random_f64, random_int};
use crate::vec3::{Point3};

pub struct Perlin<const N: usize> {
    rand_float: [f64; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}
impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        let mut rand_float = [0.0; N];
        for i in 0..N {
            rand_float[i] = random_f64();
        }
        Self{
            perm_x,
            perm_y,
            perm_z,
            rand_float,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = ((i + di as i32) & 255) as usize;
                    let y = ((j + dj as i32) & 255) as usize;
                    let z = ((k + dk as i32) & 255) as usize;
                    c[di][dj][dk] = self.rand_float[x^y^z];
                }
            }
        }
        Self::trilinear_interp(c,u,v,w)
    }
    fn perlin_generate_perm() -> [usize; N] {
        let mut p = [0;N];
        for i in 0..N {
            p[i]=i;
        }
        for i in (0..N).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
        p
    }
    fn trilinear_interp(c:[[[f64;2];2];2],u:f64,v:f64,w:f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum+=(i as f64*u+(1.0-i as f64)*(1.0-u)) * (j as f64*v+(1.0-j as f64)*(1.0-v)) * (k as f64*w+(1.0-k as f64)*(1.0-w)) * c[i][j][k];
                }
            }
        }
        accum
    }
}