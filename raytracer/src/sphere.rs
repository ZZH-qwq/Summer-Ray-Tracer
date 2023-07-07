// 球体类

use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
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
                    &self.material,
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
                &self.material,
                ray,
            ))
        }
    }
}
