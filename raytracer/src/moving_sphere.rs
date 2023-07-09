// 运动的球体类

use crate::aabb::AABB;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct MovingSphere<M: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
                    (p - self.center(ray.time)) / self.radius,
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
                (p - self.center(ray.time)) / self.radius,
                &self.material,
                ray,
            ))
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::surrounding_box(
            &AABB::new(
                self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
                self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
            ),
            &AABB::new(
                self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
                self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
            ),
        ))
    }
}
