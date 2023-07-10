// 球体类

use crate::aabb::Aabb;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

// 轴对齐的x-y平面
#[derive(Debug, Copy, Clone)]
pub struct XYRect<M: Material> {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub k: f64,
    pub material: M,
}

impl<M: Material> XYRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: M) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for XYRect<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord::new(
            ray.at(t),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            Vec3::new(0.0, 0.0, 1.0),
            &self.material,
            ray,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
