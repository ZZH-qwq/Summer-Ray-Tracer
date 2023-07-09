// 柏林噪声

use crate::vec3::Vec3;
use rand::{seq::SliceRandom, Rng};
use std::vec::Vec;

pub struct Perlin {
    pub point_count: usize,
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: usize = 256;
        let mut rng = rand::thread_rng();
        let ranfloat: Vec<f64> = (0..point_count).map(|_| rng.gen()).collect();
        Self {
            point_count,
            ranfloat,
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

    fn trilinear_interp(c: Vec<Vec<Vec<f64>>>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
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

        let mut c = vec![vec![vec![0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = ((i + di) & 255) as usize;
                    let y = ((j + dj) & 255) as usize;
                    let z = ((k + dk) & 255) as usize;
                    c[di as usize][dj as usize][dk as usize] =
                        self.ranfloat[self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z]];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }
}
