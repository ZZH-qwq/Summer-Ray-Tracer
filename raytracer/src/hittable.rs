// 可交互物体抽象类
// 具体物体需要实现 Hittable 的 trait
// 以计算与光线碰撞的具体位置

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3,
        t: f64,
        outward_normal: Vec3,
        material: &'a dyn Material,
        ray: Ray,
    ) -> Self {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            // ray is outside the sphere
            outward_normal
        } else {
            // ray is inside the sphere
            -outward_normal
        };
        Self {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
