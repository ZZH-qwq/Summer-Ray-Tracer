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

// 轴对齐的x-z平面
#[derive(Debug, Copy, Clone)]
pub struct XZRect<M: Material> {
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub k: f64,
    pub material: M,
}

impl<M: Material> XZRect<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            x0,
            z0,
            x1,
            z1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for XZRect<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            ray.at(t),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            Vec3::new(0.0, 1.0, 0.0),
            &self.material,
            ray,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

// 轴对齐的y-z平面
#[derive(Debug, Copy, Clone)]
pub struct YZRect<M: Material> {
    pub y0: f64,
    pub z0: f64,
    pub y1: f64,
    pub z1: f64,
    pub k: f64,
    pub material: M,
}

impl<M: Material> YZRect<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            y0,
            z0,
            y1,
            z1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for YZRect<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            ray.at(t),
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            Vec3::new(1.0, 0.0, 0.0),
            &self.material,
            ray,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
