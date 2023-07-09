// 轴对齐边界框 Axis-Aligned Bounding Boxes

use crate::vec3::Vec3;
use std::mem;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Vec3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        Self {
            min: small,
            max: big,
        }
    }

    pub fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let invd = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * invd;
            let mut t1 = (self.max[i] - ray.origin[i]) * invd;
            if invd < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            let tmin = t0.max(t_min);
            let tmax = t1.min(t_max);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
