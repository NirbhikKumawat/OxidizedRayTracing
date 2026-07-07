use crate::utility::random_int;
use crate::vec3::{Point3, Vec3};

pub struct Perlin<const N: usize> {
    rand_vec: [Vec3; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}
impl<const N: usize> Default for Perlin<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        let mut rand_vec = [Vec3::default(); N];
        for i in 0..N {
            rand_vec[i] = Vec3::random_vector3(-1.0, 1.0);
        }
        Self {
            perm_x,
            perm_y,
            perm_z,
            rand_vec,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = ((i + di as i32) & 255) as usize;
                    let y = ((j + dj as i32) & 255) as usize;
                    let z = ((k + dk as i32) & 255) as usize;
                    let perm_idx = self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z];

                    c[di][dj][dk] = self.rand_vec[perm_idx];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }
    fn perlin_generate_perm() -> [usize; N] {
        let mut p = [0; N];
        for i in 0..N {
            p[i] = i;
        }
        for i in (0..N).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
        p
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * Vec3::dot(c[i][j][k], weight);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
