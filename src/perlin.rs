use crate::utility::{random_f64, random_int};
use crate::vec3::Point3;

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
        let i = (4.0 * p.x()).floor() as i32;
        let j = (4.0 * p.y()).floor() as i32;
        let k = (4.0 * p.z()).floor() as i32;

        let mask = (N - 1) as i32;
        let i = (i & mask) as usize;
        let j = (j & mask) as usize;
        let k = (k & mask) as usize;

        let index_x = self.perm_x[i];
        let index_y = self.perm_y[(index_x + j) & (N - 1)];
        let index_z = self.perm_z[(index_y + k) & (N - 1)];

        self.rand_float[index_z]
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
}