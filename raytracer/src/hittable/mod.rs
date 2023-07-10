// 可交互物体抽象类
// 具体物体需要实现 Hittable 的 trait
// 以计算与光线碰撞的具体位置

pub mod aarect;
pub mod generator;
pub mod hittable_list;
pub mod instance;
pub mod moving_sphere;
pub mod rectbox;
pub mod sphere;
use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vec3,
        material: &'a dyn Material,
        ray: Ray,
    ) -> Self {
        let (normal, front_face) = HitRecord::set_face_normal(outward_normal, &ray);
        Self {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn set_face_normal(outward_normal: Vec3, ray: &Ray) -> (Vec3, bool) {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        if front_face {
            // ray is outside the sphere
            (outward_normal, true)
        } else {
            // ray is inside the sphere
            (-outward_normal, false)
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
