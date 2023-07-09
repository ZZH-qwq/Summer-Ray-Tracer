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
        let mut p: Vec<usize> = (0..len).map(|i| i).collect();
        let mut rng = rand::thread_rng();
        p.shuffle(&mut rng);
        p
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
