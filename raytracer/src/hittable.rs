use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(point: Vec3, t: f64, outward_normal: Vec3, ray: Ray) -> Self {
        let normal = if Vec3::dot(ray.direction, outward_normal) > 0.0 {
            // ray is inside the sphere
            -outward_normal
        } else {
            // ray is outside the sphere
            outward_normal
        };
        Self { point, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
