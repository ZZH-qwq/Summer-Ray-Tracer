// 柏林噪声

use crate::vec3::Vec3;
use rand::seq::SliceRandom;
use std::vec::Vec;

pub struct Perlin {
    pub point_count: usize,
    pub ranvec: Vec<Vec3>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: usize = 256;
        let ranvec: Vec<Vec3> = (0..point_count)
            .map(|_| Vec3::random_unit_vector())
            .collect();
        Self {
            point_count,
            ranvec,
            perm_x: Perlin::perlin_generate_perm(point_count),
            perm_y: Perlin::perlin_generate_perm(point_count),
            perm_z: Perlin::perlin_generate_perm(point_count),
        }
    }

    fn perlin_generate_perm(len: usize) -> Vec<usize> {
        let mut p: Vec<usize> = (0..len).collect();
        let mut rng = rand::thread_rng();
        p.shuffle(&mut rng);
        p
    }

    fn perlin_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, it3) in c.iter().enumerate().take(2) {
            for (j, it2) in it3.iter().enumerate().take(2) {
                for (k, it) in it2.iter().enumerate().take(2) {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * Vec3::dot(*it, weight_v);
                }
            }
        }
        accum
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = vec![vec![vec![Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = ((i + di) & 255) as usize;
                    let y = ((j + dj) & 255) as usize;
                    let z = ((k + dk) & 255) as usize;
                    c[di as usize][dj as usize][dk as usize] =
                        self.ranvec[self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z]];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }
}
