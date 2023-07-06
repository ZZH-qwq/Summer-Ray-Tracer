// 球体类

use crate::hittable::*;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let half_b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        // 按情况返回值
        if discriminant < 0.0 {
            // 无实根
            return None;
        }
        // 优先返回较小的根
        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            // 若不在范围内再对较大的根进行比较
            let root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                // 两个根均不在范围内
                None
            } else {
                let p = ray.at(root);
                // 较大的根
                Some(HitRecord::new(
                    p,
                    root,
                    (p - self.center) / self.radius,
                    ray,
                ))
            }
        } else {
            let p = ray.at(root);
            // 较小的根
            Some(HitRecord::new(
                p,
                root,
                (p - self.center) / self.radius,
                ray,
            ))
        }
    }
}
